pub fn sub_83232068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232068 size=20
    let mut pc: u32 = 0x83232068;
    'dispatch: loop {
        match pc {
            0x83232068 => {
    //   block [0x83232068..0x8323207C)
	// 83232068: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8323206C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232070: 388BD28C  addi r4, r11, -0x2d74
	ctx.r[4].s64 = ctx.r[11].s64 + -11636;
	// 83232074: 386A6634  addi r3, r10, 0x6634
	ctx.r[3].s64 = ctx.r[10].s64 + 26164;
	// 83232078: 4BBC74B8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232080 size=20
    let mut pc: u32 = 0x83232080;
    'dispatch: loop {
        match pc {
            0x83232080 => {
    //   block [0x83232080..0x83232094)
	// 83232080: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83232084: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232088: 388BAC50  addi r4, r11, -0x53b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21424;
	// 8323208C: 386A6638  addi r3, r10, 0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + 26168;
	// 83232090: 4BBC74A0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232098 size=20
    let mut pc: u32 = 0x83232098;
    'dispatch: loop {
        match pc {
            0x83232098 => {
    //   block [0x83232098..0x832320AC)
	// 83232098: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323209C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832320A0: 388BBD94  addi r4, r11, -0x426c
	ctx.r[4].s64 = ctx.r[11].s64 + -17004;
	// 832320A4: 386A663C  addi r3, r10, 0x663c
	ctx.r[3].s64 = ctx.r[10].s64 + 26172;
	// 832320A8: 4BBC7488  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832320B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832320B0 size=20
    let mut pc: u32 = 0x832320B0;
    'dispatch: loop {
        match pc {
            0x832320B0 => {
    //   block [0x832320B0..0x832320C4)
	// 832320B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832320B4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832320B8: 388B296C  addi r4, r11, 0x296c
	ctx.r[4].s64 = ctx.r[11].s64 + 10604;
	// 832320BC: 386A6640  addi r3, r10, 0x6640
	ctx.r[3].s64 = ctx.r[10].s64 + 26176;
	// 832320C0: 4BBC7470  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832320C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832320C8 size=20
    let mut pc: u32 = 0x832320C8;
    'dispatch: loop {
        match pc {
            0x832320C8 => {
    //   block [0x832320C8..0x832320DC)
	// 832320C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832320CC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832320D0: 388BBD9C  addi r4, r11, -0x4264
	ctx.r[4].s64 = ctx.r[11].s64 + -16996;
	// 832320D4: 386A6644  addi r3, r10, 0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + 26180;
	// 832320D8: 4BBC7458  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832320E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832320E0 size=20
    let mut pc: u32 = 0x832320E0;
    'dispatch: loop {
        match pc {
            0x832320E0 => {
    //   block [0x832320E0..0x832320F4)
	// 832320E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832320E4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832320E8: 388BBDA8  addi r4, r11, -0x4258
	ctx.r[4].s64 = ctx.r[11].s64 + -16984;
	// 832320EC: 386A6648  addi r3, r10, 0x6648
	ctx.r[3].s64 = ctx.r[10].s64 + 26184;
	// 832320F0: 4BBC7440  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832320F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832320F8 size=20
    let mut pc: u32 = 0x832320F8;
    'dispatch: loop {
        match pc {
            0x832320F8 => {
    //   block [0x832320F8..0x8323210C)
	// 832320F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832320FC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232100: 388BBDBC  addi r4, r11, -0x4244
	ctx.r[4].s64 = ctx.r[11].s64 + -16964;
	// 83232104: 386A664C  addi r3, r10, 0x664c
	ctx.r[3].s64 = ctx.r[10].s64 + 26188;
	// 83232108: 4BBC7428  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232110 size=20
    let mut pc: u32 = 0x83232110;
    'dispatch: loop {
        match pc {
            0x83232110 => {
    //   block [0x83232110..0x83232124)
	// 83232110: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232114: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232118: 388BC3E4  addi r4, r11, -0x3c1c
	ctx.r[4].s64 = ctx.r[11].s64 + -15388;
	// 8323211C: 386A6660  addi r3, r10, 0x6660
	ctx.r[3].s64 = ctx.r[10].s64 + 26208;
	// 83232120: 4BBC7410  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232128 size=20
    let mut pc: u32 = 0x83232128;
    'dispatch: loop {
        match pc {
            0x83232128 => {
    //   block [0x83232128..0x8323213C)
	// 83232128: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323212C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232130: 388BC3F0  addi r4, r11, -0x3c10
	ctx.r[4].s64 = ctx.r[11].s64 + -15376;
	// 83232134: 386A6664  addi r3, r10, 0x6664
	ctx.r[3].s64 = ctx.r[10].s64 + 26212;
	// 83232138: 4BBC73F8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232140 size=20
    let mut pc: u32 = 0x83232140;
    'dispatch: loop {
        match pc {
            0x83232140 => {
    //   block [0x83232140..0x83232154)
	// 83232140: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232144: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232148: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8323214C: 386A6668  addi r3, r10, 0x6668
	ctx.r[3].s64 = ctx.r[10].s64 + 26216;
	// 83232150: 4BBC73E0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232158 size=20
    let mut pc: u32 = 0x83232158;
    'dispatch: loop {
        match pc {
            0x83232158 => {
    //   block [0x83232158..0x8323216C)
	// 83232158: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323215C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232160: 388BC408  addi r4, r11, -0x3bf8
	ctx.r[4].s64 = ctx.r[11].s64 + -15352;
	// 83232164: 386A666C  addi r3, r10, 0x666c
	ctx.r[3].s64 = ctx.r[10].s64 + 26220;
	// 83232168: 4BBC73C8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232170 size=20
    let mut pc: u32 = 0x83232170;
    'dispatch: loop {
        match pc {
            0x83232170 => {
    //   block [0x83232170..0x83232184)
	// 83232170: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232174: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232178: 388BC414  addi r4, r11, -0x3bec
	ctx.r[4].s64 = ctx.r[11].s64 + -15340;
	// 8323217C: 386A6670  addi r3, r10, 0x6670
	ctx.r[3].s64 = ctx.r[10].s64 + 26224;
	// 83232180: 4BBC73B0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232188 size=20
    let mut pc: u32 = 0x83232188;
    'dispatch: loop {
        match pc {
            0x83232188 => {
    //   block [0x83232188..0x8323219C)
	// 83232188: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323218C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232190: 388BC420  addi r4, r11, -0x3be0
	ctx.r[4].s64 = ctx.r[11].s64 + -15328;
	// 83232194: 386A6674  addi r3, r10, 0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + 26228;
	// 83232198: 4BBC7398  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832321A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832321A0 size=20
    let mut pc: u32 = 0x832321A0;
    'dispatch: loop {
        match pc {
            0x832321A0 => {
    //   block [0x832321A0..0x832321B4)
	// 832321A0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832321A4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832321A8: 388BC42C  addi r4, r11, -0x3bd4
	ctx.r[4].s64 = ctx.r[11].s64 + -15316;
	// 832321AC: 386A6678  addi r3, r10, 0x6678
	ctx.r[3].s64 = ctx.r[10].s64 + 26232;
	// 832321B0: 4BBC7380  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832321B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832321B8 size=20
    let mut pc: u32 = 0x832321B8;
    'dispatch: loop {
        match pc {
            0x832321B8 => {
    //   block [0x832321B8..0x832321CC)
	// 832321B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832321BC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832321C0: 388BC438  addi r4, r11, -0x3bc8
	ctx.r[4].s64 = ctx.r[11].s64 + -15304;
	// 832321C4: 386A667C  addi r3, r10, 0x667c
	ctx.r[3].s64 = ctx.r[10].s64 + 26236;
	// 832321C8: 4BBC7368  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832321D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832321D0 size=20
    let mut pc: u32 = 0x832321D0;
    'dispatch: loop {
        match pc {
            0x832321D0 => {
    //   block [0x832321D0..0x832321E4)
	// 832321D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832321D4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832321D8: 388BC444  addi r4, r11, -0x3bbc
	ctx.r[4].s64 = ctx.r[11].s64 + -15292;
	// 832321DC: 386A6680  addi r3, r10, 0x6680
	ctx.r[3].s64 = ctx.r[10].s64 + 26240;
	// 832321E0: 4BBC7350  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832321E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832321E8 size=20
    let mut pc: u32 = 0x832321E8;
    'dispatch: loop {
        match pc {
            0x832321E8 => {
    //   block [0x832321E8..0x832321FC)
	// 832321E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832321EC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832321F0: 388BC450  addi r4, r11, -0x3bb0
	ctx.r[4].s64 = ctx.r[11].s64 + -15280;
	// 832321F4: 386A6684  addi r3, r10, 0x6684
	ctx.r[3].s64 = ctx.r[10].s64 + 26244;
	// 832321F8: 4BBC7338  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232200 size=20
    let mut pc: u32 = 0x83232200;
    'dispatch: loop {
        match pc {
            0x83232200 => {
    //   block [0x83232200..0x83232214)
	// 83232200: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232204: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232208: 388BC45C  addi r4, r11, -0x3ba4
	ctx.r[4].s64 = ctx.r[11].s64 + -15268;
	// 8323220C: 386A6688  addi r3, r10, 0x6688
	ctx.r[3].s64 = ctx.r[10].s64 + 26248;
	// 83232210: 4BBC7320  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232218 size=20
    let mut pc: u32 = 0x83232218;
    'dispatch: loop {
        match pc {
            0x83232218 => {
    //   block [0x83232218..0x8323222C)
	// 83232218: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323221C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232220: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 83232224: 386A668C  addi r3, r10, 0x668c
	ctx.r[3].s64 = ctx.r[10].s64 + 26252;
	// 83232228: 4BBC7308  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232230 size=20
    let mut pc: u32 = 0x83232230;
    'dispatch: loop {
        match pc {
            0x83232230 => {
    //   block [0x83232230..0x83232244)
	// 83232230: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232234: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232238: 388BC474  addi r4, r11, -0x3b8c
	ctx.r[4].s64 = ctx.r[11].s64 + -15244;
	// 8323223C: 386A6690  addi r3, r10, 0x6690
	ctx.r[3].s64 = ctx.r[10].s64 + 26256;
	// 83232240: 4BBC72F0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232248 size=20
    let mut pc: u32 = 0x83232248;
    'dispatch: loop {
        match pc {
            0x83232248 => {
    //   block [0x83232248..0x8323225C)
	// 83232248: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323224C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232250: 388BC480  addi r4, r11, -0x3b80
	ctx.r[4].s64 = ctx.r[11].s64 + -15232;
	// 83232254: 386A6694  addi r3, r10, 0x6694
	ctx.r[3].s64 = ctx.r[10].s64 + 26260;
	// 83232258: 4BBC72D8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232260 size=20
    let mut pc: u32 = 0x83232260;
    'dispatch: loop {
        match pc {
            0x83232260 => {
    //   block [0x83232260..0x83232274)
	// 83232260: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232264: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232268: 388BC48C  addi r4, r11, -0x3b74
	ctx.r[4].s64 = ctx.r[11].s64 + -15220;
	// 8323226C: 386A6698  addi r3, r10, 0x6698
	ctx.r[3].s64 = ctx.r[10].s64 + 26264;
	// 83232270: 4BBC72C0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232278 size=20
    let mut pc: u32 = 0x83232278;
    'dispatch: loop {
        match pc {
            0x83232278 => {
    //   block [0x83232278..0x8323228C)
	// 83232278: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323227C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232280: 388BC498  addi r4, r11, -0x3b68
	ctx.r[4].s64 = ctx.r[11].s64 + -15208;
	// 83232284: 386A669C  addi r3, r10, 0x669c
	ctx.r[3].s64 = ctx.r[10].s64 + 26268;
	// 83232288: 4BBC72A8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232290 size=20
    let mut pc: u32 = 0x83232290;
    'dispatch: loop {
        match pc {
            0x83232290 => {
    //   block [0x83232290..0x832322A4)
	// 83232290: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232294: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232298: 388BC4A4  addi r4, r11, -0x3b5c
	ctx.r[4].s64 = ctx.r[11].s64 + -15196;
	// 8323229C: 386A66A0  addi r3, r10, 0x66a0
	ctx.r[3].s64 = ctx.r[10].s64 + 26272;
	// 832322A0: 4BBC7290  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832322A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832322A8 size=20
    let mut pc: u32 = 0x832322A8;
    'dispatch: loop {
        match pc {
            0x832322A8 => {
    //   block [0x832322A8..0x832322BC)
	// 832322A8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832322AC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832322B0: 388BC4AC  addi r4, r11, -0x3b54
	ctx.r[4].s64 = ctx.r[11].s64 + -15188;
	// 832322B4: 386A66A4  addi r3, r10, 0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26276;
	// 832322B8: 4BBC7278  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832322C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832322C0 size=20
    let mut pc: u32 = 0x832322C0;
    'dispatch: loop {
        match pc {
            0x832322C0 => {
    //   block [0x832322C0..0x832322D4)
	// 832322C0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832322C4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832322C8: 388BC4BC  addi r4, r11, -0x3b44
	ctx.r[4].s64 = ctx.r[11].s64 + -15172;
	// 832322CC: 386A66A8  addi r3, r10, 0x66a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26280;
	// 832322D0: 4BBC7260  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832322D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832322D8 size=20
    let mut pc: u32 = 0x832322D8;
    'dispatch: loop {
        match pc {
            0x832322D8 => {
    //   block [0x832322D8..0x832322EC)
	// 832322D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832322DC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832322E0: 388BC4CC  addi r4, r11, -0x3b34
	ctx.r[4].s64 = ctx.r[11].s64 + -15156;
	// 832322E4: 386A66AC  addi r3, r10, 0x66ac
	ctx.r[3].s64 = ctx.r[10].s64 + 26284;
	// 832322E8: 4BBC7248  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832322F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832322F0 size=20
    let mut pc: u32 = 0x832322F0;
    'dispatch: loop {
        match pc {
            0x832322F0 => {
    //   block [0x832322F0..0x83232304)
	// 832322F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832322F4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832322F8: 388BAC50  addi r4, r11, -0x53b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21424;
	// 832322FC: 386A66B0  addi r3, r10, 0x66b0
	ctx.r[3].s64 = ctx.r[10].s64 + 26288;
	// 83232300: 4BBC7230  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232308 size=20
    let mut pc: u32 = 0x83232308;
    'dispatch: loop {
        match pc {
            0x83232308 => {
    //   block [0x83232308..0x8323231C)
	// 83232308: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323230C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232310: 388BBD94  addi r4, r11, -0x426c
	ctx.r[4].s64 = ctx.r[11].s64 + -17004;
	// 83232314: 386A66B4  addi r3, r10, 0x66b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26292;
	// 83232318: 4BBC7218  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232320 size=20
    let mut pc: u32 = 0x83232320;
    'dispatch: loop {
        match pc {
            0x83232320 => {
    //   block [0x83232320..0x83232334)
	// 83232320: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 83232324: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232328: 388B296C  addi r4, r11, 0x296c
	ctx.r[4].s64 = ctx.r[11].s64 + 10604;
	// 8323232C: 386A66B8  addi r3, r10, 0x66b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26296;
	// 83232330: 4BBC7200  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232338 size=20
    let mut pc: u32 = 0x83232338;
    'dispatch: loop {
        match pc {
            0x83232338 => {
    //   block [0x83232338..0x8323234C)
	// 83232338: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323233C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232340: 388BBD9C  addi r4, r11, -0x4264
	ctx.r[4].s64 = ctx.r[11].s64 + -16996;
	// 83232344: 386A66BC  addi r3, r10, 0x66bc
	ctx.r[3].s64 = ctx.r[10].s64 + 26300;
	// 83232348: 4BBC71E8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232350 size=20
    let mut pc: u32 = 0x83232350;
    'dispatch: loop {
        match pc {
            0x83232350 => {
    //   block [0x83232350..0x83232364)
	// 83232350: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83232354: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232358: 388BBDA8  addi r4, r11, -0x4258
	ctx.r[4].s64 = ctx.r[11].s64 + -16984;
	// 8323235C: 386A66C0  addi r3, r10, 0x66c0
	ctx.r[3].s64 = ctx.r[10].s64 + 26304;
	// 83232360: 4BBC71D0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232368 size=20
    let mut pc: u32 = 0x83232368;
    'dispatch: loop {
        match pc {
            0x83232368 => {
    //   block [0x83232368..0x8323237C)
	// 83232368: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323236C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232370: 388BBDBC  addi r4, r11, -0x4244
	ctx.r[4].s64 = ctx.r[11].s64 + -16964;
	// 83232374: 386A66C4  addi r3, r10, 0x66c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26308;
	// 83232378: 4BBC71B8  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232380 size=388
    let mut pc: u32 = 0x83232380;
    'dispatch: loop {
        match pc {
            0x83232380 => {
    //   block [0x83232380..0x83232504)
	// 83232380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232384: 4BF75DB1  bl 0x831a8134
	ctx.lr = 0x83232388;
	sub_831A8130(ctx, base);
	// 83232388: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8323238C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232390: 3CC08332  lis r6, -0x7cce
	ctx.r[6].s64 = -2093875200;
	// 83232394: 3C608338  lis r3, -0x7cc8
	ctx.r[3].s64 = -2093481984;
	// 83232398: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 8323239C: 810B6660  lwz r8, 0x6660(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26208 as u32) ) } as u64;
	// 832323A0: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832323A4: 816A66B0  lwz r11, 0x66b0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26288 as u32) ) } as u64;
	// 832323A8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832323AC: 3B26AD98  addi r25, r6, -0x5268
	ctx.r[25].s64 = ctx.r[6].s64 + -21096;
	// 832323B0: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832323B4: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
	// 832323B8: 9106AD98  stw r8, -0x5268(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-21096 as u32), ctx.r[8].u32 ) };
	// 832323BC: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832323C0: 80CA6668  lwz r6, 0x6668(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26216 as u32) ) } as u64;
	// 832323C4: 3C808338  lis r4, -0x7cc8
	ctx.r[4].s64 = -2093481984;
	// 832323C8: 814366B4  lwz r10, 0x66b4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(26292 as u32) ) } as u64;
	// 832323CC: 3FC08338  lis r30, -0x7cc8
	ctx.r[30].s64 = -2093481984;
	// 832323D0: 807F6674  lwz r3, 0x6674(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26228 as u32) ) } as u64;
	// 832323D4: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 832323D8: 812966B8  lwz r9, 0x66b8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26296 as u32) ) } as u64;
	// 832323DC: 3F808338  lis r28, -0x7cc8
	ctx.r[28].s64 = -2093481984;
	// 832323E0: 83E86678  lwz r31, 0x6678(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(26232 as u32) ) } as u64;
	// 832323E4: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832323E8: 3F608338  lis r27, -0x7cc8
	ctx.r[27].s64 = -2093481984;
	// 832323EC: 80E76664  lwz r7, 0x6664(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(26212 as u32) ) } as u64;
	// 832323F0: 3F408338  lis r26, -0x7cc8
	ctx.r[26].s64 = -2093481984;
	// 832323F4: 80A5666C  lwz r5, 0x666c(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(26220 as u32) ) } as u64;
	// 832323F8: 3F008338  lis r24, -0x7cc8
	ctx.r[24].s64 = -2093481984;
	// 832323FC: 80846670  lwz r4, 0x6670(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(26224 as u32) ) } as u64;
	// 83232400: 3EE08338  lis r23, -0x7cc8
	ctx.r[23].s64 = -2093481984;
	// 83232404: 91390084  stw r9, 0x84(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 83232408: 91390094  stw r9, 0x94(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 8323240C: 3EC08338  lis r22, -0x7cc8
	ctx.r[22].s64 = -2093481984;
	// 83232410: 913900A4  stw r9, 0xa4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 83232414: 3EA08338  lis r21, -0x7cc8
	ctx.r[21].s64 = -2093481984;
	// 83232418: 913900B4  stw r9, 0xb4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 8323241C: 3E808338  lis r20, -0x7cc8
	ctx.r[20].s64 = -2093481984;
	// 83232420: 83DE667C  lwz r30, 0x667c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26236 as u32) ) } as u64;
	// 83232424: 3E608338  lis r19, -0x7cc8
	ctx.r[19].s64 = -2093481984;
	// 83232428: 81286690  lwz r9, 0x6690(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(26256 as u32) ) } as u64;
	// 8323242C: 3E408338  lis r18, -0x7cc8
	ctx.r[18].s64 = -2093481984;
	// 83232430: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83232434: 3E208338  lis r17, -0x7cc8
	ctx.r[17].s64 = -2093481984;
	// 83232438: 91790014  stw r11, 0x14(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8323243C: 3E008338  lis r16, -0x7cc8
	ctx.r[16].s64 = -2093481984;
	// 83232440: 91790024  stw r11, 0x24(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83232444: 3DE08338  lis r15, -0x7cc8
	ctx.r[15].s64 = -2093481984;
	// 83232448: 91790034  stw r11, 0x34(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8323244C: 83BD6680  lwz r29, 0x6680(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(26240 as u32) ) } as u64;
	// 83232450: 839C6684  lwz r28, 0x6684(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(26244 as u32) ) } as u64;
	// 83232454: 837B6688  lwz r27, 0x6688(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(26248 as u32) ) } as u64;
	// 83232458: 835A668C  lwz r26, 0x668c(r26)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(26252 as u32) ) } as u64;
	// 8323245C: 817866BC  lwz r11, 0x66bc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(26300 as u32) ) } as u64;
	// 83232460: 81176694  lwz r8, 0x6694(r23)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(26260 as u32) ) } as u64;
	// 83232464: 90F90010  stw r7, 0x10(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 83232468: 90D90020  stw r6, 0x20(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8323246C: 90B90030  stw r5, 0x30(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 83232470: 90990040  stw r4, 0x40(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 83232474: 91590044  stw r10, 0x44(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 83232478: 90790050  stw r3, 0x50(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8323247C: 91590054  stw r10, 0x54(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83232480: 93F90060  stw r31, 0x60(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 83232484: 91590064  stw r10, 0x64(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 83232488: 93D90070  stw r30, 0x70(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 8323248C: 91590074  stw r10, 0x74(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83232490: 80F66698  lwz r7, 0x6698(r22)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26264 as u32) ) } as u64;
	// 83232494: 80D5669C  lwz r6, 0x669c(r21)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(26268 as u32) ) } as u64;
	// 83232498: 80B466A4  lwz r5, 0x66a4(r20)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(26276 as u32) ) } as u64;
	// 8323249C: 815366C0  lwz r10, 0x66c0(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(26304 as u32) ) } as u64;
	// 832324A0: 809266A8  lwz r4, 0x66a8(r18)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(26280 as u32) ) } as u64;
	// 832324A4: 807166AC  lwz r3, 0x66ac(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(26284 as u32) ) } as u64;
	// 832324A8: 83F066A0  lwz r31, 0x66a0(r16)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(26272 as u32) ) } as u64;
	// 832324AC: 83CF66C4  lwz r30, 0x66c4(r15)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(26308 as u32) ) } as u64;
	// 832324B0: 93B90080  stw r29, 0x80(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 832324B4: 93990090  stw r28, 0x90(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 832324B8: 937900A0  stw r27, 0xa0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 832324BC: 935900B0  stw r26, 0xb0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(176 as u32), ctx.r[26].u32 ) };
	// 832324C0: 913900C0  stw r9, 0xc0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 832324C4: 917900C4  stw r11, 0xc4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 832324C8: 911900D0  stw r8, 0xd0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 832324CC: 917900D4  stw r11, 0xd4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 832324D0: 90F900E0  stw r7, 0xe0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(224 as u32), ctx.r[7].u32 ) };
	// 832324D4: 917900E4  stw r11, 0xe4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832324D8: 90D900F0  stw r6, 0xf0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(240 as u32), ctx.r[6].u32 ) };
	// 832324DC: 917900F4  stw r11, 0xf4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 832324E0: 90B90100  stw r5, 0x100(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(256 as u32), ctx.r[5].u32 ) };
	// 832324E4: 91590104  stw r10, 0x104(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 832324E8: 90990110  stw r4, 0x110(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(272 as u32), ctx.r[4].u32 ) };
	// 832324EC: 91590114  stw r10, 0x114(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(276 as u32), ctx.r[10].u32 ) };
	// 832324F0: 90790120  stw r3, 0x120(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(288 as u32), ctx.r[3].u32 ) };
	// 832324F4: 91590124  stw r10, 0x124(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(292 as u32), ctx.r[10].u32 ) };
	// 832324F8: 93F90130  stw r31, 0x130(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(304 as u32), ctx.r[31].u32 ) };
	// 832324FC: 93D90134  stw r30, 0x134(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 83232500: 4BF75C84  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232508 size=88
    let mut pc: u32 = 0x83232508;
    'dispatch: loop {
        match pc {
            0x83232508 => {
    //   block [0x83232508..0x83232560)
	// 83232508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83232514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83232518: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323251C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83232520: 3BE0000B  li r31, 0xb
	ctx.r[31].s64 = 11;
	// 83232524: 3BCB67AC  addi r30, r11, 0x67ac
	ctx.r[30].s64 = ctx.r[11].s64 + 26540;
	// 83232528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8323252C: 4BBC0BC5  bl 0x82df30f0
	ctx.lr = 0x83232530;
	sub_82DF30F0(ctx, base);
	// 83232530: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83232534: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83232538: 4080FFF0  bge 0x83232528
	if !ctx.cr[0].lt {
	pc = 0x83232528; continue 'dispatch;
	}
	// 8323253C: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 83232540: 386B1DA8  addi r3, r11, 0x1da8
	ctx.r[3].s64 = ctx.r[11].s64 + 7592;
	// 83232544: 4BF75F95  bl 0x831a84d8
	ctx.lr = 0x83232548;
	sub_831A84D8(ctx, base);
	// 83232548: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323254C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232554: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83232558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232560 size=88
    let mut pc: u32 = 0x83232560;
    'dispatch: loop {
        match pc {
            0x83232560 => {
    //   block [0x83232560..0x832325B8)
	// 83232560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323256C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83232570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232574: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83232578: 3BE0000B  li r31, 0xb
	ctx.r[31].s64 = 11;
	// 8323257C: 3BCB67DC  addi r30, r11, 0x67dc
	ctx.r[30].s64 = ctx.r[11].s64 + 26588;
	// 83232580: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83232584: 4BBC0B6D  bl 0x82df30f0
	ctx.lr = 0x83232588;
	sub_82DF30F0(ctx, base);
	// 83232588: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8323258C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83232590: 4080FFF0  bge 0x83232580
	if !ctx.cr[0].lt {
	pc = 0x83232580; continue 'dispatch;
	}
	// 83232594: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 83232598: 386B1DF8  addi r3, r11, 0x1df8
	ctx.r[3].s64 = ctx.r[11].s64 + 7672;
	// 8323259C: 4BF75F3D  bl 0x831a84d8
	ctx.lr = 0x832325A0;
	sub_831A84D8(ctx, base);
	// 832325A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832325A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832325A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832325AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832325B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832325B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832325B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832325B8 size=12
    let mut pc: u32 = 0x832325B8;
    'dispatch: loop {
        match pc {
            0x832325B8 => {
    //   block [0x832325B8..0x832325C4)
	// 832325B8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 832325BC: 386B1E58  addi r3, r11, 0x1e58
	ctx.r[3].s64 = ctx.r[11].s64 + 7768;
	// 832325C0: 4BF75F18  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832325C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832325C8 size=96
    let mut pc: u32 = 0x832325C8;
    'dispatch: loop {
        match pc {
            0x832325C8 => {
    //   block [0x832325C8..0x83232628)
	// 832325C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832325CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832325D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832325D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832325D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832325DC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832325E0: 38696880  addi r3, r9, 0x6880
	ctx.r[3].s64 = ctx.r[9].s64 + 26752;
	// 832325E4: C18B08A4  lfs f12, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832325E8: C16A08A8  lfs f11, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832325EC: FD406090  fmr f10, f12
	ctx.f[10].f64 = ctx.f[12].f64;
	// 832325F0: FD206090  fmr f9, f12
	ctx.f[9].f64 = ctx.f[12].f64;
	// 832325F4: FD006090  fmr f8, f12
	ctx.f[8].f64 = ctx.f[12].f64;
	// 832325F8: FCE06090  fmr f7, f12
	ctx.f[7].f64 = ctx.f[12].f64;
	// 832325FC: FCC05890  fmr f6, f11
	ctx.f[6].f64 = ctx.f[11].f64;
	// 83232600: FCA06090  fmr f5, f12
	ctx.f[5].f64 = ctx.f[12].f64;
	// 83232604: FC806090  fmr f4, f12
	ctx.f[4].f64 = ctx.f[12].f64;
	// 83232608: FC606090  fmr f3, f12
	ctx.f[3].f64 = ctx.f[12].f64;
	// 8323260C: FC406090  fmr f2, f12
	ctx.f[2].f64 = ctx.f[12].f64;
	// 83232610: FC205890  fmr f1, f11
	ctx.f[1].f64 = ctx.f[11].f64;
	// 83232614: 4B0B5ECD  bl 0x822e84e0
	ctx.lr = 0x83232618;
	sub_822E84E0(ctx, base);
	// 83232618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323261C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83232628 size=92
    let mut pc: u32 = 0x83232628;
    'dispatch: loop {
        match pc {
            0x83232628 => {
    //   block [0x83232628..0x83232684)
	// 83232628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323262C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232634: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83232638: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323263C: 386A68C0  addi r3, r10, 0x68c0
	ctx.r[3].s64 = ctx.r[10].s64 + 26816;
	// 83232640: C18B08A4  lfs f12, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83232644: FD606090  fmr f11, f12
	ctx.f[11].f64 = ctx.f[12].f64;
	// 83232648: FD406090  fmr f10, f12
	ctx.f[10].f64 = ctx.f[12].f64;
	// 8323264C: FD206090  fmr f9, f12
	ctx.f[9].f64 = ctx.f[12].f64;
	// 83232650: FD006090  fmr f8, f12
	ctx.f[8].f64 = ctx.f[12].f64;
	// 83232654: FCE06090  fmr f7, f12
	ctx.f[7].f64 = ctx.f[12].f64;
	// 83232658: FCC06090  fmr f6, f12
	ctx.f[6].f64 = ctx.f[12].f64;
	// 8323265C: FCA06090  fmr f5, f12
	ctx.f[5].f64 = ctx.f[12].f64;
	// 83232660: FC806090  fmr f4, f12
	ctx.f[4].f64 = ctx.f[12].f64;
	// 83232664: FC606090  fmr f3, f12
	ctx.f[3].f64 = ctx.f[12].f64;
	// 83232668: FC406090  fmr f2, f12
	ctx.f[2].f64 = ctx.f[12].f64;
	// 8323266C: FC206090  fmr f1, f12
	ctx.f[1].f64 = ctx.f[12].f64;
	// 83232670: 4B0B5E71  bl 0x822e84e0
	ctx.lr = 0x83232674;
	sub_822E84E0(ctx, base);
	// 83232674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323267C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232688 size=20
    let mut pc: u32 = 0x83232688;
    'dispatch: loop {
        match pc {
            0x83232688 => {
    //   block [0x83232688..0x8323269C)
	// 83232688: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8323268C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232690: 388BCE10  addi r4, r11, -0x31f0
	ctx.r[4].s64 = ctx.r[11].s64 + -12784;
	// 83232694: 386A693C  addi r3, r10, 0x693c
	ctx.r[3].s64 = ctx.r[10].s64 + 26940;
	// 83232698: 4BBC6E98  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832326A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832326A0 size=20
    let mut pc: u32 = 0x832326A0;
    'dispatch: loop {
        match pc {
            0x832326A0 => {
    //   block [0x832326A0..0x832326B4)
	// 832326A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832326A4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832326A8: 388B9430  addi r4, r11, -0x6bd0
	ctx.r[4].s64 = ctx.r[11].s64 + -27600;
	// 832326AC: 386A6948  addi r3, r10, 0x6948
	ctx.r[3].s64 = ctx.r[10].s64 + 26952;
	// 832326B0: 4BBC6E80  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832326B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832326B8 size=20
    let mut pc: u32 = 0x832326B8;
    'dispatch: loop {
        match pc {
            0x832326B8 => {
    //   block [0x832326B8..0x832326CC)
	// 832326B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832326BC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832326C0: 388BC508  addi r4, r11, -0x3af8
	ctx.r[4].s64 = ctx.r[11].s64 + -15096;
	// 832326C4: 386A694C  addi r3, r10, 0x694c
	ctx.r[3].s64 = ctx.r[10].s64 + 26956;
	// 832326C8: 4BBC6E68  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832326D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832326D0 size=20
    let mut pc: u32 = 0x832326D0;
    'dispatch: loop {
        match pc {
            0x832326D0 => {
    //   block [0x832326D0..0x832326E4)
	// 832326D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832326D4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832326D8: 388B9428  addi r4, r11, -0x6bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -27608;
	// 832326DC: 386A6950  addi r3, r10, 0x6950
	ctx.r[3].s64 = ctx.r[10].s64 + 26960;
	// 832326E0: 4BBC6E50  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832326E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832326E8 size=20
    let mut pc: u32 = 0x832326E8;
    'dispatch: loop {
        match pc {
            0x832326E8 => {
    //   block [0x832326E8..0x832326FC)
	// 832326E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832326EC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832326F0: 388BCE10  addi r4, r11, -0x31f0
	ctx.r[4].s64 = ctx.r[11].s64 + -12784;
	// 832326F4: 386A6974  addi r3, r10, 0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + 26996;
	// 832326F8: 4BBC6E38  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232700 size=20
    let mut pc: u32 = 0x83232700;
    'dispatch: loop {
        match pc {
            0x83232700 => {
    //   block [0x83232700..0x83232714)
	// 83232700: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83232704: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232708: 388B9428  addi r4, r11, -0x6bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -27608;
	// 8323270C: 386A6978  addi r3, r10, 0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + 27000;
	// 83232710: 4BBC6E20  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232718 size=20
    let mut pc: u32 = 0x83232718;
    'dispatch: loop {
        match pc {
            0x83232718 => {
    //   block [0x83232718..0x8323272C)
	// 83232718: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8323271C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232720: 388B9430  addi r4, r11, -0x6bd0
	ctx.r[4].s64 = ctx.r[11].s64 + -27600;
	// 83232724: 386A697C  addi r3, r10, 0x697c
	ctx.r[3].s64 = ctx.r[10].s64 + 27004;
	// 83232728: 4BBC6E08  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232730 size=20
    let mut pc: u32 = 0x83232730;
    'dispatch: loop {
        match pc {
            0x83232730 => {
    //   block [0x83232730..0x83232744)
	// 83232730: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83232734: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232738: 388BC508  addi r4, r11, -0x3af8
	ctx.r[4].s64 = ctx.r[11].s64 + -15096;
	// 8323273C: 386A6980  addi r3, r10, 0x6980
	ctx.r[3].s64 = ctx.r[10].s64 + 27008;
	// 83232740: 4BBC6DF0  b 0x82df9530
	sub_82DF9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232748 size=12
    let mut pc: u32 = 0x83232748;
    'dispatch: loop {
        match pc {
            0x83232748 => {
    //   block [0x83232748..0x83232754)
	// 83232748: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8323274C: 386B69A8  addi r3, r11, 0x69a8
	ctx.r[3].s64 = ctx.r[11].s64 + 27048;
	// 83232750: 4BF69130  b 0x8319b880
	sub_8319B880(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232758 size=76
    let mut pc: u32 = 0x83232758;
    'dispatch: loop {
        match pc {
            0x83232758 => {
    //   block [0x83232758..0x832327A4)
	// 83232758: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323275C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83232760: 392BBD70  addi r9, r11, -0x4290
	ctx.r[9].s64 = ctx.r[11].s64 + -17040;
	// 83232764: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83232768: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 8323276C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83232770: 38AA69F0  addi r5, r10, 0x69f0
	ctx.r[5].s64 = ctx.r[10].s64 + 27120;
	// 83232774: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832327A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832327A8 size=32
    let mut pc: u32 = 0x832327A8;
    'dispatch: loop {
        match pc {
            0x832327A8 => {
    //   block [0x832327A8..0x832327C8)
	// 832327A8: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832327AC: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 832327B0: 38E8BEE0  addi r7, r8, -0x4120
	ctx.r[7].s64 = ctx.r[8].s64 + -16672;
	// 832327B4: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 832327B8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832327BC: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 832327C0: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832327C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832327C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832327C8 size=108
    let mut pc: u32 = 0x832327C8;
    'dispatch: loop {
        match pc {
            0x832327C8 => {
    //   block [0x832327C8..0x83232834)
	// 832327C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832327CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832327D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832327D4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832327D8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832327DC: 38EAEDE8  addi r7, r10, -0x1218
	ctx.r[7].s64 = ctx.r[10].s64 + -4632;
	// 832327E0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832327E4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 832327E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832327EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832327F0: 3889EE60  addi r4, r9, -0x11a0
	ctx.r[4].s64 = ctx.r[9].s64 + -4512;
	// 832327F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832327F8: 38686A3C  addi r3, r8, 0x6a3c
	ctx.r[3].s64 = ctx.r[8].s64 + 27196;
	// 832327FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232804: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232808: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323280C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232818: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323281C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232820: 4BC759A9  bl 0x82ea81c8
	ctx.lr = 0x83232824;
	sub_82EA81C8(ctx, base);
	// 83232824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323282C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232838 size=108
    let mut pc: u32 = 0x83232838;
    'dispatch: loop {
        match pc {
            0x83232838 => {
    //   block [0x83232838..0x832328A4)
	// 83232838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323283C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232844: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232848: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323284C: 38EAEE74  addi r7, r10, -0x118c
	ctx.r[7].s64 = ctx.r[10].s64 + -4492;
	// 83232850: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232854: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83232858: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323285C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232860: 3889EE8C  addi r4, r9, -0x1174
	ctx.r[4].s64 = ctx.r[9].s64 + -4468;
	// 83232864: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232868: 38686A6C  addi r3, r8, 0x6a6c
	ctx.r[3].s64 = ctx.r[8].s64 + 27244;
	// 8323286C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232874: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232878: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323287C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232880: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232888: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323288C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232890: 4BC75939  bl 0x82ea81c8
	ctx.lr = 0x83232894;
	sub_82EA81C8(ctx, base);
	// 83232894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323289C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832328A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832328A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832328A8 size=108
    let mut pc: u32 = 0x832328A8;
    'dispatch: loop {
        match pc {
            0x832328A8 => {
    //   block [0x832328A8..0x83232914)
	// 832328A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832328AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832328B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832328B4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832328B8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832328BC: 38EAEEB8  addi r7, r10, -0x1148
	ctx.r[7].s64 = ctx.r[10].s64 + -4424;
	// 832328C0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832328C4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832328C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832328CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832328D0: 3889EF78  addi r4, r9, -0x1088
	ctx.r[4].s64 = ctx.r[9].s64 + -4232;
	// 832328D4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832328D8: 38686A9C  addi r3, r8, 0x6a9c
	ctx.r[3].s64 = ctx.r[8].s64 + 27292;
	// 832328DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832328E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832328E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832328E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832328EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832328F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832328F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832328F8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832328FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232900: 4BC758C9  bl 0x82ea81c8
	ctx.lr = 0x83232904;
	sub_82EA81C8(ctx, base);
	// 83232904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323290C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232918 size=108
    let mut pc: u32 = 0x83232918;
    'dispatch: loop {
        match pc {
            0x83232918 => {
    //   block [0x83232918..0x83232984)
	// 83232918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232924: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232928: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323292C: 38EAEF18  addi r7, r10, -0x10e8
	ctx.r[7].s64 = ctx.r[10].s64 + -4328;
	// 83232930: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232934: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83232938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323293C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232940: 3889EF90  addi r4, r9, -0x1070
	ctx.r[4].s64 = ctx.r[9].s64 + -4208;
	// 83232944: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232948: 38686ACC  addi r3, r8, 0x6acc
	ctx.r[3].s64 = ctx.r[8].s64 + 27340;
	// 8323294C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232954: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232958: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323295C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232960: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232968: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323296C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232970: 4BC75859  bl 0x82ea81c8
	ctx.lr = 0x83232974;
	sub_82EA81C8(ctx, base);
	// 83232974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323297C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232988 size=112
    let mut pc: u32 = 0x83232988;
    'dispatch: loop {
        match pc {
            0x83232988 => {
    //   block [0x83232988..0x832329F8)
	// 83232988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232994: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232998: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323299C: 38CAF090  addi r6, r10, -0xf70
	ctx.r[6].s64 = ctx.r[10].s64 + -3952;
	// 832329A0: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832329A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832329A8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832329AC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832329B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832329B4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 832329B8: 3888F0D8  addi r4, r8, -0xf28
	ctx.r[4].s64 = ctx.r[8].s64 + -3880;
	// 832329BC: 38676AFC  addi r3, r7, 0x6afc
	ctx.r[3].s64 = ctx.r[7].s64 + 27388;
	// 832329C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832329C4: 3929F068  addi r9, r9, -0xf98
	ctx.r[9].s64 = ctx.r[9].s64 + -3992;
	// 832329C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832329CC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832329D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832329D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832329D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832329DC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832329E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832329E4: 4BC757E5  bl 0x82ea81c8
	ctx.lr = 0x832329E8;
	sub_82EA81C8(ctx, base);
	// 832329E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832329EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832329F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832329F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832329F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832329F8 size=108
    let mut pc: u32 = 0x832329F8;
    'dispatch: loop {
        match pc {
            0x832329F8 => {
    //   block [0x832329F8..0x83232A64)
	// 832329F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832329FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232A04: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232A08: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232A0C: 38EAF180  addi r7, r10, -0xe80
	ctx.r[7].s64 = ctx.r[10].s64 + -3712;
	// 83232A10: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232A14: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83232A18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232A1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232A20: 3889F258  addi r4, r9, -0xda8
	ctx.r[4].s64 = ctx.r[9].s64 + -3496;
	// 83232A24: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232A28: 38686B2C  addi r3, r8, 0x6b2c
	ctx.r[3].s64 = ctx.r[8].s64 + 27436;
	// 83232A2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232A34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232A38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232A3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232A40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232A48: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 83232A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232A50: 4BC75779  bl 0x82ea81c8
	ctx.lr = 0x83232A54;
	sub_82EA81C8(ctx, base);
	// 83232A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232A68 size=108
    let mut pc: u32 = 0x83232A68;
    'dispatch: loop {
        match pc {
            0x83232A68 => {
    //   block [0x83232A68..0x83232AD4)
	// 83232A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232A74: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232A78: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232A7C: 38EAF304  addi r7, r10, -0xcfc
	ctx.r[7].s64 = ctx.r[10].s64 + -3324;
	// 83232A80: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232A84: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83232A88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232A8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232A90: 3889F378  addi r4, r9, -0xc88
	ctx.r[4].s64 = ctx.r[9].s64 + -3208;
	// 83232A94: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232A98: 38686B5C  addi r3, r8, 0x6b5c
	ctx.r[3].s64 = ctx.r[8].s64 + 27484;
	// 83232A9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232AA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232AA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232AAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232AB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232AB8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83232ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232AC0: 4BC75709  bl 0x82ea81c8
	ctx.lr = 0x83232AC4;
	sub_82EA81C8(ctx, base);
	// 83232AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232AD8 size=108
    let mut pc: u32 = 0x83232AD8;
    'dispatch: loop {
        match pc {
            0x83232AD8 => {
    //   block [0x83232AD8..0x83232B44)
	// 83232AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232AE4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232AE8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232AEC: 38EAF334  addi r7, r10, -0xccc
	ctx.r[7].s64 = ctx.r[10].s64 + -3276;
	// 83232AF0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232AF4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83232AF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232AFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232B00: 3889F39C  addi r4, r9, -0xc64
	ctx.r[4].s64 = ctx.r[9].s64 + -3172;
	// 83232B04: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232B08: 38686B8C  addi r3, r8, 0x6b8c
	ctx.r[3].s64 = ctx.r[8].s64 + 27532;
	// 83232B0C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232B14: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232B18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232B1C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232B20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232B28: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83232B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232B30: 4BC75699  bl 0x82ea81c8
	ctx.lr = 0x83232B34;
	sub_82EA81C8(ctx, base);
	// 83232B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232B48 size=24
    let mut pc: u32 = 0x83232B48;
    'dispatch: loop {
        match pc {
            0x83232B48 => {
    //   block [0x83232B48..0x83232B60)
	// 83232B48: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83232B4C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232B50: 392ABF90  addi r9, r10, -0x4070
	ctx.r[9].s64 = ctx.r[10].s64 + -16496;
	// 83232B54: 816BBF58  lwz r11, -0x40a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16552 as u32) ) } as u64;
	// 83232B58: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83232B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232B60 size=112
    let mut pc: u32 = 0x83232B60;
    'dispatch: loop {
        match pc {
            0x83232B60 => {
    //   block [0x83232B60..0x83232BD0)
	// 83232B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232B6C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232B70: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83232B74: 38CABF90  addi r6, r10, -0x4070
	ctx.r[6].s64 = ctx.r[10].s64 + -16496;
	// 83232B78: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83232B7C: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83232B80: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83232B84: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232B8C: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83232B90: 3888F3B8  addi r4, r8, -0xc48
	ctx.r[4].s64 = ctx.r[8].s64 + -3144;
	// 83232B94: 38676BBC  addi r3, r7, 0x6bbc
	ctx.r[3].s64 = ctx.r[7].s64 + 27580;
	// 83232B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232B9C: 3929F364  addi r9, r9, -0xc9c
	ctx.r[9].s64 = ctx.r[9].s64 + -3228;
	// 83232BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232BA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83232BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232BB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232BB4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 83232BB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232BBC: 4BC7560D  bl 0x82ea81c8
	ctx.lr = 0x83232BC0;
	sub_82EA81C8(ctx, base);
	// 83232BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232BD0 size=108
    let mut pc: u32 = 0x83232BD0;
    'dispatch: loop {
        match pc {
            0x83232BD0 => {
    //   block [0x83232BD0..0x83232C3C)
	// 83232BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232BDC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232BE0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232BE4: 38EAF424  addi r7, r10, -0xbdc
	ctx.r[7].s64 = ctx.r[10].s64 + -3036;
	// 83232BE8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232BEC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83232BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232BF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232BF8: 3889F46C  addi r4, r9, -0xb94
	ctx.r[4].s64 = ctx.r[9].s64 + -2964;
	// 83232BFC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232C00: 38686BEC  addi r3, r8, 0x6bec
	ctx.r[3].s64 = ctx.r[8].s64 + 27628;
	// 83232C04: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232C0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232C10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232C14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232C18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232C20: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83232C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232C28: 4BC755A1  bl 0x82ea81c8
	ctx.lr = 0x83232C2C;
	sub_82EA81C8(ctx, base);
	// 83232C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232C40 size=108
    let mut pc: u32 = 0x83232C40;
    'dispatch: loop {
        match pc {
            0x83232C40 => {
    //   block [0x83232C40..0x83232CAC)
	// 83232C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232C4C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232C50: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232C54: 38EAF454  addi r7, r10, -0xbac
	ctx.r[7].s64 = ctx.r[10].s64 + -2988;
	// 83232C58: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232C5C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83232C60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232C64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232C68: 3889F488  addi r4, r9, -0xb78
	ctx.r[4].s64 = ctx.r[9].s64 + -2936;
	// 83232C6C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232C70: 38686C1C  addi r3, r8, 0x6c1c
	ctx.r[3].s64 = ctx.r[8].s64 + 27676;
	// 83232C74: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232C7C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232C80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232C84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232C88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232C90: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83232C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232C98: 4BC75531  bl 0x82ea81c8
	ctx.lr = 0x83232C9C;
	sub_82EA81C8(ctx, base);
	// 83232C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232CB0 size=112
    let mut pc: u32 = 0x83232CB0;
    'dispatch: loop {
        match pc {
            0x83232CB0 => {
    //   block [0x83232CB0..0x83232D20)
	// 83232CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232CBC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232CC0: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83232CC4: 38CAF568  addi r6, r10, -0xa98
	ctx.r[6].s64 = ctx.r[10].s64 + -2712;
	// 83232CC8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83232CCC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83232CD0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83232CD4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232CDC: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83232CE0: 3888F5E0  addi r4, r8, -0xa20
	ctx.r[4].s64 = ctx.r[8].s64 + -2592;
	// 83232CE4: 38676C4C  addi r3, r7, 0x6c4c
	ctx.r[3].s64 = ctx.r[7].s64 + 27724;
	// 83232CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232CEC: 3929F550  addi r9, r9, -0xab0
	ctx.r[9].s64 = ctx.r[9].s64 + -2736;
	// 83232CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232CF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83232CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232D00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232D04: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83232D08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232D0C: 4BC754BD  bl 0x82ea81c8
	ctx.lr = 0x83232D10;
	sub_82EA81C8(ctx, base);
	// 83232D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232D20 size=108
    let mut pc: u32 = 0x83232D20;
    'dispatch: loop {
        match pc {
            0x83232D20 => {
    //   block [0x83232D20..0x83232D8C)
	// 83232D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232D2C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232D30: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232D34: 38EAF60C  addi r7, r10, -0x9f4
	ctx.r[7].s64 = ctx.r[10].s64 + -2548;
	// 83232D38: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232D3C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83232D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232D48: 3889F63C  addi r4, r9, -0x9c4
	ctx.r[4].s64 = ctx.r[9].s64 + -2500;
	// 83232D4C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232D50: 38686C7C  addi r3, r8, 0x6c7c
	ctx.r[3].s64 = ctx.r[8].s64 + 27772;
	// 83232D54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232D5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232D60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232D64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232D68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232D70: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83232D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232D78: 4BC75451  bl 0x82ea81c8
	ctx.lr = 0x83232D7C;
	sub_82EA81C8(ctx, base);
	// 83232D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232D90 size=40
    let mut pc: u32 = 0x83232D90;
    'dispatch: loop {
        match pc {
            0x83232D90 => {
    //   block [0x83232D90..0x83232DB8)
	// 83232D90: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83232D94: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232D98: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 83232D9C: 3909C0D0  addi r8, r9, -0x3f30
	ctx.r[8].s64 = ctx.r[9].s64 + -16176;
	// 83232DA0: 816BC0B0  lwz r11, -0x3f50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16208 as u32) ) } as u64;
	// 83232DA4: 814AC0B4  lwz r10, -0x3f4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16204 as u32) ) } as u64;
	// 83232DA8: 91680050  stw r11, 0x50(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83232DAC: 91680068  stw r11, 0x68(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83232DB0: 91480098  stw r10, 0x98(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 83232DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232DB8 size=112
    let mut pc: u32 = 0x83232DB8;
    'dispatch: loop {
        match pc {
            0x83232DB8 => {
    //   block [0x83232DB8..0x83232E28)
	// 83232DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232DC4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232DC8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83232DCC: 38CAC0D0  addi r6, r10, -0x3f30
	ctx.r[6].s64 = ctx.r[10].s64 + -16176;
	// 83232DD0: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83232DD4: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83232DD8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83232DDC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232DE4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83232DE8: 3888FAA4  addi r4, r8, -0x55c
	ctx.r[4].s64 = ctx.r[8].s64 + -1372;
	// 83232DEC: 38676CAC  addi r3, r7, 0x6cac
	ctx.r[3].s64 = ctx.r[7].s64 + 27820;
	// 83232DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232DF4: 3929FA68  addi r9, r9, -0x598
	ctx.r[9].s64 = ctx.r[9].s64 + -1432;
	// 83232DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232DFC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83232E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232E08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232E0C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83232E10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232E14: 4BC753B5  bl 0x82ea81c8
	ctx.lr = 0x83232E18;
	sub_82EA81C8(ctx, base);
	// 83232E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232E28 size=108
    let mut pc: u32 = 0x83232E28;
    'dispatch: loop {
        match pc {
            0x83232E28 => {
    //   block [0x83232E28..0x83232E94)
	// 83232E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232E34: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232E38: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232E3C: 38EAFAF4  addi r7, r10, -0x50c
	ctx.r[7].s64 = ctx.r[10].s64 + -1292;
	// 83232E40: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232E44: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83232E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232E50: 3889FB24  addi r4, r9, -0x4dc
	ctx.r[4].s64 = ctx.r[9].s64 + -1244;
	// 83232E54: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232E58: 38686CDC  addi r3, r8, 0x6cdc
	ctx.r[3].s64 = ctx.r[8].s64 + 27868;
	// 83232E5C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232E64: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232E68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232E6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232E70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232E78: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83232E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232E80: 4BC75349  bl 0x82ea81c8
	ctx.lr = 0x83232E84;
	sub_82EA81C8(ctx, base);
	// 83232E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232E98 size=24
    let mut pc: u32 = 0x83232E98;
    'dispatch: loop {
        match pc {
            0x83232E98 => {
    //   block [0x83232E98..0x83232EB0)
	// 83232E98: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83232E9C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232EA0: 392AC1D0  addi r9, r10, -0x3e30
	ctx.r[9].s64 = ctx.r[10].s64 + -15920;
	// 83232EA4: 816BC1A8  lwz r11, -0x3e58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15960 as u32) ) } as u64;
	// 83232EA8: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83232EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232EB0 size=112
    let mut pc: u32 = 0x83232EB0;
    'dispatch: loop {
        match pc {
            0x83232EB0 => {
    //   block [0x83232EB0..0x83232F20)
	// 83232EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232EBC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232EC0: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83232EC4: 38CAC1D0  addi r6, r10, -0x3e30
	ctx.r[6].s64 = ctx.r[10].s64 + -15920;
	// 83232EC8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83232ECC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83232ED0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83232ED4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232EDC: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83232EE0: 3888FB34  addi r4, r8, -0x4cc
	ctx.r[4].s64 = ctx.r[8].s64 + -1228;
	// 83232EE4: 38676D0C  addi r3, r7, 0x6d0c
	ctx.r[3].s64 = ctx.r[7].s64 + 27916;
	// 83232EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232EEC: 3929FAE0  addi r9, r9, -0x520
	ctx.r[9].s64 = ctx.r[9].s64 + -1312;
	// 83232EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232EF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83232EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232F00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232F04: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83232F08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232F0C: 4BC752BD  bl 0x82ea81c8
	ctx.lr = 0x83232F10;
	sub_82EA81C8(ctx, base);
	// 83232F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83232F20 size=24
    let mut pc: u32 = 0x83232F20;
    'dispatch: loop {
        match pc {
            0x83232F20 => {
    //   block [0x83232F20..0x83232F38)
	// 83232F20: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83232F24: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232F28: 392AC248  addi r9, r10, -0x3db8
	ctx.r[9].s64 = ctx.r[10].s64 + -15800;
	// 83232F2C: 816BC234  lwz r11, -0x3dcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15820 as u32) ) } as u64;
	// 83232F30: 916900C8  stw r11, 0xc8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83232F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232F38 size=112
    let mut pc: u32 = 0x83232F38;
    'dispatch: loop {
        match pc {
            0x83232F38 => {
    //   block [0x83232F38..0x83232FA8)
	// 83232F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232F44: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83232F48: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83232F4C: 38CAC248  addi r6, r10, -0x3db8
	ctx.r[6].s64 = ctx.r[10].s64 + -15800;
	// 83232F50: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83232F54: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 83232F58: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83232F5C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232F64: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83232F68: 3888FC14  addi r4, r8, -0x3ec
	ctx.r[4].s64 = ctx.r[8].s64 + -1004;
	// 83232F6C: 38676D3C  addi r3, r7, 0x6d3c
	ctx.r[3].s64 = ctx.r[7].s64 + 27964;
	// 83232F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232F74: 3929FBEC  addi r9, r9, -0x414
	ctx.r[9].s64 = ctx.r[9].s64 + -1044;
	// 83232F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232F7C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83232F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232F88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232F8C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83232F90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83232F94: 4BC75235  bl 0x82ea81c8
	ctx.lr = 0x83232F98;
	sub_82EA81C8(ctx, base);
	// 83232F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83232F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83232FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83232FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83232FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83232FA8 size=108
    let mut pc: u32 = 0x83232FA8;
    'dispatch: loop {
        match pc {
            0x83232FA8 => {
    //   block [0x83232FA8..0x83233014)
	// 83232FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83232FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83232FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83232FB4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83232FB8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83232FBC: 38EAFC48  addi r7, r10, -0x3b8
	ctx.r[7].s64 = ctx.r[10].s64 + -952;
	// 83232FC0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83232FC4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83232FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83232FCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83232FD0: 3889FC78  addi r4, r9, -0x388
	ctx.r[4].s64 = ctx.r[9].s64 + -904;
	// 83232FD4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83232FD8: 38686D6C  addi r3, r8, 0x6d6c
	ctx.r[3].s64 = ctx.r[8].s64 + 28012;
	// 83232FDC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83232FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83232FE4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83232FE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83232FEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83232FF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83232FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83232FF8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83232FFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233000: 4BC751C9  bl 0x82ea81c8
	ctx.lr = 0x83233004;
	sub_82EA81C8(ctx, base);
	// 83233004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233018 size=108
    let mut pc: u32 = 0x83233018;
    'dispatch: loop {
        match pc {
            0x83233018 => {
    //   block [0x83233018..0x83233084)
	// 83233018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233024: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233028: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323302C: 38EAFCC0  addi r7, r10, -0x340
	ctx.r[7].s64 = ctx.r[10].s64 + -832;
	// 83233030: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233034: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83233038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323303C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233040: 3889FD80  addi r4, r9, -0x280
	ctx.r[4].s64 = ctx.r[9].s64 + -640;
	// 83233044: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233048: 38686D9C  addi r3, r8, 0x6d9c
	ctx.r[3].s64 = ctx.r[8].s64 + 28060;
	// 8323304C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233054: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233058: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323305C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233060: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233068: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323306C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233070: 4BC75159  bl 0x82ea81c8
	ctx.lr = 0x83233074;
	sub_82EA81C8(ctx, base);
	// 83233074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323307C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233088 size=108
    let mut pc: u32 = 0x83233088;
    'dispatch: loop {
        match pc {
            0x83233088 => {
    //   block [0x83233088..0x832330F4)
	// 83233088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233094: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233098: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323309C: 38EAFCF0  addi r7, r10, -0x310
	ctx.r[7].s64 = ctx.r[10].s64 + -784;
	// 832330A0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832330A4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 832330A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832330AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832330B0: 3889FD88  addi r4, r9, -0x278
	ctx.r[4].s64 = ctx.r[9].s64 + -632;
	// 832330B4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832330B8: 38686DCC  addi r3, r8, 0x6dcc
	ctx.r[3].s64 = ctx.r[8].s64 + 28108;
	// 832330BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832330C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832330C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832330C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832330CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832330D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832330D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832330D8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832330DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832330E0: 4BC750E9  bl 0x82ea81c8
	ctx.lr = 0x832330E4;
	sub_82EA81C8(ctx, base);
	// 832330E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832330E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832330EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832330F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832330F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832330F8 size=120
    let mut pc: u32 = 0x832330F8;
    'dispatch: loop {
        match pc {
            0x832330F8 => {
    //   block [0x832330F8..0x83233170)
	// 832330F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832330FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83233104: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233108: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323310C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83233110: 38CAFDB8  addi r6, r10, -0x248
	ctx.r[6].s64 = ctx.r[10].s64 + -584;
	// 83233114: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83233118: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323311C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83233120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233124: 38A96E2C  addi r5, r9, 0x6e2c
	ctx.r[5].s64 = ctx.r[9].s64 + 28204;
	// 83233128: 3888FDE8  addi r4, r8, -0x218
	ctx.r[4].s64 = ctx.r[8].s64 + -536;
	// 8323312C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233130: 38676DFC  addi r3, r7, 0x6dfc
	ctx.r[3].s64 = ctx.r[7].s64 + 28156;
	// 83233134: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233138: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323313C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233148: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323314C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233154: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233158: 4BC75071  bl 0x82ea81c8
	ctx.lr = 0x8323315C;
	sub_82EA81C8(ctx, base);
	// 8323315C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83233160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233170 size=96
    let mut pc: u32 = 0x83233170;
    'dispatch: loop {
        match pc {
            0x83233170 => {
    //   block [0x83233170..0x832331D0)
	// 83233170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323317C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233180: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83233184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233188: 388AE7F4  addi r4, r10, -0x180c
	ctx.r[4].s64 = ctx.r[10].s64 + -6156;
	// 8323318C: 38696E2C  addi r3, r9, 0x6e2c
	ctx.r[3].s64 = ctx.r[9].s64 + 28204;
	// 83233190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323319C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832331A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832331A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832331A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832331AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832331B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832331B4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832331B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832331BC: 4BC7500D  bl 0x82ea81c8
	ctx.lr = 0x832331C0;
	sub_82EA81C8(ctx, base);
	// 832331C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832331C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832331C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832331CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832331D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832331D0 size=536
    let mut pc: u32 = 0x832331D0;
    'dispatch: loop {
        match pc {
            0x832331D0 => {
    //   block [0x832331D0..0x832333E8)
	// 832331D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832331D4: 4BF74F75  bl 0x831a8148
	ctx.lr = 0x832331D8;
	sub_831A8130(ctx, base);
	// 832331D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832331DC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832331E0: 3921FF20  addi r9, r1, -0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + -224;
	// 832331E4: 9141FF20  stw r10, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[10].u32 ) };
	// 832331E8: 3901FF30  addi r8, r1, -0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + -208;
	// 832331EC: 9141FF24  stw r10, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[10].u32 ) };
	// 832331F0: 38E1FF40  addi r7, r1, -0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + -192;
	// 832331F4: 9141FF28  stw r10, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[10].u32 ) };
	// 832331F8: 9141FF2C  stw r10, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[10].u32 ) };
	// 832331FC: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 83233200: 9141FF30  stw r10, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[10].u32 ) };
	// 83233204: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83233208: 9141FF34  stw r10, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[10].u32 ) };
	// 8323320C: 3BC1FF70  addi r30, r1, -0x90
	ctx.r[30].s64 = ctx.r[1].s64 + -144;
	// 83233210: 9141FF38  stw r10, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[10].u32 ) };
	// 83233214: 3B61FF80  addi r27, r1, -0x80
	ctx.r[27].s64 = ctx.r[1].s64 + -128;
	// 83233218: 9161FF3C  stw r11, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[11].u32 ) };
	// 8323321C: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 83233220: 9141FF40  stw r10, -0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), ctx.r[10].u32 ) };
	// 83233224: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83233228: 9141FF44  stw r10, -0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), ctx.r[10].u32 ) };
	// 8323322C: 38866E60  addi r4, r6, 0x6e60
	ctx.r[4].s64 = ctx.r[6].s64 + 28256;
	// 83233230: 9161FF48  stw r11, -0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), ctx.r[11].u32 ) };
	// 83233234: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83233238: 9141FF4C  stw r10, -0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), ctx.r[10].u32 ) };
	// 8323323C: 3BA00030  li r29, 0x30
	ctx.r[29].s64 = 48;
	// 83233240: 9141FF50  stw r10, -0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.r[10].u32 ) };
	// 83233244: 3B81FF30  addi r28, r1, -0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + -208;
	// 83233248: 9141FF54  stw r10, -0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), ctx.r[10].u32 ) };
	// 8323324C: 3B41FF20  addi r26, r1, -0xe0
	ctx.r[26].s64 = ctx.r[1].s64 + -224;
	// 83233250: 9161FF58  stw r11, -0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.r[11].u32 ) };
	// 83233254: 3B200040  li r25, 0x40
	ctx.r[25].s64 = 64;
	// 83233258: 9161FF5C  stw r11, -0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), ctx.r[11].u32 ) };
	// 8323325C: 3B000050  li r24, 0x50
	ctx.r[24].s64 = 80;
	// 83233260: 9161FF7C  stw r11, -0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), ctx.r[11].u32 ) };
	// 83233264: 3AE1FF40  addi r23, r1, -0xc0
	ctx.r[23].s64 = ctx.r[1].s64 + -192;
	// 83233268: 9141FF60  stw r10, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[10].u32 ) };
	// 8323326C: 3AC00060  li r22, 0x60
	ctx.r[22].s64 = 96;
	// 83233270: 9161FF64  stw r11, -0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), ctx.r[11].u32 ) };
	// 83233274: 3AA00070  li r21, 0x70
	ctx.r[21].s64 = 112;
	// 83233278: 9141FF68  stw r10, -0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.r[10].u32 ) };
	// 8323327C: 3A800080  li r20, 0x80
	ctx.r[20].s64 = 128;
	// 83233280: 9141FF6C  stw r10, -0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), ctx.r[10].u32 ) };
	// 83233284: 9141FF70  stw r10, -0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.r[10].u32 ) };
	// 83233288: 9161FF74  stw r11, -0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), ctx.r[11].u32 ) };
	// 8323328C: 9141FF78  stw r10, -0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832333E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832333E8 size=536
    let mut pc: u32 = 0x832333E8;
    'dispatch: loop {
        match pc {
            0x832333E8 => {
    //   block [0x832333E8..0x83233600)
	// 832333E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832333EC: 4BF74D5D  bl 0x831a8148
	ctx.lr = 0x832333F0;
	sub_831A8130(ctx, base);
	// 832333F0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832333F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832333F8: 3921FF20  addi r9, r1, -0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + -224;
	// 832333FC: 9161FF20  stw r11, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[11].u32 ) };
	// 83233400: 3901FF30  addi r8, r1, -0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + -208;
	// 83233404: 9161FF24  stw r11, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[11].u32 ) };
	// 83233408: 38E1FF40  addi r7, r1, -0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + -192;
	// 8323340C: 9161FF28  stw r11, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[11].u32 ) };
	// 83233410: 9161FF2C  stw r11, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[11].u32 ) };
	// 83233414: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 83233418: 9161FF30  stw r11, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[11].u32 ) };
	// 8323341C: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83233420: 9161FF34  stw r11, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[11].u32 ) };
	// 83233424: 3BC1FF70  addi r30, r1, -0x90
	ctx.r[30].s64 = ctx.r[1].s64 + -144;
	// 83233428: 9161FF38  stw r11, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[11].u32 ) };
	// 8323342C: 3B61FF80  addi r27, r1, -0x80
	ctx.r[27].s64 = ctx.r[1].s64 + -128;
	// 83233430: 9141FF3C  stw r10, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[10].u32 ) };
	// 83233434: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 83233438: 9161FF40  stw r11, -0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), ctx.r[11].u32 ) };
	// 8323343C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83233440: 9161FF44  stw r11, -0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), ctx.r[11].u32 ) };
	// 83233444: 38866F60  addi r4, r6, 0x6f60
	ctx.r[4].s64 = ctx.r[6].s64 + 28512;
	// 83233448: 9141FF48  stw r10, -0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), ctx.r[10].u32 ) };
	// 8323344C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83233450: 9161FF4C  stw r11, -0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), ctx.r[11].u32 ) };
	// 83233454: 3BA00030  li r29, 0x30
	ctx.r[29].s64 = 48;
	// 83233458: 9161FF50  stw r11, -0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.r[11].u32 ) };
	// 8323345C: 3B81FF30  addi r28, r1, -0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + -208;
	// 83233460: 9161FF54  stw r11, -0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), ctx.r[11].u32 ) };
	// 83233464: 3B41FF20  addi r26, r1, -0xe0
	ctx.r[26].s64 = ctx.r[1].s64 + -224;
	// 83233468: 9141FF58  stw r10, -0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.r[10].u32 ) };
	// 8323346C: 3B200040  li r25, 0x40
	ctx.r[25].s64 = 64;
	// 83233470: 9141FF5C  stw r10, -0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), ctx.r[10].u32 ) };
	// 83233474: 3B000050  li r24, 0x50
	ctx.r[24].s64 = 80;
	// 83233478: 9141FF7C  stw r10, -0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), ctx.r[10].u32 ) };
	// 8323347C: 3AE1FF40  addi r23, r1, -0xc0
	ctx.r[23].s64 = ctx.r[1].s64 + -192;
	// 83233480: 9161FF60  stw r11, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[11].u32 ) };
	// 83233484: 3AC00060  li r22, 0x60
	ctx.r[22].s64 = 96;
	// 83233488: 9141FF64  stw r10, -0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), ctx.r[10].u32 ) };
	// 8323348C: 3AA00070  li r21, 0x70
	ctx.r[21].s64 = 112;
	// 83233490: 9161FF68  stw r11, -0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.r[11].u32 ) };
	// 83233494: 3A800080  li r20, 0x80
	ctx.r[20].s64 = 128;
	// 83233498: 9161FF6C  stw r11, -0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), ctx.r[11].u32 ) };
	// 8323349C: 9161FF70  stw r11, -0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.r[11].u32 ) };
	// 832334A0: 9141FF74  stw r10, -0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), ctx.r[10].u32 ) };
	// 832334A4: 9161FF78  stw r11, -0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233600 size=28
    let mut pc: u32 = 0x83233600;
    'dispatch: loop {
        match pc {
            0x83233600 => {
    //   block [0x83233600..0x8323361C)
	// 83233600: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83233604: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83233608: 392BFF90  addi r9, r11, -0x70
	ctx.r[9].s64 = ctx.r[11].s64 + -112;
	// 8323360C: 390A7060  addi r8, r10, 0x7060
	ctx.r[8].s64 = ctx.r[10].s64 + 28768;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233620 size=28
    let mut pc: u32 = 0x83233620;
    'dispatch: loop {
        match pc {
            0x83233620 => {
    //   block [0x83233620..0x8323363C)
	// 83233620: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 83233624: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83233628: 392B4BC0  addi r9, r11, 0x4bc0
	ctx.r[9].s64 = ctx.r[11].s64 + 19392;
	// 8323362C: 390A7070  addi r8, r10, 0x7070
	ctx.r[8].s64 = ctx.r[10].s64 + 28784;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233640 size=60
    let mut pc: u32 = 0x83233640;
    'dispatch: loop {
        match pc {
            0x83233640 => {
    //   block [0x83233640..0x8323367C)
	// 83233640: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83233644: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83233648: 392BC470  addi r9, r11, -0x3b90
	ctx.r[9].s64 = ctx.r[11].s64 + -15248;
	// 8323364C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83233650: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 83233654: 38CA7080  addi r6, r10, 0x7080
	ctx.r[6].s64 = ctx.r[10].s64 + 28800;
	// 83233658: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8323365C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233680 size=76
    let mut pc: u32 = 0x83233680;
    'dispatch: loop {
        match pc {
            0x83233680 => {
    //   block [0x83233680..0x832336CC)
	// 83233680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323368C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233690: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83233694: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83233698: 3BEB70B0  addi r31, r11, 0x70b0
	ctx.r[31].s64 = ctx.r[11].s64 + 28848;
	// 8323369C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832336A0: 4800F7AD  bl 0x83242e4c
	ctx.lr = 0x832336A4;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 832336A4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832336A8: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 832336AC: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 832336B0: 386A1E98  addi r3, r10, 0x1e98
	ctx.r[3].s64 = ctx.r[10].s64 + 7832;
	// 832336B4: 4BF74E25  bl 0x831a84d8
	ctx.lr = 0x832336B8;
	sub_831A84D8(ctx, base);
	// 832336B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832336BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832336C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832336C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832336C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832336D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832336D0 size=32
    let mut pc: u32 = 0x832336D0;
    'dispatch: loop {
        match pc {
            0x832336D0 => {
    //   block [0x832336D0..0x832336F0)
	// 832336D0: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832336D4: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 832336D8: 38E8C818  addi r7, r8, -0x37e8
	ctx.r[7].s64 = ctx.r[8].s64 + -14312;
	// 832336DC: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 832336E0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832336E4: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 832336E8: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832336EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832336F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832336F0 size=32
    let mut pc: u32 = 0x832336F0;
    'dispatch: loop {
        match pc {
            0x832336F0 => {
    //   block [0x832336F0..0x83233710)
	// 832336F0: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832336F4: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 832336F8: 38E8C82C  addi r7, r8, -0x37d4
	ctx.r[7].s64 = ctx.r[8].s64 + -14292;
	// 832336FC: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 83233700: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83233704: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 83233708: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8323370C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233710 size=72
    let mut pc: u32 = 0x83233710;
    'dispatch: loop {
        match pc {
            0x83233710 => {
    //   block [0x83233710..0x83233758)
	// 83233710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323371C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83233720: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83233724: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83233728: 38CB0D70  addi r6, r11, 0xd70
	ctx.r[6].s64 = ctx.r[11].s64 + 3440;
	// 8323372C: 388AB3C8  addi r4, r10, -0x4c38
	ctx.r[4].s64 = ctx.r[10].s64 + -19512;
	// 83233730: 38697104  addi r3, r9, 0x7104
	ctx.r[3].s64 = ctx.r[9].s64 + 28932;
	// 83233734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83233738: 4BC88E11  bl 0x82ebc548
	ctx.lr = 0x8323373C;
	sub_82EBC548(ctx, base);
	// 8323373C: 3D008324  lis r8, -0x7cdc
	ctx.r[8].s64 = -2094792704;
	// 83233740: 38681EA8  addi r3, r8, 0x1ea8
	ctx.r[3].s64 = ctx.r[8].s64 + 7848;
	// 83233744: 4BF74D95  bl 0x831a84d8
	ctx.lr = 0x83233748;
	sub_831A84D8(ctx, base);
	// 83233748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323374C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233758 size=32
    let mut pc: u32 = 0x83233758;
    'dispatch: loop {
        match pc {
            0x83233758 => {
    //   block [0x83233758..0x83233778)
	// 83233758: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323375C: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 83233760: 38E8C958  addi r7, r8, -0x36a8
	ctx.r[7].s64 = ctx.r[8].s64 + -13992;
	// 83233764: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 83233768: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8323376C: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 83233770: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83233774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233778 size=108
    let mut pc: u32 = 0x83233778;
    'dispatch: loop {
        match pc {
            0x83233778 => {
    //   block [0x83233778..0x832337E4)
	// 83233778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323377C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233784: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233788: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323378C: 38EA0DC8  addi r7, r10, 0xdc8
	ctx.r[7].s64 = ctx.r[10].s64 + 3528;
	// 83233790: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233794: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83233798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323379C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832337A0: 38890E28  addi r4, r9, 0xe28
	ctx.r[4].s64 = ctx.r[9].s64 + 3624;
	// 832337A4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832337A8: 3868711C  addi r3, r8, 0x711c
	ctx.r[3].s64 = ctx.r[8].s64 + 28956;
	// 832337AC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832337B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832337B4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832337B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832337BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832337C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832337C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832337C8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832337CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832337D0: 4BC749F9  bl 0x82ea81c8
	ctx.lr = 0x832337D4;
	sub_82EA81C8(ctx, base);
	// 832337D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832337D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832337DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832337E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832337E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832337E8 size=108
    let mut pc: u32 = 0x832337E8;
    'dispatch: loop {
        match pc {
            0x832337E8 => {
    //   block [0x832337E8..0x83233854)
	// 832337E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832337EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832337F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832337F4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832337F8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832337FC: 38EA0E10  addi r7, r10, 0xe10
	ctx.r[7].s64 = ctx.r[10].s64 + 3600;
	// 83233800: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233804: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323380C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233810: 38890E4C  addi r4, r9, 0xe4c
	ctx.r[4].s64 = ctx.r[9].s64 + 3660;
	// 83233814: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233818: 3868714C  addi r3, r8, 0x714c
	ctx.r[3].s64 = ctx.r[8].s64 + 29004;
	// 8323381C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233824: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233828: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323382C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233830: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233838: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323383C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233840: 4BC74989  bl 0x82ea81c8
	ctx.lr = 0x83233844;
	sub_82EA81C8(ctx, base);
	// 83233844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323384C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233858 size=108
    let mut pc: u32 = 0x83233858;
    'dispatch: loop {
        match pc {
            0x83233858 => {
    //   block [0x83233858..0x832338C4)
	// 83233858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233864: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233868: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323386C: 38EA0EF8  addi r7, r10, 0xef8
	ctx.r[7].s64 = ctx.r[10].s64 + 3832;
	// 83233870: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233874: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83233878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323387C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233880: 38890FD0  addi r4, r9, 0xfd0
	ctx.r[4].s64 = ctx.r[9].s64 + 4048;
	// 83233884: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233888: 3868717C  addi r3, r8, 0x717c
	ctx.r[3].s64 = ctx.r[8].s64 + 29052;
	// 8323388C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233894: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233898: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323389C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832338A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832338A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832338A8: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832338AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832338B0: 4BC74919  bl 0x82ea81c8
	ctx.lr = 0x832338B4;
	sub_82EA81C8(ctx, base);
	// 832338B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832338B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832338BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832338C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832338C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832338C8 size=108
    let mut pc: u32 = 0x832338C8;
    'dispatch: loop {
        match pc {
            0x832338C8 => {
    //   block [0x832338C8..0x83233934)
	// 832338C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832338CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832338D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832338D4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832338D8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832338DC: 38EA10A0  addi r7, r10, 0x10a0
	ctx.r[7].s64 = ctx.r[10].s64 + 4256;
	// 832338E0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832338E4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 832338E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832338EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832338F0: 388911A8  addi r4, r9, 0x11a8
	ctx.r[4].s64 = ctx.r[9].s64 + 4520;
	// 832338F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832338F8: 386871AC  addi r3, r8, 0x71ac
	ctx.r[3].s64 = ctx.r[8].s64 + 29100;
	// 832338FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233904: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233908: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323390C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233910: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233918: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323391C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233920: 4BC748A9  bl 0x82ea81c8
	ctx.lr = 0x83233924;
	sub_82EA81C8(ctx, base);
	// 83233924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323392C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233938 size=32
    let mut pc: u32 = 0x83233938;
    'dispatch: loop {
        match pc {
            0x83233938 => {
    //   block [0x83233938..0x83233958)
	// 83233938: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323393C: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 83233940: 38E8CAA0  addi r7, r8, -0x3560
	ctx.r[7].s64 = ctx.r[8].s64 + -13664;
	// 83233944: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 83233948: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8323394C: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 83233950: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83233954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83233958 size=36
    let mut pc: u32 = 0x83233958;
    'dispatch: loop {
        match pc {
            0x83233958 => {
    //   block [0x83233958..0x8323397C)
	// 83233958: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8323395C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83233960: 392ACE24  addi r9, r10, -0x31dc
	ctx.r[9].s64 = ctx.r[10].s64 + -12764;
	// 83233964: 816B1D5C  lwz r11, 0x1d5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7516 as u32) ) } as u64;
	// 83233968: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 8323396C: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 83233970: 54EBDFFE  rlwinm r11, r7, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 83233974: 99690001  stb r11, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83233978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233980 size=108
    let mut pc: u32 = 0x83233980;
    'dispatch: loop {
        match pc {
            0x83233980 => {
    //   block [0x83233980..0x832339EC)
	// 83233980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323398C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233990: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233994: 38EA22E8  addi r7, r10, 0x22e8
	ctx.r[7].s64 = ctx.r[10].s64 + 8936;
	// 83233998: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323399C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 832339A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832339A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832339A8: 38892300  addi r4, r9, 0x2300
	ctx.r[4].s64 = ctx.r[9].s64 + 8960;
	// 832339AC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832339B0: 3868720C  addi r3, r8, 0x720c
	ctx.r[3].s64 = ctx.r[8].s64 + 29196;
	// 832339B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832339B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832339BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832339C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832339C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832339C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832339CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832339D0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832339D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832339D8: 4BC747F1  bl 0x82ea81c8
	ctx.lr = 0x832339DC;
	sub_82EA81C8(ctx, base);
	// 832339DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832339E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832339E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832339E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832339F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832339F0 size=108
    let mut pc: u32 = 0x832339F0;
    'dispatch: loop {
        match pc {
            0x832339F0 => {
    //   block [0x832339F0..0x83233A5C)
	// 832339F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832339F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832339F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832339FC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233A00: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233A04: 38EA2330  addi r7, r10, 0x2330
	ctx.r[7].s64 = ctx.r[10].s64 + 9008;
	// 83233A08: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233A0C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233A14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233A18: 38892348  addi r4, r9, 0x2348
	ctx.r[4].s64 = ctx.r[9].s64 + 9032;
	// 83233A1C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233A20: 3868723C  addi r3, r8, 0x723c
	ctx.r[3].s64 = ctx.r[8].s64 + 29244;
	// 83233A24: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233A2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233A30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233A34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233A38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233A40: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233A48: 4BC74781  bl 0x82ea81c8
	ctx.lr = 0x83233A4C;
	sub_82EA81C8(ctx, base);
	// 83233A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233A60 size=108
    let mut pc: u32 = 0x83233A60;
    'dispatch: loop {
        match pc {
            0x83233A60 => {
    //   block [0x83233A60..0x83233ACC)
	// 83233A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233A6C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233A70: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233A74: 38EA2380  addi r7, r10, 0x2380
	ctx.r[7].s64 = ctx.r[10].s64 + 9088;
	// 83233A78: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233A7C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83233A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233A84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233A88: 38892428  addi r4, r9, 0x2428
	ctx.r[4].s64 = ctx.r[9].s64 + 9256;
	// 83233A8C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233A90: 3868726C  addi r3, r8, 0x726c
	ctx.r[3].s64 = ctx.r[8].s64 + 29292;
	// 83233A94: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233A9C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233AA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233AA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233AA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233AB0: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83233AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233AB8: 4BC74711  bl 0x82ea81c8
	ctx.lr = 0x83233ABC;
	sub_82EA81C8(ctx, base);
	// 83233ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233AD0 size=108
    let mut pc: u32 = 0x83233AD0;
    'dispatch: loop {
        match pc {
            0x83233AD0 => {
    //   block [0x83233AD0..0x83233B3C)
	// 83233AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233ADC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233AE0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233AE4: 38EA2448  addi r7, r10, 0x2448
	ctx.r[7].s64 = ctx.r[10].s64 + 9288;
	// 83233AE8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233AEC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233AF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233AF8: 38892508  addi r4, r9, 0x2508
	ctx.r[4].s64 = ctx.r[9].s64 + 9480;
	// 83233AFC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233B00: 3868729C  addi r3, r8, 0x729c
	ctx.r[3].s64 = ctx.r[8].s64 + 29340;
	// 83233B04: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233B0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233B10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233B14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233B20: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83233B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233B28: 4BC746A1  bl 0x82ea81c8
	ctx.lr = 0x83233B2C;
	sub_82EA81C8(ctx, base);
	// 83233B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233B40 size=108
    let mut pc: u32 = 0x83233B40;
    'dispatch: loop {
        match pc {
            0x83233B40 => {
    //   block [0x83233B40..0x83233BAC)
	// 83233B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233B4C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233B50: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233B54: 38EA2520  addi r7, r10, 0x2520
	ctx.r[7].s64 = ctx.r[10].s64 + 9504;
	// 83233B58: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233B5C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83233B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233B64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233B68: 388925F8  addi r4, r9, 0x25f8
	ctx.r[4].s64 = ctx.r[9].s64 + 9720;
	// 83233B6C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233B70: 386872CC  addi r3, r8, 0x72cc
	ctx.r[3].s64 = ctx.r[8].s64 + 29388;
	// 83233B74: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233B7C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233B80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233B84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233B88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233B90: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83233B94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233B98: 4BC74631  bl 0x82ea81c8
	ctx.lr = 0x83233B9C;
	sub_82EA81C8(ctx, base);
	// 83233B9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233BB0 size=108
    let mut pc: u32 = 0x83233BB0;
    'dispatch: loop {
        match pc {
            0x83233BB0 => {
    //   block [0x83233BB0..0x83233C1C)
	// 83233BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233BBC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233BC0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233BC4: 38EA2620  addi r7, r10, 0x2620
	ctx.r[7].s64 = ctx.r[10].s64 + 9760;
	// 83233BC8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233BCC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233BD8: 388926E0  addi r4, r9, 0x26e0
	ctx.r[4].s64 = ctx.r[9].s64 + 9952;
	// 83233BDC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233BE0: 386872FC  addi r3, r8, 0x72fc
	ctx.r[3].s64 = ctx.r[8].s64 + 29436;
	// 83233BE4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233BEC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233BF0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233BF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233BF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233C00: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83233C04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233C08: 4BC745C1  bl 0x82ea81c8
	ctx.lr = 0x83233C0C;
	sub_82EA81C8(ctx, base);
	// 83233C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233C20 size=108
    let mut pc: u32 = 0x83233C20;
    'dispatch: loop {
        match pc {
            0x83233C20 => {
    //   block [0x83233C20..0x83233C8C)
	// 83233C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233C2C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233C30: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233C34: 38EA26F8  addi r7, r10, 0x26f8
	ctx.r[7].s64 = ctx.r[10].s64 + 9976;
	// 83233C38: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233C3C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83233C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233C44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233C48: 38892788  addi r4, r9, 0x2788
	ctx.r[4].s64 = ctx.r[9].s64 + 10120;
	// 83233C4C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233C50: 3868732C  addi r3, r8, 0x732c
	ctx.r[3].s64 = ctx.r[8].s64 + 29484;
	// 83233C54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233C5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233C60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233C64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233C68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233C70: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83233C74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233C78: 4BC74551  bl 0x82ea81c8
	ctx.lr = 0x83233C7C;
	sub_82EA81C8(ctx, base);
	// 83233C7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233C90 size=108
    let mut pc: u32 = 0x83233C90;
    'dispatch: loop {
        match pc {
            0x83233C90 => {
    //   block [0x83233C90..0x83233CFC)
	// 83233C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233C9C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233CA0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233CA4: 38EA27AC  addi r7, r10, 0x27ac
	ctx.r[7].s64 = ctx.r[10].s64 + 10156;
	// 83233CA8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233CAC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233CB8: 388927C4  addi r4, r9, 0x27c4
	ctx.r[4].s64 = ctx.r[9].s64 + 10180;
	// 83233CBC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233CC0: 3868735C  addi r3, r8, 0x735c
	ctx.r[3].s64 = ctx.r[8].s64 + 29532;
	// 83233CC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233CCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233CD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233CD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233CD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233CE0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233CE8: 4BC744E1  bl 0x82ea81c8
	ctx.lr = 0x83233CEC;
	sub_82EA81C8(ctx, base);
	// 83233CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233D00 size=104
    let mut pc: u32 = 0x83233D00;
    'dispatch: loop {
        match pc {
            0x83233D00 => {
    //   block [0x83233D00..0x83233D68)
	// 83233D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233D0C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233D10: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233D14: 38EA28A8  addi r7, r10, 0x28a8
	ctx.r[7].s64 = ctx.r[10].s64 + 10408;
	// 83233D18: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233D1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233D24: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83233D28: 388929C8  addi r4, r9, 0x29c8
	ctx.r[4].s64 = ctx.r[9].s64 + 10696;
	// 83233D2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233D30: 3868738C  addi r3, r8, 0x738c
	ctx.r[3].s64 = ctx.r[8].s64 + 29580;
	// 83233D34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233D3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233D40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233D44: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233D48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233D54: 4BC74475  bl 0x82ea81c8
	ctx.lr = 0x83233D58;
	sub_82EA81C8(ctx, base);
	// 83233D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233D68 size=108
    let mut pc: u32 = 0x83233D68;
    'dispatch: loop {
        match pc {
            0x83233D68 => {
    //   block [0x83233D68..0x83233DD4)
	// 83233D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233D74: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233D78: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233D7C: 38EA29E8  addi r7, r10, 0x29e8
	ctx.r[7].s64 = ctx.r[10].s64 + 10728;
	// 83233D80: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233D84: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233D88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233D8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233D90: 38892A00  addi r4, r9, 0x2a00
	ctx.r[4].s64 = ctx.r[9].s64 + 10752;
	// 83233D94: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233D98: 386873BC  addi r3, r8, 0x73bc
	ctx.r[3].s64 = ctx.r[8].s64 + 29628;
	// 83233D9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233DA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233DA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233DAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233DB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233DB8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233DC0: 4BC74409  bl 0x82ea81c8
	ctx.lr = 0x83233DC4;
	sub_82EA81C8(ctx, base);
	// 83233DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233DD8 size=108
    let mut pc: u32 = 0x83233DD8;
    'dispatch: loop {
        match pc {
            0x83233DD8 => {
    //   block [0x83233DD8..0x83233E44)
	// 83233DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233DE4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233DE8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233DEC: 38EA2A28  addi r7, r10, 0x2a28
	ctx.r[7].s64 = ctx.r[10].s64 + 10792;
	// 83233DF0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233DF4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83233DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233DFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233E00: 38892A58  addi r4, r9, 0x2a58
	ctx.r[4].s64 = ctx.r[9].s64 + 10840;
	// 83233E04: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233E08: 386873EC  addi r3, r8, 0x73ec
	ctx.r[3].s64 = ctx.r[8].s64 + 29676;
	// 83233E0C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233E14: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233E18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233E1C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233E20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233E28: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83233E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233E30: 4BC74399  bl 0x82ea81c8
	ctx.lr = 0x83233E34;
	sub_82EA81C8(ctx, base);
	// 83233E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233E48 size=108
    let mut pc: u32 = 0x83233E48;
    'dispatch: loop {
        match pc {
            0x83233E48 => {
    //   block [0x83233E48..0x83233EB4)
	// 83233E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233E54: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233E58: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233E5C: 38EA2A7C  addi r7, r10, 0x2a7c
	ctx.r[7].s64 = ctx.r[10].s64 + 10876;
	// 83233E60: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233E64: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233E68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233E70: 38892A94  addi r4, r9, 0x2a94
	ctx.r[4].s64 = ctx.r[9].s64 + 10900;
	// 83233E74: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233E78: 3868741C  addi r3, r8, 0x741c
	ctx.r[3].s64 = ctx.r[8].s64 + 29724;
	// 83233E7C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233E84: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233E88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233E8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233E98: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83233E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233EA0: 4BC74329  bl 0x82ea81c8
	ctx.lr = 0x83233EA4;
	sub_82EA81C8(ctx, base);
	// 83233EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233EB8 size=108
    let mut pc: u32 = 0x83233EB8;
    'dispatch: loop {
        match pc {
            0x83233EB8 => {
    //   block [0x83233EB8..0x83233F24)
	// 83233EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233EC4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233EC8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233ECC: 38EA2AC4  addi r7, r10, 0x2ac4
	ctx.r[7].s64 = ctx.r[10].s64 + 10948;
	// 83233ED0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233ED4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83233ED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233EDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233EE0: 38892AF4  addi r4, r9, 0x2af4
	ctx.r[4].s64 = ctx.r[9].s64 + 10996;
	// 83233EE4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233EE8: 3868744C  addi r3, r8, 0x744c
	ctx.r[3].s64 = ctx.r[8].s64 + 29772;
	// 83233EEC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233EF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233EF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233EFC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233F00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233F08: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83233F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233F10: 4BC742B9  bl 0x82ea81c8
	ctx.lr = 0x83233F14;
	sub_82EA81C8(ctx, base);
	// 83233F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233F28 size=108
    let mut pc: u32 = 0x83233F28;
    'dispatch: loop {
        match pc {
            0x83233F28 => {
    //   block [0x83233F28..0x83233F94)
	// 83233F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233F34: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233F38: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233F3C: 38EA2B14  addi r7, r10, 0x2b14
	ctx.r[7].s64 = ctx.r[10].s64 + 11028;
	// 83233F40: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233F44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233F4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233F50: 38892B2C  addi r4, r9, 0x2b2c
	ctx.r[4].s64 = ctx.r[9].s64 + 11052;
	// 83233F54: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233F58: 3868747C  addi r3, r8, 0x747c
	ctx.r[3].s64 = ctx.r[8].s64 + 29820;
	// 83233F5C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233F64: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233F68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233F6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233F70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233F78: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83233F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233F80: 4BC74249  bl 0x82ea81c8
	ctx.lr = 0x83233F84;
	sub_82EA81C8(ctx, base);
	// 83233F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83233F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83233F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83233F98 size=108
    let mut pc: u32 = 0x83233F98;
    'dispatch: loop {
        match pc {
            0x83233F98 => {
    //   block [0x83233F98..0x83234004)
	// 83233F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83233F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83233FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83233FA4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83233FA8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83233FAC: 38EA2B4C  addi r7, r10, 0x2b4c
	ctx.r[7].s64 = ctx.r[10].s64 + 11084;
	// 83233FB0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83233FB4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83233FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83233FBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83233FC0: 38892B94  addi r4, r9, 0x2b94
	ctx.r[4].s64 = ctx.r[9].s64 + 11156;
	// 83233FC4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83233FC8: 386874AC  addi r3, r8, 0x74ac
	ctx.r[3].s64 = ctx.r[8].s64 + 29868;
	// 83233FCC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83233FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83233FD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83233FD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83233FDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83233FE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83233FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83233FE8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83233FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83233FF0: 4BC741D9  bl 0x82ea81c8
	ctx.lr = 0x83233FF4;
	sub_82EA81C8(ctx, base);
	// 83233FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83233FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83233FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234008 size=108
    let mut pc: u32 = 0x83234008;
    'dispatch: loop {
        match pc {
            0x83234008 => {
    //   block [0x83234008..0x83234074)
	// 83234008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323400C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234014: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234018: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323401C: 38EA2B64  addi r7, r10, 0x2b64
	ctx.r[7].s64 = ctx.r[10].s64 + 11108;
	// 83234020: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234024: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323402C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234030: 38892BB8  addi r4, r9, 0x2bb8
	ctx.r[4].s64 = ctx.r[9].s64 + 11192;
	// 83234034: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234038: 386874DC  addi r3, r8, 0x74dc
	ctx.r[3].s64 = ctx.r[8].s64 + 29916;
	// 8323403C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234044: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234048: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323404C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234058: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323405C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234060: 4BC74169  bl 0x82ea81c8
	ctx.lr = 0x83234064;
	sub_82EA81C8(ctx, base);
	// 83234064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234078 size=108
    let mut pc: u32 = 0x83234078;
    'dispatch: loop {
        match pc {
            0x83234078 => {
    //   block [0x83234078..0x832340E4)
	// 83234078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323407C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234084: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234088: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323408C: 38EA2BDC  addi r7, r10, 0x2bdc
	ctx.r[7].s64 = ctx.r[10].s64 + 11228;
	// 83234090: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234094: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323409C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832340A0: 38892C0C  addi r4, r9, 0x2c0c
	ctx.r[4].s64 = ctx.r[9].s64 + 11276;
	// 832340A4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832340A8: 3868750C  addi r3, r8, 0x750c
	ctx.r[3].s64 = ctx.r[8].s64 + 29964;
	// 832340AC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832340B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832340B4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832340B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832340BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832340C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832340C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832340C8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832340CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832340D0: 4BC740F9  bl 0x82ea81c8
	ctx.lr = 0x832340D4;
	sub_82EA81C8(ctx, base);
	// 832340D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832340D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832340DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832340E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832340E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832340E8 size=120
    let mut pc: u32 = 0x832340E8;
    'dispatch: loop {
        match pc {
            0x832340E8 => {
    //   block [0x832340E8..0x83234160)
	// 832340E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832340EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832340F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832340F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832340F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832340FC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234100: 38CA2C24  addi r6, r10, 0x2c24
	ctx.r[6].s64 = ctx.r[10].s64 + 11300;
	// 83234104: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234108: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323410C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234114: 38A9750C  addi r5, r9, 0x750c
	ctx.r[5].s64 = ctx.r[9].s64 + 29964;
	// 83234118: 38882C3C  addi r4, r8, 0x2c3c
	ctx.r[4].s64 = ctx.r[8].s64 + 11324;
	// 8323411C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234120: 3867753C  addi r3, r7, 0x753c
	ctx.r[3].s64 = ctx.r[7].s64 + 30012;
	// 83234124: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234128: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323412C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234138: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323413C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234144: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83234148: 4BC74081  bl 0x82ea81c8
	ctx.lr = 0x8323414C;
	sub_82EA81C8(ctx, base);
	// 8323414C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234160 size=108
    let mut pc: u32 = 0x83234160;
    'dispatch: loop {
        match pc {
            0x83234160 => {
    //   block [0x83234160..0x832341CC)
	// 83234160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323416C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234170: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234174: 38EA2C5C  addi r7, r10, 0x2c5c
	ctx.r[7].s64 = ctx.r[10].s64 + 11356;
	// 83234178: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323417C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234188: 38892C8C  addi r4, r9, 0x2c8c
	ctx.r[4].s64 = ctx.r[9].s64 + 11404;
	// 8323418C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234190: 3868756C  addi r3, r8, 0x756c
	ctx.r[3].s64 = ctx.r[8].s64 + 30060;
	// 83234194: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323419C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832341A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832341A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832341A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832341AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832341B0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832341B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832341B8: 4BC74011  bl 0x82ea81c8
	ctx.lr = 0x832341BC;
	sub_82EA81C8(ctx, base);
	// 832341BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832341C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832341C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832341C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832341D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832341D0 size=108
    let mut pc: u32 = 0x832341D0;
    'dispatch: loop {
        match pc {
            0x832341D0 => {
    //   block [0x832341D0..0x8323423C)
	// 832341D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832341D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832341D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832341DC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832341E0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832341E4: 38EA2CD8  addi r7, r10, 0x2cd8
	ctx.r[7].s64 = ctx.r[10].s64 + 11480;
	// 832341E8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832341EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832341F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832341F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832341F8: 38892D38  addi r4, r9, 0x2d38
	ctx.r[4].s64 = ctx.r[9].s64 + 11576;
	// 832341FC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234200: 3868759C  addi r3, r8, 0x759c
	ctx.r[3].s64 = ctx.r[8].s64 + 30108;
	// 83234204: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323420C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234210: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234214: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234218: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323421C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234220: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 83234224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234228: 4BC73FA1  bl 0x82ea81c8
	ctx.lr = 0x8323422C;
	sub_82EA81C8(ctx, base);
	// 8323422C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234240 size=108
    let mut pc: u32 = 0x83234240;
    'dispatch: loop {
        match pc {
            0x83234240 => {
    //   block [0x83234240..0x832342AC)
	// 83234240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323424C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83234250: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234254: 396B2DF0  addi r11, r11, 0x2df0
	ctx.r[11].s64 = ctx.r[11].s64 + 11760;
	// 83234258: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323425C: 38EB0138  addi r7, r11, 0x138
	ctx.r[7].s64 = ctx.r[11].s64 + 312;
	// 83234260: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83234264: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 83234268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323426C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 83234270: 38892F8C  addi r4, r9, 0x2f8c
	ctx.r[4].s64 = ctx.r[9].s64 + 12172;
	// 83234274: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234278: 386875CC  addi r3, r8, 0x75cc
	ctx.r[3].s64 = ctx.r[8].s64 + 30156;
	// 8323427C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83234280: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234284: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83234288: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323428C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234290: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83234294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234298: 4BC73F31  bl 0x82ea81c8
	ctx.lr = 0x8323429C;
	sub_82EA81C8(ctx, base);
	// 8323429C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832342A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832342A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832342A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832342B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832342B0 size=120
    let mut pc: u32 = 0x832342B0;
    'dispatch: loop {
        match pc {
            0x832342B0 => {
    //   block [0x832342B0..0x83234328)
	// 832342B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832342B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832342B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832342BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832342C0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832342C4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832342C8: 38CA2FA8  addi r6, r10, 0x2fa8
	ctx.r[6].s64 = ctx.r[10].s64 + 12200;
	// 832342CC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832342D0: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832342D4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832342D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832342DC: 38A978CC  addi r5, r9, 0x78cc
	ctx.r[5].s64 = ctx.r[9].s64 + 30924;
	// 832342E0: 38882FD8  addi r4, r8, 0x2fd8
	ctx.r[4].s64 = ctx.r[8].s64 + 12248;
	// 832342E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832342E8: 386775FC  addi r3, r7, 0x75fc
	ctx.r[3].s64 = ctx.r[7].s64 + 30204;
	// 832342EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832342F0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832342F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832342F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832342FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234300: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234308: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323430C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83234310: 4BC73EB9  bl 0x82ea81c8
	ctx.lr = 0x83234314;
	sub_82EA81C8(ctx, base);
	// 83234314: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323431C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234320: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83234324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234328 size=108
    let mut pc: u32 = 0x83234328;
    'dispatch: loop {
        match pc {
            0x83234328 => {
    //   block [0x83234328..0x83234394)
	// 83234328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323432C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234334: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234338: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323433C: 38EA3038  addi r7, r10, 0x3038
	ctx.r[7].s64 = ctx.r[10].s64 + 12344;
	// 83234340: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234344: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323434C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234350: 38893110  addi r4, r9, 0x3110
	ctx.r[4].s64 = ctx.r[9].s64 + 12560;
	// 83234354: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234358: 3868762C  addi r3, r8, 0x762c
	ctx.r[3].s64 = ctx.r[8].s64 + 30252;
	// 8323435C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234364: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234368: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323436C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234370: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234378: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323437C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234380: 4BC73E49  bl 0x82ea81c8
	ctx.lr = 0x83234384;
	sub_82EA81C8(ctx, base);
	// 83234384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323438C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234398 size=120
    let mut pc: u32 = 0x83234398;
    'dispatch: loop {
        match pc {
            0x83234398 => {
    //   block [0x83234398..0x83234410)
	// 83234398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832343A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832343A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832343A8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832343AC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832343B0: 38CA3068  addi r6, r10, 0x3068
	ctx.r[6].s64 = ctx.r[10].s64 + 12392;
	// 832343B4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832343B8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832343BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832343C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832343C4: 38A978CC  addi r5, r9, 0x78cc
	ctx.r[5].s64 = ctx.r[9].s64 + 30924;
	// 832343C8: 38883128  addi r4, r8, 0x3128
	ctx.r[4].s64 = ctx.r[8].s64 + 12584;
	// 832343CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832343D0: 3867765C  addi r3, r7, 0x765c
	ctx.r[3].s64 = ctx.r[7].s64 + 30300;
	// 832343D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832343D8: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 832343DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832343E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832343E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832343E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832343EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832343F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832343F4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 832343F8: 4BC73DD1  bl 0x82ea81c8
	ctx.lr = 0x832343FC;
	sub_82EA81C8(ctx, base);
	// 832343FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323440C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234410 size=120
    let mut pc: u32 = 0x83234410;
    'dispatch: loop {
        match pc {
            0x83234410 => {
    //   block [0x83234410..0x83234488)
	// 83234410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323441C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234420: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234424: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234428: 38CA3150  addi r6, r10, 0x3150
	ctx.r[6].s64 = ctx.r[10].s64 + 12624;
	// 8323442C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234430: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234434: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323443C: 38A978CC  addi r5, r9, 0x78cc
	ctx.r[5].s64 = ctx.r[9].s64 + 30924;
	// 83234440: 388831B0  addi r4, r8, 0x31b0
	ctx.r[4].s64 = ctx.r[8].s64 + 12720;
	// 83234444: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234448: 3867768C  addi r3, r7, 0x768c
	ctx.r[3].s64 = ctx.r[7].s64 + 30348;
	// 8323444C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234450: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83234454: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323445C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234460: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234468: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323446C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83234470: 4BC73D59  bl 0x82ea81c8
	ctx.lr = 0x83234474;
	sub_82EA81C8(ctx, base);
	// 83234474: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323447C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234480: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83234484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234488 size=108
    let mut pc: u32 = 0x83234488;
    'dispatch: loop {
        match pc {
            0x83234488 => {
    //   block [0x83234488..0x832344F4)
	// 83234488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234494: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234498: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323449C: 38EA3180  addi r7, r10, 0x3180
	ctx.r[7].s64 = ctx.r[10].s64 + 12672;
	// 832344A0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832344A4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 832344A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832344AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832344B0: 388931C8  addi r4, r9, 0x31c8
	ctx.r[4].s64 = ctx.r[9].s64 + 12744;
	// 832344B4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832344B8: 386876BC  addi r3, r8, 0x76bc
	ctx.r[3].s64 = ctx.r[8].s64 + 30396;
	// 832344BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832344C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832344C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832344C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832344CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832344D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832344D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832344D8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832344DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832344E0: 4BC73CE9  bl 0x82ea81c8
	ctx.lr = 0x832344E4;
	sub_82EA81C8(ctx, base);
	// 832344E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832344E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832344EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832344F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832344F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832344F8 size=108
    let mut pc: u32 = 0x832344F8;
    'dispatch: loop {
        match pc {
            0x832344F8 => {
    //   block [0x832344F8..0x83234564)
	// 832344F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832344FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234504: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234508: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323450C: 38EA3210  addi r7, r10, 0x3210
	ctx.r[7].s64 = ctx.r[10].s64 + 12816;
	// 83234510: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234514: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83234518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323451C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234520: 38893270  addi r4, r9, 0x3270
	ctx.r[4].s64 = ctx.r[9].s64 + 12912;
	// 83234524: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234528: 386876EC  addi r3, r8, 0x76ec
	ctx.r[3].s64 = ctx.r[8].s64 + 30444;
	// 8323452C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234534: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234538: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323453C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234548: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234550: 4BC73C79  bl 0x82ea81c8
	ctx.lr = 0x83234554;
	sub_82EA81C8(ctx, base);
	// 83234554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323455C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234568 size=108
    let mut pc: u32 = 0x83234568;
    'dispatch: loop {
        match pc {
            0x83234568 => {
    //   block [0x83234568..0x832345D4)
	// 83234568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234574: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234578: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323457C: 38EA3394  addi r7, r10, 0x3394
	ctx.r[7].s64 = ctx.r[10].s64 + 13204;
	// 83234580: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234584: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323458C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234590: 38893488  addi r4, r9, 0x3488
	ctx.r[4].s64 = ctx.r[9].s64 + 13448;
	// 83234594: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234598: 3868771C  addi r3, r8, 0x771c
	ctx.r[3].s64 = ctx.r[8].s64 + 30492;
	// 8323459C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832345A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832345A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832345A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832345AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832345B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832345B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832345B8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832345BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832345C0: 4BC73C09  bl 0x82ea81c8
	ctx.lr = 0x832345C4;
	sub_82EA81C8(ctx, base);
	// 832345C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832345C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832345CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832345D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832345D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832345D8 size=120
    let mut pc: u32 = 0x832345D8;
    'dispatch: loop {
        match pc {
            0x832345D8 => {
    //   block [0x832345D8..0x83234650)
	// 832345D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832345DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832345E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832345E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832345E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832345EC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832345F0: 392B3380  addi r9, r11, 0x3380
	ctx.r[9].s64 = ctx.r[11].s64 + 13184;
	// 832345F4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832345F8: 38C90048  addi r6, r9, 0x48
	ctx.r[6].s64 = ctx.r[9].s64 + 72;
	// 832345FC: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234604: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234608: 38AA78CC  addi r5, r10, 0x78cc
	ctx.r[5].s64 = ctx.r[10].s64 + 30924;
	// 8323460C: 388834A0  addi r4, r8, 0x34a0
	ctx.r[4].s64 = ctx.r[8].s64 + 13472;
	// 83234610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234614: 3867774C  addi r3, r7, 0x774c
	ctx.r[3].s64 = ctx.r[7].s64 + 30540;
	// 83234618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323461C: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 83234620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234624: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83234628: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323462C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234634: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83234638: 4BC73B91  bl 0x82ea81c8
	ctx.lr = 0x8323463C;
	sub_82EA81C8(ctx, base);
	// 8323463C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323464C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234650 size=24
    let mut pc: u32 = 0x83234650;
    'dispatch: loop {
        match pc {
            0x83234650 => {
    //   block [0x83234650..0x83234668)
	// 83234650: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83234654: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234658: 392AD138  addi r9, r10, -0x2ec8
	ctx.r[9].s64 = ctx.r[10].s64 + -11976;
	// 8323465C: 816BD120  lwz r11, -0x2ee0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12000 as u32) ) } as u64;
	// 83234660: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83234664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234668 size=112
    let mut pc: u32 = 0x83234668;
    'dispatch: loop {
        match pc {
            0x83234668 => {
    //   block [0x83234668..0x832346D8)
	// 83234668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234674: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234678: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323467C: 38CAD138  addi r6, r10, -0x2ec8
	ctx.r[6].s64 = ctx.r[10].s64 + -11976;
	// 83234680: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234684: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83234688: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323468C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234694: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83234698: 3888354C  addi r4, r8, 0x354c
	ctx.r[4].s64 = ctx.r[8].s64 + 13644;
	// 8323469C: 3867777C  addi r3, r7, 0x777c
	ctx.r[3].s64 = ctx.r[7].s64 + 30588;
	// 832346A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832346A4: 39293538  addi r9, r9, 0x3538
	ctx.r[9].s64 = ctx.r[9].s64 + 13624;
	// 832346A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832346AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832346B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832346B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832346B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832346BC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832346C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832346C4: 4BC73B05  bl 0x82ea81c8
	ctx.lr = 0x832346C8;
	sub_82EA81C8(ctx, base);
	// 832346C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832346CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832346D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832346D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832346D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832346D8 size=24
    let mut pc: u32 = 0x832346D8;
    'dispatch: loop {
        match pc {
            0x832346D8 => {
    //   block [0x832346D8..0x832346F0)
	// 832346D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832346DC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832346E0: 392AD198  addi r9, r10, -0x2e68
	ctx.r[9].s64 = ctx.r[10].s64 + -11880;
	// 832346E4: 816BD180  lwz r11, -0x2e80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11904 as u32) ) } as u64;
	// 832346E8: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832346EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832346F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832346F0 size=112
    let mut pc: u32 = 0x832346F0;
    'dispatch: loop {
        match pc {
            0x832346F0 => {
    //   block [0x832346F0..0x83234760)
	// 832346F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832346F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832346F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832346FC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234700: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234704: 38CAD198  addi r6, r10, -0x2e68
	ctx.r[6].s64 = ctx.r[10].s64 + -11880;
	// 83234708: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323470C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83234710: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234714: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323471C: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83234720: 388835C4  addi r4, r8, 0x35c4
	ctx.r[4].s64 = ctx.r[8].s64 + 13764;
	// 83234724: 386777AC  addi r3, r7, 0x77ac
	ctx.r[3].s64 = ctx.r[7].s64 + 30636;
	// 83234728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323472C: 392935B0  addi r9, r9, 0x35b0
	ctx.r[9].s64 = ctx.r[9].s64 + 13744;
	// 83234730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234734: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83234738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323473C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234740: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234744: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83234748: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323474C: 4BC73A7D  bl 0x82ea81c8
	ctx.lr = 0x83234750;
	sub_82EA81C8(ctx, base);
	// 83234750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323475C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234760 size=24
    let mut pc: u32 = 0x83234760;
    'dispatch: loop {
        match pc {
            0x83234760 => {
    //   block [0x83234760..0x83234778)
	// 83234760: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83234764: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234768: 392AD228  addi r9, r10, -0x2dd8
	ctx.r[9].s64 = ctx.r[10].s64 + -11736;
	// 8323476C: 816BD210  lwz r11, -0x2df0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11760 as u32) ) } as u64;
	// 83234770: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83234774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234778 size=112
    let mut pc: u32 = 0x83234778;
    'dispatch: loop {
        match pc {
            0x83234778 => {
    //   block [0x83234778..0x832347E8)
	// 83234778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323477C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234784: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234788: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323478C: 38CAD228  addi r6, r10, -0x2dd8
	ctx.r[6].s64 = ctx.r[10].s64 + -11736;
	// 83234790: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234794: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83234798: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323479C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832347A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832347A4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 832347A8: 388836A0  addi r4, r8, 0x36a0
	ctx.r[4].s64 = ctx.r[8].s64 + 13984;
	// 832347AC: 386777DC  addi r3, r7, 0x77dc
	ctx.r[3].s64 = ctx.r[7].s64 + 30684;
	// 832347B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832347B4: 3929368C  addi r9, r9, 0x368c
	ctx.r[9].s64 = ctx.r[9].s64 + 13964;
	// 832347B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832347BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832347C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832347C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832347C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832347CC: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 832347D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832347D4: 4BC739F5  bl 0x82ea81c8
	ctx.lr = 0x832347D8;
	sub_82EA81C8(ctx, base);
	// 832347D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832347DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832347E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832347E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832347E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832347E8 size=108
    let mut pc: u32 = 0x832347E8;
    'dispatch: loop {
        match pc {
            0x832347E8 => {
    //   block [0x832347E8..0x83234854)
	// 832347E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832347EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832347F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832347F4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832347F8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832347FC: 38EA36DC  addi r7, r10, 0x36dc
	ctx.r[7].s64 = ctx.r[10].s64 + 14044;
	// 83234800: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234804: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323480C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234810: 38893724  addi r4, r9, 0x3724
	ctx.r[4].s64 = ctx.r[9].s64 + 14116;
	// 83234814: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234818: 3868780C  addi r3, r8, 0x780c
	ctx.r[3].s64 = ctx.r[8].s64 + 30732;
	// 8323481C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234824: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234828: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323482C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234830: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234838: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323483C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234840: 4BC73989  bl 0x82ea81c8
	ctx.lr = 0x83234844;
	sub_82EA81C8(ctx, base);
	// 83234844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323484C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234858 size=108
    let mut pc: u32 = 0x83234858;
    'dispatch: loop {
        match pc {
            0x83234858 => {
    //   block [0x83234858..0x832348C4)
	// 83234858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323485C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234864: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234868: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323486C: 38EA370C  addi r7, r10, 0x370c
	ctx.r[7].s64 = ctx.r[10].s64 + 14092;
	// 83234870: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234874: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83234878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323487C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234880: 3889373C  addi r4, r9, 0x373c
	ctx.r[4].s64 = ctx.r[9].s64 + 14140;
	// 83234884: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234888: 3868783C  addi r3, r8, 0x783c
	ctx.r[3].s64 = ctx.r[8].s64 + 30780;
	// 8323488C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234894: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234898: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323489C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832348A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832348A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832348A8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832348AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832348B0: 4BC73919  bl 0x82ea81c8
	ctx.lr = 0x832348B4;
	sub_82EA81C8(ctx, base);
	// 832348B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832348B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832348BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832348C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832348C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832348C8 size=108
    let mut pc: u32 = 0x832348C8;
    'dispatch: loop {
        match pc {
            0x832348C8 => {
    //   block [0x832348C8..0x83234934)
	// 832348C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832348CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832348D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832348D4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832348D8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832348DC: 38EA375C  addi r7, r10, 0x375c
	ctx.r[7].s64 = ctx.r[10].s64 + 14172;
	// 832348E0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832348E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 832348E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832348EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832348F0: 38893774  addi r4, r9, 0x3774
	ctx.r[4].s64 = ctx.r[9].s64 + 14196;
	// 832348F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832348F8: 3868786C  addi r3, r8, 0x786c
	ctx.r[3].s64 = ctx.r[8].s64 + 30828;
	// 832348FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234904: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234908: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323490C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234910: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234918: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323491C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234920: 4BC738A9  bl 0x82ea81c8
	ctx.lr = 0x83234924;
	sub_82EA81C8(ctx, base);
	// 83234924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323492C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234938 size=108
    let mut pc: u32 = 0x83234938;
    'dispatch: loop {
        match pc {
            0x83234938 => {
    //   block [0x83234938..0x832349A4)
	// 83234938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234944: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234948: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323494C: 38EA37B8  addi r7, r10, 0x37b8
	ctx.r[7].s64 = ctx.r[10].s64 + 14264;
	// 83234950: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234954: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83234958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323495C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234960: 38893860  addi r4, r9, 0x3860
	ctx.r[4].s64 = ctx.r[9].s64 + 14432;
	// 83234964: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234968: 3868789C  addi r3, r8, 0x789c
	ctx.r[3].s64 = ctx.r[8].s64 + 30876;
	// 8323496C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234974: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234978: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323497C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234988: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323498C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234990: 4BC73839  bl 0x82ea81c8
	ctx.lr = 0x83234994;
	sub_82EA81C8(ctx, base);
	// 83234994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323499C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832349A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832349A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832349A8 size=108
    let mut pc: u32 = 0x832349A8;
    'dispatch: loop {
        match pc {
            0x832349A8 => {
    //   block [0x832349A8..0x83234A14)
	// 832349A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832349AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832349B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832349B4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832349B8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832349BC: 38EA387C  addi r7, r10, 0x387c
	ctx.r[7].s64 = ctx.r[10].s64 + 14460;
	// 832349C0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832349C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 832349C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832349CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832349D0: 38893894  addi r4, r9, 0x3894
	ctx.r[4].s64 = ctx.r[9].s64 + 14484;
	// 832349D4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832349D8: 386878CC  addi r3, r8, 0x78cc
	ctx.r[3].s64 = ctx.r[8].s64 + 30924;
	// 832349DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832349E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832349E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832349E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832349EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832349F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832349F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832349F8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832349FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234A00: 4BC737C9  bl 0x82ea81c8
	ctx.lr = 0x83234A04;
	sub_82EA81C8(ctx, base);
	// 83234A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234A18 size=108
    let mut pc: u32 = 0x83234A18;
    'dispatch: loop {
        match pc {
            0x83234A18 => {
    //   block [0x83234A18..0x83234A84)
	// 83234A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234A24: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234A28: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234A2C: 38EA38A8  addi r7, r10, 0x38a8
	ctx.r[7].s64 = ctx.r[10].s64 + 14504;
	// 83234A30: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234A34: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234A3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234A40: 388938D8  addi r4, r9, 0x38d8
	ctx.r[4].s64 = ctx.r[9].s64 + 14552;
	// 83234A44: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234A48: 386878FC  addi r3, r8, 0x78fc
	ctx.r[3].s64 = ctx.r[8].s64 + 30972;
	// 83234A4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234A54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234A58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234A5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234A68: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234A70: 4BC73759  bl 0x82ea81c8
	ctx.lr = 0x83234A74;
	sub_82EA81C8(ctx, base);
	// 83234A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234A88 size=108
    let mut pc: u32 = 0x83234A88;
    'dispatch: loop {
        match pc {
            0x83234A88 => {
    //   block [0x83234A88..0x83234AF4)
	// 83234A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234A94: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83234A98: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234A9C: 392B3978  addi r9, r11, 0x3978
	ctx.r[9].s64 = ctx.r[11].s64 + 14712;
	// 83234AA0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234AA4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 83234AA8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234AAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234AB4: 388A39BC  addi r4, r10, 0x39bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14780;
	// 83234AB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234ABC: 3868792C  addi r3, r8, 0x792c
	ctx.r[3].s64 = ctx.r[8].s64 + 31020;
	// 83234AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234AC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83234AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234AD8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234AE0: 4BC736E9  bl 0x82ea81c8
	ctx.lr = 0x83234AE4;
	sub_82EA81C8(ctx, base);
	// 83234AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234AF8 size=24
    let mut pc: u32 = 0x83234AF8;
    'dispatch: loop {
        match pc {
            0x83234AF8 => {
    //   block [0x83234AF8..0x83234B10)
	// 83234AF8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83234AFC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234B00: 392AD324  addi r9, r10, -0x2cdc
	ctx.r[9].s64 = ctx.r[10].s64 + -11484;
	// 83234B04: 816BD300  lwz r11, -0x2d00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11520 as u32) ) } as u64;
	// 83234B08: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83234B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234B10 size=108
    let mut pc: u32 = 0x83234B10;
    'dispatch: loop {
        match pc {
            0x83234B10 => {
    //   block [0x83234B10..0x83234B7C)
	// 83234B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234B1C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234B20: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234B24: 38EAD324  addi r7, r10, -0x2cdc
	ctx.r[7].s64 = ctx.r[10].s64 + -11484;
	// 83234B28: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234B2C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234B30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234B34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234B38: 388939D4  addi r4, r9, 0x39d4
	ctx.r[4].s64 = ctx.r[9].s64 + 14804;
	// 83234B3C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234B40: 3868795C  addi r3, r8, 0x795c
	ctx.r[3].s64 = ctx.r[8].s64 + 31068;
	// 83234B44: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234B4C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234B50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234B54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234B58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234B60: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234B68: 4BC73661  bl 0x82ea81c8
	ctx.lr = 0x83234B6C;
	sub_82EA81C8(ctx, base);
	// 83234B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234B80 size=108
    let mut pc: u32 = 0x83234B80;
    'dispatch: loop {
        match pc {
            0x83234B80 => {
    //   block [0x83234B80..0x83234BEC)
	// 83234B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234B8C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234B90: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234B94: 38EA39F4  addi r7, r10, 0x39f4
	ctx.r[7].s64 = ctx.r[10].s64 + 14836;
	// 83234B98: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234B9C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83234BA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234BA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234BA8: 38893A0C  addi r4, r9, 0x3a0c
	ctx.r[4].s64 = ctx.r[9].s64 + 14860;
	// 83234BAC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234BB0: 3868798C  addi r3, r8, 0x798c
	ctx.r[3].s64 = ctx.r[8].s64 + 31116;
	// 83234BB4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234BBC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234BC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234BC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234BC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234BD0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83234BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234BD8: 4BC735F1  bl 0x82ea81c8
	ctx.lr = 0x83234BDC;
	sub_82EA81C8(ctx, base);
	// 83234BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234BF0 size=24
    let mut pc: u32 = 0x83234BF0;
    'dispatch: loop {
        match pc {
            0x83234BF0 => {
    //   block [0x83234BF0..0x83234C08)
	// 83234BF0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83234BF4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234BF8: 392AD374  addi r9, r10, -0x2c8c
	ctx.r[9].s64 = ctx.r[10].s64 + -11404;
	// 83234BFC: 816BD300  lwz r11, -0x2d00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11520 as u32) ) } as u64;
	// 83234C00: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83234C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234C08 size=108
    let mut pc: u32 = 0x83234C08;
    'dispatch: loop {
        match pc {
            0x83234C08 => {
    //   block [0x83234C08..0x83234C74)
	// 83234C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234C14: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234C18: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234C1C: 38EAD374  addi r7, r10, -0x2c8c
	ctx.r[7].s64 = ctx.r[10].s64 + -11404;
	// 83234C20: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234C24: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234C2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234C30: 38893A30  addi r4, r9, 0x3a30
	ctx.r[4].s64 = ctx.r[9].s64 + 14896;
	// 83234C34: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234C38: 386879BC  addi r3, r8, 0x79bc
	ctx.r[3].s64 = ctx.r[8].s64 + 31164;
	// 83234C3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234C44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234C48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234C4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234C50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234C58: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234C60: 4BC73569  bl 0x82ea81c8
	ctx.lr = 0x83234C64;
	sub_82EA81C8(ctx, base);
	// 83234C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234C78 size=24
    let mut pc: u32 = 0x83234C78;
    'dispatch: loop {
        match pc {
            0x83234C78 => {
    //   block [0x83234C78..0x83234C90)
	// 83234C78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83234C7C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234C80: 392AD3B4  addi r9, r10, -0x2c4c
	ctx.r[9].s64 = ctx.r[10].s64 + -11340;
	// 83234C84: 816BD300  lwz r11, -0x2d00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11520 as u32) ) } as u64;
	// 83234C88: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83234C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234C90 size=108
    let mut pc: u32 = 0x83234C90;
    'dispatch: loop {
        match pc {
            0x83234C90 => {
    //   block [0x83234C90..0x83234CFC)
	// 83234C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234C9C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83234CA0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234CA4: 38EAD3B4  addi r7, r10, -0x2c4c
	ctx.r[7].s64 = ctx.r[10].s64 + -11340;
	// 83234CA8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234CAC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83234CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83234CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234CB8: 38893A4C  addi r4, r9, 0x3a4c
	ctx.r[4].s64 = ctx.r[9].s64 + 14924;
	// 83234CBC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234CC0: 386879EC  addi r3, r8, 0x79ec
	ctx.r[3].s64 = ctx.r[8].s64 + 31212;
	// 83234CC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234CCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234CD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234CD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234CD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234CE0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234CE8: 4BC734E1  bl 0x82ea81c8
	ctx.lr = 0x83234CEC;
	sub_82EA81C8(ctx, base);
	// 83234CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83234D00 size=32
    let mut pc: u32 = 0x83234D00;
    'dispatch: loop {
        match pc {
            0x83234D00 => {
    //   block [0x83234D00..0x83234D20)
	// 83234D00: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234D04: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 83234D08: 38E8D400  addi r7, r8, -0x2c00
	ctx.r[7].s64 = ctx.r[8].s64 + -11264;
	// 83234D0C: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 83234D10: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83234D14: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 83234D18: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83234D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234D20 size=108
    let mut pc: u32 = 0x83234D20;
    'dispatch: loop {
        match pc {
            0x83234D20 => {
    //   block [0x83234D20..0x83234D8C)
	// 83234D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234D2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83234D30: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234D34: 396B3C78  addi r11, r11, 0x3c78
	ctx.r[11].s64 = ctx.r[11].s64 + 15480;
	// 83234D38: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234D3C: 38EB0120  addi r7, r11, 0x120
	ctx.r[7].s64 = ctx.r[11].s64 + 288;
	// 83234D40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83234D44: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83234D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234D4C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 83234D50: 38893DF4  addi r4, r9, 0x3df4
	ctx.r[4].s64 = ctx.r[9].s64 + 15860;
	// 83234D54: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83234D58: 38687A20  addi r3, r8, 0x7a20
	ctx.r[3].s64 = ctx.r[8].s64 + 31264;
	// 83234D5C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83234D60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234D64: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83234D68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234D70: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83234D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234D78: 4BC73451  bl 0x82ea81c8
	ctx.lr = 0x83234D7C;
	sub_82EA81C8(ctx, base);
	// 83234D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234D90 size=96
    let mut pc: u32 = 0x83234D90;
    'dispatch: loop {
        match pc {
            0x83234D90 => {
    //   block [0x83234D90..0x83234DF0)
	// 83234D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234D9C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234DA0: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234DA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234DA8: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 83234DAC: 38697A50  addi r3, r9, 0x7a50
	ctx.r[3].s64 = ctx.r[9].s64 + 31312;
	// 83234DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234DC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83234DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83234DD4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83234DD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83234DDC: 4BC733ED  bl 0x82ea81c8
	ctx.lr = 0x83234DE0;
	sub_82EA81C8(ctx, base);
	// 83234DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234DF0 size=120
    let mut pc: u32 = 0x83234DF0;
    'dispatch: loop {
        match pc {
            0x83234DF0 => {
    //   block [0x83234DF0..0x83234E68)
	// 83234DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83234DFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234E00: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234E04: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234E08: 38CA3E70  addi r6, r10, 0x3e70
	ctx.r[6].s64 = ctx.r[10].s64 + 15984;
	// 83234E0C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234E10: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234E14: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234E1C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 83234E20: 38883EE4  addi r4, r8, 0x3ee4
	ctx.r[4].s64 = ctx.r[8].s64 + 16100;
	// 83234E24: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234E28: 38677A80  addi r3, r7, 0x7a80
	ctx.r[3].s64 = ctx.r[7].s64 + 31360;
	// 83234E2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234E30: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83234E34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234E40: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234E48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234E4C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83234E50: 4BC73379  bl 0x82ea81c8
	ctx.lr = 0x83234E54;
	sub_82EA81C8(ctx, base);
	// 83234E54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83234E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234E68 size=116
    let mut pc: u32 = 0x83234E68;
    'dispatch: loop {
        match pc {
            0x83234E68 => {
    //   block [0x83234E68..0x83234EDC)
	// 83234E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234E74: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234E78: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234E7C: 38AA49C0  addi r5, r10, 0x49c0
	ctx.r[5].s64 = ctx.r[10].s64 + 18880;
	// 83234E80: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 83234E84: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 83234E88: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83234E8C: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 83234E90: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83234E94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234E98: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83234E9C: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83234EA0: 38874A50  addi r4, r7, 0x4a50
	ctx.r[4].s64 = ctx.r[7].s64 + 19024;
	// 83234EA4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234EA8: 38667AB4  addi r3, r6, 0x7ab4
	ctx.r[3].s64 = ctx.r[6].s64 + 31412;
	// 83234EAC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234EB0: 392949AC  addi r9, r9, 0x49ac
	ctx.r[9].s64 = ctx.r[9].s64 + 18860;
	// 83234EB4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234EB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83234EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234EC4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83234EC8: 4BC73301  bl 0x82ea81c8
	ctx.lr = 0x83234ECC;
	sub_82EA81C8(ctx, base);
	// 83234ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83234ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234EE0 size=120
    let mut pc: u32 = 0x83234EE0;
    'dispatch: loop {
        match pc {
            0x83234EE0 => {
    //   block [0x83234EE0..0x83234F58)
	// 83234EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83234EEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234EF0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234EF4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234EF8: 38CA4A68  addi r6, r10, 0x4a68
	ctx.r[6].s64 = ctx.r[10].s64 + 19048;
	// 83234EFC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234F00: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234F04: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234F0C: 38A97D54  addi r5, r9, 0x7d54
	ctx.r[5].s64 = ctx.r[9].s64 + 32084;
	// 83234F10: 38884A80  addi r4, r8, 0x4a80
	ctx.r[4].s64 = ctx.r[8].s64 + 19072;
	// 83234F14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234F18: 38677AE4  addi r3, r7, 0x7ae4
	ctx.r[3].s64 = ctx.r[7].s64 + 31460;
	// 83234F1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234F20: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83234F24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234F30: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234F38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234F3C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 83234F40: 4BC73289  bl 0x82ea81c8
	ctx.lr = 0x83234F44;
	sub_82EA81C8(ctx, base);
	// 83234F44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234F50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83234F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234F58 size=120
    let mut pc: u32 = 0x83234F58;
    'dispatch: loop {
        match pc {
            0x83234F58 => {
    //   block [0x83234F58..0x83234FD0)
	// 83234F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234F60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83234F64: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234F68: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234F6C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83234F70: 38CA4A9C  addi r6, r10, 0x4a9c
	ctx.r[6].s64 = ctx.r[10].s64 + 19100;
	// 83234F74: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83234F78: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83234F7C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83234F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234F84: 38A97B74  addi r5, r9, 0x7b74
	ctx.r[5].s64 = ctx.r[9].s64 + 31604;
	// 83234F88: 38884AB4  addi r4, r8, 0x4ab4
	ctx.r[4].s64 = ctx.r[8].s64 + 19124;
	// 83234F8C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234F90: 38677B14  addi r3, r7, 0x7b14
	ctx.r[3].s64 = ctx.r[7].s64 + 31508;
	// 83234F94: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83234F98: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83234F9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83234FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83234FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83234FA8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83234FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83234FB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83234FB4: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 83234FB8: 4BC73211  bl 0x82ea81c8
	ctx.lr = 0x83234FBC;
	sub_82EA81C8(ctx, base);
	// 83234FBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83234FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83234FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83234FC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83234FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83234FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83234FD0 size=104
    let mut pc: u32 = 0x83234FD0;
    'dispatch: loop {
        match pc {
            0x83234FD0 => {
    //   block [0x83234FD0..0x83235038)
	// 83234FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83234FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83234FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83234FDC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83234FE0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83234FE4: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 83234FE8: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 83234FEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83234FF0: 392A4B44  addi r9, r10, 0x4b44
	ctx.r[9].s64 = ctx.r[10].s64 + 19268;
	// 83234FF4: 38A87C34  addi r5, r8, 0x7c34
	ctx.r[5].s64 = ctx.r[8].s64 + 31796;
	// 83234FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83234FFC: 38874B58  addi r4, r7, 0x4b58
	ctx.r[4].s64 = ctx.r[7].s64 + 19288;
	// 83235000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235004: 38667B44  addi r3, r6, 0x7b44
	ctx.r[3].s64 = ctx.r[6].s64 + 31556;
	// 83235008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323500C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83235010: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83235014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235018: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323501C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235020: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83235024: 4BC731A5  bl 0x82ea81c8
	ctx.lr = 0x83235028;
	sub_82EA81C8(ctx, base);
	// 83235028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323502C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235038 size=120
    let mut pc: u32 = 0x83235038;
    'dispatch: loop {
        match pc {
            0x83235038 => {
    //   block [0x83235038..0x832350B0)
	// 83235038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323503C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235040: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235044: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235048: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323504C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235050: 38CA4B98  addi r6, r10, 0x4b98
	ctx.r[6].s64 = ctx.r[10].s64 + 19352;
	// 83235054: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235058: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323505C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235064: 38A97AB4  addi r5, r9, 0x7ab4
	ctx.r[5].s64 = ctx.r[9].s64 + 31412;
	// 83235068: 38884BC8  addi r4, r8, 0x4bc8
	ctx.r[4].s64 = ctx.r[8].s64 + 19400;
	// 8323506C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235070: 38677B74  addi r3, r7, 0x7b74
	ctx.r[3].s64 = ctx.r[7].s64 + 31604;
	// 83235074: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235078: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323507C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235088: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235090: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235094: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 83235098: 4BC73131  bl 0x82ea81c8
	ctx.lr = 0x8323509C;
	sub_82EA81C8(ctx, base);
	// 8323509C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832350A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832350A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832350A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832350AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832350B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832350B0 size=100
    let mut pc: u32 = 0x832350B0;
    'dispatch: loop {
        match pc {
            0x832350B0 => {
    //   block [0x832350B0..0x83235114)
	// 832350B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832350B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832350B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832350BC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832350C0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832350C4: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832350C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832350CC: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 832350D0: 38894BD4  addi r4, r9, 0x4bd4
	ctx.r[4].s64 = ctx.r[9].s64 + 19412;
	// 832350D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832350D8: 38687BA4  addi r3, r8, 0x7ba4
	ctx.r[3].s64 = ctx.r[8].s64 + 31652;
	// 832350DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832350E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832350E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832350E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832350EC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832350F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832350F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832350F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832350FC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83235100: 4BC730C9  bl 0x82ea81c8
	ctx.lr = 0x83235104;
	sub_82EA81C8(ctx, base);
	// 83235104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323510C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83235118 size=24
    let mut pc: u32 = 0x83235118;
    'dispatch: loop {
        match pc {
            0x83235118 => {
    //   block [0x83235118..0x83235130)
	// 83235118: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323511C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83235120: 392AD8E8  addi r9, r10, -0x2718
	ctx.r[9].s64 = ctx.r[10].s64 + -10008;
	// 83235124: 816BD8E0  lwz r11, -0x2720(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10016 as u32) ) } as u64;
	// 83235128: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235130 size=116
    let mut pc: u32 = 0x83235130;
    'dispatch: loop {
        match pc {
            0x83235130 => {
    //   block [0x83235130..0x832351A4)
	// 83235130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323513C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83235140: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83235144: 38AAD8E8  addi r5, r10, -0x2718
	ctx.r[5].s64 = ctx.r[10].s64 + -10008;
	// 83235148: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 8323514C: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 83235150: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83235154: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 83235158: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323515C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235160: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83235164: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83235168: 38874E10  addi r4, r7, 0x4e10
	ctx.r[4].s64 = ctx.r[7].s64 + 19984;
	// 8323516C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235170: 38667BD4  addi r3, r6, 0x7bd4
	ctx.r[3].s64 = ctx.r[6].s64 + 31700;
	// 83235174: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235178: 39294D30  addi r9, r9, 0x4d30
	ctx.r[9].s64 = ctx.r[9].s64 + 19760;
	// 8323517C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235180: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83235184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323518C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83235190: 4BC73039  bl 0x82ea81c8
	ctx.lr = 0x83235194;
	sub_82EA81C8(ctx, base);
	// 83235194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323519C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832351A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832351A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832351A8 size=120
    let mut pc: u32 = 0x832351A8;
    'dispatch: loop {
        match pc {
            0x832351A8 => {
    //   block [0x832351A8..0x83235220)
	// 832351A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832351AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832351B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832351B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832351B8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832351BC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832351C0: 38CA4E34  addi r6, r10, 0x4e34
	ctx.r[6].s64 = ctx.r[10].s64 + 20020;
	// 832351C4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832351C8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832351CC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832351D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832351D4: 38A97C64  addi r5, r9, 0x7c64
	ctx.r[5].s64 = ctx.r[9].s64 + 31844;
	// 832351D8: 38884E64  addi r4, r8, 0x4e64
	ctx.r[4].s64 = ctx.r[8].s64 + 20068;
	// 832351DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832351E0: 38677C04  addi r3, r7, 0x7c04
	ctx.r[3].s64 = ctx.r[7].s64 + 31748;
	// 832351E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832351E8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832351EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832351F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832351F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832351F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832351FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235200: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235204: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83235208: 4BC72FC1  bl 0x82ea81c8
	ctx.lr = 0x8323520C;
	sub_82EA81C8(ctx, base);
	// 8323520C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323521C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235220 size=100
    let mut pc: u32 = 0x83235220;
    'dispatch: loop {
        match pc {
            0x83235220 => {
    //   block [0x83235220..0x83235284)
	// 83235220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323522C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83235230: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83235234: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83235238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323523C: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 83235240: 38894E84  addi r4, r9, 0x4e84
	ctx.r[4].s64 = ctx.r[9].s64 + 20100;
	// 83235244: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235248: 38687C34  addi r3, r8, 0x7c34
	ctx.r[3].s64 = ctx.r[8].s64 + 31796;
	// 8323524C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235254: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235258: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323525C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83235260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235264: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83235268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323526C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83235270: 4BC72F59  bl 0x82ea81c8
	ctx.lr = 0x83235274;
	sub_82EA81C8(ctx, base);
	// 83235274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323527C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83235288 size=24
    let mut pc: u32 = 0x83235288;
    'dispatch: loop {
        match pc {
            0x83235288 => {
    //   block [0x83235288..0x832352A0)
	// 83235288: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323528C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83235290: 392AD9F4  addi r9, r10, -0x260c
	ctx.r[9].s64 = ctx.r[10].s64 + -9740;
	// 83235294: 816BD9F0  lwz r11, -0x2610(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9744 as u32) ) } as u64;
	// 83235298: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323529C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832352A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832352A0 size=112
    let mut pc: u32 = 0x832352A0;
    'dispatch: loop {
        match pc {
            0x832352A0 => {
    //   block [0x832352A0..0x83235310)
	// 832352A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832352A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832352A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832352AC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832352B0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832352B4: 38AAD9F4  addi r5, r10, -0x260c
	ctx.r[5].s64 = ctx.r[10].s64 + -9740;
	// 832352B8: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 832352BC: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 832352C0: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 832352C4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832352C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832352CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832352D0: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 832352D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832352D8: 38874F40  addi r4, r7, 0x4f40
	ctx.r[4].s64 = ctx.r[7].s64 + 20288;
	// 832352DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832352E0: 38667C64  addi r3, r6, 0x7c64
	ctx.r[3].s64 = ctx.r[6].s64 + 31844;
	// 832352E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832352E8: 39294F2C  addi r9, r9, 0x4f2c
	ctx.r[9].s64 = ctx.r[9].s64 + 20268;
	// 832352EC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832352F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832352F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832352F8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832352FC: 4BC72ECD  bl 0x82ea81c8
	ctx.lr = 0x83235300;
	sub_82EA81C8(ctx, base);
	// 83235300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323530C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235310 size=120
    let mut pc: u32 = 0x83235310;
    'dispatch: loop {
        match pc {
            0x83235310 => {
    //   block [0x83235310..0x83235388)
	// 83235310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323531C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235320: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83235324: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83235328: 392B5290  addi r9, r11, 0x5290
	ctx.r[9].s64 = ctx.r[11].s64 + 21136;
	// 8323532C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235330: 38C90014  addi r6, r9, 0x14
	ctx.r[6].s64 = ctx.r[9].s64 + 20;
	// 83235334: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323533C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235340: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 83235344: 388852BC  addi r4, r8, 0x52bc
	ctx.r[4].s64 = ctx.r[8].s64 + 21180;
	// 83235348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323534C: 38677C94  addi r3, r7, 0x7c94
	ctx.r[3].s64 = ctx.r[7].s64 + 31892;
	// 83235350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235354: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83235358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323535C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83235360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235364: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235368: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323536C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83235370: 4BC72E59  bl 0x82ea81c8
	ctx.lr = 0x83235374;
	sub_82EA81C8(ctx, base);
	// 83235374: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323537C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235388 size=100
    let mut pc: u32 = 0x83235388;
    'dispatch: loop {
        match pc {
            0x83235388 => {
    //   block [0x83235388..0x832353EC)
	// 83235388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235394: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83235398: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323539C: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832353A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832353A4: 38AA7C94  addi r5, r10, 0x7c94
	ctx.r[5].s64 = ctx.r[10].s64 + 31892;
	// 832353A8: 388952D0  addi r4, r9, 0x52d0
	ctx.r[4].s64 = ctx.r[9].s64 + 21200;
	// 832353AC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832353B0: 38687CC4  addi r3, r8, 0x7cc4
	ctx.r[3].s64 = ctx.r[8].s64 + 31940;
	// 832353B4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832353B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832353BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832353C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832353C4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832353C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832353CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832353D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832353D4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832353D8: 4BC72DF1  bl 0x82ea81c8
	ctx.lr = 0x832353DC;
	sub_82EA81C8(ctx, base);
	// 832353DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832353E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832353E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832353E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832353F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832353F0 size=120
    let mut pc: u32 = 0x832353F0;
    'dispatch: loop {
        match pc {
            0x832353F0 => {
    //   block [0x832353F0..0x83235468)
	// 832353F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832353F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832353F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832353FC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235400: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235404: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235408: 38CA52F8  addi r6, r10, 0x52f8
	ctx.r[6].s64 = ctx.r[10].s64 + 21240;
	// 8323540C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235410: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235414: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323541C: 38A97D54  addi r5, r9, 0x7d54
	ctx.r[5].s64 = ctx.r[9].s64 + 32084;
	// 83235420: 38885328  addi r4, r8, 0x5328
	ctx.r[4].s64 = ctx.r[8].s64 + 21288;
	// 83235424: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235428: 38677CF4  addi r3, r7, 0x7cf4
	ctx.r[3].s64 = ctx.r[7].s64 + 31988;
	// 8323542C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235430: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83235434: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323543C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235440: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235448: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323544C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83235450: 4BC72D79  bl 0x82ea81c8
	ctx.lr = 0x83235454;
	sub_82EA81C8(ctx, base);
	// 83235454: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323545C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235468 size=120
    let mut pc: u32 = 0x83235468;
    'dispatch: loop {
        match pc {
            0x83235468 => {
    //   block [0x83235468..0x832354E0)
	// 83235468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323546C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235474: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235478: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323547C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235480: 38CA5344  addi r6, r10, 0x5344
	ctx.r[6].s64 = ctx.r[10].s64 + 21316;
	// 83235484: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235488: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323548C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235494: 38A97D54  addi r5, r9, 0x7d54
	ctx.r[5].s64 = ctx.r[9].s64 + 32084;
	// 83235498: 3888535C  addi r4, r8, 0x535c
	ctx.r[4].s64 = ctx.r[8].s64 + 21340;
	// 8323549C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832354A0: 38677D24  addi r3, r7, 0x7d24
	ctx.r[3].s64 = ctx.r[7].s64 + 32036;
	// 832354A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832354A8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 832354AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832354B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832354B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832354B8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832354BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832354C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832354C4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832354C8: 4BC72D01  bl 0x82ea81c8
	ctx.lr = 0x832354CC;
	sub_82EA81C8(ctx, base);
	// 832354CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832354D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832354D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832354D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832354DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832354E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832354E0 size=120
    let mut pc: u32 = 0x832354E0;
    'dispatch: loop {
        match pc {
            0x832354E0 => {
    //   block [0x832354E0..0x83235558)
	// 832354E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832354E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832354E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832354EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832354F0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832354F4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832354F8: 38CA5378  addi r6, r10, 0x5378
	ctx.r[6].s64 = ctx.r[10].s64 + 21368;
	// 832354FC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235500: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235504: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323550C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 83235510: 388853D8  addi r4, r8, 0x53d8
	ctx.r[4].s64 = ctx.r[8].s64 + 21464;
	// 83235514: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235518: 38677D54  addi r3, r7, 0x7d54
	ctx.r[3].s64 = ctx.r[7].s64 + 32084;
	// 8323551C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235520: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83235524: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323552C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235530: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323553C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83235540: 4BC72C89  bl 0x82ea81c8
	ctx.lr = 0x83235544;
	sub_82EA81C8(ctx, base);
	// 83235544: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323554C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235558 size=120
    let mut pc: u32 = 0x83235558;
    'dispatch: loop {
        match pc {
            0x83235558 => {
    //   block [0x83235558..0x832355D0)
	// 83235558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323555C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235564: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235568: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323556C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235570: 38CA53F8  addi r6, r10, 0x53f8
	ctx.r[6].s64 = ctx.r[10].s64 + 21496;
	// 83235574: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235578: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323557C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235584: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 83235588: 38885410  addi r4, r8, 0x5410
	ctx.r[4].s64 = ctx.r[8].s64 + 21520;
	// 8323558C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235590: 38677D84  addi r3, r7, 0x7d84
	ctx.r[3].s64 = ctx.r[7].s64 + 32132;
	// 83235594: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235598: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323559C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832355A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832355A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832355A8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832355AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832355B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832355B4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832355B8: 4BC72C11  bl 0x82ea81c8
	ctx.lr = 0x832355BC;
	sub_82EA81C8(ctx, base);
	// 832355BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832355C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832355C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832355C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832355CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832355D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832355D0 size=24
    let mut pc: u32 = 0x832355D0;
    'dispatch: loop {
        match pc {
            0x832355D0 => {
    //   block [0x832355D0..0x832355E8)
	// 832355D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832355D4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832355D8: 392ADBFC  addi r9, r10, -0x2404
	ctx.r[9].s64 = ctx.r[10].s64 + -9220;
	// 832355DC: 816BDA10  lwz r11, -0x25f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9712 as u32) ) } as u64;
	// 832355E0: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832355E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832355E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832355E8 size=112
    let mut pc: u32 = 0x832355E8;
    'dispatch: loop {
        match pc {
            0x832355E8 => {
    //   block [0x832355E8..0x83235658)
	// 832355E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832355EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832355F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832355F4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832355F8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832355FC: 38CADBFC  addi r6, r10, -0x2404
	ctx.r[6].s64 = ctx.r[10].s64 + -9220;
	// 83235600: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235604: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83235608: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323560C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83235610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235614: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83235618: 38886450  addi r4, r8, 0x6450
	ctx.r[4].s64 = ctx.r[8].s64 + 25680;
	// 8323561C: 38677DB4  addi r3, r7, 0x7db4
	ctx.r[3].s64 = ctx.r[7].s64 + 32180;
	// 83235620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235624: 39295C40  addi r9, r9, 0x5c40
	ctx.r[9].s64 = ctx.r[9].s64 + 23616;
	// 83235628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323562C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83235630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235638: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323563C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83235640: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83235644: 4BC72B85  bl 0x82ea81c8
	ctx.lr = 0x83235648;
	sub_82EA81C8(ctx, base);
	// 83235648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323564C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235658 size=120
    let mut pc: u32 = 0x83235658;
    'dispatch: loop {
        match pc {
            0x83235658 => {
    //   block [0x83235658..0x832356D0)
	// 83235658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323565C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235664: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235668: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323566C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235670: 38CA5C68  addi r6, r10, 0x5c68
	ctx.r[6].s64 = ctx.r[10].s64 + 23656;
	// 83235674: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235678: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 8323567C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235684: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235688: 38886464  addi r4, r8, 0x6464
	ctx.r[4].s64 = ctx.r[8].s64 + 25700;
	// 8323568C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235690: 38677DE4  addi r3, r7, 0x7de4
	ctx.r[3].s64 = ctx.r[7].s64 + 32228;
	// 83235694: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235698: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323569C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832356A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832356A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832356A8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832356AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832356B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832356B4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832356B8: 4BC72B11  bl 0x82ea81c8
	ctx.lr = 0x832356BC;
	sub_82EA81C8(ctx, base);
	// 832356BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832356C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832356C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832356C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832356CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832356D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832356D0 size=108
    let mut pc: u32 = 0x832356D0;
    'dispatch: loop {
        match pc {
            0x832356D0 => {
    //   block [0x832356D0..0x8323573C)
	// 832356D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832356D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832356D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832356DC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832356E0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832356E4: 38EA5C98  addi r7, r10, 0x5c98
	ctx.r[7].s64 = ctx.r[10].s64 + 23704;
	// 832356E8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832356EC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 832356F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832356F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832356F8: 3889647C  addi r4, r9, 0x647c
	ctx.r[4].s64 = ctx.r[9].s64 + 25724;
	// 832356FC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83235700: 38687E14  addi r3, r8, 0x7e14
	ctx.r[3].s64 = ctx.r[8].s64 + 32276;
	// 83235704: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323570C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235710: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235714: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235718: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323571C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235720: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83235724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83235728: 4BC72AA1  bl 0x82ea81c8
	ctx.lr = 0x8323572C;
	sub_82EA81C8(ctx, base);
	// 8323572C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235740 size=120
    let mut pc: u32 = 0x83235740;
    'dispatch: loop {
        match pc {
            0x83235740 => {
    //   block [0x83235740..0x832357B8)
	// 83235740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323574C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235750: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235754: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235758: 38CA5CB0  addi r6, r10, 0x5cb0
	ctx.r[6].s64 = ctx.r[10].s64 + 23728;
	// 8323575C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235760: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235764: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323576C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235770: 3888648C  addi r4, r8, 0x648c
	ctx.r[4].s64 = ctx.r[8].s64 + 25740;
	// 83235774: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235778: 38677E44  addi r3, r7, 0x7e44
	ctx.r[3].s64 = ctx.r[7].s64 + 32324;
	// 8323577C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235780: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 83235784: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323578C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235790: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235798: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323579C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832357A0: 4BC72A29  bl 0x82ea81c8
	ctx.lr = 0x832357A4;
	sub_82EA81C8(ctx, base);
	// 832357A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832357A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832357AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832357B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832357B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832357B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832357B8 size=100
    let mut pc: u32 = 0x832357B8;
    'dispatch: loop {
        match pc {
            0x832357B8 => {
    //   block [0x832357B8..0x8323581C)
	// 832357B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832357BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832357C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832357C4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832357C8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832357CC: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832357D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832357D4: 38AA7DB4  addi r5, r10, 0x7db4
	ctx.r[5].s64 = ctx.r[10].s64 + 32180;
	// 832357D8: 388964AC  addi r4, r9, 0x64ac
	ctx.r[4].s64 = ctx.r[9].s64 + 25772;
	// 832357DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832357E0: 38687E74  addi r3, r8, 0x7e74
	ctx.r[3].s64 = ctx.r[8].s64 + 32372;
	// 832357E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832357E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832357EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832357F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832357F4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832357F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832357FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83235800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235804: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83235808: 4BC729C1  bl 0x82ea81c8
	ctx.lr = 0x8323580C;
	sub_82EA81C8(ctx, base);
	// 8323580C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235820 size=120
    let mut pc: u32 = 0x83235820;
    'dispatch: loop {
        match pc {
            0x83235820 => {
    //   block [0x83235820..0x83235898)
	// 83235820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323582C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235830: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235834: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235838: 38CA5D70  addi r6, r10, 0x5d70
	ctx.r[6].s64 = ctx.r[10].s64 + 23920;
	// 8323583C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235840: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235844: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323584C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235850: 388864C8  addi r4, r8, 0x64c8
	ctx.r[4].s64 = ctx.r[8].s64 + 25800;
	// 83235854: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235858: 38677EA4  addi r3, r7, 0x7ea4
	ctx.r[3].s64 = ctx.r[7].s64 + 32420;
	// 8323585C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235860: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83235864: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323586C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235870: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235878: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323587C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83235880: 4BC72949  bl 0x82ea81c8
	ctx.lr = 0x83235884;
	sub_82EA81C8(ctx, base);
	// 83235884: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323588C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235898 size=120
    let mut pc: u32 = 0x83235898;
    'dispatch: loop {
        match pc {
            0x83235898 => {
    //   block [0x83235898..0x83235910)
	// 83235898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832358A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832358A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832358A8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832358AC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832358B0: 38CA5D88  addi r6, r10, 0x5d88
	ctx.r[6].s64 = ctx.r[10].s64 + 23944;
	// 832358B4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832358B8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832358BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832358C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832358C4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 832358C8: 388864E8  addi r4, r8, 0x64e8
	ctx.r[4].s64 = ctx.r[8].s64 + 25832;
	// 832358CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832358D0: 38677ED4  addi r3, r7, 0x7ed4
	ctx.r[3].s64 = ctx.r[7].s64 + 32468;
	// 832358D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832358D8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832358DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832358E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832358E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832358E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832358EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832358F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832358F4: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 832358F8: 4BC728D1  bl 0x82ea81c8
	ctx.lr = 0x832358FC;
	sub_82EA81C8(ctx, base);
	// 832358FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323590C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235910 size=120
    let mut pc: u32 = 0x83235910;
    'dispatch: loop {
        match pc {
            0x83235910 => {
    //   block [0x83235910..0x83235988)
	// 83235910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323591C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235920: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235924: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235928: 38CA5DB8  addi r6, r10, 0x5db8
	ctx.r[6].s64 = ctx.r[10].s64 + 23992;
	// 8323592C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235930: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235934: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323593C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235940: 3888650C  addi r4, r8, 0x650c
	ctx.r[4].s64 = ctx.r[8].s64 + 25868;
	// 83235944: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235948: 38677F04  addi r3, r7, 0x7f04
	ctx.r[3].s64 = ctx.r[7].s64 + 32516;
	// 8323594C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235950: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83235954: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235960: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235968: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323596C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83235970: 4BC72859  bl 0x82ea81c8
	ctx.lr = 0x83235974;
	sub_82EA81C8(ctx, base);
	// 83235974: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323597C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235988 size=120
    let mut pc: u32 = 0x83235988;
    'dispatch: loop {
        match pc {
            0x83235988 => {
    //   block [0x83235988..0x83235A00)
	// 83235988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235994: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235998: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323599C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832359A0: 38CA5DE8  addi r6, r10, 0x5de8
	ctx.r[6].s64 = ctx.r[10].s64 + 24040;
	// 832359A4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832359A8: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 832359AC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832359B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832359B4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 832359B8: 38886534  addi r4, r8, 0x6534
	ctx.r[4].s64 = ctx.r[8].s64 + 25908;
	// 832359BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832359C0: 38677F34  addi r3, r7, 0x7f34
	ctx.r[3].s64 = ctx.r[7].s64 + 32564;
	// 832359C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832359C8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832359CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832359D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832359D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832359D8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832359DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832359E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832359E4: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 832359E8: 4BC727E1  bl 0x82ea81c8
	ctx.lr = 0x832359EC;
	sub_82EA81C8(ctx, base);
	// 832359EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832359F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832359F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832359F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832359FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235A00 size=120
    let mut pc: u32 = 0x83235A00;
    'dispatch: loop {
        match pc {
            0x83235A00 => {
    //   block [0x83235A00..0x83235A78)
	// 83235A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235A0C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235A10: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235A14: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235A18: 38CA5E18  addi r6, r10, 0x5e18
	ctx.r[6].s64 = ctx.r[10].s64 + 24088;
	// 83235A1C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235A20: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235A24: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235A2C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235A30: 38886558  addi r4, r8, 0x6558
	ctx.r[4].s64 = ctx.r[8].s64 + 25944;
	// 83235A34: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235A38: 38677F64  addi r3, r7, 0x7f64
	ctx.r[3].s64 = ctx.r[7].s64 + 32612;
	// 83235A3C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235A40: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83235A44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235A50: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235A58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235A5C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83235A60: 4BC72769  bl 0x82ea81c8
	ctx.lr = 0x83235A64;
	sub_82EA81C8(ctx, base);
	// 83235A64: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235A78 size=120
    let mut pc: u32 = 0x83235A78;
    'dispatch: loop {
        match pc {
            0x83235A78 => {
    //   block [0x83235A78..0x83235AF0)
	// 83235A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235A84: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235A88: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235A8C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235A90: 38CA5E30  addi r6, r10, 0x5e30
	ctx.r[6].s64 = ctx.r[10].s64 + 24112;
	// 83235A94: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235A98: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235A9C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235AA4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235AA8: 38886578  addi r4, r8, 0x6578
	ctx.r[4].s64 = ctx.r[8].s64 + 25976;
	// 83235AAC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235AB0: 38677F94  addi r3, r7, 0x7f94
	ctx.r[3].s64 = ctx.r[7].s64 + 32660;
	// 83235AB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235AB8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83235ABC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235AC8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235AD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235AD4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83235AD8: 4BC726F1  bl 0x82ea81c8
	ctx.lr = 0x83235ADC;
	sub_82EA81C8(ctx, base);
	// 83235ADC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235AF0 size=120
    let mut pc: u32 = 0x83235AF0;
    'dispatch: loop {
        match pc {
            0x83235AF0 => {
    //   block [0x83235AF0..0x83235B68)
	// 83235AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235AFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235B00: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235B04: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235B08: 38CA5E48  addi r6, r10, 0x5e48
	ctx.r[6].s64 = ctx.r[10].s64 + 24136;
	// 83235B0C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235B10: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235B14: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235B1C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235B20: 38886590  addi r4, r8, 0x6590
	ctx.r[4].s64 = ctx.r[8].s64 + 26000;
	// 83235B24: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235B28: 38677FC4  addi r3, r7, 0x7fc4
	ctx.r[3].s64 = ctx.r[7].s64 + 32708;
	// 83235B2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235B30: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83235B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235B40: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235B48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235B4C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83235B50: 4BC72679  bl 0x82ea81c8
	ctx.lr = 0x83235B54;
	sub_82EA81C8(ctx, base);
	// 83235B54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235B60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235B68 size=120
    let mut pc: u32 = 0x83235B68;
    'dispatch: loop {
        match pc {
            0x83235B68 => {
    //   block [0x83235B68..0x83235BE0)
	// 83235B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235B74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235B78: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235B7C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235B80: 38CA5E90  addi r6, r10, 0x5e90
	ctx.r[6].s64 = ctx.r[10].s64 + 24208;
	// 83235B84: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235B88: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 83235B8C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235B94: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235B98: 388865AC  addi r4, r8, 0x65ac
	ctx.r[4].s64 = ctx.r[8].s64 + 26028;
	// 83235B9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235BA0: 38677FF4  addi r3, r7, 0x7ff4
	ctx.r[3].s64 = ctx.r[7].s64 + 32756;
	// 83235BA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235BA8: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83235BAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235BB8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235BC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235BC4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83235BC8: 4BC72601  bl 0x82ea81c8
	ctx.lr = 0x83235BCC;
	sub_82EA81C8(ctx, base);
	// 83235BCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235BE0 size=120
    let mut pc: u32 = 0x83235BE0;
    'dispatch: loop {
        match pc {
            0x83235BE0 => {
    //   block [0x83235BE0..0x83235C58)
	// 83235BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235BEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235BF0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235BF4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235BF8: 38CA5ED8  addi r6, r10, 0x5ed8
	ctx.r[6].s64 = ctx.r[10].s64 + 24280;
	// 83235BFC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235C00: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235C04: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235C0C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235C10: 388865C8  addi r4, r8, 0x65c8
	ctx.r[4].s64 = ctx.r[8].s64 + 26056;
	// 83235C14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235C18: 38678024  addi r3, r7, -0x7fdc
	ctx.r[3].s64 = ctx.r[7].s64 + -32732;
	// 83235C1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235C20: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83235C24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235C30: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235C38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235C3C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83235C40: 4BC72589  bl 0x82ea81c8
	ctx.lr = 0x83235C44;
	sub_82EA81C8(ctx, base);
	// 83235C44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235C58 size=120
    let mut pc: u32 = 0x83235C58;
    'dispatch: loop {
        match pc {
            0x83235C58 => {
    //   block [0x83235C58..0x83235CD0)
	// 83235C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235C64: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235C68: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235C6C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235C70: 38CA5EF0  addi r6, r10, 0x5ef0
	ctx.r[6].s64 = ctx.r[10].s64 + 24304;
	// 83235C74: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235C78: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235C7C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235C84: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235C88: 388865E0  addi r4, r8, 0x65e0
	ctx.r[4].s64 = ctx.r[8].s64 + 26080;
	// 83235C8C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235C90: 38678054  addi r3, r7, -0x7fac
	ctx.r[3].s64 = ctx.r[7].s64 + -32684;
	// 83235C94: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235C98: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83235C9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235CA8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235CB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235CB4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83235CB8: 4BC72511  bl 0x82ea81c8
	ctx.lr = 0x83235CBC;
	sub_82EA81C8(ctx, base);
	// 83235CBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235CC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235CD0 size=112
    let mut pc: u32 = 0x83235CD0;
    'dispatch: loop {
        match pc {
            0x83235CD0 => {
    //   block [0x83235CD0..0x83235D40)
	// 83235CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235CDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83235CE0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83235CE4: 396B5F20  addi r11, r11, 0x5f20
	ctx.r[11].s64 = ctx.r[11].s64 + 24352;
	// 83235CE8: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235CEC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83235CF0: 38CB0078  addi r6, r11, 0x78
	ctx.r[6].s64 = ctx.r[11].s64 + 120;
	// 83235CF4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83235CF8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235CFC: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235D00: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 83235D04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235D08: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235D0C: 388865F8  addi r4, r8, 0x65f8
	ctx.r[4].s64 = ctx.r[8].s64 + 26104;
	// 83235D10: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83235D14: 38678084  addi r3, r7, -0x7f7c
	ctx.r[3].s64 = ctx.r[7].s64 + -32636;
	// 83235D18: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83235D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235D20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235D28: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83235D2C: 4BC7249D  bl 0x82ea81c8
	ctx.lr = 0x83235D30;
	sub_82EA81C8(ctx, base);
	// 83235D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235D40 size=112
    let mut pc: u32 = 0x83235D40;
    'dispatch: loop {
        match pc {
            0x83235D40 => {
    //   block [0x83235D40..0x83235DB0)
	// 83235D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235D4C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83235D50: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83235D54: 396B5FB0  addi r11, r11, 0x5fb0
	ctx.r[11].s64 = ctx.r[11].s64 + 24496;
	// 83235D58: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235D5C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83235D60: 38CB0090  addi r6, r11, 0x90
	ctx.r[6].s64 = ctx.r[11].s64 + 144;
	// 83235D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83235D68: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235D6C: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235D70: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 83235D74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235D78: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235D7C: 38886614  addi r4, r8, 0x6614
	ctx.r[4].s64 = ctx.r[8].s64 + 26132;
	// 83235D80: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83235D84: 386780B4  addi r3, r7, -0x7f4c
	ctx.r[3].s64 = ctx.r[7].s64 + -32588;
	// 83235D88: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83235D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235D90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235D98: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83235D9C: 4BC7242D  bl 0x82ea81c8
	ctx.lr = 0x83235DA0;
	sub_82EA81C8(ctx, base);
	// 83235DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83235DB0 size=24
    let mut pc: u32 = 0x83235DB0;
    'dispatch: loop {
        match pc {
            0x83235DB0 => {
    //   block [0x83235DB0..0x83235DC8)
	// 83235DB0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83235DB4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83235DB8: 392ADC18  addi r9, r10, -0x23e8
	ctx.r[9].s64 = ctx.r[10].s64 + -9192;
	// 83235DBC: 816BDA18  lwz r11, -0x25e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9704 as u32) ) } as u64;
	// 83235DC0: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83235DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235DC8 size=120
    let mut pc: u32 = 0x83235DC8;
    'dispatch: loop {
        match pc {
            0x83235DC8 => {
    //   block [0x83235DC8..0x83235E40)
	// 83235DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235DD4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83235DD8: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83235DDC: 392B606C  addi r9, r11, 0x606c
	ctx.r[9].s64 = ctx.r[11].s64 + 24684;
	// 83235DE0: 388ADC18  addi r4, r10, -0x23e8
	ctx.r[4].s64 = ctx.r[10].s64 + -9192;
	// 83235DE4: 38A90014  addi r5, r9, 0x14
	ctx.r[5].s64 = ctx.r[9].s64 + 20;
	// 83235DE8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83235DEC: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83235DF0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83235DF4: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83235DF8: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 83235DFC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83235E00: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83235E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235E08: 38A87DB4  addi r5, r8, 0x7db4
	ctx.r[5].s64 = ctx.r[8].s64 + 32180;
	// 83235E0C: 38876630  addi r4, r7, 0x6630
	ctx.r[4].s64 = ctx.r[7].s64 + 26160;
	// 83235E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235E14: 386680E4  addi r3, r6, -0x7f1c
	ctx.r[3].s64 = ctx.r[6].s64 + -32540;
	// 83235E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235E1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83235E20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235E28: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83235E2C: 4BC7239D  bl 0x82ea81c8
	ctx.lr = 0x83235E30;
	sub_82EA81C8(ctx, base);
	// 83235E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83235E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235E40 size=120
    let mut pc: u32 = 0x83235E40;
    'dispatch: loop {
        match pc {
            0x83235E40 => {
    //   block [0x83235E40..0x83235EB8)
	// 83235E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235E4C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235E50: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235E54: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235E58: 38CA60A8  addi r6, r10, 0x60a8
	ctx.r[6].s64 = ctx.r[10].s64 + 24744;
	// 83235E5C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235E60: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235E64: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235E6C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235E70: 38886664  addi r4, r8, 0x6664
	ctx.r[4].s64 = ctx.r[8].s64 + 26212;
	// 83235E74: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235E78: 38678114  addi r3, r7, -0x7eec
	ctx.r[3].s64 = ctx.r[7].s64 + -32492;
	// 83235E7C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235E80: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83235E84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235E90: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235E98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235E9C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83235EA0: 4BC72329  bl 0x82ea81c8
	ctx.lr = 0x83235EA4;
	sub_82EA81C8(ctx, base);
	// 83235EA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235EB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235EB8 size=120
    let mut pc: u32 = 0x83235EB8;
    'dispatch: loop {
        match pc {
            0x83235EB8 => {
    //   block [0x83235EB8..0x83235F30)
	// 83235EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235EC4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235EC8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235ECC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235ED0: 38CA6108  addi r6, r10, 0x6108
	ctx.r[6].s64 = ctx.r[10].s64 + 24840;
	// 83235ED4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235ED8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235EDC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235EE4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235EE8: 38886684  addi r4, r8, 0x6684
	ctx.r[4].s64 = ctx.r[8].s64 + 26244;
	// 83235EEC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235EF0: 38678144  addi r3, r7, -0x7ebc
	ctx.r[3].s64 = ctx.r[7].s64 + -32444;
	// 83235EF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235EF8: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 83235EFC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235F08: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235F14: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83235F18: 4BC722B1  bl 0x82ea81c8
	ctx.lr = 0x83235F1C;
	sub_82EA81C8(ctx, base);
	// 83235F1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235F28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235F30 size=120
    let mut pc: u32 = 0x83235F30;
    'dispatch: loop {
        match pc {
            0x83235F30 => {
    //   block [0x83235F30..0x83235FA8)
	// 83235F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235F3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235F40: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235F44: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235F48: 38CA61B0  addi r6, r10, 0x61b0
	ctx.r[6].s64 = ctx.r[10].s64 + 25008;
	// 83235F4C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235F50: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235F54: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235F5C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235F60: 388866A0  addi r4, r8, 0x66a0
	ctx.r[4].s64 = ctx.r[8].s64 + 26272;
	// 83235F64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235F68: 38678174  addi r3, r7, -0x7e8c
	ctx.r[3].s64 = ctx.r[7].s64 + -32396;
	// 83235F6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235F70: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 83235F74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235F80: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83235F88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83235F8C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83235F90: 4BC72239  bl 0x82ea81c8
	ctx.lr = 0x83235F94;
	sub_82EA81C8(ctx, base);
	// 83235F94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83235F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83235F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83235FA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83235FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83235FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83235FA8 size=120
    let mut pc: u32 = 0x83235FA8;
    'dispatch: loop {
        match pc {
            0x83235FA8 => {
    //   block [0x83235FA8..0x83236020)
	// 83235FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83235FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83235FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83235FB4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83235FB8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83235FBC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83235FC0: 38CA6228  addi r6, r10, 0x6228
	ctx.r[6].s64 = ctx.r[10].s64 + 25128;
	// 83235FC4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83235FC8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83235FCC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83235FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83235FD4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83235FD8: 388866C0  addi r4, r8, 0x66c0
	ctx.r[4].s64 = ctx.r[8].s64 + 26304;
	// 83235FDC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83235FE0: 386781A4  addi r3, r7, -0x7e5c
	ctx.r[3].s64 = ctx.r[7].s64 + -32348;
	// 83235FE4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83235FE8: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83235FEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83235FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83235FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83235FF8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83235FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236000: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236004: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83236008: 4BC721C1  bl 0x82ea81c8
	ctx.lr = 0x8323600C;
	sub_82EA81C8(ctx, base);
	// 8323600C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323601C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236020 size=120
    let mut pc: u32 = 0x83236020;
    'dispatch: loop {
        match pc {
            0x83236020 => {
    //   block [0x83236020..0x83236098)
	// 83236020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323602C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236030: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236034: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236038: 38CA6270  addi r6, r10, 0x6270
	ctx.r[6].s64 = ctx.r[10].s64 + 25200;
	// 8323603C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236040: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236044: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323604C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83236050: 388866E0  addi r4, r8, 0x66e0
	ctx.r[4].s64 = ctx.r[8].s64 + 26336;
	// 83236054: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236058: 386781D4  addi r3, r7, -0x7e2c
	ctx.r[3].s64 = ctx.r[7].s64 + -32300;
	// 8323605C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236060: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 83236064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323606C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236070: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323607C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83236080: 4BC72149  bl 0x82ea81c8
	ctx.lr = 0x83236084;
	sub_82EA81C8(ctx, base);
	// 83236084: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323608C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236098 size=120
    let mut pc: u32 = 0x83236098;
    'dispatch: loop {
        match pc {
            0x83236098 => {
    //   block [0x83236098..0x83236110)
	// 83236098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832360A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832360A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832360A8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832360AC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832360B0: 38CA6300  addi r6, r10, 0x6300
	ctx.r[6].s64 = ctx.r[10].s64 + 25344;
	// 832360B4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832360B8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832360BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832360C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832360C4: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 832360C8: 388866FC  addi r4, r8, 0x66fc
	ctx.r[4].s64 = ctx.r[8].s64 + 26364;
	// 832360CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832360D0: 38678204  addi r3, r7, -0x7dfc
	ctx.r[3].s64 = ctx.r[7].s64 + -32252;
	// 832360D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832360D8: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 832360DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832360E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832360E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832360E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832360EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832360F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832360F4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832360F8: 4BC720D1  bl 0x82ea81c8
	ctx.lr = 0x832360FC;
	sub_82EA81C8(ctx, base);
	// 832360FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323610C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236110 size=120
    let mut pc: u32 = 0x83236110;
    'dispatch: loop {
        match pc {
            0x83236110 => {
    //   block [0x83236110..0x83236188)
	// 83236110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323611C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236120: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236124: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236128: 38CA6360  addi r6, r10, 0x6360
	ctx.r[6].s64 = ctx.r[10].s64 + 25440;
	// 8323612C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236130: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236134: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323613C: 38A97DB4  addi r5, r9, 0x7db4
	ctx.r[5].s64 = ctx.r[9].s64 + 32180;
	// 83236140: 38886714  addi r4, r8, 0x6714
	ctx.r[4].s64 = ctx.r[8].s64 + 26388;
	// 83236144: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236148: 38678234  addi r3, r7, -0x7dcc
	ctx.r[3].s64 = ctx.r[7].s64 + -32204;
	// 8323614C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236150: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83236154: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323615C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236160: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236168: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323616C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83236170: 4BC72059  bl 0x82ea81c8
	ctx.lr = 0x83236174;
	sub_82EA81C8(ctx, base);
	// 83236174: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323617C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236188 size=120
    let mut pc: u32 = 0x83236188;
    'dispatch: loop {
        match pc {
            0x83236188 => {
    //   block [0x83236188..0x83236200)
	// 83236188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323618C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236194: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236198: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323619C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832361A0: 38CA63C0  addi r6, r10, 0x63c0
	ctx.r[6].s64 = ctx.r[10].s64 + 25536;
	// 832361A4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832361A8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832361AC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832361B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832361B4: 38A98234  addi r5, r9, -0x7dcc
	ctx.r[5].s64 = ctx.r[9].s64 + -32204;
	// 832361B8: 38886730  addi r4, r8, 0x6730
	ctx.r[4].s64 = ctx.r[8].s64 + 26416;
	// 832361BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832361C0: 38678264  addi r3, r7, -0x7d9c
	ctx.r[3].s64 = ctx.r[7].s64 + -32156;
	// 832361C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832361C8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832361CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832361D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832361D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832361D8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832361DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832361E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832361E4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832361E8: 4BC71FE1  bl 0x82ea81c8
	ctx.lr = 0x832361EC;
	sub_82EA81C8(ctx, base);
	// 832361EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832361F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832361F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832361F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832361FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236200 size=120
    let mut pc: u32 = 0x83236200;
    'dispatch: loop {
        match pc {
            0x83236200 => {
    //   block [0x83236200..0x83236278)
	// 83236200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323620C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236210: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236214: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83236218: 38CA63F0  addi r6, r10, 0x63f0
	ctx.r[6].s64 = ctx.r[10].s64 + 25584;
	// 8323621C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236220: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236224: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323622C: 38A98234  addi r5, r9, -0x7dcc
	ctx.r[5].s64 = ctx.r[9].s64 + -32204;
	// 83236230: 38886758  addi r4, r8, 0x6758
	ctx.r[4].s64 = ctx.r[8].s64 + 26456;
	// 83236234: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236238: 38678294  addi r3, r7, -0x7d6c
	ctx.r[3].s64 = ctx.r[7].s64 + -32108;
	// 8323623C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236240: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83236244: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323624C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236250: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236258: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323625C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83236260: 4BC71F69  bl 0x82ea81c8
	ctx.lr = 0x83236264;
	sub_82EA81C8(ctx, base);
	// 83236264: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323626C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236278 size=100
    let mut pc: u32 = 0x83236278;
    'dispatch: loop {
        match pc {
            0x83236278 => {
    //   block [0x83236278..0x832362DC)
	// 83236278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323627C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236284: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83236288: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323628C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236294: 38AA8234  addi r5, r10, -0x7dcc
	ctx.r[5].s64 = ctx.r[10].s64 + -32204;
	// 83236298: 38896780  addi r4, r9, 0x6780
	ctx.r[4].s64 = ctx.r[9].s64 + 26496;
	// 8323629C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832362A0: 386882C4  addi r3, r8, -0x7d3c
	ctx.r[3].s64 = ctx.r[8].s64 + -32060;
	// 832362A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832362A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832362AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832362B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832362B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832362B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832362BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832362C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832362C4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832362C8: 4BC71F01  bl 0x82ea81c8
	ctx.lr = 0x832362CC;
	sub_82EA81C8(ctx, base);
	// 832362CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832362D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832362D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832362D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832362E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832362E0 size=120
    let mut pc: u32 = 0x832362E0;
    'dispatch: loop {
        match pc {
            0x832362E0 => {
    //   block [0x832362E0..0x83236358)
	// 832362E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832362E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832362E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832362EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832362F0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832362F4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832362F8: 38CA6438  addi r6, r10, 0x6438
	ctx.r[6].s64 = ctx.r[10].s64 + 25656;
	// 832362FC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236300: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236304: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323630C: 38A98234  addi r5, r9, -0x7dcc
	ctx.r[5].s64 = ctx.r[9].s64 + -32204;
	// 83236310: 388867A8  addi r4, r8, 0x67a8
	ctx.r[4].s64 = ctx.r[8].s64 + 26536;
	// 83236314: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236318: 386782F4  addi r3, r7, -0x7d0c
	ctx.r[3].s64 = ctx.r[7].s64 + -32012;
	// 8323631C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236320: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236324: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323632C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236330: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236338: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323633C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83236340: 4BC71E89  bl 0x82ea81c8
	ctx.lr = 0x83236344;
	sub_82EA81C8(ctx, base);
	// 83236344: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323634C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236358 size=100
    let mut pc: u32 = 0x83236358;
    'dispatch: loop {
        match pc {
            0x83236358 => {
    //   block [0x83236358..0x832363BC)
	// 83236358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323635C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236364: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83236368: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323636C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236374: 38AA8234  addi r5, r10, -0x7dcc
	ctx.r[5].s64 = ctx.r[10].s64 + -32204;
	// 83236378: 388967D0  addi r4, r9, 0x67d0
	ctx.r[4].s64 = ctx.r[9].s64 + 26576;
	// 8323637C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236380: 38688324  addi r3, r8, -0x7cdc
	ctx.r[3].s64 = ctx.r[8].s64 + -31964;
	// 83236384: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323638C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236390: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236394: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83236398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323639C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832363A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832363A4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832363A8: 4BC71E21  bl 0x82ea81c8
	ctx.lr = 0x832363AC;
	sub_82EA81C8(ctx, base);
	// 832363AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832363B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832363B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832363B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832363C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832363C0 size=108
    let mut pc: u32 = 0x832363C0;
    'dispatch: loop {
        match pc {
            0x832363C0 => {
    //   block [0x832363C0..0x8323642C)
	// 832363C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832363C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832363C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832363CC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832363D0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832363D4: 38EA67F0  addi r7, r10, 0x67f0
	ctx.r[7].s64 = ctx.r[10].s64 + 26608;
	// 832363D8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832363DC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 832363E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832363E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832363E8: 38896838  addi r4, r9, 0x6838
	ctx.r[4].s64 = ctx.r[9].s64 + 26680;
	// 832363EC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832363F0: 38688354  addi r3, r8, -0x7cac
	ctx.r[3].s64 = ctx.r[8].s64 + -31916;
	// 832363F4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832363F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832363FC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236400: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236404: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323640C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236410: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83236414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236418: 4BC71DB1  bl 0x82ea81c8
	ctx.lr = 0x8323641C;
	sub_82EA81C8(ctx, base);
	// 8323641C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236430 size=108
    let mut pc: u32 = 0x83236430;
    'dispatch: loop {
        match pc {
            0x83236430 => {
    //   block [0x83236430..0x8323649C)
	// 83236430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323643C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236440: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236444: 392B6948  addi r9, r11, 0x6948
	ctx.r[9].s64 = ctx.r[11].s64 + 26952;
	// 83236448: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323644C: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 83236450: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83236454: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323645C: 388A6A84  addi r4, r10, 0x6a84
	ctx.r[4].s64 = ctx.r[10].s64 + 27268;
	// 83236460: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236464: 38688384  addi r3, r8, -0x7c7c
	ctx.r[3].s64 = ctx.r[8].s64 + -31868;
	// 83236468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323646C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83236470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323647C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236480: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 83236484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236488: 4BC71D41  bl 0x82ea81c8
	ctx.lr = 0x8323648C;
	sub_82EA81C8(ctx, base);
	// 8323648C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832364A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832364A0 size=120
    let mut pc: u32 = 0x832364A0;
    'dispatch: loop {
        match pc {
            0x832364A0 => {
    //   block [0x832364A0..0x83236518)
	// 832364A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832364A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832364A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832364AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832364B0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832364B4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832364B8: 38CA6A08  addi r6, r10, 0x6a08
	ctx.r[6].s64 = ctx.r[10].s64 + 27144;
	// 832364BC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832364C0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832364C4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832364C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832364CC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 832364D0: 38886AA0  addi r4, r8, 0x6aa0
	ctx.r[4].s64 = ctx.r[8].s64 + 27296;
	// 832364D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832364D8: 386783B4  addi r3, r7, -0x7c4c
	ctx.r[3].s64 = ctx.r[7].s64 + -31820;
	// 832364DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832364E0: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 832364E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832364E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832364EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832364F0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832364F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832364F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832364FC: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 83236500: 4BC71CC9  bl 0x82ea81c8
	ctx.lr = 0x83236504;
	sub_82EA81C8(ctx, base);
	// 83236504: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323650C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236518 size=108
    let mut pc: u32 = 0x83236518;
    'dispatch: loop {
        match pc {
            0x83236518 => {
    //   block [0x83236518..0x83236584)
	// 83236518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323651C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236524: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236528: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323652C: 38EA6AC0  addi r7, r10, 0x6ac0
	ctx.r[7].s64 = ctx.r[10].s64 + 27328;
	// 83236530: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236534: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83236538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323653C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236540: 38896B3C  addi r4, r9, 0x6b3c
	ctx.r[4].s64 = ctx.r[9].s64 + 27452;
	// 83236544: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236548: 386883E4  addi r3, r8, -0x7c1c
	ctx.r[3].s64 = ctx.r[8].s64 + -31772;
	// 8323654C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236554: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236558: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323655C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236568: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323656C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236570: 4BC71C59  bl 0x82ea81c8
	ctx.lr = 0x83236574;
	sub_82EA81C8(ctx, base);
	// 83236574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323657C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236588 size=120
    let mut pc: u32 = 0x83236588;
    'dispatch: loop {
        match pc {
            0x83236588 => {
    //   block [0x83236588..0x83236600)
	// 83236588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323658C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236594: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236598: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323659C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832365A0: 38CA6AF0  addi r6, r10, 0x6af0
	ctx.r[6].s64 = ctx.r[10].s64 + 27376;
	// 832365A4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832365A8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832365AC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832365B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832365B4: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 832365B8: 38886B60  addi r4, r8, 0x6b60
	ctx.r[4].s64 = ctx.r[8].s64 + 27488;
	// 832365BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832365C0: 38678414  addi r3, r7, -0x7bec
	ctx.r[3].s64 = ctx.r[7].s64 + -31724;
	// 832365C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832365C8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 832365CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832365D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832365D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832365D8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832365DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832365E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832365E4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832365E8: 4BC71BE1  bl 0x82ea81c8
	ctx.lr = 0x832365EC;
	sub_82EA81C8(ctx, base);
	// 832365EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832365F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832365F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832365F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832365FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236600 size=108
    let mut pc: u32 = 0x83236600;
    'dispatch: loop {
        match pc {
            0x83236600 => {
    //   block [0x83236600..0x8323666C)
	// 83236600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323660C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236610: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236614: 38EA6BB8  addi r7, r10, 0x6bb8
	ctx.r[7].s64 = ctx.r[10].s64 + 27576;
	// 83236618: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323661C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83236620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236628: 38896C78  addi r4, r9, 0x6c78
	ctx.r[4].s64 = ctx.r[9].s64 + 27768;
	// 8323662C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236630: 38688444  addi r3, r8, -0x7bbc
	ctx.r[3].s64 = ctx.r[8].s64 + -31676;
	// 83236634: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323663C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236640: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236644: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236648: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323664C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236650: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83236654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236658: 4BC71B71  bl 0x82ea81c8
	ctx.lr = 0x8323665C;
	sub_82EA81C8(ctx, base);
	// 8323665C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236670 size=92
    let mut pc: u32 = 0x83236670;
    'dispatch: loop {
        match pc {
            0x83236670 => {
    //   block [0x83236670..0x832366CC)
	// 83236670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236678: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323667C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83236680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83236684: 4BCB821D  bl 0x82eee8a0
	ctx.lr = 0x83236688;
	sub_82EEE8A0(ctx, base);
	// 83236688: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323668C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236690: 39098474  addi r8, r9, -0x7b8c
	ctx.r[8].s64 = ctx.r[9].s64 + -31628;
	// 83236694: 396B6CA0  addi r11, r11, 0x6ca0
	ctx.r[11].s64 = ctx.r[11].s64 + 27808;
	// 83236698: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 8323669C: 91698474  stw r11, -0x7b8c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-31628 as u32), ctx.r[11].u32 ) };
	// 832366A0: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 832366A4: 394A6E70  addi r10, r10, 0x6e70
	ctx.r[10].s64 = ctx.r[10].s64 + 28272;
	// 832366A8: 39296E88  addi r9, r9, 0x6e88
	ctx.r[9].s64 = ctx.r[9].s64 + 28296;
	// 832366AC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832366B0: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832366B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832366B8: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832366BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832366C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832366C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832366C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832366D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832366D0 size=120
    let mut pc: u32 = 0x832366D0;
    'dispatch: loop {
        match pc {
            0x832366D0 => {
    //   block [0x832366D0..0x83236748)
	// 832366D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832366D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832366D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832366DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832366E0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832366E4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832366E8: 38CA6C00  addi r6, r10, 0x6c00
	ctx.r[6].s64 = ctx.r[10].s64 + 27648;
	// 832366EC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832366F0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832366F4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832366F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832366FC: 38A97CC4  addi r5, r9, 0x7cc4
	ctx.r[5].s64 = ctx.r[9].s64 + 31940;
	// 83236700: 38886CA0  addi r4, r8, 0x6ca0
	ctx.r[4].s64 = ctx.r[8].s64 + 27808;
	// 83236704: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236708: 38678484  addi r3, r7, -0x7b7c
	ctx.r[3].s64 = ctx.r[7].s64 + -31612;
	// 8323670C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236710: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 83236714: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323671C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236720: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236728: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323672C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83236730: 4BC71A99  bl 0x82ea81c8
	ctx.lr = 0x83236734;
	sub_82EA81C8(ctx, base);
	// 83236734: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236748 size=108
    let mut pc: u32 = 0x83236748;
    'dispatch: loop {
        match pc {
            0x83236748 => {
    //   block [0x83236748..0x832367B4)
	// 83236748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236754: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236758: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323675C: 38EA6D70  addi r7, r10, 0x6d70
	ctx.r[7].s64 = ctx.r[10].s64 + 28016;
	// 83236760: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236764: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83236768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323676C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236770: 38896E64  addi r4, r9, 0x6e64
	ctx.r[4].s64 = ctx.r[9].s64 + 28260;
	// 83236774: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236778: 386884B4  addi r3, r8, -0x7b4c
	ctx.r[3].s64 = ctx.r[8].s64 + -31564;
	// 8323677C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236784: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236788: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323678C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236790: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236798: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323679C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832367A0: 4BC71A29  bl 0x82ea81c8
	ctx.lr = 0x832367A4;
	sub_82EA81C8(ctx, base);
	// 832367A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832367A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832367AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832367B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832367B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832367B8 size=120
    let mut pc: u32 = 0x832367B8;
    'dispatch: loop {
        match pc {
            0x832367B8 => {
    //   block [0x832367B8..0x83236830)
	// 832367B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832367BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832367C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832367C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832367C8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832367CC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832367D0: 38CA6DA0  addi r6, r10, 0x6da0
	ctx.r[6].s64 = ctx.r[10].s64 + 28064;
	// 832367D4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832367D8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832367DC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832367E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832367E4: 38A97B44  addi r5, r9, 0x7b44
	ctx.r[5].s64 = ctx.r[9].s64 + 31556;
	// 832367E8: 38886E8C  addi r4, r8, 0x6e8c
	ctx.r[4].s64 = ctx.r[8].s64 + 28300;
	// 832367EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832367F0: 386784E4  addi r3, r7, -0x7b1c
	ctx.r[3].s64 = ctx.r[7].s64 + -31516;
	// 832367F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832367F8: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 832367FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236808: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323680C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236810: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236814: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83236818: 4BC719B1  bl 0x82ea81c8
	ctx.lr = 0x8323681C;
	sub_82EA81C8(ctx, base);
	// 8323681C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236830 size=108
    let mut pc: u32 = 0x83236830;
    'dispatch: loop {
        match pc {
            0x83236830 => {
    //   block [0x83236830..0x8323689C)
	// 83236830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323683C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236840: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236844: 38EA6EC0  addi r7, r10, 0x6ec0
	ctx.r[7].s64 = ctx.r[10].s64 + 28352;
	// 83236848: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323684C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83236850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236854: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236858: 38896F34  addi r4, r9, 0x6f34
	ctx.r[4].s64 = ctx.r[9].s64 + 28468;
	// 8323685C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236860: 38688514  addi r3, r8, -0x7aec
	ctx.r[3].s64 = ctx.r[8].s64 + -31468;
	// 83236864: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323686C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236870: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236874: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236878: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323687C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236880: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83236884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236888: 4BC71941  bl 0x82ea81c8
	ctx.lr = 0x8323688C;
	sub_82EA81C8(ctx, base);
	// 8323688C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832368A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832368A0 size=92
    let mut pc: u32 = 0x832368A0;
    'dispatch: loop {
        match pc {
            0x832368A0 => {
    //   block [0x832368A0..0x832368FC)
	// 832368A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832368A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832368A8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832368AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832368B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832368B4: 4BCA06D5  bl 0x82ed6f88
	ctx.lr = 0x832368B8;
	sub_82ED6F88(ctx, base);
	// 832368B8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832368BC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832368C0: 39098544  addi r8, r9, -0x7abc
	ctx.r[8].s64 = ctx.r[9].s64 + -31420;
	// 832368C4: 396B6F5C  addi r11, r11, 0x6f5c
	ctx.r[11].s64 = ctx.r[11].s64 + 28508;
	// 832368C8: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 832368CC: 91698544  stw r11, -0x7abc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-31420 as u32), ctx.r[11].u32 ) };
	// 832368D0: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 832368D4: 394A6F40  addi r10, r10, 0x6f40
	ctx.r[10].s64 = ctx.r[10].s64 + 28480;
	// 832368D8: 39296F28  addi r9, r9, 0x6f28
	ctx.r[9].s64 = ctx.r[9].s64 + 28456;
	// 832368DC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832368E0: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832368E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832368E8: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832368EC: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 832368F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832368F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832368F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236900 size=120
    let mut pc: u32 = 0x83236900;
    'dispatch: loop {
        match pc {
            0x83236900 => {
    //   block [0x83236900..0x83236978)
	// 83236900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323690C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236910: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236914: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236918: 38CA6ED8  addi r6, r10, 0x6ed8
	ctx.r[6].s64 = ctx.r[10].s64 + 28376;
	// 8323691C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236920: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236924: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323692C: 38A97B14  addi r5, r9, 0x7b14
	ctx.r[5].s64 = ctx.r[9].s64 + 31508;
	// 83236930: 38886F5C  addi r4, r8, 0x6f5c
	ctx.r[4].s64 = ctx.r[8].s64 + 28508;
	// 83236934: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236938: 38678554  addi r3, r7, -0x7aac
	ctx.r[3].s64 = ctx.r[7].s64 + -31404;
	// 8323693C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236940: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236944: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323694C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236950: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236958: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323695C: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 83236960: 4BC71869  bl 0x82ea81c8
	ctx.lr = 0x83236964;
	sub_82EA81C8(ctx, base);
	// 83236964: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323696C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236978 size=108
    let mut pc: u32 = 0x83236978;
    'dispatch: loop {
        match pc {
            0x83236978 => {
    //   block [0x83236978..0x832369E4)
	// 83236978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236984: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236988: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323698C: 38EA6F84  addi r7, r10, 0x6f84
	ctx.r[7].s64 = ctx.r[10].s64 + 28548;
	// 83236990: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236994: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83236998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323699C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832369A0: 38897000  addi r4, r9, 0x7000
	ctx.r[4].s64 = ctx.r[9].s64 + 28672;
	// 832369A4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832369A8: 38688584  addi r3, r8, -0x7a7c
	ctx.r[3].s64 = ctx.r[8].s64 + -31356;
	// 832369AC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832369B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832369B4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832369B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832369BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832369C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832369C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832369C8: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 832369CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832369D0: 4BC717F9  bl 0x82ea81c8
	ctx.lr = 0x832369D4;
	sub_82EA81C8(ctx, base);
	// 832369D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832369D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832369DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832369E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832369E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832369E8 size=120
    let mut pc: u32 = 0x832369E8;
    'dispatch: loop {
        match pc {
            0x832369E8 => {
    //   block [0x832369E8..0x83236A60)
	// 832369E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832369EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832369F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832369F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832369F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832369FC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236A00: 38CA6FB4  addi r6, r10, 0x6fb4
	ctx.r[6].s64 = ctx.r[10].s64 + 28596;
	// 83236A04: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236A08: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236A0C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236A14: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83236A18: 38887024  addi r4, r8, 0x7024
	ctx.r[4].s64 = ctx.r[8].s64 + 28708;
	// 83236A1C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236A20: 386785B4  addi r3, r7, -0x7a4c
	ctx.r[3].s64 = ctx.r[7].s64 + -31308;
	// 83236A24: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236A28: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236A2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236A38: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236A44: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 83236A48: 4BC71781  bl 0x82ea81c8
	ctx.lr = 0x83236A4C;
	sub_82EA81C8(ctx, base);
	// 83236A4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236A60 size=108
    let mut pc: u32 = 0x83236A60;
    'dispatch: loop {
        match pc {
            0x83236A60 => {
    //   block [0x83236A60..0x83236ACC)
	// 83236A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236A6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236A70: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236A74: 392B70A8  addi r9, r11, 0x70a8
	ctx.r[9].s64 = ctx.r[11].s64 + 28840;
	// 83236A78: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236A7C: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 83236A80: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83236A84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236A88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236A8C: 388A716C  addi r4, r10, 0x716c
	ctx.r[4].s64 = ctx.r[10].s64 + 29036;
	// 83236A90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236A94: 386885E4  addi r3, r8, -0x7a1c
	ctx.r[3].s64 = ctx.r[8].s64 + -31260;
	// 83236A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236A9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83236AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236AB0: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 83236AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236AB8: 4BC71711  bl 0x82ea81c8
	ctx.lr = 0x83236ABC;
	sub_82EA81C8(ctx, base);
	// 83236ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236AD0 size=120
    let mut pc: u32 = 0x83236AD0;
    'dispatch: loop {
        match pc {
            0x83236AD0 => {
    //   block [0x83236AD0..0x83236B48)
	// 83236AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236AD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236ADC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236AE0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236AE4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236AE8: 38CA7120  addi r6, r10, 0x7120
	ctx.r[6].s64 = ctx.r[10].s64 + 28960;
	// 83236AEC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236AF0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236AF4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236AFC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83236B00: 38887188  addi r4, r8, 0x7188
	ctx.r[4].s64 = ctx.r[8].s64 + 29064;
	// 83236B04: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236B08: 38678614  addi r3, r7, -0x79ec
	ctx.r[3].s64 = ctx.r[7].s64 + -31212;
	// 83236B0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236B10: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236B14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236B20: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236B28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236B2C: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 83236B30: 4BC71699  bl 0x82ea81c8
	ctx.lr = 0x83236B34;
	sub_82EA81C8(ctx, base);
	// 83236B34: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236B48 size=108
    let mut pc: u32 = 0x83236B48;
    'dispatch: loop {
        match pc {
            0x83236B48 => {
    //   block [0x83236B48..0x83236BB4)
	// 83236B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236B54: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236B58: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236B5C: 392B724C  addi r9, r11, 0x724c
	ctx.r[9].s64 = ctx.r[11].s64 + 29260;
	// 83236B60: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236B64: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 83236B68: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83236B6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236B70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236B74: 388A7354  addi r4, r10, 0x7354
	ctx.r[4].s64 = ctx.r[10].s64 + 29524;
	// 83236B78: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236B7C: 38688644  addi r3, r8, -0x79bc
	ctx.r[3].s64 = ctx.r[8].s64 + -31164;
	// 83236B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236B84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83236B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236B98: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 83236B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236BA0: 4BC71629  bl 0x82ea81c8
	ctx.lr = 0x83236BA4;
	sub_82EA81C8(ctx, base);
	// 83236BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236BB8 size=120
    let mut pc: u32 = 0x83236BB8;
    'dispatch: loop {
        match pc {
            0x83236BB8 => {
    //   block [0x83236BB8..0x83236C30)
	// 83236BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236BC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236BC4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236BC8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236BCC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83236BD0: 392B7220  addi r9, r11, 0x7220
	ctx.r[9].s64 = ctx.r[11].s64 + 29216;
	// 83236BD4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236BD8: 38C900E8  addi r6, r9, 0xe8
	ctx.r[6].s64 = ctx.r[9].s64 + 232;
	// 83236BDC: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236BE4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236BE8: 38AA7C94  addi r5, r10, 0x7c94
	ctx.r[5].s64 = ctx.r[10].s64 + 31892;
	// 83236BEC: 38887374  addi r4, r8, 0x7374
	ctx.r[4].s64 = ctx.r[8].s64 + 29556;
	// 83236BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236BF4: 38678674  addi r3, r7, -0x798c
	ctx.r[3].s64 = ctx.r[7].s64 + -31116;
	// 83236BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236BFC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236C04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83236C08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236C0C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236C10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236C14: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 83236C18: 4BC715B1  bl 0x82ea81c8
	ctx.lr = 0x83236C1C;
	sub_82EA81C8(ctx, base);
	// 83236C1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236C28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236C30 size=108
    let mut pc: u32 = 0x83236C30;
    'dispatch: loop {
        match pc {
            0x83236C30 => {
    //   block [0x83236C30..0x83236C9C)
	// 83236C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236C3C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236C40: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236C44: 38EA73A8  addi r7, r10, 0x73a8
	ctx.r[7].s64 = ctx.r[10].s64 + 29608;
	// 83236C48: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236C4C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83236C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236C54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236C58: 38897424  addi r4, r9, 0x7424
	ctx.r[4].s64 = ctx.r[9].s64 + 29732;
	// 83236C5C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236C60: 386886A4  addi r3, r8, -0x795c
	ctx.r[3].s64 = ctx.r[8].s64 + -31068;
	// 83236C64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236C6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236C70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236C74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236C78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236C80: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83236C84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236C88: 4BC71541  bl 0x82ea81c8
	ctx.lr = 0x83236C8C;
	sub_82EA81C8(ctx, base);
	// 83236C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236CA0 size=120
    let mut pc: u32 = 0x83236CA0;
    'dispatch: loop {
        match pc {
            0x83236CA0 => {
    //   block [0x83236CA0..0x83236D18)
	// 83236CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236CAC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236CB0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236CB4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236CB8: 38CA73D8  addi r6, r10, 0x73d8
	ctx.r[6].s64 = ctx.r[10].s64 + 29656;
	// 83236CBC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236CC0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236CC4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236CCC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83236CD0: 38887444  addi r4, r8, 0x7444
	ctx.r[4].s64 = ctx.r[8].s64 + 29764;
	// 83236CD4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236CD8: 386786D4  addi r3, r7, -0x792c
	ctx.r[3].s64 = ctx.r[7].s64 + -31020;
	// 83236CDC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236CE0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236CE4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236CF0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236CF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236CFC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83236D00: 4BC714C9  bl 0x82ea81c8
	ctx.lr = 0x83236D04;
	sub_82EA81C8(ctx, base);
	// 83236D04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236D10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236D18 size=108
    let mut pc: u32 = 0x83236D18;
    'dispatch: loop {
        match pc {
            0x83236D18 => {
    //   block [0x83236D18..0x83236D84)
	// 83236D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236D24: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236D28: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236D2C: 38EA7470  addi r7, r10, 0x7470
	ctx.r[7].s64 = ctx.r[10].s64 + 29808;
	// 83236D30: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236D34: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83236D38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236D3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236D40: 388974D0  addi r4, r9, 0x74d0
	ctx.r[4].s64 = ctx.r[9].s64 + 29904;
	// 83236D44: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236D48: 38688704  addi r3, r8, -0x78fc
	ctx.r[3].s64 = ctx.r[8].s64 + -30972;
	// 83236D4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236D54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236D58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236D5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236D60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236D68: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83236D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236D70: 4BC71459  bl 0x82ea81c8
	ctx.lr = 0x83236D74;
	sub_82EA81C8(ctx, base);
	// 83236D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236D88 size=108
    let mut pc: u32 = 0x83236D88;
    'dispatch: loop {
        match pc {
            0x83236D88 => {
    //   block [0x83236D88..0x83236DF4)
	// 83236D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236D94: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236D98: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236D9C: 38EA7488  addi r7, r10, 0x7488
	ctx.r[7].s64 = ctx.r[10].s64 + 29832;
	// 83236DA0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236DA4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83236DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236DB0: 388974E4  addi r4, r9, 0x74e4
	ctx.r[4].s64 = ctx.r[9].s64 + 29924;
	// 83236DB4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236DB8: 38688734  addi r3, r8, -0x78cc
	ctx.r[3].s64 = ctx.r[8].s64 + -30924;
	// 83236DBC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236DC4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236DC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236DCC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236DD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236DD8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83236DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236DE0: 4BC713E9  bl 0x82ea81c8
	ctx.lr = 0x83236DE4;
	sub_82EA81C8(ctx, base);
	// 83236DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236DF8 size=108
    let mut pc: u32 = 0x83236DF8;
    'dispatch: loop {
        match pc {
            0x83236DF8 => {
    //   block [0x83236DF8..0x83236E64)
	// 83236DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236E04: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236E08: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236E0C: 392B7538  addi r9, r11, 0x7538
	ctx.r[9].s64 = ctx.r[11].s64 + 30008;
	// 83236E10: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236E14: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 83236E18: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83236E1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236E24: 388A7644  addi r4, r10, 0x7644
	ctx.r[4].s64 = ctx.r[10].s64 + 30276;
	// 83236E28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236E2C: 38688764  addi r3, r8, -0x789c
	ctx.r[3].s64 = ctx.r[8].s64 + -30876;
	// 83236E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236E34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83236E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236E48: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 83236E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236E50: 4BC71379  bl 0x82ea81c8
	ctx.lr = 0x83236E54;
	sub_82EA81C8(ctx, base);
	// 83236E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236E68 size=120
    let mut pc: u32 = 0x83236E68;
    'dispatch: loop {
        match pc {
            0x83236E68 => {
    //   block [0x83236E68..0x83236EE0)
	// 83236E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83236E74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236E78: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236E7C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236E80: 38CA75F8  addi r6, r10, 0x75f8
	ctx.r[6].s64 = ctx.r[10].s64 + 30200;
	// 83236E84: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236E88: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236E8C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83236E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236E94: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83236E98: 38887664  addi r4, r8, 0x7664
	ctx.r[4].s64 = ctx.r[8].s64 + 30308;
	// 83236E9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236EA0: 38678794  addi r3, r7, -0x786c
	ctx.r[3].s64 = ctx.r[7].s64 + -30828;
	// 83236EA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236EA8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83236EAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236EB8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83236EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236EC4: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 83236EC8: 4BC71301  bl 0x82ea81c8
	ctx.lr = 0x83236ECC;
	sub_82EA81C8(ctx, base);
	// 83236ECC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83236ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83236EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236EE0 size=108
    let mut pc: u32 = 0x83236EE0;
    'dispatch: loop {
        match pc {
            0x83236EE0 => {
    //   block [0x83236EE0..0x83236F4C)
	// 83236EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236EEC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83236EF0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83236EF4: 38EA76E0  addi r7, r10, 0x76e0
	ctx.r[7].s64 = ctx.r[10].s64 + 30432;
	// 83236EF8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83236EFC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83236F00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83236F04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83236F08: 3889787C  addi r4, r9, 0x787c
	ctx.r[4].s64 = ctx.r[9].s64 + 30844;
	// 83236F0C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83236F10: 386887C4  addi r3, r8, -0x783c
	ctx.r[3].s64 = ctx.r[8].s64 + -30780;
	// 83236F14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83236F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236F1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83236F20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83236F24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83236F28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83236F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83236F30: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83236F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83236F38: 4BC71291  bl 0x82ea81c8
	ctx.lr = 0x83236F3C;
	sub_82EA81C8(ctx, base);
	// 83236F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83236F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236F50 size=92
    let mut pc: u32 = 0x83236F50;
    'dispatch: loop {
        match pc {
            0x83236F50 => {
    //   block [0x83236F50..0x83236FAC)
	// 83236F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236F58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236F5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83236F60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83236F64: 4BCB9B5D  bl 0x82ef0ac0
	ctx.lr = 0x83236F68;
	sub_82EF0AC0(ctx, base);
	// 83236F68: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83236F6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236F70: 390987F4  addi r8, r9, -0x780c
	ctx.r[8].s64 = ctx.r[9].s64 + -30732;
	// 83236F74: 396B78A0  addi r11, r11, 0x78a0
	ctx.r[11].s64 = ctx.r[11].s64 + 30880;
	// 83236F78: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 83236F7C: 916987F4  stw r11, -0x780c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30732 as u32), ctx.r[11].u32 ) };
	// 83236F80: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 83236F84: 394A7458  addi r10, r10, 0x7458
	ctx.r[10].s64 = ctx.r[10].s64 + 29784;
	// 83236F88: 39297470  addi r9, r9, 0x7470
	ctx.r[9].s64 = ctx.r[9].s64 + 29808;
	// 83236F8C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83236F90: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83236F94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83236F98: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83236F9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83236FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83236FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83236FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83236FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83236FB0 size=112
    let mut pc: u32 = 0x83236FB0;
    'dispatch: loop {
        match pc {
            0x83236FB0 => {
    //   block [0x83236FB0..0x83237020)
	// 83236FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83236FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83236FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83236FBC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83236FC0: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 83236FC4: 396B7770  addi r11, r11, 0x7770
	ctx.r[11].s64 = ctx.r[11].s64 + 30576;
	// 83236FC8: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83236FCC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83236FD0: 38CB00D8  addi r6, r11, 0xd8
	ctx.r[6].s64 = ctx.r[11].s64 + 216;
	// 83236FD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83236FD8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83236FDC: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83236FE0: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 83236FE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83236FE8: 38A97CC4  addi r5, r9, 0x7cc4
	ctx.r[5].s64 = ctx.r[9].s64 + 31940;
	// 83236FEC: 388878A0  addi r4, r8, 0x78a0
	ctx.r[4].s64 = ctx.r[8].s64 + 30880;
	// 83236FF0: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83236FF4: 38678804  addi r3, r7, -0x77fc
	ctx.r[3].s64 = ctx.r[7].s64 + -30716;
	// 83236FF8: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83236FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237000: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237008: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323700C: 4BC711BD  bl 0x82ea81c8
	ctx.lr = 0x83237010;
	sub_82EA81C8(ctx, base);
	// 83237010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323701C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237020 size=108
    let mut pc: u32 = 0x83237020;
    'dispatch: loop {
        match pc {
            0x83237020 => {
    //   block [0x83237020..0x8323708C)
	// 83237020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323702C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83237030: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83237034: 38EA78B8  addi r7, r10, 0x78b8
	ctx.r[7].s64 = ctx.r[10].s64 + 30904;
	// 83237038: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323703C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83237040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83237044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237048: 38897934  addi r4, r9, 0x7934
	ctx.r[4].s64 = ctx.r[9].s64 + 31028;
	// 8323704C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237050: 38688834  addi r3, r8, -0x77cc
	ctx.r[3].s64 = ctx.r[8].s64 + -30668;
	// 83237054: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323705C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237060: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237068: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323706C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237070: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 83237074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83237078: 4BC71151  bl 0x82ea81c8
	ctx.lr = 0x8323707C;
	sub_82EA81C8(ctx, base);
	// 8323707C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237090 size=120
    let mut pc: u32 = 0x83237090;
    'dispatch: loop {
        match pc {
            0x83237090 => {
    //   block [0x83237090..0x83237108)
	// 83237090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323709C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832370A0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832370A4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832370A8: 38CA78E8  addi r6, r10, 0x78e8
	ctx.r[6].s64 = ctx.r[10].s64 + 30952;
	// 832370AC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832370B0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832370B4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832370B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832370BC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 832370C0: 38887958  addi r4, r8, 0x7958
	ctx.r[4].s64 = ctx.r[8].s64 + 31064;
	// 832370C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832370C8: 38678864  addi r3, r7, -0x779c
	ctx.r[3].s64 = ctx.r[7].s64 + -30620;
	// 832370CC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832370D0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 832370D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832370D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832370DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832370E0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832370E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832370E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832370EC: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 832370F0: 4BC710D9  bl 0x82ea81c8
	ctx.lr = 0x832370F4;
	sub_82EA81C8(ctx, base);
	// 832370F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832370F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832370FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83237108 size=24
    let mut pc: u32 = 0x83237108;
    'dispatch: loop {
        match pc {
            0x83237108 => {
    //   block [0x83237108..0x83237120)
	// 83237108: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323710C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83237110: 392AE090  addi r9, r10, -0x1f70
	ctx.r[9].s64 = ctx.r[10].s64 + -8048;
	// 83237114: 816BE078  lwz r11, -0x1f88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8072 as u32) ) } as u64;
	// 83237118: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323711C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237120 size=112
    let mut pc: u32 = 0x83237120;
    'dispatch: loop {
        match pc {
            0x83237120 => {
    //   block [0x83237120..0x83237190)
	// 83237120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323712C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83237130: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83237134: 38CAE090  addi r6, r10, -0x1f70
	ctx.r[6].s64 = ctx.r[10].s64 + -8048;
	// 83237138: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323713C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83237140: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237144: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 83237148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323714C: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 83237150: 38887A34  addi r4, r8, 0x7a34
	ctx.r[4].s64 = ctx.r[8].s64 + 31284;
	// 83237154: 38678894  addi r3, r7, -0x776c
	ctx.r[3].s64 = ctx.r[7].s64 + -30572;
	// 83237158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323715C: 39297A20  addi r9, r9, 0x7a20
	ctx.r[9].s64 = ctx.r[9].s64 + 31264;
	// 83237160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237164: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83237168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323716C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237170: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237174: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83237178: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323717C: 4BC7104D  bl 0x82ea81c8
	ctx.lr = 0x83237180;
	sub_82EA81C8(ctx, base);
	// 83237180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323718C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237190 size=108
    let mut pc: u32 = 0x83237190;
    'dispatch: loop {
        match pc {
            0x83237190 => {
    //   block [0x83237190..0x832371FC)
	// 83237190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323719C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 832371A0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832371A4: 392B7A98  addi r9, r11, 0x7a98
	ctx.r[9].s64 = ctx.r[11].s64 + 31384;
	// 832371A8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832371AC: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 832371B0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 832371B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832371B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832371BC: 388A7B8C  addi r4, r10, 0x7b8c
	ctx.r[4].s64 = ctx.r[10].s64 + 31628;
	// 832371C0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832371C4: 386888C4  addi r3, r8, -0x773c
	ctx.r[3].s64 = ctx.r[8].s64 + -30524;
	// 832371C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832371CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832371D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832371D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832371D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832371DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832371E0: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 832371E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832371E8: 4BC70FE1  bl 0x82ea81c8
	ctx.lr = 0x832371EC;
	sub_82EA81C8(ctx, base);
	// 832371EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832371F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832371F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832371F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237200 size=120
    let mut pc: u32 = 0x83237200;
    'dispatch: loop {
        match pc {
            0x83237200 => {
    //   block [0x83237200..0x83237278)
	// 83237200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323720C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237210: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83237214: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237218: 38CA7B40  addi r6, r10, 0x7b40
	ctx.r[6].s64 = ctx.r[10].s64 + 31552;
	// 8323721C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83237220: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83237224: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323722C: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83237230: 38887BB0  addi r4, r8, 0x7bb0
	ctx.r[4].s64 = ctx.r[8].s64 + 31664;
	// 83237234: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237238: 386788F4  addi r3, r7, -0x770c
	ctx.r[3].s64 = ctx.r[7].s64 + -30476;
	// 8323723C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237240: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83237244: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237250: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237258: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323725C: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 83237260: 4BC70F69  bl 0x82ea81c8
	ctx.lr = 0x83237264;
	sub_82EA81C8(ctx, base);
	// 83237264: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323726C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237278 size=108
    let mut pc: u32 = 0x83237278;
    'dispatch: loop {
        match pc {
            0x83237278 => {
    //   block [0x83237278..0x832372E4)
	// 83237278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237284: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83237288: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323728C: 392B7BE8  addi r9, r11, 0x7be8
	ctx.r[9].s64 = ctx.r[11].s64 + 31720;
	// 83237290: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237294: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 83237298: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323729C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832372A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832372A4: 388A7C94  addi r4, r10, 0x7c94
	ctx.r[4].s64 = ctx.r[10].s64 + 31892;
	// 832372A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832372AC: 38688924  addi r3, r8, -0x76dc
	ctx.r[3].s64 = ctx.r[8].s64 + -30428;
	// 832372B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832372B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832372B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832372BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832372C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832372C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832372C8: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 832372CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832372D0: 4BC70EF9  bl 0x82ea81c8
	ctx.lr = 0x832372D4;
	sub_82EA81C8(ctx, base);
	// 832372D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832372D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832372DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832372E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832372E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832372E8 size=120
    let mut pc: u32 = 0x832372E8;
    'dispatch: loop {
        match pc {
            0x832372E8 => {
    //   block [0x832372E8..0x83237360)
	// 832372E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832372EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832372F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832372F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832372F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832372FC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237300: 38CA7C48  addi r6, r10, 0x7c48
	ctx.r[6].s64 = ctx.r[10].s64 + 31816;
	// 83237304: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83237308: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323730C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237314: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83237318: 38887CAC  addi r4, r8, 0x7cac
	ctx.r[4].s64 = ctx.r[8].s64 + 31916;
	// 8323731C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237320: 38678954  addi r3, r7, -0x76ac
	ctx.r[3].s64 = ctx.r[7].s64 + -30380;
	// 83237324: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237328: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323732C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237338: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323733C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237340: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237344: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 83237348: 4BC70E81  bl 0x82ea81c8
	ctx.lr = 0x8323734C;
	sub_82EA81C8(ctx, base);
	// 8323734C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237360 size=108
    let mut pc: u32 = 0x83237360;
    'dispatch: loop {
        match pc {
            0x83237360 => {
    //   block [0x83237360..0x832373CC)
	// 83237360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323736C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83237370: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83237374: 392B7CC8  addi r9, r11, 0x7cc8
	ctx.r[9].s64 = ctx.r[11].s64 + 31944;
	// 83237378: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323737C: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 83237380: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83237384: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323738C: 388A7D74  addi r4, r10, 0x7d74
	ctx.r[4].s64 = ctx.r[10].s64 + 32116;
	// 83237390: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237394: 38688984  addi r3, r8, -0x767c
	ctx.r[3].s64 = ctx.r[8].s64 + -30332;
	// 83237398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323739C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832373A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832373A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832373A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832373AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832373B0: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 832373B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832373B8: 4BC70E11  bl 0x82ea81c8
	ctx.lr = 0x832373BC;
	sub_82EA81C8(ctx, base);
	// 832373BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832373C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832373C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832373C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832373D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832373D0 size=120
    let mut pc: u32 = 0x832373D0;
    'dispatch: loop {
        match pc {
            0x832373D0 => {
    //   block [0x832373D0..0x83237448)
	// 832373D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832373D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832373D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832373DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832373E0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832373E4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832373E8: 38CA7D28  addi r6, r10, 0x7d28
	ctx.r[6].s64 = ctx.r[10].s64 + 32040;
	// 832373EC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 832373F0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832373F4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832373F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832373FC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83237400: 38887D90  addi r4, r8, 0x7d90
	ctx.r[4].s64 = ctx.r[8].s64 + 32144;
	// 83237404: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237408: 386789B4  addi r3, r7, -0x764c
	ctx.r[3].s64 = ctx.r[7].s64 + -30284;
	// 8323740C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237410: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83237414: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323741C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237420: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237428: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323742C: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 83237430: 4BC70D99  bl 0x82ea81c8
	ctx.lr = 0x83237434;
	sub_82EA81C8(ctx, base);
	// 83237434: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237448 size=108
    let mut pc: u32 = 0x83237448;
    'dispatch: loop {
        match pc {
            0x83237448 => {
    //   block [0x83237448..0x832374B4)
	// 83237448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237454: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83237458: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323745C: 38EA7E08  addi r7, r10, 0x7e08
	ctx.r[7].s64 = ctx.r[10].s64 + 32264;
	// 83237460: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237464: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83237468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323746C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237470: 38897EE0  addi r4, r9, 0x7ee0
	ctx.r[4].s64 = ctx.r[9].s64 + 32480;
	// 83237474: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237478: 386889E4  addi r3, r8, -0x761c
	ctx.r[3].s64 = ctx.r[8].s64 + -30236;
	// 8323747C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237484: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237488: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323748C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237490: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237498: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323749C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832374A0: 4BC70D29  bl 0x82ea81c8
	ctx.lr = 0x832374A4;
	sub_82EA81C8(ctx, base);
	// 832374A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832374A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832374AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832374B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832374B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832374B8 size=108
    let mut pc: u32 = 0x832374B8;
    'dispatch: loop {
        match pc {
            0x832374B8 => {
    //   block [0x832374B8..0x83237524)
	// 832374B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832374BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832374C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832374C4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 832374C8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 832374CC: 38EA7E68  addi r7, r10, 0x7e68
	ctx.r[7].s64 = ctx.r[10].s64 + 32360;
	// 832374D0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832374D4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 832374D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832374DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832374E0: 38897F10  addi r4, r9, 0x7f10
	ctx.r[4].s64 = ctx.r[9].s64 + 32528;
	// 832374E4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832374E8: 38688A14  addi r3, r8, -0x75ec
	ctx.r[3].s64 = ctx.r[8].s64 + -30188;
	// 832374EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832374F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832374F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832374F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832374FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237500: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237508: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323750C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83237510: 4BC70CB9  bl 0x82ea81c8
	ctx.lr = 0x83237514;
	sub_82EA81C8(ctx, base);
	// 83237514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237528 size=108
    let mut pc: u32 = 0x83237528;
    'dispatch: loop {
        match pc {
            0x83237528 => {
    //   block [0x83237528..0x83237594)
	// 83237528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237534: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237538: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323753C: 38EA81B0  addi r7, r10, -0x7e50
	ctx.r[7].s64 = ctx.r[10].s64 + -32336;
	// 83237540: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237544: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83237548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323754C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237550: 38898468  addi r4, r9, -0x7b98
	ctx.r[4].s64 = ctx.r[9].s64 + -31640;
	// 83237554: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237558: 38688A44  addi r3, r8, -0x75bc
	ctx.r[3].s64 = ctx.r[8].s64 + -30140;
	// 8323755C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237564: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237568: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323756C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237570: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237578: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323757C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83237580: 4BC70C49  bl 0x82ea81c8
	ctx.lr = 0x83237584;
	sub_82EA81C8(ctx, base);
	// 83237584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323758C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237598 size=108
    let mut pc: u32 = 0x83237598;
    'dispatch: loop {
        match pc {
            0x83237598 => {
    //   block [0x83237598..0x83237604)
	// 83237598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832375A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832375A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832375A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832375AC: 38EA81F8  addi r7, r10, -0x7e08
	ctx.r[7].s64 = ctx.r[10].s64 + -32264;
	// 832375B0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832375B4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832375B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832375BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832375C0: 38898494  addi r4, r9, -0x7b6c
	ctx.r[4].s64 = ctx.r[9].s64 + -31596;
	// 832375C4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832375C8: 38688A74  addi r3, r8, -0x758c
	ctx.r[3].s64 = ctx.r[8].s64 + -30092;
	// 832375CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832375D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832375D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832375D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832375DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832375E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832375E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832375E8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832375EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832375F0: 4BC70BD9  bl 0x82ea81c8
	ctx.lr = 0x832375F4;
	sub_82EA81C8(ctx, base);
	// 832375F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832375F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832375FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237608 size=108
    let mut pc: u32 = 0x83237608;
    'dispatch: loop {
        match pc {
            0x83237608 => {
    //   block [0x83237608..0x83237674)
	// 83237608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237614: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237618: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323761C: 38EA8258  addi r7, r10, -0x7da8
	ctx.r[7].s64 = ctx.r[10].s64 + -32168;
	// 83237620: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237624: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83237628: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323762C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237630: 388984B4  addi r4, r9, -0x7b4c
	ctx.r[4].s64 = ctx.r[9].s64 + -31564;
	// 83237634: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237638: 38688AA4  addi r3, r8, -0x755c
	ctx.r[3].s64 = ctx.r[8].s64 + -30044;
	// 8323763C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237644: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237648: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323764C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237658: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323765C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83237660: 4BC70B69  bl 0x82ea81c8
	ctx.lr = 0x83237664;
	sub_82EA81C8(ctx, base);
	// 83237664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323766C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237678 size=92
    let mut pc: u32 = 0x83237678;
    'dispatch: loop {
        match pc {
            0x83237678 => {
    //   block [0x83237678..0x832376D4)
	// 83237678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237680: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83237688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323768C: 4BCA1C35  bl 0x82ed92c0
	ctx.lr = 0x83237690;
	sub_82ED92C0(ctx, base);
	// 83237690: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83237694: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83237698: 39098AD4  addi r8, r9, -0x752c
	ctx.r[8].s64 = ctx.r[9].s64 + -29996;
	// 8323769C: 396B84D0  addi r11, r11, -0x7b30
	ctx.r[11].s64 = ctx.r[11].s64 + -31536;
	// 832376A0: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 832376A4: 91698AD4  stw r11, -0x752c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29996 as u32), ctx.r[11].u32 ) };
	// 832376A8: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 832376AC: 394A7850  addi r10, r10, 0x7850
	ctx.r[10].s64 = ctx.r[10].s64 + 30800;
	// 832376B0: 39297838  addi r9, r9, 0x7838
	ctx.r[9].s64 = ctx.r[9].s64 + 30776;
	// 832376B4: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832376B8: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832376BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832376C0: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832376C4: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 832376C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832376CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832376D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832376D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832376D8 size=120
    let mut pc: u32 = 0x832376D8;
    'dispatch: loop {
        match pc {
            0x832376D8 => {
    //   block [0x832376D8..0x83237750)
	// 832376D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832376DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832376E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832376E4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 832376E8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832376EC: 392B8198  addi r9, r11, -0x7e68
	ctx.r[9].s64 = ctx.r[11].s64 + -32360;
	// 832376F0: 388A8288  addi r4, r10, -0x7d78
	ctx.r[4].s64 = ctx.r[10].s64 + -32120;
	// 832376F4: 38A90288  addi r5, r9, 0x288
	ctx.r[5].s64 = ctx.r[9].s64 + 648;
	// 832376F8: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 832376FC: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83237700: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83237704: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83237708: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323770C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83237710: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83237714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237718: 38A87AB4  addi r5, r8, 0x7ab4
	ctx.r[5].s64 = ctx.r[8].s64 + 31412;
	// 8323771C: 388784D0  addi r4, r7, -0x7b30
	ctx.r[4].s64 = ctx.r[7].s64 + -31536;
	// 83237720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237724: 38668AE4  addi r3, r6, -0x751c
	ctx.r[3].s64 = ctx.r[6].s64 + -29980;
	// 83237728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323772C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83237730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237738: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8323773C: 4BC70A8D  bl 0x82ea81c8
	ctx.lr = 0x83237740;
	sub_82EA81C8(ctx, base);
	// 83237740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323774C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237750 size=108
    let mut pc: u32 = 0x83237750;
    'dispatch: loop {
        match pc {
            0x83237750 => {
    //   block [0x83237750..0x832377BC)
	// 83237750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323775C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237760: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237764: 38EA84E0  addi r7, r10, -0x7b20
	ctx.r[7].s64 = ctx.r[10].s64 + -31520;
	// 83237768: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323776C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83237770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83237774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237778: 388985A0  addi r4, r9, -0x7a60
	ctx.r[4].s64 = ctx.r[9].s64 + -31328;
	// 8323777C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83237780: 38688B14  addi r3, r8, -0x74ec
	ctx.r[3].s64 = ctx.r[8].s64 + -29932;
	// 83237784: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323778C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237790: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237794: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237798: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323779C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832377A0: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832377A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832377A8: 4BC70A21  bl 0x82ea81c8
	ctx.lr = 0x832377AC;
	sub_82EA81C8(ctx, base);
	// 832377AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832377B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832377B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832377B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832377C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832377C0 size=92
    let mut pc: u32 = 0x832377C0;
    'dispatch: loop {
        match pc {
            0x832377C0 => {
    //   block [0x832377C0..0x8323781C)
	// 832377C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832377C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832377C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832377CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832377D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832377D4: 4BCBB775  bl 0x82ef2f48
	ctx.lr = 0x832377D8;
	sub_82EF2F48(ctx, base);
	// 832377D8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832377DC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 832377E0: 39098B44  addi r8, r9, -0x74bc
	ctx.r[8].s64 = ctx.r[9].s64 + -29884;
	// 832377E4: 396B85C8  addi r11, r11, -0x7a38
	ctx.r[11].s64 = ctx.r[11].s64 + -31288;
	// 832377E8: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 832377EC: 91698B44  stw r11, -0x74bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29884 as u32), ctx.r[11].u32 ) };
	// 832377F0: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 832377F4: 394A79C8  addi r10, r10, 0x79c8
	ctx.r[10].s64 = ctx.r[10].s64 + 31176;
	// 832377F8: 392979E0  addi r9, r9, 0x79e0
	ctx.r[9].s64 = ctx.r[9].s64 + 31200;
	// 832377FC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83237800: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83237804: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83237808: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323780C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83237810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237820 size=120
    let mut pc: u32 = 0x83237820;
    'dispatch: loop {
        match pc {
            0x83237820 => {
    //   block [0x83237820..0x83237898)
	// 83237820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323782C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237830: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237834: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237838: 38CA8510  addi r6, r10, -0x7af0
	ctx.r[6].s64 = ctx.r[10].s64 + -31472;
	// 8323783C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83237840: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83237844: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323784C: 38A97CC4  addi r5, r9, 0x7cc4
	ctx.r[5].s64 = ctx.r[9].s64 + 31940;
	// 83237850: 388885C8  addi r4, r8, -0x7a38
	ctx.r[4].s64 = ctx.r[8].s64 + -31288;
	// 83237854: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237858: 38678B54  addi r3, r7, -0x74ac
	ctx.r[3].s64 = ctx.r[7].s64 + -29868;
	// 8323785C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237860: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 83237864: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323786C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237870: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237878: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323787C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 83237880: 4BC70949  bl 0x82ea81c8
	ctx.lr = 0x83237884;
	sub_82EA81C8(ctx, base);
	// 83237884: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237898 size=108
    let mut pc: u32 = 0x83237898;
    'dispatch: loop {
        match pc {
            0x83237898 => {
    //   block [0x83237898..0x83237904)
	// 83237898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832378A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832378A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832378A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832378AC: 38EA85E0  addi r7, r10, -0x7a20
	ctx.r[7].s64 = ctx.r[10].s64 + -31264;
	// 832378B0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832378B4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 832378B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832378BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832378C0: 3889865C  addi r4, r9, -0x79a4
	ctx.r[4].s64 = ctx.r[9].s64 + -31140;
	// 832378C4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832378C8: 38688B84  addi r3, r8, -0x747c
	ctx.r[3].s64 = ctx.r[8].s64 + -29820;
	// 832378CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832378D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832378D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832378D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832378DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832378E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832378E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832378E8: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832378EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832378F0: 4BC708D9  bl 0x82ea81c8
	ctx.lr = 0x832378F4;
	sub_82EA81C8(ctx, base);
	// 832378F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832378F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832378FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237908 size=120
    let mut pc: u32 = 0x83237908;
    'dispatch: loop {
        match pc {
            0x83237908 => {
    //   block [0x83237908..0x83237980)
	// 83237908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83237914: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237918: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323791C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237920: 38CA8610  addi r6, r10, -0x79f0
	ctx.r[6].s64 = ctx.r[10].s64 + -31216;
	// 83237924: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83237928: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323792C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237934: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83237938: 38888680  addi r4, r8, -0x7980
	ctx.r[4].s64 = ctx.r[8].s64 + -31104;
	// 8323793C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237940: 38678BB4  addi r3, r7, -0x744c
	ctx.r[3].s64 = ctx.r[7].s64 + -29772;
	// 83237944: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237948: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323794C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237958: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323795C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237960: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237964: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 83237968: 4BC70861  bl 0x82ea81c8
	ctx.lr = 0x8323796C;
	sub_82EA81C8(ctx, base);
	// 8323796C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237978: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323797C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237980 size=92
    let mut pc: u32 = 0x83237980;
    'dispatch: loop {
        match pc {
            0x83237980 => {
    //   block [0x83237980..0x832379DC)
	// 83237980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237988: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323798C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83237990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83237994: 4BCA31CD  bl 0x82edab60
	ctx.lr = 0x83237998;
	sub_82EDAB60(ctx, base);
	// 83237998: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323799C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 832379A0: 39098BE4  addi r8, r9, -0x741c
	ctx.r[8].s64 = ctx.r[9].s64 + -29724;
	// 832379A4: 396B8BA8  addi r11, r11, -0x7458
	ctx.r[11].s64 = ctx.r[11].s64 + -29784;
	// 832379A8: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 832379AC: 91698BE4  stw r11, -0x741c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29724 as u32), ctx.r[11].u32 ) };
	// 832379B0: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 832379B4: 394A7A80  addi r10, r10, 0x7a80
	ctx.r[10].s64 = ctx.r[10].s64 + 31360;
	// 832379B8: 39297A98  addi r9, r9, 0x7a98
	ctx.r[9].s64 = ctx.r[9].s64 + 31384;
	// 832379BC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832379C0: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832379C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832379C8: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832379CC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 832379D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832379D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832379D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832379E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832379E0 size=48
    let mut pc: u32 = 0x832379E0;
    'dispatch: loop {
        match pc {
            0x832379E0 => {
    //   block [0x832379E0..0x83237A10)
	// 832379E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832379E4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832379E8: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 832379EC: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 832379F0: 816BE27C  lwz r11, -0x1d84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7556 as u32) ) } as u64;
	// 832379F4: 38E8E280  addi r7, r8, -0x1d80
	ctx.r[7].s64 = ctx.r[8].s64 + -7552;
	// 832379F8: 814AE278  lwz r10, -0x1d88(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7560 as u32) ) } as u64;
	// 832379FC: 8129E274  lwz r9, -0x1d8c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-7564 as u32) ) } as u64;
	// 83237A00: 91670050  stw r11, 0x50(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83237A04: 91470140  stw r10, 0x140(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 83237A08: 91270380  stw r9, 0x380(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(896 as u32), ctx.r[9].u32 ) };
	// 83237A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237A10 size=120
    let mut pc: u32 = 0x83237A10;
    'dispatch: loop {
        match pc {
            0x83237A10 => {
    //   block [0x83237A10..0x83237A88)
	// 83237A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237A1C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83237A20: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83237A24: 392B8A40  addi r9, r11, -0x75c0
	ctx.r[9].s64 = ctx.r[11].s64 + -30144;
	// 83237A28: 388AE280  addi r4, r10, -0x1d80
	ctx.r[4].s64 = ctx.r[10].s64 + -7552;
	// 83237A2C: 38A90050  addi r5, r9, 0x50
	ctx.r[5].s64 = ctx.r[9].s64 + 80;
	// 83237A30: 3860002A  li r3, 0x2a
	ctx.r[3].s64 = 42;
	// 83237A34: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83237A38: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83237A3C: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83237A40: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83237A44: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83237A48: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83237A4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237A50: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83237A54: 38878BA8  addi r4, r7, -0x7458
	ctx.r[4].s64 = ctx.r[7].s64 + -29784;
	// 83237A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237A5C: 38668BF4  addi r3, r6, -0x740c
	ctx.r[3].s64 = ctx.r[6].s64 + -29708;
	// 83237A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237A64: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83237A68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237A70: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 83237A74: 4BC70755  bl 0x82ea81c8
	ctx.lr = 0x83237A78;
	sub_82EA81C8(ctx, base);
	// 83237A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237A88 size=92
    let mut pc: u32 = 0x83237A88;
    'dispatch: loop {
        match pc {
            0x83237A88 => {
    //   block [0x83237A88..0x83237AE4)
	// 83237A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237A90: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237A94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83237A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83237A9C: 4BC99455  bl 0x82ed0ef0
	ctx.lr = 0x83237AA0;
	sub_82ED0EF0(ctx, base);
	// 83237AA0: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83237AA4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83237AA8: 39098C24  addi r8, r9, -0x73dc
	ctx.r[8].s64 = ctx.r[9].s64 + -29660;
	// 83237AAC: 396B4520  addi r11, r11, 0x4520
	ctx.r[11].s64 = ctx.r[11].s64 + 17696;
	// 83237AB0: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 83237AB4: 91698C24  stw r11, -0x73dc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29660 as u32), ctx.r[11].u32 ) };
	// 83237AB8: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 83237ABC: 394A7AE0  addi r10, r10, 0x7ae0
	ctx.r[10].s64 = ctx.r[10].s64 + 31456;
	// 83237AC0: 39297AF8  addi r9, r9, 0x7af8
	ctx.r[9].s64 = ctx.r[9].s64 + 31480;
	// 83237AC4: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83237AC8: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83237ACC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83237AD0: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83237AD4: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 83237AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237AE8 size=120
    let mut pc: u32 = 0x83237AE8;
    'dispatch: loop {
        match pc {
            0x83237AE8 => {
    //   block [0x83237AE8..0x83237B60)
	// 83237AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237AF4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237AF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83237AFC: 38AA9890  addi r5, r10, -0x6770
	ctx.r[5].s64 = ctx.r[10].s64 + -26480;
	// 83237B00: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 83237B04: 90810074  stw r4, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[4].u32 ) };
	// 83237B08: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83237B0C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83237B10: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 83237B14: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83237B18: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83237B1C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237B24: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83237B28: 38874520  addi r4, r7, 0x4520
	ctx.r[4].s64 = ctx.r[7].s64 + 17696;
	// 83237B2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237B30: 38668C34  addi r3, r6, -0x73cc
	ctx.r[3].s64 = ctx.r[6].s64 + -29644;
	// 83237B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237B38: 39299868  addi r9, r9, -0x6798
	ctx.r[9].s64 = ctx.r[9].s64 + -26520;
	// 83237B3C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83237B40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237B48: 38C00310  li r6, 0x310
	ctx.r[6].s64 = 784;
	// 83237B4C: 4BC7067D  bl 0x82ea81c8
	ctx.lr = 0x83237B50;
	sub_82EA81C8(ctx, base);
	// 83237B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


