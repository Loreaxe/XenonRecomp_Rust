pub fn sub_832B8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8718 size=20
    let mut pc: u32 = 0x832B8718;
    'dispatch: loop {
        match pc {
            0x832B8718 => {
    //   block [0x832B8718..0x832B872C)
	// 832B8718: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B871C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8720: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8724: 916A6A34  stw r11, 0x6a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(27188 as u32), ctx.r[11].u32 ) };
	// 832B8728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8730 size=20
    let mut pc: u32 = 0x832B8730;
    'dispatch: loop {
        match pc {
            0x832B8730 => {
    //   block [0x832B8730..0x832B8744)
	// 832B8730: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8734: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8738: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B873C: 916A6B48  stw r11, 0x6b48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(27464 as u32), ctx.r[11].u32 ) };
	// 832B8740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8748 size=20
    let mut pc: u32 = 0x832B8748;
    'dispatch: loop {
        match pc {
            0x832B8748 => {
    //   block [0x832B8748..0x832B875C)
	// 832B8748: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B874C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8750: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8754: 916A6C5C  stw r11, 0x6c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(27740 as u32), ctx.r[11].u32 ) };
	// 832B8758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8760 size=20
    let mut pc: u32 = 0x832B8760;
    'dispatch: loop {
        match pc {
            0x832B8760 => {
    //   block [0x832B8760..0x832B8774)
	// 832B8760: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8764: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8768: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B876C: 916A6D70  stw r11, 0x6d70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28016 as u32), ctx.r[11].u32 ) };
	// 832B8770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8778 size=20
    let mut pc: u32 = 0x832B8778;
    'dispatch: loop {
        match pc {
            0x832B8778 => {
    //   block [0x832B8778..0x832B878C)
	// 832B8778: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B877C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8780: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8784: 916A6E84  stw r11, 0x6e84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28292 as u32), ctx.r[11].u32 ) };
	// 832B8788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8790 size=20
    let mut pc: u32 = 0x832B8790;
    'dispatch: loop {
        match pc {
            0x832B8790 => {
    //   block [0x832B8790..0x832B87A4)
	// 832B8790: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8794: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8798: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B879C: 916A6F98  stw r11, 0x6f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28568 as u32), ctx.r[11].u32 ) };
	// 832B87A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B87A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B87A8 size=20
    let mut pc: u32 = 0x832B87A8;
    'dispatch: loop {
        match pc {
            0x832B87A8 => {
    //   block [0x832B87A8..0x832B87BC)
	// 832B87A8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B87AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B87B0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B87B4: 916A70AC  stw r11, 0x70ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28844 as u32), ctx.r[11].u32 ) };
	// 832B87B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B87C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B87C0 size=20
    let mut pc: u32 = 0x832B87C0;
    'dispatch: loop {
        match pc {
            0x832B87C0 => {
    //   block [0x832B87C0..0x832B87D4)
	// 832B87C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B87C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B87C8: 396B71C0  addi r11, r11, 0x71c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29120;
	// 832B87CC: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B87D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B87D8 size=20
    let mut pc: u32 = 0x832B87D8;
    'dispatch: loop {
        match pc {
            0x832B87D8 => {
    //   block [0x832B87D8..0x832B87EC)
	// 832B87D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B87DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B87E0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B87E4: 916A75C8  stw r11, 0x75c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30152 as u32), ctx.r[11].u32 ) };
	// 832B87E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B87F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B87F0 size=20
    let mut pc: u32 = 0x832B87F0;
    'dispatch: loop {
        match pc {
            0x832B87F0 => {
    //   block [0x832B87F0..0x832B8804)
	// 832B87F0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B87F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B87F8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B87FC: 916A76DC  stw r11, 0x76dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30428 as u32), ctx.r[11].u32 ) };
	// 832B8800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8808 size=20
    let mut pc: u32 = 0x832B8808;
    'dispatch: loop {
        match pc {
            0x832B8808 => {
    //   block [0x832B8808..0x832B881C)
	// 832B8808: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B880C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8810: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8814: 916A7838  stw r11, 0x7838(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30776 as u32), ctx.r[11].u32 ) };
	// 832B8818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8820 size=20
    let mut pc: u32 = 0x832B8820;
    'dispatch: loop {
        match pc {
            0x832B8820 => {
    //   block [0x832B8820..0x832B8834)
	// 832B8820: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B8824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8828: 396B7950  addi r11, r11, 0x7950
	ctx.r[11].s64 = ctx.r[11].s64 + 31056;
	// 832B882C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8838 size=20
    let mut pc: u32 = 0x832B8838;
    'dispatch: loop {
        match pc {
            0x832B8838 => {
    //   block [0x832B8838..0x832B884C)
	// 832B8838: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B883C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8840: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8844: 916A8570  stw r11, -0x7a90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31376 as u32), ctx.r[11].u32 ) };
	// 832B8848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8850 size=20
    let mut pc: u32 = 0x832B8850;
    'dispatch: loop {
        match pc {
            0x832B8850 => {
    //   block [0x832B8850..0x832B8864)
	// 832B8850: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8858: 396B8168  addi r11, r11, -0x7e98
	ctx.r[11].s64 = ctx.r[11].s64 + -32408;
	// 832B885C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8868 size=20
    let mut pc: u32 = 0x832B8868;
    'dispatch: loop {
        match pc {
            0x832B8868 => {
    //   block [0x832B8868..0x832B887C)
	// 832B8868: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B886C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8870: 396B7D60  addi r11, r11, 0x7d60
	ctx.r[11].s64 = ctx.r[11].s64 + 32096;
	// 832B8874: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8880 size=12
    let mut pc: u32 = 0x832B8880;
    'dispatch: loop {
        match pc {
            0x832B8880 => {
    //   block [0x832B8880..0x832B888C)
	// 832B8880: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8884: 386B8698  addi r3, r11, -0x7968
	ctx.r[3].s64 = ctx.r[11].s64 + -31080;
	// 832B8888: 4BBCC3E0  b 0x82e84c68
	sub_82E84C68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8890 size=20
    let mut pc: u32 = 0x832B8890;
    'dispatch: loop {
        match pc {
            0x832B8890 => {
    //   block [0x832B8890..0x832B88A4)
	// 832B8890: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8894: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B8898: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B889C: 916ADE74  stw r11, -0x218c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8588 as u32), ctx.r[11].u32 ) };
	// 832B88A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B88A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B88A8 size=20
    let mut pc: u32 = 0x832B88A8;
    'dispatch: loop {
        match pc {
            0x832B88A8 => {
    //   block [0x832B88A8..0x832B88BC)
	// 832B88A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B88AC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B88B0: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B88B4: 916A8B58  stw r11, -0x74a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29864 as u32), ctx.r[11].u32 ) };
	// 832B88B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B88C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B88C0 size=20
    let mut pc: u32 = 0x832B88C0;
    'dispatch: loop {
        match pc {
            0x832B88C0 => {
    //   block [0x832B88C0..0x832B88D4)
	// 832B88C0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B88C4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B88C8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B88CC: 916A8C64  stw r11, -0x739c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29596 as u32), ctx.r[11].u32 ) };
	// 832B88D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B88D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B88D8 size=24
    let mut pc: u32 = 0x832B88D8;
    'dispatch: loop {
        match pc {
            0x832B88D8 => {
    //   block [0x832B88D8..0x832B88F0)
	// 832B88D8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B88DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B88E0: 396B8700  addi r11, r11, -0x7900
	ctx.r[11].s64 = ctx.r[11].s64 + -30976;
	// 832B88E4: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 832B88E8: 994B0044  stb r10, 0x44(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832B88EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B88F0 size=24
    let mut pc: u32 = 0x832B88F0;
    'dispatch: loop {
        match pc {
            0x832B88F0 => {
    //   block [0x832B88F0..0x832B8908)
	// 832B88F0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B88F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B88F8: 396B86B0  addi r11, r11, -0x7950
	ctx.r[11].s64 = ctx.r[11].s64 + -31056;
	// 832B88FC: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 832B8900: 994B0044  stb r10, 0x44(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832B8904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8908 size=20
    let mut pc: u32 = 0x832B8908;
    'dispatch: loop {
        match pc {
            0x832B8908 => {
    //   block [0x832B8908..0x832B891C)
	// 832B8908: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B890C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8910: 396B8750  addi r11, r11, -0x78b0
	ctx.r[11].s64 = ctx.r[11].s64 + -30896;
	// 832B8914: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8920 size=20
    let mut pc: u32 = 0x832B8920;
    'dispatch: loop {
        match pc {
            0x832B8920 => {
    //   block [0x832B8920..0x832B8934)
	// 832B8920: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8924: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8928: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B892C: 916A8E84  stw r11, -0x717c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29052 as u32), ctx.r[11].u32 ) };
	// 832B8930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8938 size=20
    let mut pc: u32 = 0x832B8938;
    'dispatch: loop {
        match pc {
            0x832B8938 => {
    //   block [0x832B8938..0x832B894C)
	// 832B8938: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B893C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8940: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8944: 916A8D78  stw r11, -0x7288(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29320 as u32), ctx.r[11].u32 ) };
	// 832B8948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8950 size=20
    let mut pc: u32 = 0x832B8950;
    'dispatch: loop {
        match pc {
            0x832B8950 => {
    //   block [0x832B8950..0x832B8964)
	// 832B8950: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8954: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8958: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B895C: 916A91A8  stw r11, -0x6e58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28248 as u32), ctx.r[11].u32 ) };
	// 832B8960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8968 size=20
    let mut pc: u32 = 0x832B8968;
    'dispatch: loop {
        match pc {
            0x832B8968 => {
    //   block [0x832B8968..0x832B897C)
	// 832B8968: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B896C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8970: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8974: 916A97F0  stw r11, -0x6810(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26640 as u32), ctx.r[11].u32 ) };
	// 832B8978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8980 size=20
    let mut pc: u32 = 0x832B8980;
    'dispatch: loop {
        match pc {
            0x832B8980 => {
    //   block [0x832B8980..0x832B8994)
	// 832B8980: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8984: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8988: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B898C: 916A98FC  stw r11, -0x6704(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26372 as u32), ctx.r[11].u32 ) };
	// 832B8990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8998 size=20
    let mut pc: u32 = 0x832B8998;
    'dispatch: loop {
        match pc {
            0x832B8998 => {
    //   block [0x832B8998..0x832B89AC)
	// 832B8998: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B899C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B89A0: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B89A4: 916A8F90  stw r11, -0x7070(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28784 as u32), ctx.r[11].u32 ) };
	// 832B89A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B89B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B89B0 size=20
    let mut pc: u32 = 0x832B89B0;
    'dispatch: loop {
        match pc {
            0x832B89B0 => {
    //   block [0x832B89B0..0x832B89C4)
	// 832B89B0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B89B4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B89B8: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B89BC: 916A94CC  stw r11, -0x6b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27444 as u32), ctx.r[11].u32 ) };
	// 832B89C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B89C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B89C8 size=20
    let mut pc: u32 = 0x832B89C8;
    'dispatch: loop {
        match pc {
            0x832B89C8 => {
    //   block [0x832B89C8..0x832B89DC)
	// 832B89C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B89CC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B89D0: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B89D4: 916A92B4  stw r11, -0x6d4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27980 as u32), ctx.r[11].u32 ) };
	// 832B89D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B89E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B89E0 size=20
    let mut pc: u32 = 0x832B89E0;
    'dispatch: loop {
        match pc {
            0x832B89E0 => {
    //   block [0x832B89E0..0x832B89F4)
	// 832B89E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B89E4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B89E8: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B89EC: 916A9B14  stw r11, -0x64ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25836 as u32), ctx.r[11].u32 ) };
	// 832B89F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B89F8 size=20
    let mut pc: u32 = 0x832B89F8;
    'dispatch: loop {
        match pc {
            0x832B89F8 => {
    //   block [0x832B89F8..0x832B8A0C)
	// 832B89F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B89FC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A00: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A04: 916A9A08  stw r11, -0x65f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26104 as u32), ctx.r[11].u32 ) };
	// 832B8A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A10 size=20
    let mut pc: u32 = 0x832B8A10;
    'dispatch: loop {
        match pc {
            0x832B8A10 => {
    //   block [0x832B8A10..0x832B8A24)
	// 832B8A10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A14: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A18: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A1C: 916A93C0  stw r11, -0x6c40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27712 as u32), ctx.r[11].u32 ) };
	// 832B8A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A28 size=20
    let mut pc: u32 = 0x832B8A28;
    'dispatch: loop {
        match pc {
            0x832B8A28 => {
    //   block [0x832B8A28..0x832B8A3C)
	// 832B8A28: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A2C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A30: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A34: 916A9D2C  stw r11, -0x62d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25300 as u32), ctx.r[11].u32 ) };
	// 832B8A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A40 size=20
    let mut pc: u32 = 0x832B8A40;
    'dispatch: loop {
        match pc {
            0x832B8A40 => {
    //   block [0x832B8A40..0x832B8A54)
	// 832B8A40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A44: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A48: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A4C: 916A95D8  stw r11, -0x6a28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27176 as u32), ctx.r[11].u32 ) };
	// 832B8A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A58 size=20
    let mut pc: u32 = 0x832B8A58;
    'dispatch: loop {
        match pc {
            0x832B8A58 => {
    //   block [0x832B8A58..0x832B8A6C)
	// 832B8A58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A5C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A60: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A64: 916A909C  stw r11, -0x6f64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28516 as u32), ctx.r[11].u32 ) };
	// 832B8A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A70 size=20
    let mut pc: u32 = 0x832B8A70;
    'dispatch: loop {
        match pc {
            0x832B8A70 => {
    //   block [0x832B8A70..0x832B8A84)
	// 832B8A70: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A74: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A78: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A7C: 916A96E4  stw r11, -0x691c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26908 as u32), ctx.r[11].u32 ) };
	// 832B8A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8A88 size=20
    let mut pc: u32 = 0x832B8A88;
    'dispatch: loop {
        match pc {
            0x832B8A88 => {
    //   block [0x832B8A88..0x832B8A9C)
	// 832B8A88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B8A8C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8A90: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B8A94: 916A9C20  stw r11, -0x63e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25568 as u32), ctx.r[11].u32 ) };
	// 832B8A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8AA0 size=20
    let mut pc: u32 = 0x832B8AA0;
    'dispatch: loop {
        match pc {
            0x832B8AA0 => {
    //   block [0x832B8AA0..0x832B8AB4)
	// 832B8AA0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8AA4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8AA8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8AAC: 916A9E5C  stw r11, -0x61a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24996 as u32), ctx.r[11].u32 ) };
	// 832B8AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8AB8 size=20
    let mut pc: u32 = 0x832B8AB8;
    'dispatch: loop {
        match pc {
            0x832B8AB8 => {
    //   block [0x832B8AB8..0x832B8ACC)
	// 832B8AB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8ABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8AC0: 396BC7C0  addi r11, r11, -0x3840
	ctx.r[11].s64 = ctx.r[11].s64 + -14400;
	// 832B8AC4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8AD0 size=20
    let mut pc: u32 = 0x832B8AD0;
    'dispatch: loop {
        match pc {
            0x832B8AD0 => {
    //   block [0x832B8AD0..0x832B8AE4)
	// 832B8AD0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8AD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8AD8: 396BC3B8  addi r11, r11, -0x3c48
	ctx.r[11].s64 = ctx.r[11].s64 + -15432;
	// 832B8ADC: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8AE8 size=20
    let mut pc: u32 = 0x832B8AE8;
    'dispatch: loop {
        match pc {
            0x832B8AE8 => {
    //   block [0x832B8AE8..0x832B8AFC)
	// 832B8AE8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8AEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8AF0: 396BBFB0  addi r11, r11, -0x4050
	ctx.r[11].s64 = ctx.r[11].s64 + -16464;
	// 832B8AF4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B00 size=20
    let mut pc: u32 = 0x832B8B00;
    'dispatch: loop {
        match pc {
            0x832B8B00 => {
    //   block [0x832B8B00..0x832B8B14)
	// 832B8B00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B08: 396BBBA8  addi r11, r11, -0x4458
	ctx.r[11].s64 = ctx.r[11].s64 + -17496;
	// 832B8B0C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B18 size=20
    let mut pc: u32 = 0x832B8B18;
    'dispatch: loop {
        match pc {
            0x832B8B18 => {
    //   block [0x832B8B18..0x832B8B2C)
	// 832B8B18: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B20: 396BB7A0  addi r11, r11, -0x4860
	ctx.r[11].s64 = ctx.r[11].s64 + -18528;
	// 832B8B24: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B30 size=20
    let mut pc: u32 = 0x832B8B30;
    'dispatch: loop {
        match pc {
            0x832B8B30 => {
    //   block [0x832B8B30..0x832B8B44)
	// 832B8B30: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B38: 396BB398  addi r11, r11, -0x4c68
	ctx.r[11].s64 = ctx.r[11].s64 + -19560;
	// 832B8B3C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B48 size=20
    let mut pc: u32 = 0x832B8B48;
    'dispatch: loop {
        match pc {
            0x832B8B48 => {
    //   block [0x832B8B48..0x832B8B5C)
	// 832B8B48: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B50: 396BAF90  addi r11, r11, -0x5070
	ctx.r[11].s64 = ctx.r[11].s64 + -20592;
	// 832B8B54: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B60 size=20
    let mut pc: u32 = 0x832B8B60;
    'dispatch: loop {
        match pc {
            0x832B8B60 => {
    //   block [0x832B8B60..0x832B8B74)
	// 832B8B60: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B68: 396BAB88  addi r11, r11, -0x5478
	ctx.r[11].s64 = ctx.r[11].s64 + -21624;
	// 832B8B6C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B78 size=20
    let mut pc: u32 = 0x832B8B78;
    'dispatch: loop {
        match pc {
            0x832B8B78 => {
    //   block [0x832B8B78..0x832B8B8C)
	// 832B8B78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B80: 396BA780  addi r11, r11, -0x5880
	ctx.r[11].s64 = ctx.r[11].s64 + -22656;
	// 832B8B84: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8B90 size=20
    let mut pc: u32 = 0x832B8B90;
    'dispatch: loop {
        match pc {
            0x832B8B90 => {
    //   block [0x832B8B90..0x832B8BA4)
	// 832B8B90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8B94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8B98: 396BA378  addi r11, r11, -0x5c88
	ctx.r[11].s64 = ctx.r[11].s64 + -23688;
	// 832B8B9C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8BA8 size=20
    let mut pc: u32 = 0x832B8BA8;
    'dispatch: loop {
        match pc {
            0x832B8BA8 => {
    //   block [0x832B8BA8..0x832B8BBC)
	// 832B8BA8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832B8BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8BB0: 396B9F70  addi r11, r11, -0x6090
	ctx.r[11].s64 = ctx.r[11].s64 + -24720;
	// 832B8BB4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8BC0 size=20
    let mut pc: u32 = 0x832B8BC0;
    'dispatch: loop {
        match pc {
            0x832B8BC0 => {
    //   block [0x832B8BC0..0x832B8BD4)
	// 832B8BC0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8BC4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8BC8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8BCC: 916ACBC8  stw r11, -0x3438(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13368 as u32), ctx.r[11].u32 ) };
	// 832B8BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8BD8 size=20
    let mut pc: u32 = 0x832B8BD8;
    'dispatch: loop {
        match pc {
            0x832B8BD8 => {
    //   block [0x832B8BD8..0x832B8BEC)
	// 832B8BD8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8BDC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8BE0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8BE4: 916ACCDC  stw r11, -0x3324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13092 as u32), ctx.r[11].u32 ) };
	// 832B8BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8BF0 size=20
    let mut pc: u32 = 0x832B8BF0;
    'dispatch: loop {
        match pc {
            0x832B8BF0 => {
    //   block [0x832B8BF0..0x832B8C04)
	// 832B8BF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8BF4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8BF8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8BFC: 916ACDF0  stw r11, -0x3210(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12816 as u32), ctx.r[11].u32 ) };
	// 832B8C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C08 size=20
    let mut pc: u32 = 0x832B8C08;
    'dispatch: loop {
        match pc {
            0x832B8C08 => {
    //   block [0x832B8C08..0x832B8C1C)
	// 832B8C08: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C0C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C10: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C14: 916ACF04  stw r11, -0x30fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12540 as u32), ctx.r[11].u32 ) };
	// 832B8C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C20 size=20
    let mut pc: u32 = 0x832B8C20;
    'dispatch: loop {
        match pc {
            0x832B8C20 => {
    //   block [0x832B8C20..0x832B8C34)
	// 832B8C20: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C24: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C28: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C2C: 916AD018  stw r11, -0x2fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12264 as u32), ctx.r[11].u32 ) };
	// 832B8C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C38 size=20
    let mut pc: u32 = 0x832B8C38;
    'dispatch: loop {
        match pc {
            0x832B8C38 => {
    //   block [0x832B8C38..0x832B8C4C)
	// 832B8C38: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C3C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C40: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C44: 916AD12C  stw r11, -0x2ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11988 as u32), ctx.r[11].u32 ) };
	// 832B8C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C50 size=20
    let mut pc: u32 = 0x832B8C50;
    'dispatch: loop {
        match pc {
            0x832B8C50 => {
    //   block [0x832B8C50..0x832B8C64)
	// 832B8C50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C54: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C58: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C5C: 916AD244  stw r11, -0x2dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11708 as u32), ctx.r[11].u32 ) };
	// 832B8C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C68 size=20
    let mut pc: u32 = 0x832B8C68;
    'dispatch: loop {
        match pc {
            0x832B8C68 => {
    //   block [0x832B8C68..0x832B8C7C)
	// 832B8C68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C6C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C70: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C74: 916AD358  stw r11, -0x2ca8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11432 as u32), ctx.r[11].u32 ) };
	// 832B8C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C80 size=20
    let mut pc: u32 = 0x832B8C80;
    'dispatch: loop {
        match pc {
            0x832B8C80 => {
    //   block [0x832B8C80..0x832B8C94)
	// 832B8C80: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C84: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8C88: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8C8C: 916AD47C  stw r11, -0x2b84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11140 as u32), ctx.r[11].u32 ) };
	// 832B8C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8C98 size=20
    let mut pc: u32 = 0x832B8C98;
    'dispatch: loop {
        match pc {
            0x832B8C98 => {
    //   block [0x832B8C98..0x832B8CAC)
	// 832B8C98: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8C9C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8CA0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8CA4: 916AD590  stw r11, -0x2a70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10864 as u32), ctx.r[11].u32 ) };
	// 832B8CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8CB0 size=20
    let mut pc: u32 = 0x832B8CB0;
    'dispatch: loop {
        match pc {
            0x832B8CB0 => {
    //   block [0x832B8CB0..0x832B8CC4)
	// 832B8CB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8CB4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8CB8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8CBC: 916AD6A4  stw r11, -0x295c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10588 as u32), ctx.r[11].u32 ) };
	// 832B8CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8CC8 size=20
    let mut pc: u32 = 0x832B8CC8;
    'dispatch: loop {
        match pc {
            0x832B8CC8 => {
    //   block [0x832B8CC8..0x832B8CDC)
	// 832B8CC8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8CCC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8CD0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8CD4: 916AD7B8  stw r11, -0x2848(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10312 as u32), ctx.r[11].u32 ) };
	// 832B8CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8CE0 size=20
    let mut pc: u32 = 0x832B8CE0;
    'dispatch: loop {
        match pc {
            0x832B8CE0 => {
    //   block [0x832B8CE0..0x832B8CF4)
	// 832B8CE0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8CE4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8CE8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8CEC: 916AD8CC  stw r11, -0x2734(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10036 as u32), ctx.r[11].u32 ) };
	// 832B8CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8CF8 size=20
    let mut pc: u32 = 0x832B8CF8;
    'dispatch: loop {
        match pc {
            0x832B8CF8 => {
    //   block [0x832B8CF8..0x832B8D0C)
	// 832B8CF8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8CFC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D00: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D04: 916AD9E0  stw r11, -0x2620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9760 as u32), ctx.r[11].u32 ) };
	// 832B8D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D10 size=20
    let mut pc: u32 = 0x832B8D10;
    'dispatch: loop {
        match pc {
            0x832B8D10 => {
    //   block [0x832B8D10..0x832B8D24)
	// 832B8D10: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D14: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D18: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D1C: 916ADAF4  stw r11, -0x250c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9484 as u32), ctx.r[11].u32 ) };
	// 832B8D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D28 size=20
    let mut pc: u32 = 0x832B8D28;
    'dispatch: loop {
        match pc {
            0x832B8D28 => {
    //   block [0x832B8D28..0x832B8D3C)
	// 832B8D28: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D2C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D30: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D34: 916ADC08  stw r11, -0x23f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9208 as u32), ctx.r[11].u32 ) };
	// 832B8D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D40 size=20
    let mut pc: u32 = 0x832B8D40;
    'dispatch: loop {
        match pc {
            0x832B8D40 => {
    //   block [0x832B8D40..0x832B8D54)
	// 832B8D40: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D44: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D48: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D4C: 916ADD1C  stw r11, -0x22e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8932 as u32), ctx.r[11].u32 ) };
	// 832B8D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D58 size=20
    let mut pc: u32 = 0x832B8D58;
    'dispatch: loop {
        match pc {
            0x832B8D58 => {
    //   block [0x832B8D58..0x832B8D6C)
	// 832B8D58: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D5C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D60: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D64: 916ADE30  stw r11, -0x21d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8656 as u32), ctx.r[11].u32 ) };
	// 832B8D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D70 size=20
    let mut pc: u32 = 0x832B8D70;
    'dispatch: loop {
        match pc {
            0x832B8D70 => {
    //   block [0x832B8D70..0x832B8D84)
	// 832B8D70: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D74: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D78: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D7C: 916ADF44  stw r11, -0x20bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8380 as u32), ctx.r[11].u32 ) };
	// 832B8D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8D88 size=20
    let mut pc: u32 = 0x832B8D88;
    'dispatch: loop {
        match pc {
            0x832B8D88 => {
    //   block [0x832B8D88..0x832B8D9C)
	// 832B8D88: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8D8C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8D90: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8D94: 916AE058  stw r11, -0x1fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8104 as u32), ctx.r[11].u32 ) };
	// 832B8D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8DA0 size=20
    let mut pc: u32 = 0x832B8DA0;
    'dispatch: loop {
        match pc {
            0x832B8DA0 => {
    //   block [0x832B8DA0..0x832B8DB4)
	// 832B8DA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8DA4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8DA8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8DAC: 916AE16C  stw r11, -0x1e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7828 as u32), ctx.r[11].u32 ) };
	// 832B8DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8DB8 size=20
    let mut pc: u32 = 0x832B8DB8;
    'dispatch: loop {
        match pc {
            0x832B8DB8 => {
    //   block [0x832B8DB8..0x832B8DCC)
	// 832B8DB8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8DBC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8DC0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8DC4: 916AE284  stw r11, -0x1d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7548 as u32), ctx.r[11].u32 ) };
	// 832B8DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8DD0 size=20
    let mut pc: u32 = 0x832B8DD0;
    'dispatch: loop {
        match pc {
            0x832B8DD0 => {
    //   block [0x832B8DD0..0x832B8DE4)
	// 832B8DD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8DD4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8DD8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8DDC: 916AE398  stw r11, -0x1c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7272 as u32), ctx.r[11].u32 ) };
	// 832B8DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8DE8 size=20
    let mut pc: u32 = 0x832B8DE8;
    'dispatch: loop {
        match pc {
            0x832B8DE8 => {
    //   block [0x832B8DE8..0x832B8DFC)
	// 832B8DE8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8DEC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8DF0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8DF4: 916AE4AC  stw r11, -0x1b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6996 as u32), ctx.r[11].u32 ) };
	// 832B8DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E00 size=20
    let mut pc: u32 = 0x832B8E00;
    'dispatch: loop {
        match pc {
            0x832B8E00 => {
    //   block [0x832B8E00..0x832B8E14)
	// 832B8E00: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E04: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E08: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E0C: 916AE5C0  stw r11, -0x1a40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6720 as u32), ctx.r[11].u32 ) };
	// 832B8E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E18 size=20
    let mut pc: u32 = 0x832B8E18;
    'dispatch: loop {
        match pc {
            0x832B8E18 => {
    //   block [0x832B8E18..0x832B8E2C)
	// 832B8E18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E1C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E20: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E24: 916AE6D4  stw r11, -0x192c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6444 as u32), ctx.r[11].u32 ) };
	// 832B8E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E30 size=20
    let mut pc: u32 = 0x832B8E30;
    'dispatch: loop {
        match pc {
            0x832B8E30 => {
    //   block [0x832B8E30..0x832B8E44)
	// 832B8E30: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E34: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E38: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E3C: 916AE7E8  stw r11, -0x1818(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6168 as u32), ctx.r[11].u32 ) };
	// 832B8E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E48 size=20
    let mut pc: u32 = 0x832B8E48;
    'dispatch: loop {
        match pc {
            0x832B8E48 => {
    //   block [0x832B8E48..0x832B8E5C)
	// 832B8E48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E4C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E50: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E54: 916AE8FC  stw r11, -0x1704(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5892 as u32), ctx.r[11].u32 ) };
	// 832B8E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E60 size=20
    let mut pc: u32 = 0x832B8E60;
    'dispatch: loop {
        match pc {
            0x832B8E60 => {
    //   block [0x832B8E60..0x832B8E74)
	// 832B8E60: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E64: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E68: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E6C: 916AEA10  stw r11, -0x15f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5616 as u32), ctx.r[11].u32 ) };
	// 832B8E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E78 size=20
    let mut pc: u32 = 0x832B8E78;
    'dispatch: loop {
        match pc {
            0x832B8E78 => {
    //   block [0x832B8E78..0x832B8E8C)
	// 832B8E78: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E7C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E80: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E84: 916AEB24  stw r11, -0x14dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5340 as u32), ctx.r[11].u32 ) };
	// 832B8E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8E90 size=20
    let mut pc: u32 = 0x832B8E90;
    'dispatch: loop {
        match pc {
            0x832B8E90 => {
    //   block [0x832B8E90..0x832B8EA4)
	// 832B8E90: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8E94: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8E98: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8E9C: 916AEC38  stw r11, -0x13c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5064 as u32), ctx.r[11].u32 ) };
	// 832B8EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8EA8 size=20
    let mut pc: u32 = 0x832B8EA8;
    'dispatch: loop {
        match pc {
            0x832B8EA8 => {
    //   block [0x832B8EA8..0x832B8EBC)
	// 832B8EA8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8EAC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8EB0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8EB4: 916AED4C  stw r11, -0x12b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4788 as u32), ctx.r[11].u32 ) };
	// 832B8EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8EC0 size=20
    let mut pc: u32 = 0x832B8EC0;
    'dispatch: loop {
        match pc {
            0x832B8EC0 => {
    //   block [0x832B8EC0..0x832B8ED4)
	// 832B8EC0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8EC4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8EC8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8ECC: 916AEE60  stw r11, -0x11a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4512 as u32), ctx.r[11].u32 ) };
	// 832B8ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8ED8 size=20
    let mut pc: u32 = 0x832B8ED8;
    'dispatch: loop {
        match pc {
            0x832B8ED8 => {
    //   block [0x832B8ED8..0x832B8EEC)
	// 832B8ED8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8EDC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8EE0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8EE4: 916AEF78  stw r11, -0x1088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4232 as u32), ctx.r[11].u32 ) };
	// 832B8EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8EF0 size=20
    let mut pc: u32 = 0x832B8EF0;
    'dispatch: loop {
        match pc {
            0x832B8EF0 => {
    //   block [0x832B8EF0..0x832B8F04)
	// 832B8EF0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8EF4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832B8EF8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8EFC: 916AF090  stw r11, -0xf70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3952 as u32), ctx.r[11].u32 ) };
	// 832B8F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F08 size=20
    let mut pc: u32 = 0x832B8F08;
    'dispatch: loop {
        match pc {
            0x832B8F08 => {
    //   block [0x832B8F08..0x832B8F1C)
	// 832B8F08: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8F0C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F10: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8F14: 916A89BC  stw r11, -0x7644(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30276 as u32), ctx.r[11].u32 ) };
	// 832B8F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F20 size=20
    let mut pc: u32 = 0x832B8F20;
    'dispatch: loop {
        match pc {
            0x832B8F20 => {
    //   block [0x832B8F20..0x832B8F34)
	// 832B8F20: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8F24: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F28: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8F2C: 916A8AD0  stw r11, -0x7530(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30000 as u32), ctx.r[11].u32 ) };
	// 832B8F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F38 size=20
    let mut pc: u32 = 0x832B8F38;
    'dispatch: loop {
        match pc {
            0x832B8F38 => {
    //   block [0x832B8F38..0x832B8F4C)
	// 832B8F38: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8F3C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F40: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8F44: 916A8BE4  stw r11, -0x741c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29724 as u32), ctx.r[11].u32 ) };
	// 832B8F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F50 size=20
    let mut pc: u32 = 0x832B8F50;
    'dispatch: loop {
        match pc {
            0x832B8F50 => {
    //   block [0x832B8F50..0x832B8F64)
	// 832B8F50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8F54: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F58: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8F5C: 916A8CF8  stw r11, -0x7308(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29448 as u32), ctx.r[11].u32 ) };
	// 832B8F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F68 size=20
    let mut pc: u32 = 0x832B8F68;
    'dispatch: loop {
        match pc {
            0x832B8F68 => {
    //   block [0x832B8F68..0x832B8F7C)
	// 832B8F68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8F6C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F70: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8F74: 916A8E0C  stw r11, -0x71f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29172 as u32), ctx.r[11].u32 ) };
	// 832B8F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F80 size=20
    let mut pc: u32 = 0x832B8F80;
    'dispatch: loop {
        match pc {
            0x832B8F80 => {
    //   block [0x832B8F80..0x832B8F94)
	// 832B8F80: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8F84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B8F88: 386A8F90  addi r3, r10, -0x7070
	ctx.r[3].s64 = ctx.r[10].s64 + -28784;
	// 832B8F8C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 832B8F90: 4BC420A8  b 0x82efb038
	sub_82EFB038(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8F98 size=20
    let mut pc: u32 = 0x832B8F98;
    'dispatch: loop {
        match pc {
            0x832B8F98 => {
    //   block [0x832B8F98..0x832B8FAC)
	// 832B8F98: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8F9C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B8FA0: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 832B8FA4: 916A8FB0  stw r11, -0x7050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28752 as u32), ctx.r[11].u32 ) };
	// 832B8FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8FB0 size=20
    let mut pc: u32 = 0x832B8FB0;
    'dispatch: loop {
        match pc {
            0x832B8FB0 => {
    //   block [0x832B8FB0..0x832B8FC4)
	// 832B8FB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8FB4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B8FB8: 396BD1D4  addi r11, r11, -0x2e2c
	ctx.r[11].s64 = ctx.r[11].s64 + -11820;
	// 832B8FBC: 916AE2A0  stw r11, -0x1d60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7520 as u32), ctx.r[11].u32 ) };
	// 832B8FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B8FC8 size=64
    let mut pc: u32 = 0x832B8FC8;
    'dispatch: loop {
        match pc {
            0x832B8FC8 => {
    //   block [0x832B8FC8..0x832B9008)
	// 832B8FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B8FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B8FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B8FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B8FD8: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 832B8FDC: 397F9010  addi r11, r31, -0x6ff0
	ctx.r[11].s64 = ctx.r[31].s64 + -28656;
	// 832B8FE0: 386B0550  addi r3, r11, 0x550
	ctx.r[3].s64 = ctx.r[11].s64 + 1360;
	// 832B8FE4: 4BC42055  bl 0x82efb038
	ctx.lr = 0x832B8FE8;
	sub_82EFB038(ctx, base);
	// 832B8FE8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8FEC: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 832B8FF0: 917F9010  stw r11, -0x6ff0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28656 as u32), ctx.r[11].u32 ) };
	// 832B8FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B8FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B8FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B9004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9008 size=20
    let mut pc: u32 = 0x832B9008;
    'dispatch: loop {
        match pc {
            0x832B9008 => {
    //   block [0x832B9008..0x832B901C)
	// 832B9008: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B900C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9010: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 832B9014: 916A8FC8  stw r11, -0x7038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28728 as u32), ctx.r[11].u32 ) };
	// 832B9018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9020 size=20
    let mut pc: u32 = 0x832B9020;
    'dispatch: loop {
        match pc {
            0x832B9020 => {
    //   block [0x832B9020..0x832B9034)
	// 832B9020: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9024: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B9028: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 832B902C: 916AEC4C  stw r11, -0x13b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5044 as u32), ctx.r[11].u32 ) };
	// 832B9030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9038 size=20
    let mut pc: u32 = 0x832B9038;
    'dispatch: loop {
        match pc {
            0x832B9038 => {
    //   block [0x832B9038..0x832B904C)
	// 832B9038: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B903C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B9040: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 832B9044: 916AEC50  stw r11, -0x13b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5040 as u32), ctx.r[11].u32 ) };
	// 832B9048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9050 size=20
    let mut pc: u32 = 0x832B9050;
    'dispatch: loop {
        match pc {
            0x832B9050 => {
    //   block [0x832B9050..0x832B9064)
	// 832B9050: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9054: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9058: 396B105C  addi r11, r11, 0x105c
	ctx.r[11].s64 = ctx.r[11].s64 + 4188;
	// 832B905C: 916A959C  stw r11, -0x6a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27236 as u32), ctx.r[11].u32 ) };
	// 832B9060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9068 size=20
    let mut pc: u32 = 0x832B9068;
    'dispatch: loop {
        match pc {
            0x832B9068 => {
    //   block [0x832B9068..0x832B907C)
	// 832B9068: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B906C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9070: 396B105C  addi r11, r11, 0x105c
	ctx.r[11].s64 = ctx.r[11].s64 + 4188;
	// 832B9074: 916A958C  stw r11, -0x6a74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27252 as u32), ctx.r[11].u32 ) };
	// 832B9078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9080 size=20
    let mut pc: u32 = 0x832B9080;
    'dispatch: loop {
        match pc {
            0x832B9080 => {
    //   block [0x832B9080..0x832B9094)
	// 832B9080: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9084: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9088: 396B105C  addi r11, r11, 0x105c
	ctx.r[11].s64 = ctx.r[11].s64 + 4188;
	// 832B908C: 916A9594  stw r11, -0x6a6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27244 as u32), ctx.r[11].u32 ) };
	// 832B9090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9098 size=12
    let mut pc: u32 = 0x832B9098;
    'dispatch: loop {
        match pc {
            0x832B9098 => {
    //   block [0x832B9098..0x832B90A4)
	// 832B9098: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B909C: 386B95A8  addi r3, r11, -0x6a58
	ctx.r[3].s64 = ctx.r[11].s64 + -27224;
	// 832B90A0: 4BC469F0  b 0x82effa90
	sub_82EFFA90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B90A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B90A8 size=20
    let mut pc: u32 = 0x832B90A8;
    'dispatch: loop {
        match pc {
            0x832B90A8 => {
    //   block [0x832B90A8..0x832B90BC)
	// 832B90A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B90AC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B90B0: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 832B90B4: 916AEC54  stw r11, -0x13ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5036 as u32), ctx.r[11].u32 ) };
	// 832B90B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B90C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B90C0 size=20
    let mut pc: u32 = 0x832B90C0;
    'dispatch: loop {
        match pc {
            0x832B90C0 => {
    //   block [0x832B90C0..0x832B90D4)
	// 832B90C0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B90C4: 386B9610  addi r3, r11, -0x69f0
	ctx.r[3].s64 = ctx.r[11].s64 + -27120;
	// 832B90C8: 896B9610  lbz r11, -0x69f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-27120 as u32) ) } as u64;
	// 832B90CC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B90D0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B90D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B90D4 size=8
    let mut pc: u32 = 0x832B90D4;
    'dispatch: loop {
        match pc {
            0x832B90D4 => {
    //   block [0x832B90D4..0x832B90DC)
	// 832B90D4: 4BCB924C  b 0x82f72320
	sub_82F72320(ctx, base);
	return;
	// 832B90D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B90E0 size=20
    let mut pc: u32 = 0x832B90E0;
    'dispatch: loop {
        match pc {
            0x832B90E0 => {
    //   block [0x832B90E0..0x832B90F4)
	// 832B90E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B90E4: 386B95F8  addi r3, r11, -0x6a08
	ctx.r[3].s64 = ctx.r[11].s64 + -27144;
	// 832B90E8: 896B95F8  lbz r11, -0x6a08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-27144 as u32) ) } as u64;
	// 832B90EC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B90F0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B90F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B90F4 size=8
    let mut pc: u32 = 0x832B90F4;
    'dispatch: loop {
        match pc {
            0x832B90F4 => {
    //   block [0x832B90F4..0x832B90FC)
	// 832B90F4: 4BCB922C  b 0x82f72320
	sub_82F72320(ctx, base);
	return;
	// 832B90F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9100 size=92
    let mut pc: u32 = 0x832B9100;
    'dispatch: loop {
        match pc {
            0x832B9100 => {
    //   block [0x832B9100..0x832B915C)
	// 832B9100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B910C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B9110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9114: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832B9118: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B911C: 396BF050  addi r11, r11, -0xfb0
	ctx.r[11].s64 = ctx.r[11].s64 + -4016;
	// 832B9120: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B9124: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B9128: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B912C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B9130: 4198000C  blt cr6, 0x832b913c
	if ctx.cr[6].lt {
	pc = 0x832B913C; continue 'dispatch;
	}
	// 832B9134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B9138: 4BCB91E9  bl 0x82f72320
	ctx.lr = 0x832B913C;
	sub_82F72320(ctx, base);
	// 832B913C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B9140: 4080FFE4  bge 0x832b9124
	if !ctx.cr[0].lt {
	pc = 0x832B9124; continue 'dispatch;
	}
	// 832B9144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B9148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B914C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B9154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B9158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9160 size=92
    let mut pc: u32 = 0x832B9160;
    'dispatch: loop {
        match pc {
            0x832B9160 => {
    //   block [0x832B9160..0x832B91BC)
	// 832B9160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B916C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B9170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9174: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9178: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B917C: 396B9628  addi r11, r11, -0x69d8
	ctx.r[11].s64 = ctx.r[11].s64 + -27096;
	// 832B9180: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B9184: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B9188: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B918C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B9190: 4198000C  blt cr6, 0x832b919c
	if ctx.cr[6].lt {
	pc = 0x832B919C; continue 'dispatch;
	}
	// 832B9194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B9198: 4BCB9189  bl 0x82f72320
	ctx.lr = 0x832B919C;
	sub_82F72320(ctx, base);
	// 832B919C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B91A0: 4080FFE4  bge 0x832b9184
	if !ctx.cr[0].lt {
	pc = 0x832B9184; continue 'dispatch;
	}
	// 832B91A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B91A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B91AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B91B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B91B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B91B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B91C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B91C0 size=92
    let mut pc: u32 = 0x832B91C0;
    'dispatch: loop {
        match pc {
            0x832B91C0 => {
    //   block [0x832B91C0..0x832B921C)
	// 832B91C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B91C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B91C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B91CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B91D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B91D4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832B91D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B91DC: 396BF378  addi r11, r11, -0xc88
	ctx.r[11].s64 = ctx.r[11].s64 + -3208;
	// 832B91E0: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832B91E4: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B91E8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B91EC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B91F0: 4198000C  blt cr6, 0x832b91fc
	if ctx.cr[6].lt {
	pc = 0x832B91FC; continue 'dispatch;
	}
	// 832B91F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B91F8: 4BCB9129  bl 0x82f72320
	ctx.lr = 0x832B91FC;
	sub_82F72320(ctx, base);
	// 832B91FC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B9200: 4080FFE4  bge 0x832b91e4
	if !ctx.cr[0].lt {
	pc = 0x832B91E4; continue 'dispatch;
	}
	// 832B9204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B9208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B920C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B9214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B9218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9220 size=92
    let mut pc: u32 = 0x832B9220;
    'dispatch: loop {
        match pc {
            0x832B9220 => {
    //   block [0x832B9220..0x832B927C)
	// 832B9220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B922C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B9230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9234: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9238: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B923C: 396B9698  addi r11, r11, -0x6968
	ctx.r[11].s64 = ctx.r[11].s64 + -26984;
	// 832B9240: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832B9244: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B9248: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B924C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832B9250: 4198000C  blt cr6, 0x832b925c
	if ctx.cr[6].lt {
	pc = 0x832B925C; continue 'dispatch;
	}
	// 832B9254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B9258: 4BCB90C9  bl 0x82f72320
	ctx.lr = 0x832B925C;
	sub_82F72320(ctx, base);
	// 832B925C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B9260: 4080FFE4  bge 0x832b9244
	if !ctx.cr[0].lt {
	pc = 0x832B9244; continue 'dispatch;
	}
	// 832B9264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B9268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B926C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B9274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B9278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9280 size=20
    let mut pc: u32 = 0x832B9280;
    'dispatch: loop {
        match pc {
            0x832B9280 => {
    //   block [0x832B9280..0x832B9294)
	// 832B9280: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9284: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B9288: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 832B928C: 916AF398  stw r11, -0xc68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3176 as u32), ctx.r[11].u32 ) };
	// 832B9290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9298 size=20
    let mut pc: u32 = 0x832B9298;
    'dispatch: loop {
        match pc {
            0x832B9298 => {
    //   block [0x832B9298..0x832B92AC)
	// 832B9298: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B929C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B92A0: 396B105C  addi r11, r11, 0x105c
	ctx.r[11].s64 = ctx.r[11].s64 + 4188;
	// 832B92A4: 916A96B8  stw r11, -0x6948(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26952 as u32), ctx.r[11].u32 ) };
	// 832B92A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B92B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B92B0 size=20
    let mut pc: u32 = 0x832B92B0;
    'dispatch: loop {
        match pc {
            0x832B92B0 => {
    //   block [0x832B92B0..0x832B92C4)
	// 832B92B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B92B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B92B8: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 832B92BC: 916A96C0  stw r11, -0x6940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26944 as u32), ctx.r[11].u32 ) };
	// 832B92C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B92C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B92C8 size=20
    let mut pc: u32 = 0x832B92C8;
    'dispatch: loop {
        match pc {
            0x832B92C8 => {
    //   block [0x832B92C8..0x832B92DC)
	// 832B92C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B92CC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B92D0: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B92D4: 916A96E0  stw r11, -0x6920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26912 as u32), ctx.r[11].u32 ) };
	// 832B92D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B92E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B92E0 size=20
    let mut pc: u32 = 0x832B92E0;
    'dispatch: loop {
        match pc {
            0x832B92E0 => {
    //   block [0x832B92E0..0x832B92F4)
	// 832B92E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B92E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B92E8: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B92EC: 916A96C8  stw r11, -0x6938(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26936 as u32), ctx.r[11].u32 ) };
	// 832B92F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B92F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B92F8 size=20
    let mut pc: u32 = 0x832B92F8;
    'dispatch: loop {
        match pc {
            0x832B92F8 => {
    //   block [0x832B92F8..0x832B930C)
	// 832B92F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B92FC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9300: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B9304: 916A96D0  stw r11, -0x6930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26928 as u32), ctx.r[11].u32 ) };
	// 832B9308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9310 size=20
    let mut pc: u32 = 0x832B9310;
    'dispatch: loop {
        match pc {
            0x832B9310 => {
    //   block [0x832B9310..0x832B9324)
	// 832B9310: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9314: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9318: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B931C: 916A96D4  stw r11, -0x692c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26924 as u32), ctx.r[11].u32 ) };
	// 832B9320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9328 size=20
    let mut pc: u32 = 0x832B9328;
    'dispatch: loop {
        match pc {
            0x832B9328 => {
    //   block [0x832B9328..0x832B933C)
	// 832B9328: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B932C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9330: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B9334: 916A96D8  stw r11, -0x6928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26920 as u32), ctx.r[11].u32 ) };
	// 832B9338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9340 size=20
    let mut pc: u32 = 0x832B9340;
    'dispatch: loop {
        match pc {
            0x832B9340 => {
    //   block [0x832B9340..0x832B9354)
	// 832B9340: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9344: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9348: 396B697C  addi r11, r11, 0x697c
	ctx.r[11].s64 = ctx.r[11].s64 + 27004;
	// 832B934C: 916A96DC  stw r11, -0x6924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26916 as u32), ctx.r[11].u32 ) };
	// 832B9350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9358 size=4
    let mut pc: u32 = 0x832B9358;
    'dispatch: loop {
        match pc {
            0x832B9358 => {
    //   block [0x832B9358..0x832B935C)
	// 832B9358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9360 size=16
    let mut pc: u32 = 0x832B9360;
    'dispatch: loop {
        match pc {
            0x832B9360 => {
    //   block [0x832B9360..0x832B9370)
	// 832B9360: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9364: 396B9780  addi r11, r11, -0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + -26752;
	// 832B9368: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 832B936C: 4BD499AC  b 0x83002d18
	sub_83002D18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9370 size=40
    let mut pc: u32 = 0x832B9370;
    'dispatch: loop {
        match pc {
            0x832B9370 => {
    //   block [0x832B9370..0x832B9398)
	// 832B9370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B937C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9380: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 832B9384: 4BD4BDED  bl 0x83005170
	ctx.lr = 0x832B9388;
	sub_83005170(ctx, base);
	// 832B9388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9398 size=40
    let mut pc: u32 = 0x832B9398;
    'dispatch: loop {
        match pc {
            0x832B9398 => {
    //   block [0x832B9398..0x832B93C0)
	// 832B9398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93A4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93A8: 386B9870  addi r3, r11, -0x6790
	ctx.r[3].s64 = ctx.r[11].s64 + -26512;
	// 832B93AC: 4BD4A21D  bl 0x830035c8
	ctx.lr = 0x832B93B0;
	sub_830035C8(ctx, base);
	// 832B93B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B93B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B93B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B93BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B93C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B93C0 size=40
    let mut pc: u32 = 0x832B93C0;
    'dispatch: loop {
        match pc {
            0x832B93C0 => {
    //   block [0x832B93C0..0x832B93E8)
	// 832B93C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B93C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93CC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93D0: 386B9864  addi r3, r11, -0x679c
	ctx.r[3].s64 = ctx.r[11].s64 + -26524;
	// 832B93D4: 4BD4A2C5  bl 0x83003698
	ctx.lr = 0x832B93D8;
	sub_83003698(ctx, base);
	// 832B93D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B93DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B93E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B93E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B93E8 size=40
    let mut pc: u32 = 0x832B93E8;
    'dispatch: loop {
        match pc {
            0x832B93E8 => {
    //   block [0x832B93E8..0x832B9410)
	// 832B93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93F8: 386B98A0  addi r3, r11, -0x6760
	ctx.r[3].s64 = ctx.r[11].s64 + -26464;
	// 832B93FC: 4BD4A1CD  bl 0x830035c8
	ctx.lr = 0x832B9400;
	sub_830035C8(ctx, base);
	// 832B9400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9410 size=40
    let mut pc: u32 = 0x832B9410;
    'dispatch: loop {
        match pc {
            0x832B9410 => {
    //   block [0x832B9410..0x832B9438)
	// 832B9410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B941C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9420: 386B9894  addi r3, r11, -0x676c
	ctx.r[3].s64 = ctx.r[11].s64 + -26476;
	// 832B9424: 4BD4A275  bl 0x83003698
	ctx.lr = 0x832B9428;
	sub_83003698(ctx, base);
	// 832B9428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9438 size=40
    let mut pc: u32 = 0x832B9438;
    'dispatch: loop {
        match pc {
            0x832B9438 => {
    //   block [0x832B9438..0x832B9460)
	// 832B9438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9444: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9448: 386B98D0  addi r3, r11, -0x6730
	ctx.r[3].s64 = ctx.r[11].s64 + -26416;
	// 832B944C: 4BD4A17D  bl 0x830035c8
	ctx.lr = 0x832B9450;
	sub_830035C8(ctx, base);
	// 832B9450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9460 size=40
    let mut pc: u32 = 0x832B9460;
    'dispatch: loop {
        match pc {
            0x832B9460 => {
    //   block [0x832B9460..0x832B9488)
	// 832B9460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B946C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9470: 386B98C4  addi r3, r11, -0x673c
	ctx.r[3].s64 = ctx.r[11].s64 + -26428;
	// 832B9474: 4BD4A225  bl 0x83003698
	ctx.lr = 0x832B9478;
	sub_83003698(ctx, base);
	// 832B9478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B947C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9488 size=40
    let mut pc: u32 = 0x832B9488;
    'dispatch: loop {
        match pc {
            0x832B9488 => {
    //   block [0x832B9488..0x832B94B0)
	// 832B9488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9494: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9498: 386B9940  addi r3, r11, -0x66c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26304;
	// 832B949C: 4BD4A12D  bl 0x830035c8
	ctx.lr = 0x832B94A0;
	sub_830035C8(ctx, base);
	// 832B94A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B94B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B94B0 size=40
    let mut pc: u32 = 0x832B94B0;
    'dispatch: loop {
        match pc {
            0x832B94B0 => {
    //   block [0x832B94B0..0x832B94D8)
	// 832B94B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B94B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B94B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B94BC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B94C0: 386B9934  addi r3, r11, -0x66cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26316;
	// 832B94C4: 4BD4A1D5  bl 0x83003698
	ctx.lr = 0x832B94C8;
	sub_83003698(ctx, base);
	// 832B94C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B94D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B94D8 size=40
    let mut pc: u32 = 0x832B94D8;
    'dispatch: loop {
        match pc {
            0x832B94D8 => {
    //   block [0x832B94D8..0x832B9500)
	// 832B94D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B94DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B94E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B94E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B94E8: 386B9A00  addi r3, r11, -0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + -26112;
	// 832B94EC: 4BD4A0DD  bl 0x830035c8
	ctx.lr = 0x832B94F0;
	sub_830035C8(ctx, base);
	// 832B94F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9500 size=40
    let mut pc: u32 = 0x832B9500;
    'dispatch: loop {
        match pc {
            0x832B9500 => {
    //   block [0x832B9500..0x832B9528)
	// 832B9500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B950C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9510: 386B99F4  addi r3, r11, -0x660c
	ctx.r[3].s64 = ctx.r[11].s64 + -26124;
	// 832B9514: 4BD4A185  bl 0x83003698
	ctx.lr = 0x832B9518;
	sub_83003698(ctx, base);
	// 832B9518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9528 size=40
    let mut pc: u32 = 0x832B9528;
    'dispatch: loop {
        match pc {
            0x832B9528 => {
    //   block [0x832B9528..0x832B9550)
	// 832B9528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9534: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9538: 386B99D0  addi r3, r11, -0x6630
	ctx.r[3].s64 = ctx.r[11].s64 + -26160;
	// 832B953C: 4BD4A08D  bl 0x830035c8
	ctx.lr = 0x832B9540;
	sub_830035C8(ctx, base);
	// 832B9540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B954C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9550 size=40
    let mut pc: u32 = 0x832B9550;
    'dispatch: loop {
        match pc {
            0x832B9550 => {
    //   block [0x832B9550..0x832B9578)
	// 832B9550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B955C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9560: 386B99C4  addi r3, r11, -0x663c
	ctx.r[3].s64 = ctx.r[11].s64 + -26172;
	// 832B9564: 4BD4A135  bl 0x83003698
	ctx.lr = 0x832B9568;
	sub_83003698(ctx, base);
	// 832B9568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B956C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9578 size=40
    let mut pc: u32 = 0x832B9578;
    'dispatch: loop {
        match pc {
            0x832B9578 => {
    //   block [0x832B9578..0x832B95A0)
	// 832B9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9584: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9588: 386B99A0  addi r3, r11, -0x6660
	ctx.r[3].s64 = ctx.r[11].s64 + -26208;
	// 832B958C: 4BD4A03D  bl 0x830035c8
	ctx.lr = 0x832B9590;
	sub_830035C8(ctx, base);
	// 832B9590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B959C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95A0 size=40
    let mut pc: u32 = 0x832B95A0;
    'dispatch: loop {
        match pc {
            0x832B95A0 => {
    //   block [0x832B95A0..0x832B95C8)
	// 832B95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95AC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B95B0: 386B9994  addi r3, r11, -0x666c
	ctx.r[3].s64 = ctx.r[11].s64 + -26220;
	// 832B95B4: 4BD4A0E5  bl 0x83003698
	ctx.lr = 0x832B95B8;
	sub_83003698(ctx, base);
	// 832B95B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B95BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B95C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B95C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95C8 size=40
    let mut pc: u32 = 0x832B95C8;
    'dispatch: loop {
        match pc {
            0x832B95C8 => {
    //   block [0x832B95C8..0x832B95F0)
	// 832B95C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95D4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B95D8: 386B9970  addi r3, r11, -0x6690
	ctx.r[3].s64 = ctx.r[11].s64 + -26256;
	// 832B95DC: 4BD49FED  bl 0x830035c8
	ctx.lr = 0x832B95E0;
	sub_830035C8(ctx, base);
	// 832B95E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B95E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B95E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B95EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95F0 size=40
    let mut pc: u32 = 0x832B95F0;
    'dispatch: loop {
        match pc {
            0x832B95F0 => {
    //   block [0x832B95F0..0x832B9618)
	// 832B95F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95FC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9600: 386B9964  addi r3, r11, -0x669c
	ctx.r[3].s64 = ctx.r[11].s64 + -26268;
	// 832B9604: 4BD4A095  bl 0x83003698
	ctx.lr = 0x832B9608;
	sub_83003698(ctx, base);
	// 832B9608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9618 size=40
    let mut pc: u32 = 0x832B9618;
    'dispatch: loop {
        match pc {
            0x832B9618 => {
    //   block [0x832B9618..0x832B9640)
	// 832B9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9624: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9628: 386B9A34  addi r3, r11, -0x65cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26060;
	// 832B962C: 4BD49F9D  bl 0x830035c8
	ctx.lr = 0x832B9630;
	sub_830035C8(ctx, base);
	// 832B9630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B963C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9640 size=40
    let mut pc: u32 = 0x832B9640;
    'dispatch: loop {
        match pc {
            0x832B9640 => {
    //   block [0x832B9640..0x832B9668)
	// 832B9640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B964C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9650: 386B9A28  addi r3, r11, -0x65d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26072;
	// 832B9654: 4BD4A045  bl 0x83003698
	ctx.lr = 0x832B9658;
	sub_83003698(ctx, base);
	// 832B9658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B965C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9668 size=40
    let mut pc: u32 = 0x832B9668;
    'dispatch: loop {
        match pc {
            0x832B9668 => {
    //   block [0x832B9668..0x832B9690)
	// 832B9668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B966C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9674: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9678: 386B9A64  addi r3, r11, -0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + -26012;
	// 832B967C: 4BD49F4D  bl 0x830035c8
	ctx.lr = 0x832B9680;
	sub_830035C8(ctx, base);
	// 832B9680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B968C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9690 size=40
    let mut pc: u32 = 0x832B9690;
    'dispatch: loop {
        match pc {
            0x832B9690 => {
    //   block [0x832B9690..0x832B96B8)
	// 832B9690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B969C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96A0: 386B9A58  addi r3, r11, -0x65a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26024;
	// 832B96A4: 4BD49FF5  bl 0x83003698
	ctx.lr = 0x832B96A8;
	sub_83003698(ctx, base);
	// 832B96A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B96B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B96B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B96B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B96B8 size=40
    let mut pc: u32 = 0x832B96B8;
    'dispatch: loop {
        match pc {
            0x832B96B8 => {
    //   block [0x832B96B8..0x832B96E0)
	// 832B96B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B96BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B96C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B96C4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96C8: 386B9AC4  addi r3, r11, -0x653c
	ctx.r[3].s64 = ctx.r[11].s64 + -25916;
	// 832B96CC: 4BD49EFD  bl 0x830035c8
	ctx.lr = 0x832B96D0;
	sub_830035C8(ctx, base);
	// 832B96D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B96D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B96DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B96E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B96E0 size=40
    let mut pc: u32 = 0x832B96E0;
    'dispatch: loop {
        match pc {
            0x832B96E0 => {
    //   block [0x832B96E0..0x832B9708)
	// 832B96E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B96E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B96E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B96EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96F0: 386B9AB8  addi r3, r11, -0x6548
	ctx.r[3].s64 = ctx.r[11].s64 + -25928;
	// 832B96F4: 4BD49FA5  bl 0x83003698
	ctx.lr = 0x832B96F8;
	sub_83003698(ctx, base);
	// 832B96F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9708 size=40
    let mut pc: u32 = 0x832B9708;
    'dispatch: loop {
        match pc {
            0x832B9708 => {
    //   block [0x832B9708..0x832B9730)
	// 832B9708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9714: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9718: 386B9A94  addi r3, r11, -0x656c
	ctx.r[3].s64 = ctx.r[11].s64 + -25964;
	// 832B971C: 4BD49EAD  bl 0x830035c8
	ctx.lr = 0x832B9720;
	sub_830035C8(ctx, base);
	// 832B9720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9730 size=40
    let mut pc: u32 = 0x832B9730;
    'dispatch: loop {
        match pc {
            0x832B9730 => {
    //   block [0x832B9730..0x832B9758)
	// 832B9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B973C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9740: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 832B9744: 4BD49F55  bl 0x83003698
	ctx.lr = 0x832B9748;
	sub_83003698(ctx, base);
	// 832B9748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B974C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9758 size=40
    let mut pc: u32 = 0x832B9758;
    'dispatch: loop {
        match pc {
            0x832B9758 => {
    //   block [0x832B9758..0x832B9780)
	// 832B9758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9764: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9768: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 832B976C: 4BD4BA05  bl 0x83005170
	ctx.lr = 0x832B9770;
	sub_83005170(ctx, base);
	// 832B9770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B977C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9780 size=40
    let mut pc: u32 = 0x832B9780;
    'dispatch: loop {
        match pc {
            0x832B9780 => {
    //   block [0x832B9780..0x832B97A8)
	// 832B9780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B978C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9790: 386B9B44  addi r3, r11, -0x64bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25788;
	// 832B9794: 4BD49E35  bl 0x830035c8
	ctx.lr = 0x832B9798;
	sub_830035C8(ctx, base);
	// 832B9798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B979C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B97A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B97A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B97A8 size=40
    let mut pc: u32 = 0x832B97A8;
    'dispatch: loop {
        match pc {
            0x832B97A8 => {
    //   block [0x832B97A8..0x832B97D0)
	// 832B97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B97AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B97B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B97B4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97B8: 386B9B68  addi r3, r11, -0x6498
	ctx.r[3].s64 = ctx.r[11].s64 + -25752;
	// 832B97BC: 4BD4CA7D  bl 0x83006238
	ctx.lr = 0x832B97C0;
	sub_83006238(ctx, base);
	// 832B97C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B97C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B97C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B97CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97D0 size=16
    let mut pc: u32 = 0x832B97D0;
    'dispatch: loop {
        match pc {
            0x832B97D0 => {
    //   block [0x832B97D0..0x832B97E0)
	// 832B97D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97D4: 396BB0A0  addi r11, r11, -0x4f60
	ctx.r[11].s64 = ctx.r[11].s64 + -20320;
	// 832B97D8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 832B97DC: 4BAC5F2C  b 0x82d7f708
	sub_82D7F708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97E0 size=20
    let mut pc: u32 = 0x832B97E0;
    'dispatch: loop {
        match pc {
            0x832B97E0 => {
    //   block [0x832B97E0..0x832B97F4)
	// 832B97E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B97E4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B97E8: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B97EC: 916A1F68  stw r11, 0x1f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8040 as u32), ctx.r[11].u32 ) };
	// 832B97F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97F8 size=52
    let mut pc: u32 = 0x832B97F8;
    'dispatch: loop {
        match pc {
            0x832B97F8 => {
    //   block [0x832B97F8..0x832B982C)
	// 832B97F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97FC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9800: 390BB4A0  addi r8, r11, -0x4b60
	ctx.r[8].s64 = ctx.r[11].s64 + -19296;
	// 832B9804: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 832B9808: 39680284  addi r11, r8, 0x284
	ctx.r[11].s64 = ctx.r[8].s64 + 644;
	// 832B980C: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	// 832B9810: 396BFFD8  addi r11, r11, -0x28
	ctx.r[11].s64 = ctx.r[11].s64 + -40;
	// 832B9814: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B9818: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832B981C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9820: 4080FFF0  bge 0x832b9810
	if !ctx.cr[0].lt {
	pc = 0x832B9810; continue 'dispatch;
	}
	// 832B9824: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9830 size=20
    let mut pc: u32 = 0x832B9830;
    'dispatch: loop {
        match pc {
            0x832B9830 => {
    //   block [0x832B9830..0x832B9844)
	// 832B9830: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9834: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9838: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B983C: 916AB38C  stw r11, -0x4c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19572 as u32), ctx.r[11].u32 ) };
	// 832B9840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9848 size=52
    let mut pc: u32 = 0x832B9848;
    'dispatch: loop {
        match pc {
            0x832B9848 => {
    //   block [0x832B9848..0x832B987C)
	// 832B9848: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B984C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9850: 390BB738  addi r8, r11, -0x48c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18632;
	// 832B9854: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 832B9858: 39680284  addi r11, r8, 0x284
	ctx.r[11].s64 = ctx.r[8].s64 + 644;
	// 832B985C: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	// 832B9860: 396BFFD8  addi r11, r11, -0x28
	ctx.r[11].s64 = ctx.r[11].s64 + -40;
	// 832B9864: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B9868: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832B986C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9870: 4080FFF0  bge 0x832b9860
	if !ctx.cr[0].lt {
	pc = 0x832B9860; continue 'dispatch;
	}
	// 832B9874: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9880 size=48
    let mut pc: u32 = 0x832B9880;
    'dispatch: loop {
        match pc {
            0x832B9880 => {
    //   block [0x832B9880..0x832B98B0)
	// 832B9880: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9884: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9888: 390BB9D0  addi r8, r11, -0x4630
	ctx.r[8].s64 = ctx.r[11].s64 + -17968;
	// 832B988C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 832B9890: 396800A4  addi r11, r8, 0xa4
	ctx.r[11].s64 = ctx.r[8].s64 + 164;
	// 832B9894: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	// 832B9898: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 832B989C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B98A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B98A4: 4080FFF4  bge 0x832b9898
	if !ctx.cr[0].lt {
	pc = 0x832B9898; continue 'dispatch;
	}
	// 832B98A8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B98AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98B0 size=20
    let mut pc: u32 = 0x832B98B0;
    'dispatch: loop {
        match pc {
            0x832B98B0 => {
    //   block [0x832B98B0..0x832B98C4)
	// 832B98B0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B98B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B98B8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B98BC: 916ABA88  stw r11, -0x4578(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17784 as u32), ctx.r[11].u32 ) };
	// 832B98C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98C8 size=16
    let mut pc: u32 = 0x832B98C8;
    'dispatch: loop {
        match pc {
            0x832B98C8 => {
    //   block [0x832B98C8..0x832B98D8)
	// 832B98C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B98CC: 806BBBA4  lwz r3, -0x445c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17500 as u32) ) } as u64;
	// 832B98D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B98D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98D8 size=8
    let mut pc: u32 = 0x832B98D8;
    'dispatch: loop {
        match pc {
            0x832B98D8 => {
    //   block [0x832B98D8..0x832B98E0)
	// 832B98D8: 4B8E8940  b 0x82ba2218
	sub_82BA2218(ctx, base);
	return;
	// 832B98DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98E0 size=4
    let mut pc: u32 = 0x832B98E0;
    'dispatch: loop {
        match pc {
            0x832B98E0 => {
    //   block [0x832B98E0..0x832B98E4)
	// 832B98E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98E4 size=16
    let mut pc: u32 = 0x832B98E4;
    'dispatch: loop {
        match pc {
            0x832B98E4 => {
    //   block [0x832B98E4..0x832B98F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98F4 size=16
    let mut pc: u32 = 0x832B98F4;
    'dispatch: loop {
        match pc {
            0x832B98F4 => {
    //   block [0x832B98F4..0x832B9904)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9904 size=16
    let mut pc: u32 = 0x832B9904;
    'dispatch: loop {
        match pc {
            0x832B9904 => {
    //   block [0x832B9904..0x832B9914)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9914 size=16
    let mut pc: u32 = 0x832B9914;
    'dispatch: loop {
        match pc {
            0x832B9914 => {
    //   block [0x832B9914..0x832B9924)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9924(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9924 size=16
    let mut pc: u32 = 0x832B9924;
    'dispatch: loop {
        match pc {
            0x832B9924 => {
    //   block [0x832B9924..0x832B9934)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9934 size=16
    let mut pc: u32 = 0x832B9934;
    'dispatch: loop {
        match pc {
            0x832B9934 => {
    //   block [0x832B9934..0x832B9944)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9944 size=16
    let mut pc: u32 = 0x832B9944;
    'dispatch: loop {
        match pc {
            0x832B9944 => {
    //   block [0x832B9944..0x832B9954)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9954 size=16
    let mut pc: u32 = 0x832B9954;
    'dispatch: loop {
        match pc {
            0x832B9954 => {
    //   block [0x832B9954..0x832B9964)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9964(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9964 size=16
    let mut pc: u32 = 0x832B9964;
    'dispatch: loop {
        match pc {
            0x832B9964 => {
    //   block [0x832B9964..0x832B9974)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9974 size=16
    let mut pc: u32 = 0x832B9974;
    'dispatch: loop {
        match pc {
            0x832B9974 => {
    //   block [0x832B9974..0x832B9984)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9984 size=16
    let mut pc: u32 = 0x832B9984;
    'dispatch: loop {
        match pc {
            0x832B9984 => {
    //   block [0x832B9984..0x832B9994)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9994 size=16
    let mut pc: u32 = 0x832B9994;
    'dispatch: loop {
        match pc {
            0x832B9994 => {
    //   block [0x832B9994..0x832B99A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99A4 size=16
    let mut pc: u32 = 0x832B99A4;
    'dispatch: loop {
        match pc {
            0x832B99A4 => {
    //   block [0x832B99A4..0x832B99B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99B4 size=16
    let mut pc: u32 = 0x832B99B4;
    'dispatch: loop {
        match pc {
            0x832B99B4 => {
    //   block [0x832B99B4..0x832B99C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99C4 size=16
    let mut pc: u32 = 0x832B99C4;
    'dispatch: loop {
        match pc {
            0x832B99C4 => {
    //   block [0x832B99C4..0x832B99D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99D4 size=16
    let mut pc: u32 = 0x832B99D4;
    'dispatch: loop {
        match pc {
            0x832B99D4 => {
    //   block [0x832B99D4..0x832B99E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99E4 size=16
    let mut pc: u32 = 0x832B99E4;
    'dispatch: loop {
        match pc {
            0x832B99E4 => {
    //   block [0x832B99E4..0x832B99F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99F4 size=16
    let mut pc: u32 = 0x832B99F4;
    'dispatch: loop {
        match pc {
            0x832B99F4 => {
    //   block [0x832B99F4..0x832B9A04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A04 size=16
    let mut pc: u32 = 0x832B9A04;
    'dispatch: loop {
        match pc {
            0x832B9A04 => {
    //   block [0x832B9A04..0x832B9A14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A14 size=16
    let mut pc: u32 = 0x832B9A14;
    'dispatch: loop {
        match pc {
            0x832B9A14 => {
    //   block [0x832B9A14..0x832B9A24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A24 size=16
    let mut pc: u32 = 0x832B9A24;
    'dispatch: loop {
        match pc {
            0x832B9A24 => {
    //   block [0x832B9A24..0x832B9A34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A34 size=16
    let mut pc: u32 = 0x832B9A34;
    'dispatch: loop {
        match pc {
            0x832B9A34 => {
    //   block [0x832B9A34..0x832B9A44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A44 size=16
    let mut pc: u32 = 0x832B9A44;
    'dispatch: loop {
        match pc {
            0x832B9A44 => {
    //   block [0x832B9A44..0x832B9A54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A54 size=16
    let mut pc: u32 = 0x832B9A54;
    'dispatch: loop {
        match pc {
            0x832B9A54 => {
    //   block [0x832B9A54..0x832B9A64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A64 size=16
    let mut pc: u32 = 0x832B9A64;
    'dispatch: loop {
        match pc {
            0x832B9A64 => {
    //   block [0x832B9A64..0x832B9A74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A74 size=16
    let mut pc: u32 = 0x832B9A74;
    'dispatch: loop {
        match pc {
            0x832B9A74 => {
    //   block [0x832B9A74..0x832B9A84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A84 size=16
    let mut pc: u32 = 0x832B9A84;
    'dispatch: loop {
        match pc {
            0x832B9A84 => {
    //   block [0x832B9A84..0x832B9A94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A94 size=16
    let mut pc: u32 = 0x832B9A94;
    'dispatch: loop {
        match pc {
            0x832B9A94 => {
    //   block [0x832B9A94..0x832B9AA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AA4 size=16
    let mut pc: u32 = 0x832B9AA4;
    'dispatch: loop {
        match pc {
            0x832B9AA4 => {
    //   block [0x832B9AA4..0x832B9AB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AB4 size=16
    let mut pc: u32 = 0x832B9AB4;
    'dispatch: loop {
        match pc {
            0x832B9AB4 => {
    //   block [0x832B9AB4..0x832B9AC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AC4 size=16
    let mut pc: u32 = 0x832B9AC4;
    'dispatch: loop {
        match pc {
            0x832B9AC4 => {
    //   block [0x832B9AC4..0x832B9AD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AD4 size=16
    let mut pc: u32 = 0x832B9AD4;
    'dispatch: loop {
        match pc {
            0x832B9AD4 => {
    //   block [0x832B9AD4..0x832B9AE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AE4 size=16
    let mut pc: u32 = 0x832B9AE4;
    'dispatch: loop {
        match pc {
            0x832B9AE4 => {
    //   block [0x832B9AE4..0x832B9AF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AF4 size=16
    let mut pc: u32 = 0x832B9AF4;
    'dispatch: loop {
        match pc {
            0x832B9AF4 => {
    //   block [0x832B9AF4..0x832B9B04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B04 size=16
    let mut pc: u32 = 0x832B9B04;
    'dispatch: loop {
        match pc {
            0x832B9B04 => {
    //   block [0x832B9B04..0x832B9B14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B14 size=16
    let mut pc: u32 = 0x832B9B14;
    'dispatch: loop {
        match pc {
            0x832B9B14 => {
    //   block [0x832B9B14..0x832B9B24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B24 size=16
    let mut pc: u32 = 0x832B9B24;
    'dispatch: loop {
        match pc {
            0x832B9B24 => {
    //   block [0x832B9B24..0x832B9B34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B34 size=16
    let mut pc: u32 = 0x832B9B34;
    'dispatch: loop {
        match pc {
            0x832B9B34 => {
    //   block [0x832B9B34..0x832B9B44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B44 size=16
    let mut pc: u32 = 0x832B9B44;
    'dispatch: loop {
        match pc {
            0x832B9B44 => {
    //   block [0x832B9B44..0x832B9B54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B54 size=16
    let mut pc: u32 = 0x832B9B54;
    'dispatch: loop {
        match pc {
            0x832B9B54 => {
    //   block [0x832B9B54..0x832B9B64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B64 size=16
    let mut pc: u32 = 0x832B9B64;
    'dispatch: loop {
        match pc {
            0x832B9B64 => {
    //   block [0x832B9B64..0x832B9B74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B74 size=16
    let mut pc: u32 = 0x832B9B74;
    'dispatch: loop {
        match pc {
            0x832B9B74 => {
    //   block [0x832B9B74..0x832B9B84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B84 size=16
    let mut pc: u32 = 0x832B9B84;
    'dispatch: loop {
        match pc {
            0x832B9B84 => {
    //   block [0x832B9B84..0x832B9B94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B94 size=16
    let mut pc: u32 = 0x832B9B94;
    'dispatch: loop {
        match pc {
            0x832B9B94 => {
    //   block [0x832B9B94..0x832B9BA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BA4 size=16
    let mut pc: u32 = 0x832B9BA4;
    'dispatch: loop {
        match pc {
            0x832B9BA4 => {
    //   block [0x832B9BA4..0x832B9BB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BB4 size=16
    let mut pc: u32 = 0x832B9BB4;
    'dispatch: loop {
        match pc {
            0x832B9BB4 => {
    //   block [0x832B9BB4..0x832B9BC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BC4 size=16
    let mut pc: u32 = 0x832B9BC4;
    'dispatch: loop {
        match pc {
            0x832B9BC4 => {
    //   block [0x832B9BC4..0x832B9BD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BD4 size=16
    let mut pc: u32 = 0x832B9BD4;
    'dispatch: loop {
        match pc {
            0x832B9BD4 => {
    //   block [0x832B9BD4..0x832B9BE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BE4 size=16
    let mut pc: u32 = 0x832B9BE4;
    'dispatch: loop {
        match pc {
            0x832B9BE4 => {
    //   block [0x832B9BE4..0x832B9BF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BF4 size=16
    let mut pc: u32 = 0x832B9BF4;
    'dispatch: loop {
        match pc {
            0x832B9BF4 => {
    //   block [0x832B9BF4..0x832B9C04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C04 size=16
    let mut pc: u32 = 0x832B9C04;
    'dispatch: loop {
        match pc {
            0x832B9C04 => {
    //   block [0x832B9C04..0x832B9C14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C14 size=16
    let mut pc: u32 = 0x832B9C14;
    'dispatch: loop {
        match pc {
            0x832B9C14 => {
    //   block [0x832B9C14..0x832B9C24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C24 size=16
    let mut pc: u32 = 0x832B9C24;
    'dispatch: loop {
        match pc {
            0x832B9C24 => {
    //   block [0x832B9C24..0x832B9C34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C34 size=16
    let mut pc: u32 = 0x832B9C34;
    'dispatch: loop {
        match pc {
            0x832B9C34 => {
    //   block [0x832B9C34..0x832B9C44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C44 size=16
    let mut pc: u32 = 0x832B9C44;
    'dispatch: loop {
        match pc {
            0x832B9C44 => {
    //   block [0x832B9C44..0x832B9C54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C54 size=16
    let mut pc: u32 = 0x832B9C54;
    'dispatch: loop {
        match pc {
            0x832B9C54 => {
    //   block [0x832B9C54..0x832B9C64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C64 size=16
    let mut pc: u32 = 0x832B9C64;
    'dispatch: loop {
        match pc {
            0x832B9C64 => {
    //   block [0x832B9C64..0x832B9C74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C74 size=16
    let mut pc: u32 = 0x832B9C74;
    'dispatch: loop {
        match pc {
            0x832B9C74 => {
    //   block [0x832B9C74..0x832B9C84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C84 size=16
    let mut pc: u32 = 0x832B9C84;
    'dispatch: loop {
        match pc {
            0x832B9C84 => {
    //   block [0x832B9C84..0x832B9C94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C94 size=16
    let mut pc: u32 = 0x832B9C94;
    'dispatch: loop {
        match pc {
            0x832B9C94 => {
    //   block [0x832B9C94..0x832B9CA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CA4 size=16
    let mut pc: u32 = 0x832B9CA4;
    'dispatch: loop {
        match pc {
            0x832B9CA4 => {
    //   block [0x832B9CA4..0x832B9CB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CB4 size=16
    let mut pc: u32 = 0x832B9CB4;
    'dispatch: loop {
        match pc {
            0x832B9CB4 => {
    //   block [0x832B9CB4..0x832B9CC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CC4 size=16
    let mut pc: u32 = 0x832B9CC4;
    'dispatch: loop {
        match pc {
            0x832B9CC4 => {
    //   block [0x832B9CC4..0x832B9CD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CD4 size=16
    let mut pc: u32 = 0x832B9CD4;
    'dispatch: loop {
        match pc {
            0x832B9CD4 => {
    //   block [0x832B9CD4..0x832B9CE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CE4 size=16
    let mut pc: u32 = 0x832B9CE4;
    'dispatch: loop {
        match pc {
            0x832B9CE4 => {
    //   block [0x832B9CE4..0x832B9CF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CF4 size=16
    let mut pc: u32 = 0x832B9CF4;
    'dispatch: loop {
        match pc {
            0x832B9CF4 => {
    //   block [0x832B9CF4..0x832B9D04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D04 size=16
    let mut pc: u32 = 0x832B9D04;
    'dispatch: loop {
        match pc {
            0x832B9D04 => {
    //   block [0x832B9D04..0x832B9D14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D14 size=16
    let mut pc: u32 = 0x832B9D14;
    'dispatch: loop {
        match pc {
            0x832B9D14 => {
    //   block [0x832B9D14..0x832B9D24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D24 size=16
    let mut pc: u32 = 0x832B9D24;
    'dispatch: loop {
        match pc {
            0x832B9D24 => {
    //   block [0x832B9D24..0x832B9D34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D34 size=16
    let mut pc: u32 = 0x832B9D34;
    'dispatch: loop {
        match pc {
            0x832B9D34 => {
    //   block [0x832B9D34..0x832B9D44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D44 size=16
    let mut pc: u32 = 0x832B9D44;
    'dispatch: loop {
        match pc {
            0x832B9D44 => {
    //   block [0x832B9D44..0x832B9D54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D54 size=16
    let mut pc: u32 = 0x832B9D54;
    'dispatch: loop {
        match pc {
            0x832B9D54 => {
    //   block [0x832B9D54..0x832B9D64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D64 size=16
    let mut pc: u32 = 0x832B9D64;
    'dispatch: loop {
        match pc {
            0x832B9D64 => {
    //   block [0x832B9D64..0x832B9D74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D74 size=16
    let mut pc: u32 = 0x832B9D74;
    'dispatch: loop {
        match pc {
            0x832B9D74 => {
    //   block [0x832B9D74..0x832B9D84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D84 size=16
    let mut pc: u32 = 0x832B9D84;
    'dispatch: loop {
        match pc {
            0x832B9D84 => {
    //   block [0x832B9D84..0x832B9D94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D94 size=16
    let mut pc: u32 = 0x832B9D94;
    'dispatch: loop {
        match pc {
            0x832B9D94 => {
    //   block [0x832B9D94..0x832B9DA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DA4 size=16
    let mut pc: u32 = 0x832B9DA4;
    'dispatch: loop {
        match pc {
            0x832B9DA4 => {
    //   block [0x832B9DA4..0x832B9DB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DB4 size=16
    let mut pc: u32 = 0x832B9DB4;
    'dispatch: loop {
        match pc {
            0x832B9DB4 => {
    //   block [0x832B9DB4..0x832B9DC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DC4 size=16
    let mut pc: u32 = 0x832B9DC4;
    'dispatch: loop {
        match pc {
            0x832B9DC4 => {
    //   block [0x832B9DC4..0x832B9DD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DD4 size=16
    let mut pc: u32 = 0x832B9DD4;
    'dispatch: loop {
        match pc {
            0x832B9DD4 => {
    //   block [0x832B9DD4..0x832B9DE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DE4 size=16
    let mut pc: u32 = 0x832B9DE4;
    'dispatch: loop {
        match pc {
            0x832B9DE4 => {
    //   block [0x832B9DE4..0x832B9DF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DF4 size=16
    let mut pc: u32 = 0x832B9DF4;
    'dispatch: loop {
        match pc {
            0x832B9DF4 => {
    //   block [0x832B9DF4..0x832B9E04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E04 size=16
    let mut pc: u32 = 0x832B9E04;
    'dispatch: loop {
        match pc {
            0x832B9E04 => {
    //   block [0x832B9E04..0x832B9E14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E14 size=16
    let mut pc: u32 = 0x832B9E14;
    'dispatch: loop {
        match pc {
            0x832B9E14 => {
    //   block [0x832B9E14..0x832B9E24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E24 size=16
    let mut pc: u32 = 0x832B9E24;
    'dispatch: loop {
        match pc {
            0x832B9E24 => {
    //   block [0x832B9E24..0x832B9E34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E34 size=16
    let mut pc: u32 = 0x832B9E34;
    'dispatch: loop {
        match pc {
            0x832B9E34 => {
    //   block [0x832B9E34..0x832B9E44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E44 size=16
    let mut pc: u32 = 0x832B9E44;
    'dispatch: loop {
        match pc {
            0x832B9E44 => {
    //   block [0x832B9E44..0x832B9E54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E54 size=16
    let mut pc: u32 = 0x832B9E54;
    'dispatch: loop {
        match pc {
            0x832B9E54 => {
    //   block [0x832B9E54..0x832B9E64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E64 size=16
    let mut pc: u32 = 0x832B9E64;
    'dispatch: loop {
        match pc {
            0x832B9E64 => {
    //   block [0x832B9E64..0x832B9E74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E74 size=16
    let mut pc: u32 = 0x832B9E74;
    'dispatch: loop {
        match pc {
            0x832B9E74 => {
    //   block [0x832B9E74..0x832B9E84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E84 size=16
    let mut pc: u32 = 0x832B9E84;
    'dispatch: loop {
        match pc {
            0x832B9E84 => {
    //   block [0x832B9E84..0x832B9E94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


