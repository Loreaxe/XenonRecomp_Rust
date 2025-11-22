pub fn sub_83216838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216838 size=8
    let mut pc: u32 = 0x83216838;
    'dispatch: loop {
        match pc {
            0x83216838 => {
    //   block [0x83216838..0x83216840)
	// 83216838: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8321683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216840 size=8
    let mut pc: u32 = 0x83216840;
    'dispatch: loop {
        match pc {
            0x83216840 => {
    //   block [0x83216840..0x83216848)
	// 83216840: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83216844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216848 size=8
    let mut pc: u32 = 0x83216848;
    'dispatch: loop {
        match pc {
            0x83216848 => {
    //   block [0x83216848..0x83216850)
	// 83216848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8321684C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216850 size=24
    let mut pc: u32 = 0x83216850;
    'dispatch: loop {
        match pc {
            0x83216850 => {
    //   block [0x83216850..0x83216868)
	// 83216850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216854: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83216858: 41820010  beq 0x83216868
	if ctx.cr[0].eq {
		sub_83216868(ctx, base);
		return;
	}
	// 8321685C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83216860: 71638888  andi. r3, r11, 0x8888
	ctx.r[3].u64 = ctx.r[11].u64 & 34952;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216868 size=8
    let mut pc: u32 = 0x83216868;
    'dispatch: loop {
        match pc {
            0x83216868 => {
    //   block [0x83216868..0x83216870)
	// 83216868: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8321686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216870 size=200
    let mut pc: u32 = 0x83216870;
    'dispatch: loop {
        match pc {
            0x83216870 => {
    //   block [0x83216870..0x83216938)
	// 83216870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216874: 4BA92B91  bl 0x82ca9404
	ctx.lr = 0x83216878;
	sub_82CA93D0(ctx, base);
	// 83216878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321687C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83216880: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216884: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83216888: 4182009C  beq 0x83216924
	if ctx.cr[0].eq {
	pc = 0x83216924; continue 'dispatch;
	}
	// 8321688C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83216890: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83216894: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83216898: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8321689C: 3BE9B734  addi r31, r9, -0x48cc
	ctx.r[31].s64 = ctx.r[9].s64 + -18636;
	// 832168A0: 3BCA1700  addi r30, r10, 0x1700
	ctx.r[30].s64 = ctx.r[10].s64 + 5888;
	// 832168A4: 3BABFA28  addi r29, r11, -0x5d8
	ctx.r[29].s64 = ctx.r[11].s64 + -1496;
	// 832168A8: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 832168AC: 41980054  blt cr6, 0x83216900
	if ctx.cr[6].lt {
	pc = 0x83216900; continue 'dispatch;
	}
	// 832168B0: 419A0044  beq cr6, 0x832168f4
	if ctx.cr[6].eq {
	pc = 0x832168F4; continue 'dispatch;
	}
	// 832168B4: 2B1B0003  cmplwi cr6, r27, 3
	ctx.cr[6].compare_u32(ctx.r[27].u32, 3 as u32, &mut ctx.xer);
	// 832168B8: 41980030  blt cr6, 0x832168e8
	if ctx.cr[6].lt {
	pc = 0x832168E8; continue 'dispatch;
	}
	// 832168BC: 419A0020  beq cr6, 0x832168dc
	if ctx.cr[6].eq {
	pc = 0x832168DC; continue 'dispatch;
	}
	// 832168C0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832168C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 832168C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832168CC: 38E000A6  li r7, 0xa6
	ctx.r[7].s64 = 166;
	// 832168D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832168D4: 4BEF34DD  bl 0x83109db0
	ctx.lr = 0x832168D8;
	sub_83109DB0(ctx, base);
	// 832168D8: 48000040  b 0x83216918
	pc = 0x83216918; continue 'dispatch;
	// 832168DC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 832168E0: 556BD7BE  rlwinm r11, r11, 0x1a, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 832168E4: 48000024  b 0x83216908
	pc = 0x83216908; continue 'dispatch;
	// 832168E8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 832168EC: 556BE7BE  rlwinm r11, r11, 0x1c, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 832168F0: 48000018  b 0x83216908
	pc = 0x83216908; continue 'dispatch;
	// 832168F4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 832168F8: 556BF7BE  rlwinm r11, r11, 0x1e, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 832168FC: 4800000C  b 0x83216908
	pc = 0x83216908; continue 'dispatch;
	// 83216900: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83216904: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 83216908: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8321690C: 419A0024  beq cr6, 0x83216930
	if ctx.cr[6].eq {
	pc = 0x83216930; continue 'dispatch;
	}
	// 83216910: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83216914: 419A001C  beq cr6, 0x83216930
	if ctx.cr[6].eq {
	pc = 0x83216930; continue 'dispatch;
	}
	// 83216918: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8321691C: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 83216920: 4198FF88  blt cr6, 0x832168a8
	if ctx.cr[6].lt {
	pc = 0x832168A8; continue 'dispatch;
	}
	// 83216924: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83216928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321692C: 4BA92B28  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 83216930: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83216934: 4BFFFFF4  b 0x83216928
	pc = 0x83216928; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216938 size=88
    let mut pc: u32 = 0x83216938;
    'dispatch: loop {
        match pc {
            0x83216938 => {
    //   block [0x83216938..0x83216990)
	// 83216938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83216940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83216944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216948: 39640007  addi r11, r4, 7
	ctx.r[11].s64 = ctx.r[4].s64 + 7;
	// 8321694C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83216950: 7FEB182E  lwzx r31, r11, r3
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83216954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83216958: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321695C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216964: 4E800421  bctrl
	ctx.lr = 0x83216968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216968: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8321696C: 4182000C  beq 0x83216978
	if ctx.cr[0].eq {
	pc = 0x83216978; continue 'dispatch;
	}
	// 83216970: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83216974: 48000008  b 0x8321697c
	pc = 0x8321697C; continue 'dispatch;
	// 83216978: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8321697C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83216980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83216984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83216988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321698C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216990 size=44
    let mut pc: u32 = 0x83216990;
    'dispatch: loop {
        match pc {
            0x83216990 => {
    //   block [0x83216990..0x832169BC)
	// 83216990: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83216994: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216998: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321699C: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 832169A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 832169A4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832169A8: 41990008  bgt cr6, 0x832169b0
	if ctx.cr[6].gt {
	pc = 0x832169B0; continue 'dispatch;
	}
	// 832169AC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 832169B0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 832169B4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832169B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832169C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832169C0 size=40
    let mut pc: u32 = 0x832169C0;
    'dispatch: loop {
        match pc {
            0x832169C0 => {
    //   block [0x832169C0..0x832169E8)
	// 832169C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832169C4: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 832169C8: 41980030  blt cr6, 0x832169f8
	if ctx.cr[6].lt {
		sub_832169F8(ctx, base);
		return;
	}
	// 832169CC: 419A001C  beq cr6, 0x832169e8
	if ctx.cr[6].eq {
		sub_832169E8(ctx, base);
		return;
	}
	// 832169D0: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 832169D4: 40980028  bge cr6, 0x832169fc
	if !ctx.cr[6].lt {
		sub_832169F8(ctx, base);
		return;
	}
	// 832169D8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832169DC: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 832169E0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832169E4: 48000018  b 0x832169fc
	sub_832169F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832169E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832169E8 size=16
    let mut pc: u32 = 0x832169E8;
    'dispatch: loop {
        match pc {
            0x832169E8 => {
    //   block [0x832169E8..0x832169F8)
	// 832169E8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832169EC: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 832169F0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832169F4: 48000008  b 0x832169fc
	sub_832169F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832169F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832169F8 size=36
    let mut pc: u32 = 0x832169F8;
    'dispatch: loop {
        match pc {
            0x832169F8 => {
    //   block [0x832169F8..0x83216A1C)
	// 832169F8: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 832169FC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216A00: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83216A04: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83216A08: 41990008  bgt cr6, 0x83216a10
	if ctx.cr[6].gt {
	pc = 0x83216A10; continue 'dispatch;
	}
	// 83216A0C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83216A10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83216A14: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83216A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216A20 size=8
    let mut pc: u32 = 0x83216A20;
    'dispatch: loop {
        match pc {
            0x83216A20 => {
    //   block [0x83216A20..0x83216A28)
	// 83216A20: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216A28 size=36
    let mut pc: u32 = 0x83216A28;
    'dispatch: loop {
        match pc {
            0x83216A28 => {
    //   block [0x83216A28..0x83216A4C)
	// 83216A28: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83216A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83216A30: 396BFA84  addi r11, r11, -0x57c
	ctx.r[11].s64 = ctx.r[11].s64 + -1404;
	// 83216A34: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83216A38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83216A3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83216A40: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83216A44: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83216A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216A50 size=12
    let mut pc: u32 = 0x83216A50;
    'dispatch: loop {
        match pc {
            0x83216A50 => {
    //   block [0x83216A50..0x83216A5C)
	// 83216A50: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83216A54: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83216A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83216A60 size=12
    let mut pc: u32 = 0x83216A60;
    'dispatch: loop {
        match pc {
            0x83216A60 => {
    //   block [0x83216A60..0x83216A6C)
	// 83216A60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83216A64: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83216A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216A70 size=104
    let mut pc: u32 = 0x83216A70;
    'dispatch: loop {
        match pc {
            0x83216A70 => {
    //   block [0x83216A70..0x83216AD8)
	// 83216A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83216A78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83216A7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216A80: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83216A88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83216A8C: 419A0030  beq cr6, 0x83216abc
	if ctx.cr[6].eq {
	pc = 0x83216ABC; continue 'dispatch;
	}
	// 83216A90: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83216A94: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216A98: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83216A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216AA0: 4E800421  bctrl
	ctx.lr = 0x83216AA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83216AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83216AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83216AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83216AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83216AB8: 4E800020  blr
	return;
	// 83216ABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83216AC0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83216AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83216AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83216ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83216AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83216AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216AD8 size=284
    let mut pc: u32 = 0x83216AD8;
    'dispatch: loop {
        match pc {
            0x83216AD8 => {
    //   block [0x83216AD8..0x83216BF4)
	// 83216AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83216AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83216AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83216AE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83216AF0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83216AF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216AF8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83216AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216B00: 4E800421  bctrl
	ctx.lr = 0x83216B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216B04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216B08: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83216B0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83216B10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216B14: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216B1C: 4E800421  bctrl
	ctx.lr = 0x83216B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216B20: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216B24: 419A00B8  beq cr6, 0x83216bdc
	if ctx.cr[6].eq {
	pc = 0x83216BDC; continue 'dispatch;
	}
	// 83216B28: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83216B2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216B30: 40990040  ble cr6, 0x83216b70
	if !ctx.cr[6].gt {
	pc = 0x83216B70; continue 'dispatch;
	}
	// 83216B34: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83216B38: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216B3C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216B40: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83216B44: 419A001C  beq cr6, 0x83216b60
	if ctx.cr[6].eq {
	pc = 0x83216B60; continue 'dispatch;
	}
	// 83216B48: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83216B4C: 419A0014  beq cr6, 0x83216b60
	if ctx.cr[6].eq {
	pc = 0x83216B60; continue 'dispatch;
	}
	// 83216B50: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 83216B54: 419A000C  beq cr6, 0x83216b60
	if ctx.cr[6].eq {
	pc = 0x83216B60; continue 'dispatch;
	}
	// 83216B58: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83216B5C: 409A0054  bne cr6, 0x83216bb0
	if !ctx.cr[6].eq {
	pc = 0x83216BB0; continue 'dispatch;
	}
	// 83216B60: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83216B64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83216B68: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216B6C: 4198FFCC  blt cr6, 0x83216b38
	if ctx.cr[6].lt {
	pc = 0x83216B38; continue 'dispatch;
	}
	// 83216B70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216B74: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83216B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216B7C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83216B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216B84: 4E800421  bctrl
	ctx.lr = 0x83216B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216B88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216B8C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83216B90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83216B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216B98: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216BA0: 4E800421  bctrl
	ctx.lr = 0x83216BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216BA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216BA8: 409AFF80  bne cr6, 0x83216b28
	if !ctx.cr[6].eq {
	pc = 0x83216B28; continue 'dispatch;
	}
	// 83216BAC: 48000030  b 0x83216bdc
	pc = 0x83216BDC; continue 'dispatch;
	// 83216BB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216BB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216BB8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83216BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216BC0: 4E800421  bctrl
	ctx.lr = 0x83216BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216BC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216BC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83216BCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216BD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83216BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216BD8: 4E800421  bctrl
	ctx.lr = 0x83216BDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216BDC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83216BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83216BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83216BE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83216BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83216BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216BF8 size=544
    let mut pc: u32 = 0x83216BF8;
    'dispatch: loop {
        match pc {
            0x83216BF8 => {
    //   block [0x83216BF8..0x83216E18)
	// 83216BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216BFC: 4BA9280D  bl 0x82ca9408
	ctx.lr = 0x83216C00;
	sub_82CA93D0(ctx, base);
	// 83216C00: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216C04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83216C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83216C0C: 4BFFFECD  bl 0x83216ad8
	ctx.lr = 0x83216C10;
	sub_83216AD8(ctx, base);
	// 83216C10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83216C14: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 83216C18: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 83216C1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83216C20: 9BFE0000  stb r31, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83216C24: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216C28: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83216C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216C30: 4E800421  bctrl
	ctx.lr = 0x83216C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216C34: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216C38: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83216C3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83216C40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83216C44: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216C4C: 4E800421  bctrl
	ctx.lr = 0x83216C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216C50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216C54: 419A01B8  beq cr6, 0x83216e0c
	if ctx.cr[6].eq {
	pc = 0x83216E0C; continue 'dispatch;
	}
	// 83216C58: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83216C5C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216C60: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 83216C64: 409A000C  bne cr6, 0x83216c70
	if !ctx.cr[6].eq {
	pc = 0x83216C70; continue 'dispatch;
	}
	// 83216C68: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83216C6C: 48000014  b 0x83216c80
	pc = 0x83216C80; continue 'dispatch;
	// 83216C70: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 83216C74: 409A000C  bne cr6, 0x83216c80
	if !ctx.cr[6].eq {
	pc = 0x83216C80; continue 'dispatch;
	}
	// 83216C78: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83216C7C: 9BFE0000  stb r31, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83216C80: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 83216C84: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216C88: 40980078  bge cr6, 0x83216d00
	if !ctx.cr[6].lt {
	pc = 0x83216D00; continue 'dispatch;
	}
	// 83216C8C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83216C90: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83216C94: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83216C98: 409A0068  bne cr6, 0x83216d00
	if !ctx.cr[6].eq {
	pc = 0x83216D00; continue 'dispatch;
	}
	// 83216C9C: 39610051  addi r11, r1, 0x51
	ctx.r[11].s64 = ctx.r[1].s64 + 81;
	// 83216CA0: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83216CA4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216CA8: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 83216CAC: 419A000C  beq cr6, 0x83216cb8
	if ctx.cr[6].eq {
	pc = 0x83216CB8; continue 'dispatch;
	}
	// 83216CB0: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 83216CB4: 409A004C  bne cr6, 0x83216d00
	if !ctx.cr[6].eq {
	pc = 0x83216D00; continue 'dispatch;
	}
	// 83216CB8: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 83216CBC: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83216CC0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216CC4: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83216CC8: 4198000C  blt cr6, 0x83216cd4
	if ctx.cr[6].lt {
	pc = 0x83216CD4; continue 'dispatch;
	}
	// 83216CCC: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 83216CD0: 40990024  ble cr6, 0x83216cf4
	if !ctx.cr[6].gt {
	pc = 0x83216CF4; continue 'dispatch;
	}
	// 83216CD4: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83216CD8: 4198000C  blt cr6, 0x83216ce4
	if ctx.cr[6].lt {
	pc = 0x83216CE4; continue 'dispatch;
	}
	// 83216CDC: 2F0B0066  cmpwi cr6, r11, 0x66
	ctx.cr[6].compare_i32(ctx.r[11].s32, 102, &mut ctx.xer);
	// 83216CE0: 40990014  ble cr6, 0x83216cf4
	if !ctx.cr[6].gt {
	pc = 0x83216CF4; continue 'dispatch;
	}
	// 83216CE4: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 83216CE8: 41980018  blt cr6, 0x83216d00
	if ctx.cr[6].lt {
	pc = 0x83216D00; continue 'dispatch;
	}
	// 83216CEC: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 83216CF0: 41990010  bgt cr6, 0x83216d00
	if ctx.cr[6].gt {
	pc = 0x83216D00; continue 'dispatch;
	}
	// 83216CF4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83216CF8: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 83216CFC: 4800004C  b 0x83216d48
	pc = 0x83216D48; continue 'dispatch;
	// 83216D00: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 83216D04: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216D08: 4098003C  bge cr6, 0x83216d44
	if !ctx.cr[6].lt {
	pc = 0x83216D44; continue 'dispatch;
	}
	// 83216D0C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83216D10: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83216D14: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83216D18: 409A002C  bne cr6, 0x83216d44
	if !ctx.cr[6].eq {
	pc = 0x83216D44; continue 'dispatch;
	}
	// 83216D1C: 39610051  addi r11, r1, 0x51
	ctx.r[11].s64 = ctx.r[1].s64 + 81;
	// 83216D20: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83216D24: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216D28: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83216D2C: 41980018  blt cr6, 0x83216d44
	if ctx.cr[6].lt {
	pc = 0x83216D44; continue 'dispatch;
	}
	// 83216D30: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 83216D34: 41990010  bgt cr6, 0x83216d44
	if ctx.cr[6].gt {
	pc = 0x83216D44; continue 'dispatch;
	}
	// 83216D38: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 83216D3C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83216D40: 48000008  b 0x83216d48
	pc = 0x83216D48; continue 'dispatch;
	// 83216D44: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 83216D48: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216D4C: 40980088  bge cr6, 0x83216dd4
	if !ctx.cr[6].lt {
	pc = 0x83216DD4; continue 'dispatch;
	}
	// 83216D50: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83216D54: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83216D58: 7D3F5A14  add r9, r31, r11
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 83216D5C: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216D60: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83216D64: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216D68: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83216D6C: 41980014  blt cr6, 0x83216d80
	if ctx.cr[6].lt {
	pc = 0x83216D80; continue 'dispatch;
	}
	// 83216D70: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 83216D74: 4199000C  bgt cr6, 0x83216d80
	if ctx.cr[6].gt {
	pc = 0x83216D80; continue 'dispatch;
	}
	// 83216D78: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 83216D7C: 48000030  b 0x83216dac
	pc = 0x83216DAC; continue 'dispatch;
	// 83216D80: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 83216D84: 40990014  ble cr6, 0x83216d98
	if !ctx.cr[6].gt {
	pc = 0x83216D98; continue 'dispatch;
	}
	// 83216D88: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 83216D8C: 4199000C  bgt cr6, 0x83216d98
	if ctx.cr[6].gt {
	pc = 0x83216D98; continue 'dispatch;
	}
	// 83216D90: 394BFFC9  addi r10, r11, -0x37
	ctx.r[10].s64 = ctx.r[11].s64 + -55;
	// 83216D94: 48000018  b 0x83216dac
	pc = 0x83216DAC; continue 'dispatch;
	// 83216D98: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83216D9C: 40990010  ble cr6, 0x83216dac
	if !ctx.cr[6].gt {
	pc = 0x83216DAC; continue 'dispatch;
	}
	// 83216DA0: 2F0B0066  cmpwi cr6, r11, 0x66
	ctx.cr[6].compare_i32(ctx.r[11].s32, 102, &mut ctx.xer);
	// 83216DA4: 41990008  bgt cr6, 0x83216dac
	if ctx.cr[6].gt {
	pc = 0x83216DAC; continue 'dispatch;
	}
	// 83216DA8: 394BFFA9  addi r10, r11, -0x57
	ctx.r[10].s64 = ctx.r[11].s64 + -87;
	// 83216DAC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83216DB0: 40980024  bge cr6, 0x83216dd4
	if !ctx.cr[6].lt {
	pc = 0x83216DD4; continue 'dispatch;
	}
	// 83216DB4: 790B0020  clrldi r11, r8, 0x20
	ctx.r[11].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 83216DB8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83216DBC: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 83216DC0: 7D6BE9D2  mulld r11, r11, r29
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[29].s64;
	// 83216DC4: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83216DC8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83216DCC: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216DD0: 4198FF8C  blt cr6, 0x83216d5c
	if ctx.cr[6].lt {
	pc = 0x83216D5C; continue 'dispatch;
	}
	// 83216DD4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216DD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83216DDC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83216DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216DE4: 4E800421  bctrl
	ctx.lr = 0x83216DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216DE8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216DEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83216DF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83216DF4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83216DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216DFC: 4E800421  bctrl
	ctx.lr = 0x83216E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216E00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83216E04: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 83216E08: 4BA92650  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 83216E0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83216E10: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 83216E14: 4BA92644  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83216E18 size=392
    let mut pc: u32 = 0x83216E18;
    'dispatch: loop {
        match pc {
            0x83216E18 => {
    //   block [0x83216E18..0x83216FA0)
	// 83216E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83216E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83216E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83216E28: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216E2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83216E30: 4BFFFCA9  bl 0x83216ad8
	ctx.lr = 0x83216E34;
	sub_83216AD8(ctx, base);
	// 83216E34: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216E38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83216E3C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 83216E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216E44: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83216E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216E4C: 4E800421  bctrl
	ctx.lr = 0x83216E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216E50: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216E54: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83216E58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83216E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216E60: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216E68: 4E800421  bctrl
	ctx.lr = 0x83216E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216E6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83216E70: 419A00BC  beq cr6, 0x83216f2c
	if ctx.cr[6].eq {
	pc = 0x83216F2C; continue 'dispatch;
	}
	// 83216E74: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83216E78: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216E7C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83216E80: 4198000C  blt cr6, 0x83216e8c
	if ctx.cr[6].lt {
	pc = 0x83216E8C; continue 'dispatch;
	}
	// 83216E84: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 83216E88: 40990024  ble cr6, 0x83216eac
	if !ctx.cr[6].gt {
	pc = 0x83216EAC; continue 'dispatch;
	}
	// 83216E8C: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 83216E90: 419A001C  beq cr6, 0x83216eac
	if ctx.cr[6].eq {
	pc = 0x83216EAC; continue 'dispatch;
	}
	// 83216E94: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 83216E98: 419A0014  beq cr6, 0x83216eac
	if ctx.cr[6].eq {
	pc = 0x83216EAC; continue 'dispatch;
	}
	// 83216E9C: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 83216EA0: 419A000C  beq cr6, 0x83216eac
	if ctx.cr[6].eq {
	pc = 0x83216EAC; continue 'dispatch;
	}
	// 83216EA4: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 83216EA8: 409A0084  bne cr6, 0x83216f2c
	if !ctx.cr[6].eq {
	pc = 0x83216F2C; continue 'dispatch;
	}
	// 83216EAC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83216EB0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83216EB4: 40990078  ble cr6, 0x83216f2c
	if !ctx.cr[6].gt {
	pc = 0x83216F2C; continue 'dispatch;
	}
	// 83216EB8: 39410051  addi r10, r1, 0x51
	ctx.r[10].s64 = ctx.r[1].s64 + 81;
	// 83216EBC: 3920002E  li r9, 0x2e
	ctx.r[9].s64 = 46;
	// 83216EC0: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216EC4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83216EC8: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83216ECC: 4198000C  blt cr6, 0x83216ed8
	if ctx.cr[6].lt {
	pc = 0x83216ED8; continue 'dispatch;
	}
	// 83216ED0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 83216ED4: 40990034  ble cr6, 0x83216f08
	if !ctx.cr[6].gt {
	pc = 0x83216F08; continue 'dispatch;
	}
	// 83216ED8: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 83216EDC: 419A002C  beq cr6, 0x83216f08
	if ctx.cr[6].eq {
	pc = 0x83216F08; continue 'dispatch;
	}
	// 83216EE0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 83216EE4: 419A0024  beq cr6, 0x83216f08
	if ctx.cr[6].eq {
	pc = 0x83216F08; continue 'dispatch;
	}
	// 83216EE8: 2F0B0045  cmpwi cr6, r11, 0x45
	ctx.cr[6].compare_i32(ctx.r[11].s32, 69, &mut ctx.xer);
	// 83216EEC: 419A001C  beq cr6, 0x83216f08
	if ctx.cr[6].eq {
	pc = 0x83216F08; continue 'dispatch;
	}
	// 83216EF0: 2F0B0065  cmpwi cr6, r11, 0x65
	ctx.cr[6].compare_i32(ctx.r[11].s32, 101, &mut ctx.xer);
	// 83216EF4: 419A0014  beq cr6, 0x83216f08
	if ctx.cr[6].eq {
	pc = 0x83216F08; continue 'dispatch;
	}
	// 83216EF8: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 83216EFC: 419A001C  beq cr6, 0x83216f18
	if ctx.cr[6].eq {
	pc = 0x83216F18; continue 'dispatch;
	}
	// 83216F00: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 83216F04: 409A0028  bne cr6, 0x83216f2c
	if !ctx.cr[6].eq {
	pc = 0x83216F2C; continue 'dispatch;
	}
	// 83216F08: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 83216F0C: 419A000C  beq cr6, 0x83216f18
	if ctx.cr[6].eq {
	pc = 0x83216F18; continue 'dispatch;
	}
	// 83216F10: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 83216F14: 409A0008  bne cr6, 0x83216f1c
	if !ctx.cr[6].eq {
	pc = 0x83216F1C; continue 'dispatch;
	}
	// 83216F18: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83216F1C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83216F20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83216F24: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83216F28: 4198FF98  blt cr6, 0x83216ec0
	if ctx.cr[6].lt {
	pc = 0x83216EC0; continue 'dispatch;
	}
	// 83216F2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216F34: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83216F38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216F3C: 4E800421  bctrl
	ctx.lr = 0x83216F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216F40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216F44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83216F48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83216F4C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83216F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216F54: 4E800421  bctrl
	ctx.lr = 0x83216F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216F58: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83216F5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83216F60: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83216F64: 7D5F59AE  stbx r10, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 83216F68: 40990018  ble cr6, 0x83216f80
	if !ctx.cr[6].gt {
	pc = 0x83216F80; continue 'dispatch;
	}
	// 83216F6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83216F70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83216F74: 4BA9921D  bl 0x82cb0190
	ctx.lr = 0x83216F78;
	sub_82CB0190(ctx, base);
	// 83216F78: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83216F7C: 4800000C  b 0x83216f88
	pc = 0x83216F88; continue 'dispatch;
	// 83216F80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83216F84: C02B0EE0  lfs f1, 0xee0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3808 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83216F88: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 83216F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83216F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83216F94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83216F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83216F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83216FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83216FA0 size=412
    let mut pc: u32 = 0x83216FA0;
    'dispatch: loop {
        match pc {
            0x83216FA0 => {
    //   block [0x83216FA0..0x8321713C)
	// 83216FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83216FA4: 4BA92469  bl 0x82ca940c
	ctx.lr = 0x83216FA8;
	sub_82CA93D0(ctx, base);
	// 83216FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83216FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83216FB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83216FB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216FB8: 4BFFFB21  bl 0x83216ad8
	ctx.lr = 0x83216FBC;
	sub_83216AD8(ctx, base);
	// 83216FBC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216FC0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83216FC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216FC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83216FCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216FD0: 4E800421  bctrl
	ctx.lr = 0x83216FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216FD4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83216FD8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83216FDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83216FE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83216FE4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83216FE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83216FEC: 4E800421  bctrl
	ctx.lr = 0x83216FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83216FF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83216FF4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 83216FF8: 41980090  blt cr6, 0x83217088
	if ctx.cr[6].lt {
	pc = 0x83217088; continue 'dispatch;
	}
	// 83216FFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83217000: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83217004: 388B0F40  addi r4, r11, 0xf40
	ctx.r[4].s64 = ctx.r[11].s64 + 3904;
	// 83217008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321700C: 4BB419FD  bl 0x82d58a08
	ctx.lr = 0x83217010;
	sub_82D58A08(ctx, base);
	// 83217010: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83217014: 409A0074  bne cr6, 0x83217088
	if !ctx.cr[6].eq {
	pc = 0x83217088; continue 'dispatch;
	}
	// 83217018: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 8321701C: 419A002C  beq cr6, 0x83217048
	if ctx.cr[6].eq {
	pc = 0x83217048; continue 'dispatch;
	}
	// 83217020: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217024: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83217028: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8321702C: 419A001C  beq cr6, 0x83217048
	if ctx.cr[6].eq {
	pc = 0x83217048; continue 'dispatch;
	}
	// 83217030: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83217034: 419A0014  beq cr6, 0x83217048
	if ctx.cr[6].eq {
	pc = 0x83217048; continue 'dispatch;
	}
	// 83217038: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 8321703C: 419A000C  beq cr6, 0x83217048
	if ctx.cr[6].eq {
	pc = 0x83217048; continue 'dispatch;
	}
	// 83217040: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83217044: 409A0044  bne cr6, 0x83217088
	if !ctx.cr[6].eq {
	pc = 0x83217088; continue 'dispatch;
	}
	// 83217048: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8321704C: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83217050: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217054: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217058: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8321705C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217060: 4E800421  bctrl
	ctx.lr = 0x83217064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217064: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217068: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8321706C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217070: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83217074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217078: 4E800421  bctrl
	ctx.lr = 0x8321707C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321707C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217084: 4BA923D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 83217088: 2F1E0005  cmpwi cr6, r30, 5
	ctx.cr[6].compare_i32(ctx.r[30].s32, 5, &mut ctx.xer);
	// 8321708C: 41980090  blt cr6, 0x8321711c
	if ctx.cr[6].lt {
	pc = 0x8321711C; continue 'dispatch;
	}
	// 83217090: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83217094: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83217098: 388B1700  addi r4, r11, 0x1700
	ctx.r[4].s64 = ctx.r[11].s64 + 5888;
	// 8321709C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832170A0: 4BB41969  bl 0x82d58a08
	ctx.lr = 0x832170A4;
	sub_82D58A08(ctx, base);
	// 832170A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832170A8: 409A0074  bne cr6, 0x8321711c
	if !ctx.cr[6].eq {
	pc = 0x8321711C; continue 'dispatch;
	}
	// 832170AC: 2F1E0005  cmpwi cr6, r30, 5
	ctx.cr[6].compare_i32(ctx.r[30].s32, 5, &mut ctx.xer);
	// 832170B0: 419A002C  beq cr6, 0x832170dc
	if ctx.cr[6].eq {
	pc = 0x832170DC; continue 'dispatch;
	}
	// 832170B4: 89610055  lbz r11, 0x55(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 832170B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832170BC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 832170C0: 419A001C  beq cr6, 0x832170dc
	if ctx.cr[6].eq {
	pc = 0x832170DC; continue 'dispatch;
	}
	// 832170C4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 832170C8: 419A0014  beq cr6, 0x832170dc
	if ctx.cr[6].eq {
	pc = 0x832170DC; continue 'dispatch;
	}
	// 832170CC: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 832170D0: 419A000C  beq cr6, 0x832170dc
	if ctx.cr[6].eq {
	pc = 0x832170DC; continue 'dispatch;
	}
	// 832170D4: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 832170D8: 409A0044  bne cr6, 0x8321711c
	if !ctx.cr[6].eq {
	pc = 0x8321711C; continue 'dispatch;
	}
	// 832170DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832170E0: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 832170E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832170E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832170EC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832170F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832170F4: 4E800421  bctrl
	ctx.lr = 0x832170F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832170F8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832170FC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83217100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217104: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83217108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8321710C: 4E800421  bctrl
	ctx.lr = 0x83217110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217118: 4BA92344  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8321711C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217120: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217124: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83217128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8321712C: 4E800421  bctrl
	ctx.lr = 0x83217130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217138: 4BA92324  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217140 size=68
    let mut pc: u32 = 0x83217140;
    'dispatch: loop {
        match pc {
            0x83217140 => {
    //   block [0x83217140..0x83217184)
	// 83217140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321714C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217150: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217154: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83217158: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321715C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217160: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217168: 4E800421  bctrl
	ctx.lr = 0x8321716C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321717C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83217188 size=68
    let mut pc: u32 = 0x83217188;
    'dispatch: loop {
        match pc {
            0x83217188 => {
    //   block [0x83217188..0x832171CC)
	// 83217188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83217194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321719C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832171A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832171A4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832171A8: 4BFFFC71  bl 0x83216e18
	ctx.lr = 0x832171AC;
	sub_83216E18(ctx, base);
	// 832171AC: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832171B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832171B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832171B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832171BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832171C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832171C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832171C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832171D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832171D0 size=68
    let mut pc: u32 = 0x832171D0;
    'dispatch: loop {
        match pc {
            0x832171D0 => {
    //   block [0x832171D0..0x83217214)
	// 832171D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832171D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832171D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832171DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832171E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832171E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832171E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832171EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832171F0: 4BFFFC29  bl 0x83216e18
	ctx.lr = 0x832171F4;
	sub_83216E18(ctx, base);
	// 832171F4: D83E0000  stfd f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 832171F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832171FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321720C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217218 size=124
    let mut pc: u32 = 0x83217218;
    'dispatch: loop {
        match pc {
            0x83217218 => {
    //   block [0x83217218..0x83217294)
	// 83217218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321721C: 4BA921E9  bl 0x82ca9404
	ctx.lr = 0x83217220;
	sub_82CA93D0(ctx, base);
	// 83217220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217224: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83217228: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8321722C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83217230: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83217234: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217238: 4BFFF8A1  bl 0x83216ad8
	ctx.lr = 0x8321723C;
	sub_83216AD8(ctx, base);
	// 8321723C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83217240: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83217244: 40990030  ble cr6, 0x83217274
	if !ctx.cr[6].gt {
	pc = 0x83217274; continue 'dispatch;
	}
	// 83217248: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321724C: 4BB49AFD  bl 0x82d60d48
	ctx.lr = 0x83217250;
	sub_82D60D48(ctx, base);
	// 83217250: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83217254: 419A002C  beq cr6, 0x83217280
	if ctx.cr[6].eq {
	pc = 0x83217280; continue 'dispatch;
	}
	// 83217258: 7F6B0774  extsb r11, r27
	ctx.r[11].s64 = ctx.r[27].s8 as i64;
	// 8321725C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83217260: 419A0020  beq cr6, 0x83217280
	if ctx.cr[6].eq {
	pc = 0x83217280; continue 'dispatch;
	}
	// 83217264: 7C7FE9AE  stbx r3, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[3].u8) };
	// 83217268: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8321726C: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83217270: 4198FFD8  blt cr6, 0x83217248
	if ctx.cr[6].lt {
	pc = 0x83217248; continue 'dispatch;
	}
	// 83217274: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83217278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321727C: 4BA921D8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 83217280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83217284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217288: 7D7FE9AE  stbx r11, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 8321728C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217290: 4BA921C4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217298 size=20
    let mut pc: u32 = 0x83217298;
    'dispatch: loop {
        match pc {
            0x83217298 => {
    //   block [0x83217298..0x832172AC)
	// 83217298: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321729C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832172A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 832172A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832172A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832172B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832172B0 size=36
    let mut pc: u32 = 0x832172B0;
    'dispatch: loop {
        match pc {
            0x832172B0 => {
    //   block [0x832172B0..0x832172D4)
	// 832172B0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832172B4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 832172B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832172BC: 396BFAAC  addi r11, r11, -0x554
	ctx.r[11].s64 = ctx.r[11].s64 + -1364;
	// 832172C0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 832172C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832172C8: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832172CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832172D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832172D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832172D4 size=16
    let mut pc: u32 = 0x832172D4;
    'dispatch: loop {
        match pc {
            0x832172D4 => {
    //   block [0x832172D4..0x832172E4)
	// 832172D4: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 832172D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832172DC: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 832172E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832172E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832172E8 size=96
    let mut pc: u32 = 0x832172E8;
    'dispatch: loop {
        match pc {
            0x832172E8 => {
    //   block [0x832172E8..0x83217348)
	// 832172E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832172EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832172F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832172F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832172F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832172FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217300: 396BFAAC  addi r11, r11, -0x554
	ctx.r[11].s64 = ctx.r[11].s64 + -1364;
	// 83217304: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83217308: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8321730C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83217310: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 83217314: 806B7700  lwz r3, 0x7700(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30464 as u32) ) } as u64;
	// 83217318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321731C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217324: 4E800421  bctrl
	ctx.lr = 0x83217328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217328: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8321732C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217330: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83217334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217348 size=112
    let mut pc: u32 = 0x83217348;
    'dispatch: loop {
        match pc {
            0x83217348 => {
    //   block [0x83217348..0x832173B8)
	// 83217348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321734C: 4BA920C1  bl 0x82ca940c
	ctx.lr = 0x83217350;
	sub_82CA93D0(ctx, base);
	// 83217350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217354: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83217358: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321735C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217360: 396BFAAC  addi r11, r11, -0x554
	ctx.r[11].s64 = ctx.r[11].s64 + -1364;
	// 83217364: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83217368: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8321736C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83217370: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83217374: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 83217378: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8321737C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 83217380: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83217384: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83217388: 4BB3DEC1  bl 0x82d55248
	ctx.lr = 0x8321738C;
	sub_82D55248(ctx, base);
	// 8321738C: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 83217390: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83217394: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83217398: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321739C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 832173A0: 4BEBCEE9  bl 0x830d4288
	ctx.lr = 0x832173A4;
	sub_830D4288(ctx, base);
	// 832173A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832173A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832173AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832173B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832173B4: 4BA920A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832173B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832173B8 size=132
    let mut pc: u32 = 0x832173B8;
    'dispatch: loop {
        match pc {
            0x832173B8 => {
    //   block [0x832173B8..0x8321743C)
	// 832173B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832173BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832173C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832173C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832173C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832173CC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832173D0: 396BFAAC  addi r11, r11, -0x554
	ctx.r[11].s64 = ctx.r[11].s64 + -1364;
	// 832173D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832173D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832173DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832173E0: 419A003C  beq cr6, 0x8321741c
	if ctx.cr[6].eq {
	pc = 0x8321741C; continue 'dispatch;
	}
	// 832173E4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832173E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832173EC: 419A0030  beq cr6, 0x8321741c
	if ctx.cr[6].eq {
	pc = 0x8321741C; continue 'dispatch;
	}
	// 832173F0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 832173F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832173F8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 832173FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83217400: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 83217404: 409A0018  bne cr6, 0x8321741c
	if !ctx.cr[6].eq {
	pc = 0x8321741C; continue 'dispatch;
	}
	// 83217408: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321740C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83217410: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217418: 4E800421  bctrl
	ctx.lr = 0x8321741C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321741C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83217420: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 83217424: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321742C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217440 size=100
    let mut pc: u32 = 0x83217440;
    'dispatch: loop {
        match pc {
            0x83217440 => {
    //   block [0x83217440..0x832174A4)
	// 83217440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321744C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217458: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321745C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83217460: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217464: 4BFFF795  bl 0x83216bf8
	ctx.lr = 0x83217468;
	sub_83216BF8(ctx, base);
	// 83217468: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321746C: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83217470: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83217474: 419A0010  beq cr6, 0x83217484
	if ctx.cr[6].eq {
	pc = 0x83217484; continue 'dispatch;
	}
	// 83217478: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8321747C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83217480: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83217484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217488: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8321748C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321749C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832174A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832174A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832174A8 size=100
    let mut pc: u32 = 0x832174A8;
    'dispatch: loop {
        match pc {
            0x832174A8 => {
    //   block [0x832174A8..0x8321750C)
	// 832174A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832174AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832174B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832174B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832174B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832174BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832174C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832174C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832174C8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832174CC: 4BFFF72D  bl 0x83216bf8
	ctx.lr = 0x832174D0;
	sub_83216BF8(ctx, base);
	// 832174D0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832174D4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 832174D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832174DC: 419A0010  beq cr6, 0x832174ec
	if ctx.cr[6].eq {
	pc = 0x832174EC; continue 'dispatch;
	}
	// 832174E0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 832174E4: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 832174E8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 832174EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832174F0: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 832174F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832174F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832174FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217500: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217510 size=92
    let mut pc: u32 = 0x83217510;
    'dispatch: loop {
        match pc {
            0x83217510 => {
    //   block [0x83217510..0x8321756C)
	// 83217510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321751C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321752C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83217530: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217534: 4BFFF6C5  bl 0x83216bf8
	ctx.lr = 0x83217538;
	sub_83216BF8(ctx, base);
	// 83217538: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321753C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83217540: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83217544: 419A0008  beq cr6, 0x8321754c
	if ctx.cr[6].eq {
	pc = 0x8321754C; continue 'dispatch;
	}
	// 83217548: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8321754C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217550: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321755C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217560: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217570 size=92
    let mut pc: u32 = 0x83217570;
    'dispatch: loop {
        match pc {
            0x83217570 => {
    //   block [0x83217570..0x832175CC)
	// 83217570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321757C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321758C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83217590: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217594: 4BFFF665  bl 0x83216bf8
	ctx.lr = 0x83217598;
	sub_83216BF8(ctx, base);
	// 83217598: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321759C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832175A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832175A4: 419A0008  beq cr6, 0x832175ac
	if ctx.cr[6].eq {
	pc = 0x832175AC; continue 'dispatch;
	}
	// 832175A8: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 832175AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832175B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832175B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832175B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832175BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832175C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832175C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832175C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832175D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832175D0 size=92
    let mut pc: u32 = 0x832175D0;
    'dispatch: loop {
        match pc {
            0x832175D0 => {
    //   block [0x832175D0..0x8321762C)
	// 832175D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832175D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832175D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832175DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832175E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832175E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832175E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832175EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832175F0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832175F4: 4BFFF605  bl 0x83216bf8
	ctx.lr = 0x832175F8;
	sub_83216BF8(ctx, base);
	// 832175F8: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832175FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217600: 7D6300D0  neg r11, r3
	ctx.r[11].s64 = -ctx.r[3].s64;
	// 83217604: 409A0008  bne cr6, 0x8321760c
	if !ctx.cr[6].eq {
	pc = 0x8321760C; continue 'dispatch;
	}
	// 83217608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8321760C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217610: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83217614: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217620: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217630 size=92
    let mut pc: u32 = 0x83217630;
    'dispatch: loop {
        match pc {
            0x83217630 => {
    //   block [0x83217630..0x8321768C)
	// 83217630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321763C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217648: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321764C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83217650: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217654: 4BFFF5A5  bl 0x83216bf8
	ctx.lr = 0x83217658;
	sub_83216BF8(ctx, base);
	// 83217658: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321765C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217660: 7D6300D0  neg r11, r3
	ctx.r[11].s64 = -ctx.r[3].s64;
	// 83217664: 409A0008  bne cr6, 0x8321766c
	if !ctx.cr[6].eq {
	pc = 0x8321766C; continue 'dispatch;
	}
	// 83217668: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8321766C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217670: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83217674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321767C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217690 size=536
    let mut pc: u32 = 0x83217690;
    'dispatch: loop {
        match pc {
            0x83217690 => {
    //   block [0x83217690..0x832178A8)
	// 83217690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217694: 4BA91D71  bl 0x82ca9404
	ctx.lr = 0x83217698;
	sub_82CA93D0(ctx, base);
	// 83217698: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321769C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 832176A0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 832176A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832176A8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832176AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832176B0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 832176B4: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 832176B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832176BC: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832176C0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 832176C4: 4BB3F84D  bl 0x82d56f10
	ctx.lr = 0x832176C8;
	sub_82D56F10(ctx, base);
	// 832176C8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 832176CC: 4BFFF40D  bl 0x83216ad8
	ctx.lr = 0x832176D0;
	sub_83216AD8(ctx, base);
	// 832176D0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 832176D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832176D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832176DC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832176E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832176E4: 4E800421  bctrl
	ctx.lr = 0x832176E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832176E8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 832176EC: 4BB4965D  bl 0x82d60d48
	ctx.lr = 0x832176F0;
	sub_82D60D48(ctx, base);
	// 832176F0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 832176F4: 419A0088  beq cr6, 0x8321777c
	if ctx.cr[6].eq {
	pc = 0x8321777C; continue 'dispatch;
	}
	// 832176F8: 2F030020  cmpwi cr6, r3, 0x20
	ctx.cr[6].compare_i32(ctx.r[3].s32, 32, &mut ctx.xer);
	// 832176FC: 419A0080  beq cr6, 0x8321777c
	if ctx.cr[6].eq {
	pc = 0x8321777C; continue 'dispatch;
	}
	// 83217700: 2F030009  cmpwi cr6, r3, 9
	ctx.cr[6].compare_i32(ctx.r[3].s32, 9, &mut ctx.xer);
	// 83217704: 419A0078  beq cr6, 0x8321777c
	if ctx.cr[6].eq {
	pc = 0x8321777C; continue 'dispatch;
	}
	// 83217708: 2F03000D  cmpwi cr6, r3, 0xd
	ctx.cr[6].compare_i32(ctx.r[3].s32, 13, &mut ctx.xer);
	// 8321770C: 419A0070  beq cr6, 0x8321777c
	if ctx.cr[6].eq {
	pc = 0x8321777C; continue 'dispatch;
	}
	// 83217710: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 83217714: 419A0068  beq cr6, 0x8321777c
	if ctx.cr[6].eq {
	pc = 0x8321777C; continue 'dispatch;
	}
	// 83217718: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321771C: 7C7F0774  extsb r31, r3
	ctx.r[31].s64 = ctx.r[3].s8 as i64;
	// 83217720: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217724: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 83217728: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8321772C: 409A0010  bne cr6, 0x8321773c
	if !ctx.cr[6].eq {
	pc = 0x8321773C; continue 'dispatch;
	}
	// 83217730: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83217734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83217738: 4BB3F861  bl 0x82d56f98
	ctx.lr = 0x8321773C;
	sub_82D56F98(ctx, base);
	// 8321773C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217740: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83217744: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217748: 7FEB51AE  stbx r31, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u8) };
	// 8321774C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217750: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217754: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83217758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8321775C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217760: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83217764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217768: 4E800421  bctrl
	ctx.lr = 0x8321776C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321776C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217770: 4BB495D9  bl 0x82d60d48
	ctx.lr = 0x83217774;
	sub_82D60D48(ctx, base);
	// 83217774: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83217778: 409AFF80  bne cr6, 0x832176f8
	if !ctx.cr[6].eq {
	pc = 0x832176F8; continue 'dispatch;
	}
	// 8321777C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217784: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83217788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8321778C: 4E800421  bctrl
	ctx.lr = 0x83217790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217790: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83217794: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217798: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8321779C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832177A0: 409A0010  bne cr6, 0x832177b0
	if !ctx.cr[6].eq {
	pc = 0x832177B0; continue 'dispatch;
	}
	// 832177A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832177A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832177AC: 4BB3F7ED  bl 0x82d56f98
	ctx.lr = 0x832177B0;
	sub_82D56F98(ctx, base);
	// 832177B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832177B4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 832177B8: 7F6B51AE  stbx r27, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 832177BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 832177C0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832177C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832177C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832177CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832177D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832177D4: 419A0060  beq cr6, 0x83217834
	if ctx.cr[6].eq {
	pc = 0x83217834; continue 'dispatch;
	}
	// 832177D8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832177DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832177E0: 419A0054  beq cr6, 0x83217834
	if ctx.cr[6].eq {
	pc = 0x83217834; continue 'dispatch;
	}
	// 832177E4: 4BB41445  bl 0x82d58c28
	ctx.lr = 0x832177E8;
	sub_82D58C28(ctx, base);
	// 832177E8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 832177EC: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 832177F0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 832177F4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 832177F8: 40980024  bge cr6, 0x8321781c
	if !ctx.cr[6].lt {
	pc = 0x8321781C; continue 'dispatch;
	}
	// 832177FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83217800: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83217804: 41980008  blt cr6, 0x8321780c
	if ctx.cr[6].lt {
	pc = 0x8321780C; continue 'dispatch;
	}
	// 83217808: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8321780C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83217810: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83217814: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83217818: 4BB3F6F9  bl 0x82d56f10
	ctx.lr = 0x8321781C;
	sub_82D56F10(ctx, base);
	// 8321781C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83217820: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217824: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83217828: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8321782C: 4BB41505  bl 0x82d58d30
	ctx.lr = 0x83217830;
	sub_82D58D30(ctx, base);
	// 83217830: 48000040  b 0x83217870
	pc = 0x83217870; continue 'dispatch;
	// 83217834: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217838: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8321783C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83217840: 40980020  bge cr6, 0x83217860
	if !ctx.cr[6].lt {
	pc = 0x83217860; continue 'dispatch;
	}
	// 83217844: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83217848: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8321784C: 41990008  bgt cr6, 0x83217854
	if ctx.cr[6].gt {
	pc = 0x83217854; continue 'dispatch;
	}
	// 83217850: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83217854: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83217858: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8321785C: 4BB3F6B5  bl 0x82d56f10
	ctx.lr = 0x83217860;
	sub_82D56F10(ctx, base);
	// 83217860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83217864: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217868: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8321786C: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 83217870: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83217874: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83217878: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8321787C: 409A0020  bne cr6, 0x8321789c
	if !ctx.cr[6].eq {
	pc = 0x8321789C; continue 'dispatch;
	}
	// 83217880: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217884: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83217888: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8321788C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217890: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 83217894: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83217898: 4BB3DA31  bl 0x82d552c8
	ctx.lr = 0x8321789C;
	sub_82D552C8(ctx, base);
	// 8321789C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832178A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832178A4: 4BA91BB0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832178A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832178A8 size=100
    let mut pc: u32 = 0x832178A8;
    'dispatch: loop {
        match pc {
            0x832178A8 => {
    //   block [0x832178A8..0x8321790C)
	// 832178A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832178AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832178B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832178B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832178B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832178BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832178C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832178C4: 4BFFFAF5  bl 0x832173b8
	ctx.lr = 0x832178C8;
	sub_832173B8(ctx, base);
	// 832178C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 832178CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832178D0: 419A0020  beq cr6, 0x832178f0
	if ctx.cr[6].eq {
	pc = 0x832178F0; continue 'dispatch;
	}
	// 832178D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 832178D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 832178DC: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 832178E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832178E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832178E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832178EC: 4BB3D9DD  bl 0x82d552c8
	ctx.lr = 0x832178F0;
	sub_82D552C8(ctx, base);
	// 832178F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832178F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832178F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832178FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217900: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217910 size=128
    let mut pc: u32 = 0x83217910;
    'dispatch: loop {
        match pc {
            0x83217910 => {
    //   block [0x83217910..0x83217990)
	// 83217910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321791C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321792C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217934: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8321793C: 4E800421  bctrl
	ctx.lr = 0x83217940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83217944: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83217948: 40990030  ble cr6, 0x83217978
	if !ctx.cr[6].gt {
	pc = 0x83217978; continue 'dispatch;
	}
	// 8321794C: 7D4BF0AE  lbzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83217950: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83217954: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217958: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8321795C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83217960: 394AFFF6  addi r10, r10, -0xa
	ctx.r[10].s64 = ctx.r[10].s64 + -10;
	// 83217964: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83217968: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8321796C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83217970: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83217974: 4198FFD8  blt cr6, 0x8321794c
	if ctx.cr[6].lt {
	pc = 0x8321794C; continue 'dispatch;
	}
	// 83217978: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217984: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321798C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217990 size=64
    let mut pc: u32 = 0x83217990;
    'dispatch: loop {
        match pc {
            0x83217990 => {
    //   block [0x83217990..0x832179D0)
	// 83217990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321799C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832179A0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832179A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832179A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832179AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832179B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832179B4: 4E800421  bctrl
	ctx.lr = 0x832179B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832179B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832179BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832179C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832179C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832179C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832179CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832179D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832179D0 size=64
    let mut pc: u32 = 0x832179D0;
    'dispatch: loop {
        match pc {
            0x832179D0 => {
    //   block [0x832179D0..0x83217A10)
	// 832179D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832179D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832179D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832179DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832179E0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832179E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832179E8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832179EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832179F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832179F4: 4E800421  bctrl
	ctx.lr = 0x832179F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832179F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832179FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217A08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217A10 size=76
    let mut pc: u32 = 0x83217A10;
    'dispatch: loop {
        match pc {
            0x83217A10 => {
    //   block [0x83217A10..0x83217A5C)
	// 83217A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217A24: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217A28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217A2C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83217A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217A34: 4E800421  bctrl
	ctx.lr = 0x83217A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217A38: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83217A3C: 409A000C  bne cr6, 0x83217a48
	if !ctx.cr[6].eq {
	pc = 0x83217A48; continue 'dispatch;
	}
	// 83217A40: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217A44: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83217A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217A60 size=76
    let mut pc: u32 = 0x83217A60;
    'dispatch: loop {
        match pc {
            0x83217A60 => {
    //   block [0x83217A60..0x83217AAC)
	// 83217A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217A68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217A6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217A70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217A74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217A78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217A7C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83217A80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217A84: 4E800421  bctrl
	ctx.lr = 0x83217A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217A88: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83217A8C: 409A000C  bne cr6, 0x83217a98
	if !ctx.cr[6].eq {
	pc = 0x83217A98; continue 'dispatch;
	}
	// 83217A90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217A94: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83217A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217AA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217AB0 size=48
    let mut pc: u32 = 0x83217AB0;
    'dispatch: loop {
        match pc {
            0x83217AB0 => {
    //   block [0x83217AB0..0x83217AE0)
	// 83217AB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83217AB4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 83217AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83217ABC: 396B76CC  addi r11, r11, 0x76cc
	ctx.r[11].s64 = ctx.r[11].s64 + 30412;
	// 83217AC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83217AC4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83217AC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217ACC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83217AD0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83217AD4: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217ADC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217AE0 size=16
    let mut pc: u32 = 0x83217AE0;
    'dispatch: loop {
        match pc {
            0x83217AE0 => {
    //   block [0x83217AE0..0x83217AF0)
	// 83217AE0: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 83217AE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83217AE8: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 83217AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217AF0 size=124
    let mut pc: u32 = 0x83217AF0;
    'dispatch: loop {
        match pc {
            0x83217AF0 => {
    //   block [0x83217AF0..0x83217B6C)
	// 83217AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217B00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217B04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83217B08: 396B76CC  addi r11, r11, 0x76cc
	ctx.r[11].s64 = ctx.r[11].s64 + 30412;
	// 83217B0C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217B10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217B14: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217B18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217B1C: 419A0030  beq cr6, 0x83217b4c
	if ctx.cr[6].eq {
	pc = 0x83217B4C; continue 'dispatch;
	}
	// 83217B20: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 83217B24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83217B28: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83217B2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83217B30: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 83217B34: 409A0018  bne cr6, 0x83217b4c
	if !ctx.cr[6].eq {
	pc = 0x83217B4C; continue 'dispatch;
	}
	// 83217B38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217B3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83217B40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217B48: 4E800421  bctrl
	ctx.lr = 0x83217B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217B4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83217B50: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 83217B54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83217B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217B64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217B70 size=100
    let mut pc: u32 = 0x83217B70;
    'dispatch: loop {
        match pc {
            0x83217B70 => {
    //   block [0x83217B70..0x83217BD4)
	// 83217B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83217B78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83217B7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83217B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217B84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217B88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83217B8C: 4BFFFF65  bl 0x83217af0
	ctx.lr = 0x83217B90;
	sub_83217AF0(ctx, base);
	// 83217B90: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83217B94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217B98: 419A0020  beq cr6, 0x83217bb8
	if ctx.cr[6].eq {
	pc = 0x83217BB8; continue 'dispatch;
	}
	// 83217B9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217BA0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83217BA4: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 83217BA8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83217BB0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83217BB4: 4BB3D715  bl 0x82d552c8
	ctx.lr = 0x83217BB8;
	sub_82D552C8(ctx, base);
	// 83217BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217BBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83217BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83217BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83217BC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83217BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83217BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217BD8 size=20
    let mut pc: u32 = 0x83217BD8;
    'dispatch: loop {
        match pc {
            0x83217BD8 => {
    //   block [0x83217BD8..0x83217BEC)
	// 83217BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83217BDC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 83217BE0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217BE4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83217BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217BF0 size=4
    let mut pc: u32 = 0x83217BF0;
    'dispatch: loop {
        match pc {
            0x83217BF0 => {
    //   block [0x83217BF0..0x83217BF4)
	// 83217BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217BF8 size=16
    let mut pc: u32 = 0x83217BF8;
    'dispatch: loop {
        match pc {
            0x83217BF8 => {
    //   block [0x83217BF8..0x83217C08)
	// 83217BF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83217BFC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83217C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C04: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C08 size=20
    let mut pc: u32 = 0x83217C08;
    'dispatch: loop {
        match pc {
            0x83217C08 => {
    //   block [0x83217C08..0x83217C1C)
	// 83217C08: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217C0C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83217C10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C14: 409AFFF4  bne cr6, 0x83217c08
	if !ctx.cr[6].eq {
	pc = 0x83217C08; continue 'dispatch;
	}
	// 83217C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C20 size=16
    let mut pc: u32 = 0x83217C20;
    'dispatch: loop {
        match pc {
            0x83217C20 => {
    //   block [0x83217C20..0x83217C30)
	// 83217C20: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83217C28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C30 size=20
    let mut pc: u32 = 0x83217C30;
    'dispatch: loop {
        match pc {
            0x83217C30 => {
    //   block [0x83217C30..0x83217C44)
	// 83217C30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217C34: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83217C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C3C: 409AFFF4  bne cr6, 0x83217c30
	if !ctx.cr[6].eq {
	pc = 0x83217C30; continue 'dispatch;
	}
	// 83217C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C48 size=8
    let mut pc: u32 = 0x83217C48;
    'dispatch: loop {
        match pc {
            0x83217C48 => {
    //   block [0x83217C48..0x83217C50)
	// 83217C48: 38640014  addi r3, r4, 0x14
	ctx.r[3].s64 = ctx.r[4].s64 + 20;
	// 83217C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C50 size=8
    let mut pc: u32 = 0x83217C50;
    'dispatch: loop {
        match pc {
            0x83217C50 => {
    //   block [0x83217C50..0x83217C58)
	// 83217C50: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C58 size=8
    let mut pc: u32 = 0x83217C58;
    'dispatch: loop {
        match pc {
            0x83217C58 => {
    //   block [0x83217C58..0x83217C60)
	// 83217C58: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C60 size=56
    let mut pc: u32 = 0x83217C60;
    'dispatch: loop {
        match pc {
            0x83217C60 => {
    //   block [0x83217C60..0x83217C98)
	// 83217C60: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217C64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83217C68: 409A002C  bne cr6, 0x83217c94
	if !ctx.cr[6].eq {
	pc = 0x83217C94; continue 'dispatch;
	}
	// 83217C6C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217C70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C74: 409A001C  bne cr6, 0x83217c90
	if !ctx.cr[6].eq {
	pc = 0x83217C90; continue 'dispatch;
	}
	// 83217C78: 80840010  lwz r4, 0x10(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217C7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83217C80: 419A0018  beq cr6, 0x83217c98
	if ctx.cr[6].eq {
		sub_83217C98(ctx, base);
		return;
	}
	// 83217C84: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217C88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217C8C: 419AFFEC  beq cr6, 0x83217c78
	if ctx.cr[6].eq {
	pc = 0x83217C78; continue 'dispatch;
	}
	// 83217C90: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217C98 size=8
    let mut pc: u32 = 0x83217C98;
    'dispatch: loop {
        match pc {
            0x83217C98 => {
    //   block [0x83217C98..0x83217CA0)
	// 83217C98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83217C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217CA0 size=8
    let mut pc: u32 = 0x83217CA0;
    'dispatch: loop {
        match pc {
            0x83217CA0 => {
    //   block [0x83217CA0..0x83217CA8)
	// 83217CA0: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83217CA8 size=8
    let mut pc: u32 = 0x83217CA8;
    'dispatch: loop {
        match pc {
            0x83217CA8 => {
    //   block [0x83217CA8..0x83217CB0)
	// 83217CA8: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217CB0 size=236
    let mut pc: u32 = 0x83217CB0;
    'dispatch: loop {
        match pc {
            0x83217CB0 => {
    //   block [0x83217CB0..0x83217D9C)
	// 83217CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217CB4: 4BA91751  bl 0x82ca9404
	ctx.lr = 0x83217CB8;
	sub_82CA93D0(ctx, base);
	// 83217CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217CBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217CC0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83217CC4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83217CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83217CCC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83217CD0: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 83217CD4: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 83217CD8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83217CDC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83217CE0: 4BB3D569  bl 0x82d55248
	ctx.lr = 0x83217CE4;
	sub_82D55248(ctx, base);
	// 83217CE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83217CE8: 419A0024  beq cr6, 0x83217d0c
	if ctx.cr[6].eq {
	pc = 0x83217D0C; continue 'dispatch;
	}
	// 83217CEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83217CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83217CF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217CF8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83217CFC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83217D00: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83217D04: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83217D08: 48000008  b 0x83217d10
	pc = 0x83217D10; continue 'dispatch;
	// 83217D0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83217D10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83217D14: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83217D18: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83217D1C: 4BB41015  bl 0x82d58d30
	ctx.lr = 0x83217D20;
	sub_82D58D30(ctx, base);
	// 83217D20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83217D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83217D28: 419A0040  beq cr6, 0x83217d68
	if ctx.cr[6].eq {
	pc = 0x83217D68; continue 'dispatch;
	}
	// 83217D2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217D34: 419A0020  beq cr6, 0x83217d54
	if ctx.cr[6].eq {
	pc = 0x83217D54; continue 'dispatch;
	}
	// 83217D38: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83217D3C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217D40: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83217D44: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83217D48: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83217D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217D50: 4BA91704  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 83217D54: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83217D58: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83217D5C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83217D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217D64: 4BA916F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 83217D68: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217D6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217D70: 419A001C  beq cr6, 0x83217d8c
	if ctx.cr[6].eq {
	pc = 0x83217D8C; continue 'dispatch;
	}
	// 83217D74: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83217D78: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217D7C: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83217D80: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83217D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217D88: 4BA916CC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 83217D8C: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83217D90: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83217D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83217D98: 4BA916BC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217DA0 size=456
    let mut pc: u32 = 0x83217DA0;
    'dispatch: loop {
        match pc {
            0x83217DA0 => {
    //   block [0x83217DA0..0x83217F68)
	// 83217DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217DA4: 4BA91659  bl 0x82ca93fc
	ctx.lr = 0x83217DA8;
	sub_82CA93D0(ctx, base);
	// 83217DA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217DAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83217DB0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83217DB4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83217DB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217DBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217DC0: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 83217DC4: 409A0008  bne cr6, 0x83217dcc
	if !ctx.cr[6].eq {
	pc = 0x83217DCC; continue 'dispatch;
	}
	// 83217DC8: 833F000C  lwz r25, 0xc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217DCC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217DD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83217DD4: 419A000C  beq cr6, 0x83217de0
	if ctx.cr[6].eq {
	pc = 0x83217DE0; continue 'dispatch;
	}
	// 83217DD8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83217DDC: 48000028  b 0x83217e04
	pc = 0x83217E04; continue 'dispatch;
	// 83217DE0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217DE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83217DE8: 419A000C  beq cr6, 0x83217df4
	if ctx.cr[6].eq {
	pc = 0x83217DF4; continue 'dispatch;
	}
	// 83217DEC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217DF0: 48000014  b 0x83217e04
	pc = 0x83217E04; continue 'dispatch;
	// 83217DF4: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217DF8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83217DFC: 409A0008  bne cr6, 0x83217e04
	if !ctx.cr[6].eq {
	pc = 0x83217E04; continue 'dispatch;
	}
	// 83217E00: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83217E04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217E0C: 419A0010  beq cr6, 0x83217e1c
	if ctx.cr[6].eq {
	pc = 0x83217E1C; continue 'dispatch;
	}
	// 83217E10: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217E14: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83217E18: 48000030  b 0x83217e48
	pc = 0x83217E48; continue 'dispatch;
	// 83217E1C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83217E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83217E24: 419A0010  beq cr6, 0x83217e34
	if ctx.cr[6].eq {
	pc = 0x83217E34; continue 'dispatch;
	}
	// 83217E28: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217E2C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83217E30: 48000018  b 0x83217e48
	pc = 0x83217E48; continue 'dispatch;
	// 83217E34: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83217E38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83217E3C: 409A000C  bne cr6, 0x83217e48
	if !ctx.cr[6].eq {
	pc = 0x83217E48; continue 'dispatch;
	}
	// 83217E40: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83217E44: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83217E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83217E4C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83217E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83217E54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83217E58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83217E5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83217E60: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 83217E64: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83217E68: 4BB3F131  bl 0x82d56f98
	ctx.lr = 0x83217E6C;
	sub_82D56F98(ctx, base);
	// 83217E6C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217E70: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217E74: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83217E78: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 83217E7C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217E80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83217E84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83217E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83217E8C: 419A00A4  beq cr6, 0x83217f30
	if ctx.cr[6].eq {
	pc = 0x83217F30; continue 'dispatch;
	}
	// 83217E90: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217E94: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 83217E98: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217E9C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83217EA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83217EA4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83217EA8: 83EAFFFC  lwz r31, -4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83217EAC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83217EB0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217EB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83217EB8: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 83217EBC: 4E800421  bctrl
	ctx.lr = 0x83217EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217EC0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217EC4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83217EC8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83217ECC: 38AB0014  addi r5, r11, 0x14
	ctx.r[5].s64 = ctx.r[11].s64 + 20;
	// 83217ED0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83217ED4: 4BB3D3F5  bl 0x82d552c8
	ctx.lr = 0x83217ED8;
	sub_82D552C8(ctx, base);
	// 83217ED8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217EDC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83217EE0: 419A0048  beq cr6, 0x83217f28
	if ctx.cr[6].eq {
	pc = 0x83217F28; continue 'dispatch;
	}
	// 83217EE4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83217EE8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 83217EEC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83217EF0: 409A0014  bne cr6, 0x83217f04
	if !ctx.cr[6].eq {
	pc = 0x83217F04; continue 'dispatch;
	}
	// 83217EF4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83217EF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83217EFC: 4BB3F09D  bl 0x82d56f98
	ctx.lr = 0x83217F00;
	sub_82D56F98(ctx, base);
	// 83217F00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217F04: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217F08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83217F0C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 83217F10: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83217F14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83217F18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83217F1C: 83DE0008  lwz r30, 8(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83217F20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83217F24: 409AFFC0  bne cr6, 0x83217ee4
	if !ctx.cr[6].eq {
	pc = 0x83217EE4; continue 'dispatch;
	}
	// 83217F28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83217F2C: 409AFF6C  bne cr6, 0x83217e98
	if !ctx.cr[6].eq {
	pc = 0x83217E98; continue 'dispatch;
	}
	// 83217F30: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83217F34: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83217F38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83217F3C: 409A0020  bne cr6, 0x83217f5c
	if !ctx.cr[6].eq {
	pc = 0x83217F5C; continue 'dispatch;
	}
	// 83217F40: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83217F44: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83217F48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83217F4C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83217F50: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83217F54: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83217F58: 4BB3D371  bl 0x82d552c8
	ctx.lr = 0x83217F5C;
	sub_82D552C8(ctx, base);
	// 83217F5C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83217F60: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83217F64: 4BA914E8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83217F68 size=140
    let mut pc: u32 = 0x83217F68;
    'dispatch: loop {
        match pc {
            0x83217F68 => {
    //   block [0x83217F68..0x83217FF4)
	// 83217F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83217F6C: 4BA914A1  bl 0x82ca940c
	ctx.lr = 0x83217F70;
	sub_82CA93D0(ctx, base);
	// 83217F70: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83217F74: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83217F78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83217F7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83217F80: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83217F84: 38E00022  li r7, 0x22
	ctx.r[7].s64 = 34;
	// 83217F88: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83217F8C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 83217F90: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83217F94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83217F98: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83217F9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83217FA0: 4E800421  bctrl
	ctx.lr = 0x83217FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83217FA4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83217FA8: 4180003C  blt 0x83217fe4
	if ctx.cr[0].lt {
	pc = 0x83217FE4; continue 'dispatch;
	}
	// 83217FAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83217FB0: 419A000C  beq cr6, 0x83217fbc
	if ctx.cr[6].eq {
	pc = 0x83217FBC; continue 'dispatch;
	}
	// 83217FB4: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83217FB8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83217FBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83217FC0: 419A000C  beq cr6, 0x83217fcc
	if ctx.cr[6].eq {
	pc = 0x83217FCC; continue 'dispatch;
	}
	// 83217FC4: E9610068  ld r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83217FC8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83217FCC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83217FD0: 419A000C  beq cr6, 0x83217fdc
	if ctx.cr[6].eq {
	pc = 0x83217FDC; continue 'dispatch;
	}
	// 83217FD4: E9610070  ld r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 83217FD8: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83217FDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83217FE0: 4800000C  b 0x83217fec
	pc = 0x83217FEC; continue 'dispatch;
	// 83217FE4: 4BAB04AD  bl 0x82cc8490
	ctx.lr = 0x83217FE8;
	sub_82CC8490(ctx, base);
	// 83217FE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83217FEC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83217FF0: 4BA9146C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83217FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83217FF8 size=16
    let mut pc: u32 = 0x83217FF8;
    'dispatch: loop {
        match pc {
            0x83217FF8 => {
    //   block [0x83217FF8..0x83218008)
	// 83217FF8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83217FFC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218000: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83218004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218008 size=204
    let mut pc: u32 = 0x83218008;
    'dispatch: loop {
        match pc {
            0x83218008 => {
    //   block [0x83218008..0x832180D4)
	// 83218008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321800C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83218010: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83218014: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83218018: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8321801C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218020: 3FC0820A  lis r30, -0x7df6
	ctx.r[30].s64 = -2113273856;
	// 83218024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83218028: 397E9490  addi r11, r30, -0x6b70
	ctx.r[11].s64 = ctx.r[30].s64 + -27504;
	// 8321802C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218030: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83218034: 388B05FC  addi r4, r11, 0x5fc
	ctx.r[4].s64 = ctx.r[11].s64 + 1532;
	// 83218038: C3EBFFF4  lfs f31, -0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8321803C: D3FF0000  stfs f31, 0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83218040: 4B014E91  bl 0x8222ced0
	ctx.lr = 0x83218044;
	sub_8222CED0(ctx, base);
	// 83218044: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83218048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321804C: 4AFD6AED  bl 0x821eeb38
	ctx.lr = 0x83218050;
	sub_821EEB38(ctx, base);
	// 83218050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218054: 4B9EB79D  bl 0x82c037f0
	ctx.lr = 0x83218058;
	sub_82C037F0(ctx, base);
	// 83218058: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8321805C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218060: 4AFFCD79  bl 0x82214dd8
	ctx.lr = 0x83218064;
	sub_82214DD8(ctx, base);
	// 83218064: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83218068: 4AFFCD71  bl 0x82214dd8
	ctx.lr = 0x8321806C;
	sub_82214DD8(ctx, base);
	// 8321806C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83218070: C01E9490  lfs f0, -0x6b70(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218078: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8321807C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83218080: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83218084: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83218088: 997F0020  stb r11, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8321808C: D3FF0018  stfs f31, 0x18(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 83218090: D3FF001C  stfs f31, 0x1c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83218094: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 83218098: D3FF0028  stfs f31, 0x28(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8321809C: D3FF002C  stfs f31, 0x2c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832180A0: 997F0030  stb r11, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 832180A4: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832180A8: 997F0031  stb r11, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 832180AC: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832180B0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 832180B4: D3FF0040  stfs f31, 0x40(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 832180B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832180BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832180C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832180C4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 832180C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832180CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832180D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832180D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832180D8 size=160
    let mut pc: u32 = 0x832180D8;
    'dispatch: loop {
        match pc {
            0x832180D8 => {
    //   block [0x832180D8..0x83218178)
	// 832180D8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 832180DC: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 832180E0: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 832180E4: 38A1FFE0  addi r5, r1, -0x20
	ctx.r[5].s64 = ctx.r[1].s64 + -32;
	// 832180E8: 3901FFD0  addi r8, r1, -0x30
	ctx.r[8].s64 = ctx.r[1].s64 + -48;
	// 832180EC: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 832180F0: 11A0030A  vcfux v13, v0, 0
	// vcfux/vcuxwfp128: ctx.v[13].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[13].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
	// 832180F4: 3921FFC0  addi r9, r1, -0x40
	ctx.r[9].s64 = ctx.r[1].s64 + -64;
	// 832180F8: 38869490  addi r4, r6, -0x6b70
	ctx.r[4].s64 = ctx.r[6].s64 + -27504;
	// 832180FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83218100: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83218104: C1A69490  lfs f13, -0x6b70(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83218108: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8321810C: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 83218110: C004FFF4  lfs f0, -0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218114: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83218118: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321811C: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 83218120: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218178 size=212
    let mut pc: u32 = 0x83218178;
    'dispatch: loop {
        match pc {
            0x83218178 => {
    //   block [0x83218178..0x8321824C)
	// 83218178: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8321817C: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83218180: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83218184: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 83218188: 38C1FFE0  addi r6, r1, -0x20
	ctx.r[6].s64 = ctx.r[1].s64 + -32;
	// 8321818C: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 83218190: 11A0030A  vcfux v13, v0, 0
	// vcfux/vcuxwfp128: ctx.v[13].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[13].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
	// 83218194: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 83218198: 3881FFB0  addi r4, r1, -0x50
	ctx.r[4].s64 = ctx.r[1].s64 + -80;
	// 8321819C: 38A79490  addi r5, r7, -0x6b70
	ctx.r[5].s64 = ctx.r[7].s64 + -27504;
	// 832181A0: 3FE0820A  lis r31, -0x7df6
	ctx.r[31].s64 = -2113273856;
	// 832181A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832181A8: C1A79490  lfs f13, -0x6b70(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832181AC: 38FF9160  addi r7, r31, -0x6ea0
	ctx.r[7].s64 = ctx.r[31].s64 + -28320;
	// 832181B0: D1A1FFB0  stfs f13, -0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), tmp.u32 ) };
	// 832181B4: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 832181B8: C005FFF4  lfs f0, -0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832181BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832181C0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832181C4: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832181C8: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832181CC: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218250 size=264
    let mut pc: u32 = 0x83218250;
    'dispatch: loop {
        match pc {
            0x83218250 => {
    //   block [0x83218250..0x83218358)
	// 83218250: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83218254: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83218258: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 8321825C: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 83218260: 38A1FFE0  addi r5, r1, -0x20
	ctx.r[5].s64 = ctx.r[1].s64 + -32;
	// 83218264: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 83218268: 11A0030A  vcfux v13, v0, 0
	// vcfux/vcuxwfp128: ctx.v[13].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[13].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
	// 8321826C: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 83218270: 38869490  addi r4, r6, -0x6b70
	ctx.r[4].s64 = ctx.r[6].s64 + -27504;
	// 83218274: 3BE1FFB0  addi r31, r1, -0x50
	ctx.r[31].s64 = ctx.r[1].s64 + -80;
	// 83218278: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 8321827C: C1A69490  lfs f13, -0x6b70(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83218280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83218284: 38C89160  addi r6, r8, -0x6ea0
	ctx.r[6].s64 = ctx.r[8].s64 + -28320;
	// 83218288: D1A1FFB0  stfs f13, -0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), tmp.u32 ) };
	// 8321828C: C004FFF4  lfs f0, -0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218290: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83218294: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83218298: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218358 size=12
    let mut pc: u32 = 0x83218358;
    'dispatch: loop {
        match pc {
            0x83218358 => {
    //   block [0x83218358..0x83218364)
	// 83218358: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321835C: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218368 size=12
    let mut pc: u32 = 0x83218368;
    'dispatch: loop {
        match pc {
            0x83218368 => {
    //   block [0x83218368..0x83218374)
	// 83218368: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321836C: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218378 size=12
    let mut pc: u32 = 0x83218378;
    'dispatch: loop {
        match pc {
            0x83218378 => {
    //   block [0x83218378..0x83218384)
	// 83218378: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321837C: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218388 size=12
    let mut pc: u32 = 0x83218388;
    'dispatch: loop {
        match pc {
            0x83218388 => {
    //   block [0x83218388..0x83218394)
	// 83218388: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321838C: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218398 size=8
    let mut pc: u32 = 0x83218398;
    'dispatch: loop {
        match pc {
            0x83218398 => {
    //   block [0x83218398..0x832183A0)
	// 83218398: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8321839C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832183A0 size=8
    let mut pc: u32 = 0x832183A0;
    'dispatch: loop {
        match pc {
            0x832183A0 => {
    //   block [0x832183A0..0x832183A8)
	// 832183A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832183A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832183A8 size=12
    let mut pc: u32 = 0x832183A8;
    'dispatch: loop {
        match pc {
            0x832183A8 => {
    //   block [0x832183A8..0x832183B4)
	// 832183A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832183AC: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832183B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832183B8 size=4
    let mut pc: u32 = 0x832183B8;
    'dispatch: loop {
        match pc {
            0x832183B8 => {
    //   block [0x832183B8..0x832183BC)
	// 832183B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832183C0 size=8
    let mut pc: u32 = 0x832183C0;
    'dispatch: loop {
        match pc {
            0x832183C0 => {
    //   block [0x832183C0..0x832183C8)
	// 832183C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832183C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832183C8 size=12
    let mut pc: u32 = 0x832183C8;
    'dispatch: loop {
        match pc {
            0x832183C8 => {
    //   block [0x832183C8..0x832183D4)
	// 832183C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832183CC: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832183D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832183D8 size=8
    let mut pc: u32 = 0x832183D8;
    'dispatch: loop {
        match pc {
            0x832183D8 => {
    //   block [0x832183D8..0x832183E0)
	// 832183D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832183DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832183E0 size=12
    let mut pc: u32 = 0x832183E0;
    'dispatch: loop {
        match pc {
            0x832183E0 => {
    //   block [0x832183E0..0x832183EC)
	// 832183E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832183E4: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832183E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832183F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832183F0 size=12
    let mut pc: u32 = 0x832183F0;
    'dispatch: loop {
        match pc {
            0x832183F0 => {
    //   block [0x832183F0..0x832183FC)
	// 832183F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832183F4: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832183F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218400 size=8
    let mut pc: u32 = 0x83218400;
    'dispatch: loop {
        match pc {
            0x83218400 => {
    //   block [0x83218400..0x83218408)
	// 83218400: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83218404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218408 size=8
    let mut pc: u32 = 0x83218408;
    'dispatch: loop {
        match pc {
            0x83218408 => {
    //   block [0x83218408..0x83218410)
	// 83218408: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8321840C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218410 size=8
    let mut pc: u32 = 0x83218410;
    'dispatch: loop {
        match pc {
            0x83218410 => {
    //   block [0x83218410..0x83218418)
	// 83218410: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83218414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218418 size=8
    let mut pc: u32 = 0x83218418;
    'dispatch: loop {
        match pc {
            0x83218418 => {
    //   block [0x83218418..0x83218420)
	// 83218418: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8321841C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218420 size=12
    let mut pc: u32 = 0x83218420;
    'dispatch: loop {
        match pc {
            0x83218420 => {
    //   block [0x83218420..0x8321842C)
	// 83218420: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218424: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218430 size=8
    let mut pc: u32 = 0x83218430;
    'dispatch: loop {
        match pc {
            0x83218430 => {
    //   block [0x83218430..0x83218438)
	// 83218430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83218434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218438 size=8
    let mut pc: u32 = 0x83218438;
    'dispatch: loop {
        match pc {
            0x83218438 => {
    //   block [0x83218438..0x83218440)
	// 83218438: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8321843C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218440 size=12
    let mut pc: u32 = 0x83218440;
    'dispatch: loop {
        match pc {
            0x83218440 => {
    //   block [0x83218440..0x8321844C)
	// 83218440: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218444: C02B9484  lfs f1, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218450 size=8
    let mut pc: u32 = 0x83218450;
    'dispatch: loop {
        match pc {
            0x83218450 => {
    //   block [0x83218450..0x83218458)
	// 83218450: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83218454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218458 size=8
    let mut pc: u32 = 0x83218458;
    'dispatch: loop {
        match pc {
            0x83218458 => {
    //   block [0x83218458..0x83218460)
	// 83218458: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8321845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218460 size=8
    let mut pc: u32 = 0x83218460;
    'dispatch: loop {
        match pc {
            0x83218460 => {
    //   block [0x83218460..0x83218468)
	// 83218460: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83218464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83218468 size=4
    let mut pc: u32 = 0x83218468;
    'dispatch: loop {
        match pc {
            0x83218468 => {
    //   block [0x83218468..0x8321846C)
	// 83218468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218470 size=12
    let mut pc: u32 = 0x83218470;
    'dispatch: loop {
        match pc {
            0x83218470 => {
    //   block [0x83218470..0x8321847C)
	// 83218470: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218474: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218480 size=20
    let mut pc: u32 = 0x83218480;
    'dispatch: loop {
        match pc {
            0x83218480 => {
    //   block [0x83218480..0x83218494)
	// 83218480: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218484: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218488: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8321848C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83218490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218498 size=20
    let mut pc: u32 = 0x83218498;
    'dispatch: loop {
        match pc {
            0x83218498 => {
    //   block [0x83218498..0x832184AC)
	// 83218498: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321849C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832184A0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832184A4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832184A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832184B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832184B0 size=20
    let mut pc: u32 = 0x832184B0;
    'dispatch: loop {
        match pc {
            0x832184B0 => {
    //   block [0x832184B0..0x832184C4)
	// 832184B0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832184B4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832184B8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832184BC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832184C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832184C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832184C8 size=20
    let mut pc: u32 = 0x832184C8;
    'dispatch: loop {
        match pc {
            0x832184C8 => {
    //   block [0x832184C8..0x832184DC)
	// 832184C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832184CC: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832184D0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832184D4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832184D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832184E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832184E0 size=12
    let mut pc: u32 = 0x832184E0;
    'dispatch: loop {
        match pc {
            0x832184E0 => {
    //   block [0x832184E0..0x832184EC)
	// 832184E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832184E4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832184E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832184F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832184F0 size=84
    let mut pc: u32 = 0x832184F0;
    'dispatch: loop {
        match pc {
            0x832184F0 => {
    //   block [0x832184F0..0x83218544)
	// 832184F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832184F4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832184F8: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 832184FC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83218500: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83218504: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218508: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8321850C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83218548 size=84
    let mut pc: u32 = 0x83218548;
    'dispatch: loop {
        match pc {
            0x83218548 => {
    //   block [0x83218548..0x8321859C)
	// 83218548: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321854C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83218550: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83218554: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83218558: 38E1FFF8  addi r7, r1, -8
	ctx.r[7].s64 = ctx.r[1].s64 + -8;
	// 8321855C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218560: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83218564: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83218568: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321856C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832185A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832185A0 size=28
    let mut pc: u32 = 0x832185A0;
    'dispatch: loop {
        match pc {
            0x832185A0 => {
    //   block [0x832185A0..0x832185BC)
	// 832185A0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832185A4: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 832185A8: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832185AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832185C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832185C0 size=172
    let mut pc: u32 = 0x832185C0;
    'dispatch: loop {
        match pc {
            0x832185C0 => {
    //   block [0x832185C0..0x8321866C)
	// 832185C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832185C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832185C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832185CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832185D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832185D4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832185D8: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 832185DC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832185E0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 832185E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832185E8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 832185EC: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832185F0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 832185F4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832185F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832185FC: 419A000C  beq cr6, 0x83218608
	if ctx.cr[6].eq {
	pc = 0x83218608; continue 'dispatch;
	}
	// 83218600: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83218604: 419A0008  beq cr6, 0x8321860c
	if ctx.cr[6].eq {
	pc = 0x8321860C; continue 'dispatch;
	}
	// 83218608: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 8321860C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83218610: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83218614: 419A0040  beq cr6, 0x83218654
	if ctx.cr[6].eq {
	pc = 0x83218654; continue 'dispatch;
	}
	// 83218618: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321861C: 409A0008  bne cr6, 0x83218624
	if !ctx.cr[6].eq {
	pc = 0x83218624; continue 'dispatch;
	}
	// 83218620: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 83218624: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218628: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8321862C: 409A0008  bne cr6, 0x83218634
	if !ctx.cr[6].eq {
	pc = 0x83218634; continue 'dispatch;
	}
	// 83218630: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 83218634: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218638: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321863C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218640: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83218644: 4E800421  bctrl
	ctx.lr = 0x83218648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321864C: 4B0E7ADD  bl 0x82300128
	ctx.lr = 0x83218650;
	sub_82300128(ctx, base);
	// 83218650: 4BFFFFA4  b 0x832185f4
	pc = 0x832185F4; continue 'dispatch;
	// 83218654: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321865C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83218660: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83218664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83218668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218670 size=112
    let mut pc: u32 = 0x83218670;
    'dispatch: loop {
        match pc {
            0x83218670 => {
    //   block [0x83218670..0x832186E0)
	// 83218670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83218678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321867C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218680: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 83218684: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 83218688: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321868C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218690: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83218694: E90B0008  ld r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 83218698: F9490000  std r10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8321869C: F9090008  std r8, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 832186A0: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832186A4: 4B809FBD  bl 0x82a22660
	ctx.lr = 0x832186A8;
	sub_82A22660(ctx, base);
	// 832186A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832186AC: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832186B0: 4B809FB1  bl 0x82a22660
	ctx.lr = 0x832186B4;
	sub_82A22660(ctx, base);
	// 832186B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832186B8: C0210058  lfs f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832186BC: 4B809FA5  bl 0x82a22660
	ctx.lr = 0x832186C0;
	sub_82A22660(ctx, base);
	// 832186C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832186C4: C021005C  lfs f1, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832186C8: 4B809F99  bl 0x82a22660
	ctx.lr = 0x832186CC;
	sub_82A22660(ctx, base);
	// 832186CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832186D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832186D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832186D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832186DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832186E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832186E0 size=100
    let mut pc: u32 = 0x832186E0;
    'dispatch: loop {
        match pc {
            0x832186E0 => {
    //   block [0x832186E0..0x83218744)
	// 832186E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832186E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832186E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832186EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832186F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832186F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832186F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832186FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83218700: 4B809791  bl 0x82a21e90
	ctx.lr = 0x83218704;
	sub_82A21E90(ctx, base);
	// 83218704: E9230000  ld r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 83218708: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321870C: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 83218710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218714: F93F0010  std r9, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 83218718: E9030008  ld r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 8321871C: F91F0018  std r8, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u64 ) };
	// 83218720: 419A000C  beq cr6, 0x8321872c
	if ctx.cr[6].eq {
	pc = 0x8321872C; continue 'dispatch;
	}
	// 83218724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218728: 4BFFFE99  bl 0x832185c0
	ctx.lr = 0x8321872C;
	sub_832185C0(ctx, base);
	// 8321872C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83218730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83218734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83218738: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321873C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83218740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83218748 size=188
    let mut pc: u32 = 0x83218748;
    'dispatch: loop {
        match pc {
            0x83218748 => {
    //   block [0x83218748..0x83218804)
	// 83218748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321874C: 4BA90CBD  bl 0x82ca9408
	ctx.lr = 0x83218750;
	sub_82CA93D0(ctx, base);
	// 83218750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218754: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218758: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321875C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218760: 3BCB9F8C  addi r30, r11, -0x6074
	ctx.r[30].s64 = ctx.r[11].s64 + -24692;
	// 83218764: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83218768: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321876C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218770: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 83218774: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218778: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321877C: 4E800421  bctrl
	ctx.lr = 0x83218780;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218780: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83218784: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83218788: 419A0074  beq cr6, 0x832187fc
	if ctx.cr[6].eq {
	pc = 0x832187FC; continue 'dispatch;
	}
	// 8321878C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83218790: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218798: 4B014739  bl 0x8222ced0
	ctx.lr = 0x8321879C;
	sub_8222CED0(ctx, base);
	// 8321879C: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 832187A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832187A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832187A8: 4801CC39  bl 0x832353e0
	ctx.lr = 0x832187AC;
	sub_832353E0(ctx, base);
	// 832187AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832187B0: 4AFFC629  bl 0x82214dd8
	ctx.lr = 0x832187B4;
	sub_82214DD8(ctx, base);
	// 832187B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832187B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832187BC: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 832187C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832187C4: 4E800421  bctrl
	ctx.lr = 0x832187C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832187C8: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 832187CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832187D0: 419A0018  beq cr6, 0x832187e8
	if ctx.cr[6].eq {
	pc = 0x832187E8; continue 'dispatch;
	}
	// 832187D4: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 832187D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832187DC: 419A000C  beq cr6, 0x832187e8
	if ctx.cr[6].eq {
	pc = 0x832187E8; continue 'dispatch;
	}
	// 832187E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832187E4: 4BFFFDDD  bl 0x832185c0
	ctx.lr = 0x832187E8;
	sub_832185C0(ctx, base);
	// 832187E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832187EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832187F0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832187F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832187F8: 4E800421  bctrl
	ctx.lr = 0x832187FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832187FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83218800: 4BA90C58  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83218808 size=112
    let mut pc: u32 = 0x83218808;
    'dispatch: loop {
        match pc {
            0x83218808 => {
    //   block [0x83218808..0x83218878)
	// 83218808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321880C: 4BA90C01  bl 0x82ca940c
	ctx.lr = 0x83218810;
	sub_82CA93D0(ctx, base);
	// 83218810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218814: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83218818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321881C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218820: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218824: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218828: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321882C: 4E800421  bctrl
	ctx.lr = 0x83218830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218830: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218834: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321883C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218840: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83218844: 4E800421  bctrl
	ctx.lr = 0x83218848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218848: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321884C: 409A0024  bne cr6, 0x83218870
	if !ctx.cr[6].eq {
	pc = 0x83218870; continue 'dispatch;
	}
	// 83218850: E97E0010  ld r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	// 83218854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218858: 395E0010  addi r10, r30, 0x10
	ctx.r[10].s64 = ctx.r[30].s64 + 16;
	// 8321885C: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 83218860: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83218864: E91E0018  ld r8, 0x18(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	// 83218868: F91F0018  std r8, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u64 ) };
	// 8321886C: 4BFFFD55  bl 0x832185c0
	ctx.lr = 0x83218870;
	sub_832185C0(ctx, base);
	// 83218870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218874: 4BA90BE8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218878 size=84
    let mut pc: u32 = 0x83218878;
    'dispatch: loop {
        match pc {
            0x83218878 => {
    //   block [0x83218878..0x832188CC)
	// 83218878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321887C: 4BA90B91  bl 0x82ca940c
	ctx.lr = 0x83218880;
	sub_82CA93D0(ctx, base);
	// 83218880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83218888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321888C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83218890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218894: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 83218898: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8321889C: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832188A0: 4B809DC1  bl 0x82a22660
	ctx.lr = 0x832188A4;
	sub_82A22660(ctx, base);
	// 832188A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832188A8: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832188AC: 4B809DB5  bl 0x82a22660
	ctx.lr = 0x832188B0;
	sub_82A22660(ctx, base);
	// 832188B0: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 832188B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832188B8: 419A000C  beq cr6, 0x832188c4
	if ctx.cr[6].eq {
	pc = 0x832188C4; continue 'dispatch;
	}
	// 832188BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832188C0: 4BFFFD01  bl 0x832185c0
	ctx.lr = 0x832188C4;
	sub_832185C0(ctx, base);
	// 832188C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832188C8: 4BA90B94  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832188D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832188D0 size=96
    let mut pc: u32 = 0x832188D0;
    'dispatch: loop {
        match pc {
            0x832188D0 => {
    //   block [0x832188D0..0x83218930)
	// 832188D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832188D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832188D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832188DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832188E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832188E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832188E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832188EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 832188F0: 4B808EA9  bl 0x82a21798
	ctx.lr = 0x832188F4;
	sub_82A21798(ctx, base);
	// 832188F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832188F8: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 832188FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218900: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83218904: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218908: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321890C: 419A000C  beq cr6, 0x83218918
	if ctx.cr[6].eq {
	pc = 0x83218918; continue 'dispatch;
	}
	// 83218910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218914: 4BFFFCAD  bl 0x832185c0
	ctx.lr = 0x83218918;
	sub_832185C0(ctx, base);
	// 83218918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83218920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83218924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83218928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321892C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83218930 size=156
    let mut pc: u32 = 0x83218930;
    'dispatch: loop {
        match pc {
            0x83218930 => {
    //   block [0x83218930..0x832189CC)
	// 83218930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218934: 4BA90AD5  bl 0x82ca9408
	ctx.lr = 0x83218938;
	sub_82CA93D0(ctx, base);
	// 83218938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321893C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218940: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218944: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218948: 3BCB9F64  addi r30, r11, -0x609c
	ctx.r[30].s64 = ctx.r[11].s64 + -24732;
	// 8321894C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83218950: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218958: 389EFFE0  addi r4, r30, -0x20
	ctx.r[4].s64 = ctx.r[30].s64 + -32;
	// 8321895C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218960: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83218964: 4E800421  bctrl
	ctx.lr = 0x83218968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218968: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321896C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83218970: 419A0054  beq cr6, 0x832189c4
	if ctx.cr[6].eq {
	pc = 0x832189C4; continue 'dispatch;
	}
	// 83218974: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83218978: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321897C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218980: 4B014551  bl 0x8222ced0
	ctx.lr = 0x83218984;
	sub_8222CED0(ctx, base);
	// 83218984: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 83218988: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321898C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218990: 4801CAD1  bl 0x83235460
	ctx.lr = 0x83218994;
	sub_83235460(ctx, base);
	// 83218994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218998: 4AFFC441  bl 0x82214dd8
	ctx.lr = 0x8321899C;
	sub_82214DD8(ctx, base);
	// 8321899C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832189A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832189A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832189A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832189AC: 4E800421  bctrl
	ctx.lr = 0x832189B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832189B0: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 832189B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832189B8: 419A000C  beq cr6, 0x832189c4
	if ctx.cr[6].eq {
	pc = 0x832189C4; continue 'dispatch;
	}
	// 832189BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832189C0: 4BFFFC01  bl 0x832185c0
	ctx.lr = 0x832189C4;
	sub_832185C0(ctx, base);
	// 832189C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832189C8: 4BA90A90  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832189D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832189D0 size=104
    let mut pc: u32 = 0x832189D0;
    'dispatch: loop {
        match pc {
            0x832189D0 => {
    //   block [0x832189D0..0x83218A38)
	// 832189D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832189D4: 4BA90A39  bl 0x82ca940c
	ctx.lr = 0x832189D8;
	sub_82CA93D0(ctx, base);
	// 832189D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832189DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832189E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832189E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832189E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832189EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832189F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832189F4: 4E800421  bctrl
	ctx.lr = 0x832189F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832189F8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832189FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218A04: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218A08: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83218A0C: 4E800421  bctrl
	ctx.lr = 0x83218A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218A10: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83218A14: 409A001C  bne cr6, 0x83218a30
	if !ctx.cr[6].eq {
	pc = 0x83218A30; continue 'dispatch;
	}
	// 83218A18: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83218A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218A20: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83218A24: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218A28: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83218A2C: 4BFFFB95  bl 0x832185c0
	ctx.lr = 0x83218A30;
	sub_832185C0(ctx, base);
	// 83218A30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218A34: 4BA90A28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218A38 size=88
    let mut pc: u32 = 0x83218A38;
    'dispatch: loop {
        match pc {
            0x83218A38 => {
    //   block [0x83218A38..0x83218A90)
	// 83218A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218A3C: 4BA909D1  bl 0x82ca940c
	ctx.lr = 0x83218A40;
	sub_82CA93D0(ctx, base);
	// 83218A40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83218A48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83218A4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83218A50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218A54: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218A58: 4B809C09  bl 0x82a22660
	ctx.lr = 0x83218A5C;
	sub_82A22660(ctx, base);
	// 83218A5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218A60: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218A64: 4B809BFD  bl 0x82a22660
	ctx.lr = 0x83218A68;
	sub_82A22660(ctx, base);
	// 83218A68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218A6C: C03F0018  lfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218A70: 4B809BF1  bl 0x82a22660
	ctx.lr = 0x83218A74;
	sub_82A22660(ctx, base);
	// 83218A74: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83218A78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218A7C: 419A000C  beq cr6, 0x83218a88
	if ctx.cr[6].eq {
	pc = 0x83218A88; continue 'dispatch;
	}
	// 83218A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218A84: 4BFFFB3D  bl 0x832185c0
	ctx.lr = 0x83218A88;
	sub_832185C0(ctx, base);
	// 83218A88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218A8C: 4BA909D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218A90 size=532
    let mut pc: u32 = 0x83218A90;
    'dispatch: loop {
        match pc {
            0x83218A90 => {
    //   block [0x83218A90..0x83218CA4)
	// 83218A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218A94: 4BA90979  bl 0x82ca940c
	ctx.lr = 0x83218A98;
	sub_82CA93D0(ctx, base);
	// 83218A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218A9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218AA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83218AA4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83218AA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218AAC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83218AB0: 4198004C  blt cr6, 0x83218afc
	if ctx.cr[6].lt {
	pc = 0x83218AFC; continue 'dispatch;
	}
	// 83218AB4: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83218AB8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218ABC: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83218AC0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218AC4: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83218AC8: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83218ACC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218AD0: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83218AD4: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83218AD8: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83218ADC: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83218AE0: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 83218AE4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83218AE8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83218AEC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83218AF0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83218AF4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83218AF8: 48000014  b 0x83218b0c
	pc = 0x83218B0C; continue 'dispatch;
	// 83218AFC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83218B00: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83218B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218B08: 4B8083E1  bl 0x82a20ee8
	ctx.lr = 0x83218B0C;
	sub_82A20EE8(ctx, base);
	// 83218B0C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83218B10: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218B14: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83218B18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218B1C: 419A0024  beq cr6, 0x83218b40
	if ctx.cr[6].eq {
	pc = 0x83218B40; continue 'dispatch;
	}
	// 83218B20: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218B24: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83218B28: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 83218B2C: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83218B30: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 83218B34: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 83218B38: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83218B3C: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 83218B40: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218B44: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83218B48: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218B4C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83218B50: 4198004C  blt cr6, 0x83218b9c
	if ctx.cr[6].lt {
	pc = 0x83218B9C; continue 'dispatch;
	}
	// 83218B54: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83218B58: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218B5C: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 83218B60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218B64: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83218B68: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83218B6C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218B70: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83218B74: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83218B78: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83218B7C: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83218B80: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 83218B84: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83218B88: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83218B8C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83218B90: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83218B94: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83218B98: 48000014  b 0x83218bac
	pc = 0x83218BAC; continue 'dispatch;
	// 83218B9C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83218BA0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83218BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218BA8: 4B808341  bl 0x82a20ee8
	ctx.lr = 0x83218BAC;
	sub_82A20EE8(ctx, base);
	// 83218BAC: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83218BB0: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218BB4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83218BB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218BBC: 419A0024  beq cr6, 0x83218be0
	if ctx.cr[6].eq {
	pc = 0x83218BE0; continue 'dispatch;
	}
	// 83218BC0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218BC4: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83218BC8: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 83218BCC: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83218BD0: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 83218BD4: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 83218BD8: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83218BDC: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 83218BE0: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218BE4: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83218BE8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218BEC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83218BF0: 4198004C  blt cr6, 0x83218c3c
	if ctx.cr[6].lt {
	pc = 0x83218C3C; continue 'dispatch;
	}
	// 83218BF4: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83218BF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218BFC: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 83218C00: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218C04: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83218C08: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83218C0C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218C10: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83218C14: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83218C18: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83218C1C: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83218C20: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 83218C24: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83218C28: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83218C2C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83218C30: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83218C34: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83218C38: 48000014  b 0x83218c4c
	pc = 0x83218C4C; continue 'dispatch;
	// 83218C3C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83218C40: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 83218C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218C48: 4B8082A1  bl 0x82a20ee8
	ctx.lr = 0x83218C4C;
	sub_82A20EE8(ctx, base);
	// 83218C4C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83218C50: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218C54: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83218C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218C5C: 419A0024  beq cr6, 0x83218c80
	if ctx.cr[6].eq {
	pc = 0x83218C80; continue 'dispatch;
	}
	// 83218C60: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218C64: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83218C68: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 83218C6C: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83218C70: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 83218C74: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 83218C78: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83218C7C: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 83218C80: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83218C84: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218C88: D01E0018  stfs f0, 0x18(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 83218C8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218C90: 419A000C  beq cr6, 0x83218c9c
	if ctx.cr[6].eq {
	pc = 0x83218C9C; continue 'dispatch;
	}
	// 83218C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218C98: 4BFFF929  bl 0x832185c0
	ctx.lr = 0x83218C9C;
	sub_832185C0(ctx, base);
	// 83218C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83218CA0: 4BA907BC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83218CA8 size=292
    let mut pc: u32 = 0x83218CA8;
    'dispatch: loop {
        match pc {
            0x83218CA8 => {
    //   block [0x83218CA8..0x83218DCC)
	// 83218CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218CAC: 4BA90759  bl 0x82ca9404
	ctx.lr = 0x83218CB0;
	sub_82CA93D0(ctx, base);
	// 83218CB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218CB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218CB8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218CBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218CC0: 3BCB9F38  addi r30, r11, -0x60c8
	ctx.r[30].s64 = ctx.r[11].s64 + -24776;
	// 83218CC4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83218CC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218CD0: 389EFFC0  addi r4, r30, -0x40
	ctx.r[4].s64 = ctx.r[30].s64 + -64;
	// 83218CD4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218CD8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83218CDC: 4E800421  bctrl
	ctx.lr = 0x83218CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218CE0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83218CE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83218CE8: 419A00DC  beq cr6, 0x83218dc4
	if ctx.cr[6].eq {
	pc = 0x83218DC4; continue 'dispatch;
	}
	// 83218CEC: 389EFFDC  addi r4, r30, -0x24
	ctx.r[4].s64 = ctx.r[30].s64 + -36;
	// 83218CF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218CF8: 4B0141D9  bl 0x8222ced0
	ctx.lr = 0x83218CFC;
	sub_8222CED0(ctx, base);
	// 83218CFC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83218D00: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 83218D04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83218D08: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218D0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218D10: 419A0008  beq cr6, 0x83218d18
	if ctx.cr[6].eq {
	pc = 0x83218D18; continue 'dispatch;
	}
	// 83218D14: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218D18: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 83218D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218D20: 4B267B31  bl 0x82480850
	ctx.lr = 0x83218D24;
	sub_82480850(ctx, base);
	// 83218D24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218D28: 4AFFC0B1  bl 0x82214dd8
	ctx.lr = 0x83218D2C;
	sub_82214DD8(ctx, base);
	// 83218D2C: 389EFFF4  addi r4, r30, -0xc
	ctx.r[4].s64 = ctx.r[30].s64 + -12;
	// 83218D30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218D34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218D38: 4B014199  bl 0x8222ced0
	ctx.lr = 0x83218D3C;
	sub_8222CED0(ctx, base);
	// 83218D3C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218D40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83218D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218D48: 419A0008  beq cr6, 0x83218d50
	if ctx.cr[6].eq {
	pc = 0x83218D50; continue 'dispatch;
	}
	// 83218D4C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218D50: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 83218D54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218D58: 4B267AF9  bl 0x82480850
	ctx.lr = 0x83218D5C;
	sub_82480850(ctx, base);
	// 83218D5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218D60: 4AFFC079  bl 0x82214dd8
	ctx.lr = 0x83218D64;
	sub_82214DD8(ctx, base);
	// 83218D64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83218D68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218D6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218D70: 4B014161  bl 0x8222ced0
	ctx.lr = 0x83218D74;
	sub_8222CED0(ctx, base);
	// 83218D74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218D78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83218D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218D80: 419A0008  beq cr6, 0x83218d88
	if ctx.cr[6].eq {
	pc = 0x83218D88; continue 'dispatch;
	}
	// 83218D84: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218D88: 38BD0018  addi r5, r29, 0x18
	ctx.r[5].s64 = ctx.r[29].s64 + 24;
	// 83218D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218D90: 4B267AC1  bl 0x82480850
	ctx.lr = 0x83218D94;
	sub_82480850(ctx, base);
	// 83218D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218D98: 4AFFC041  bl 0x82214dd8
	ctx.lr = 0x83218D9C;
	sub_82214DD8(ctx, base);
	// 83218D9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218DA4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218DA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83218DAC: 4E800421  bctrl
	ctx.lr = 0x83218DB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218DB0: 5769063E  clrlwi r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 83218DB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83218DB8: 419A000C  beq cr6, 0x83218dc4
	if ctx.cr[6].eq {
	pc = 0x83218DC4; continue 'dispatch;
	}
	// 83218DBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83218DC0: 4BFFF801  bl 0x832185c0
	ctx.lr = 0x83218DC4;
	sub_832185C0(ctx, base);
	// 83218DC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83218DC8: 4BA9068C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218DD0 size=112
    let mut pc: u32 = 0x83218DD0;
    'dispatch: loop {
        match pc {
            0x83218DD0 => {
    //   block [0x83218DD0..0x83218E40)
	// 83218DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218DD4: 4BA90639  bl 0x82ca940c
	ctx.lr = 0x83218DD8;
	sub_82CA93D0(ctx, base);
	// 83218DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218DDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83218DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83218DE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218DE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218DEC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218DF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83218DF4: 4E800421  bctrl
	ctx.lr = 0x83218DF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218DF8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218DFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218E04: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218E08: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83218E0C: 4E800421  bctrl
	ctx.lr = 0x83218E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218E10: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83218E14: 409A0024  bne cr6, 0x83218e38
	if !ctx.cr[6].eq {
	pc = 0x83218E38; continue 'dispatch;
	}
	// 83218E18: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218E20: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83218E24: C1BE0014  lfs f13, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83218E28: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83218E2C: C19E0018  lfs f12, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83218E30: D19F0018  stfs f12, 0x18(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 83218E34: 4BFFF78D  bl 0x832185c0
	ctx.lr = 0x83218E38;
	sub_832185C0(ctx, base);
	// 83218E38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218E3C: 4BA90620  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218E40 size=84
    let mut pc: u32 = 0x83218E40;
    'dispatch: loop {
        match pc {
            0x83218E40 => {
    //   block [0x83218E40..0x83218E94)
	// 83218E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83218E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83218E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83218E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83218E58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83218E5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83218E60: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83218E64: 4B8097FD  bl 0x82a22660
	ctx.lr = 0x83218E68;
	sub_82A22660(ctx, base);
	// 83218E68: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 83218E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218E70: 419A000C  beq cr6, 0x83218e7c
	if ctx.cr[6].eq {
	pc = 0x83218E7C; continue 'dispatch;
	}
	// 83218E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218E78: 4BFFF749  bl 0x832185c0
	ctx.lr = 0x83218E7C;
	sub_832185C0(ctx, base);
	// 83218E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83218E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83218E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83218E88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83218E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83218E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83218E98 size=212
    let mut pc: u32 = 0x83218E98;
    'dispatch: loop {
        match pc {
            0x83218E98 => {
    //   block [0x83218E98..0x83218F6C)
	// 83218E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218E9C: 4BA90571  bl 0x82ca940c
	ctx.lr = 0x83218EA0;
	sub_82CA93D0(ctx, base);
	// 83218EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218EA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218EA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83218EAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83218EB0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83218EB4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83218EB8: 4198004C  blt cr6, 0x83218f04
	if ctx.cr[6].lt {
	pc = 0x83218F04; continue 'dispatch;
	}
	// 83218EBC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83218EC0: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83218EC4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218EC8: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 83218ECC: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 83218ED0: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 83218ED4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218ED8: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83218EDC: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83218EE0: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 83218EE4: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83218EE8: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 83218EEC: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83218EF0: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 83218EF4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83218EF8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83218EFC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83218F00: 48000014  b 0x83218f14
	pc = 0x83218F14; continue 'dispatch;
	// 83218F04: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83218F08: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83218F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218F10: 4B807FD9  bl 0x82a20ee8
	ctx.lr = 0x83218F14;
	sub_82A20EE8(ctx, base);
	// 83218F14: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83218F18: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218F1C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83218F20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218F24: 419A0024  beq cr6, 0x83218f48
	if ctx.cr[6].eq {
	pc = 0x83218F48; continue 'dispatch;
	}
	// 83218F28: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218F2C: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83218F30: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 83218F34: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83218F38: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 83218F3C: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 83218F40: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83218F44: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 83218F48: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83218F4C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83218F50: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83218F54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218F58: 419A000C  beq cr6, 0x83218f64
	if ctx.cr[6].eq {
	pc = 0x83218F64; continue 'dispatch;
	}
	// 83218F5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83218F60: 4BFFF661  bl 0x832185c0
	ctx.lr = 0x83218F64;
	sub_832185C0(ctx, base);
	// 83218F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83218F68: 4BA904F4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83218F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83218F70 size=180
    let mut pc: u32 = 0x83218F70;
    'dispatch: loop {
        match pc {
            0x83218F70 => {
    //   block [0x83218F70..0x83219024)
	// 83218F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83218F74: 4BA90495  bl 0x82ca9408
	ctx.lr = 0x83218F78;
	sub_82CA93D0(ctx, base);
	// 83218F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83218F7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83218F80: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83218F84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83218F88: 3BCB9EE8  addi r30, r11, -0x6118
	ctx.r[30].s64 = ctx.r[11].s64 + -24856;
	// 83218F8C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83218F90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218F98: 389EFFE0  addi r4, r30, -0x20
	ctx.r[4].s64 = ctx.r[30].s64 + -32;
	// 83218F9C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83218FA0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83218FA4: 4E800421  bctrl
	ctx.lr = 0x83218FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83218FA8: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83218FAC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83218FB0: 419A006C  beq cr6, 0x8321901c
	if ctx.cr[6].eq {
	pc = 0x8321901C; continue 'dispatch;
	}
	// 83218FB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83218FB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83218FBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218FC0: 4B013F11  bl 0x8222ced0
	ctx.lr = 0x83218FC4;
	sub_8222CED0(ctx, base);
	// 83218FC4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83218FC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83218FCC: 409A0010  bne cr6, 0x83218fdc
	if !ctx.cr[6].eq {
	pc = 0x83218FDC; continue 'dispatch;
	}
	// 83218FD0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83218FD4: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 83218FD8: 48000008  b 0x83218fe0
	pc = 0x83218FE0; continue 'dispatch;
	// 83218FDC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218FE0: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 83218FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218FE8: 4B267869  bl 0x82480850
	ctx.lr = 0x83218FEC;
	sub_82480850(ctx, base);
	// 83218FEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83218FF0: 4AFFBDE9  bl 0x82214dd8
	ctx.lr = 0x83218FF4;
	sub_82214DD8(ctx, base);
	// 83218FF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83218FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83218FFC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219000: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219004: 4E800421  bctrl
	ctx.lr = 0x83219008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219008: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321900C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83219010: 419A000C  beq cr6, 0x8321901c
	if ctx.cr[6].eq {
	pc = 0x8321901C; continue 'dispatch;
	}
	// 83219014: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83219018: 4BFFF5A9  bl 0x832185c0
	ctx.lr = 0x8321901C;
	sub_832185C0(ctx, base);
	// 8321901C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83219020: 4BA90438  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219028 size=96
    let mut pc: u32 = 0x83219028;
    'dispatch: loop {
        match pc {
            0x83219028 => {
    //   block [0x83219028..0x83219088)
	// 83219028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321902C: 4BA903E1  bl 0x82ca940c
	ctx.lr = 0x83219030;
	sub_82CA93D0(ctx, base);
	// 83219030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219034: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83219038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321903C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219040: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219044: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219048: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321904C: 4E800421  bctrl
	ctx.lr = 0x83219050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219050: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219054: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321905C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219060: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83219064: 4E800421  bctrl
	ctx.lr = 0x83219068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219068: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321906C: 409A0014  bne cr6, 0x83219080
	if !ctx.cr[6].eq {
	pc = 0x83219080; continue 'dispatch;
	}
	// 83219070: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83219074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219078: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321907C: 4BFFF545  bl 0x832185c0
	ctx.lr = 0x83219080;
	sub_832185C0(ctx, base);
	// 83219080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83219084: 4BA903D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219088 size=188
    let mut pc: u32 = 0x83219088;
    'dispatch: loop {
        match pc {
            0x83219088 => {
    //   block [0x83219088..0x83219144)
	// 83219088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321908C: 4BA90381  bl 0x82ca940c
	ctx.lr = 0x83219090;
	sub_82CA93D0(ctx, base);
	// 83219090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321909C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832190A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832190A4: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832190A8: 4B8095B9  bl 0x82a22660
	ctx.lr = 0x832190AC;
	sub_82A22660(ctx, base);
	// 832190AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832190B0: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832190B4: 4B8095AD  bl 0x82a22660
	ctx.lr = 0x832190B8;
	sub_82A22660(ctx, base);
	// 832190B8: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832190BC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832190C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832190C4: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 832190C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832190CC: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 832190D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832190D4: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 832190D8: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 832190DC: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 832190E0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 832190E4: 4E800421  bctrl
	ctx.lr = 0x832190E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832190E8: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 832190EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219148 size=588
    let mut pc: u32 = 0x83219148;
    'dispatch: loop {
        match pc {
            0x83219148 => {
    //   block [0x83219148..0x83219394)
	// 83219148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321914C: 4BA902C1  bl 0x82ca940c
	ctx.lr = 0x83219150;
	sub_82CA93D0(ctx, base);
	// 83219150: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219154: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219158: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321915C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83219160: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219164: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83219168: 4198004C  blt cr6, 0x832191b4
	if ctx.cr[6].lt {
	pc = 0x832191B4; continue 'dispatch;
	}
	// 8321916C: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83219170: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219174: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 83219178: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321917C: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83219180: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83219184: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219188: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321918C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219190: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83219194: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219198: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321919C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 832191A0: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 832191A4: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 832191A8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 832191AC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832191B0: 48000014  b 0x832191c4
	pc = 0x832191C4; continue 'dispatch;
	// 832191B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832191B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 832191BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832191C0: 4B807D29  bl 0x82a20ee8
	ctx.lr = 0x832191C4;
	sub_82A20EE8(ctx, base);
	// 832191C4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832191C8: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832191CC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 832191D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832191D4: 419A0024  beq cr6, 0x832191f8
	if ctx.cr[6].eq {
	pc = 0x832191F8; continue 'dispatch;
	}
	// 832191D8: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 832191DC: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 832191E0: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 832191E4: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 832191E8: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 832191EC: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 832191F0: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 832191F4: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 832191F8: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832191FC: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83219200: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219204: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83219208: 4198004C  blt cr6, 0x83219254
	if ctx.cr[6].lt {
	pc = 0x83219254; continue 'dispatch;
	}
	// 8321920C: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83219210: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219214: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 83219218: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321921C: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83219220: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83219224: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219228: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321922C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219230: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83219234: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219238: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321923C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219240: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83219244: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219248: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321924C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83219250: 48000014  b 0x83219264
	pc = 0x83219264; continue 'dispatch;
	// 83219254: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83219258: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8321925C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219260: 4B807C89  bl 0x82a20ee8
	ctx.lr = 0x83219264;
	sub_82A20EE8(ctx, base);
	// 83219264: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219268: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321926C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 83219270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219274: 419A0024  beq cr6, 0x83219298
	if ctx.cr[6].eq {
	pc = 0x83219298; continue 'dispatch;
	}
	// 83219278: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321927C: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 83219280: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 83219284: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83219288: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8321928C: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 83219290: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 83219294: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 83219298: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321929C: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832192A0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 832192A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 832192A8: 41980034  blt cr6, 0x832192dc
	if ctx.cr[6].lt {
	pc = 0x832192DC; continue 'dispatch;
	}
	// 832192AC: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 832192B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832192B4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832192B8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 832192BC: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 832192C0: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 832192C4: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832192C8: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 832192CC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 832192D0: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832192D4: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 832192D8: 48000014  b 0x832192ec
	pc = 0x832192EC; continue 'dispatch;
	// 832192DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832192E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832192E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832192E8: 4B807C01  bl 0x82a20ee8
	ctx.lr = 0x832192EC;
	sub_82A20EE8(ctx, base);
	// 832192EC: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832192F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832192F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832192F8: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 832192FC: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83219300: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 83219304: 98FE0018  stb r7, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[7].u8 ) };
	// 83219308: 4B808609  bl 0x82a21910
	ctx.lr = 0x8321930C;
	sub_82A21910(ctx, base);
	// 8321930C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219398 size=356
    let mut pc: u32 = 0x83219398;
    'dispatch: loop {
        match pc {
            0x83219398 => {
    //   block [0x83219398..0x832194FC)
	// 83219398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321939C: 4BA90069  bl 0x82ca9404
	ctx.lr = 0x832193A0;
	sub_82CA93D0(ctx, base);
	// 832193A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832193A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832193A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832193AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832193B0: 3BCB9EB4  addi r30, r11, -0x614c
	ctx.r[30].s64 = ctx.r[11].s64 + -24908;
	// 832193B4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 832193B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832193BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832193C0: 389EFF88  addi r4, r30, -0x78
	ctx.r[4].s64 = ctx.r[30].s64 + -120;
	// 832193C4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832193C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 832193CC: 4E800421  bctrl
	ctx.lr = 0x832193D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832193D0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 832193D4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832193D8: 419A011C  beq cr6, 0x832194f4
	if ctx.cr[6].eq {
	pc = 0x832194F4; continue 'dispatch;
	}
	// 832193DC: 389EFFAC  addi r4, r30, -0x54
	ctx.r[4].s64 = ctx.r[30].s64 + -84;
	// 832193E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832193E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832193E8: 4B013AE9  bl 0x8222ced0
	ctx.lr = 0x832193EC;
	sub_8222CED0(ctx, base);
	// 832193EC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832193F0: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 832193F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832193F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832193FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219400: 419A0008  beq cr6, 0x83219408
	if ctx.cr[6].eq {
	pc = 0x83219408; continue 'dispatch;
	}
	// 83219404: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219408: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321940C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219410: 4B267441  bl 0x82480850
	ctx.lr = 0x83219414;
	sub_82480850(ctx, base);
	// 83219414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219418: 4AFFB9C1  bl 0x82214dd8
	ctx.lr = 0x8321941C;
	sub_82214DD8(ctx, base);
	// 8321941C: 389EFFC0  addi r4, r30, -0x40
	ctx.r[4].s64 = ctx.r[30].s64 + -64;
	// 83219420: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219424: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219428: 4B013AA9  bl 0x8222ced0
	ctx.lr = 0x8321942C;
	sub_8222CED0(ctx, base);
	// 8321942C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219430: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83219434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219438: 419A0008  beq cr6, 0x83219440
	if ctx.cr[6].eq {
	pc = 0x83219440; continue 'dispatch;
	}
	// 8321943C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219440: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 83219444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219448: 4B267409  bl 0x82480850
	ctx.lr = 0x8321944C;
	sub_82480850(ctx, base);
	// 8321944C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219450: 4AFFB989  bl 0x82214dd8
	ctx.lr = 0x83219454;
	sub_82214DD8(ctx, base);
	// 83219454: 389EFFDC  addi r4, r30, -0x24
	ctx.r[4].s64 = ctx.r[30].s64 + -36;
	// 83219458: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321945C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219460: 4B013A71  bl 0x8222ced0
	ctx.lr = 0x83219464;
	sub_8222CED0(ctx, base);
	// 83219464: 38BD0018  addi r5, r29, 0x18
	ctx.r[5].s64 = ctx.r[29].s64 + 24;
	// 83219468: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321946C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219470: 4801C191  bl 0x83235600
	ctx.lr = 0x83219474;
	sub_83235600(ctx, base);
	// 83219474: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219478: 4AFFB961  bl 0x82214dd8
	ctx.lr = 0x8321947C;
	sub_82214DD8(ctx, base);
	// 8321947C: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 83219480: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219488: 4B013A49  bl 0x8222ced0
	ctx.lr = 0x8321948C;
	sub_8222CED0(ctx, base);
	// 8321948C: 38BD0020  addi r5, r29, 0x20
	ctx.r[5].s64 = ctx.r[29].s64 + 32;
	// 83219490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83219494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219498: 4801C079  bl 0x83235510
	ctx.lr = 0x8321949C;
	sub_83235510(ctx, base);
	// 8321949C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832194A0: 4AFFB939  bl 0x82214dd8
	ctx.lr = 0x832194A4;
	sub_82214DD8(ctx, base);
	// 832194A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832194A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832194AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832194B0: 4B013A21  bl 0x8222ced0
	ctx.lr = 0x832194B4;
	sub_8222CED0(ctx, base);
	// 832194B4: 38BD0030  addi r5, r29, 0x30
	ctx.r[5].s64 = ctx.r[29].s64 + 48;
	// 832194B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832194BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832194C0: 4801C141  bl 0x83235600
	ctx.lr = 0x832194C4;
	sub_83235600(ctx, base);
	// 832194C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832194C8: 4AFFB911  bl 0x82214dd8
	ctx.lr = 0x832194CC;
	sub_82214DD8(ctx, base);
	// 832194CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832194D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832194D4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832194D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832194DC: 4E800421  bctrl
	ctx.lr = 0x832194E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832194E0: 5769063E  clrlwi r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 832194E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832194E8: 419A000C  beq cr6, 0x832194f4
	if ctx.cr[6].eq {
	pc = 0x832194F4; continue 'dispatch;
	}
	// 832194EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832194F0: 4BFFF0D1  bl 0x832185c0
	ctx.lr = 0x832194F4;
	sub_832185C0(ctx, base);
	// 832194F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832194F8: 4BA8FF5C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219500 size=132
    let mut pc: u32 = 0x83219500;
    'dispatch: loop {
        match pc {
            0x83219500 => {
    //   block [0x83219500..0x83219584)
	// 83219500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219504: 4BA8FF09  bl 0x82ca940c
	ctx.lr = 0x83219508;
	sub_82CA93D0(ctx, base);
	// 83219508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321950C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83219510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219518: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321951C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219520: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219524: 4E800421  bctrl
	ctx.lr = 0x83219528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219528: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321952C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219534: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219538: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321953C: 4E800421  bctrl
	ctx.lr = 0x83219540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219540: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83219544: 409A0038  bne cr6, 0x8321957c
	if !ctx.cr[6].eq {
	pc = 0x8321957C; continue 'dispatch;
	}
	// 83219548: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321954C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 83219550: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83219554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219558: C1BE0014  lfs f13, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321955C: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83219560: 895E0018  lbz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219564: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219588 size=176
    let mut pc: u32 = 0x83219588;
    'dispatch: loop {
        match pc {
            0x83219588 => {
    //   block [0x83219588..0x83219638)
	// 83219588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321958C: 4BA8FE81  bl 0x82ca940c
	ctx.lr = 0x83219590;
	sub_82CA93D0(ctx, base);
	// 83219590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321959C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832195A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832195A4: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832195A8: 4B8090B9  bl 0x82a22660
	ctx.lr = 0x832195AC;
	sub_82A22660(ctx, base);
	// 832195AC: E97F0014  ld r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	// 832195B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832195B4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 832195B8: C0210058  lfs f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832195BC: 4B8090A5  bl 0x82a22660
	ctx.lr = 0x832195C0;
	sub_82A22660(ctx, base);
	// 832195C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832195C4: C021005C  lfs f1, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832195C8: 4B809099  bl 0x82a22660
	ctx.lr = 0x832195CC;
	sub_82A22660(ctx, base);
	// 832195CC: E95F001C  ld r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	// 832195D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832195D4: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 832195D8: C0210058  lfs f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832195DC: 4B809085  bl 0x82a22660
	ctx.lr = 0x832195E0;
	sub_82A22660(ctx, base);
	// 832195E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832195E4: C021005C  lfs f1, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832195E8: 4B809079  bl 0x82a22660
	ctx.lr = 0x832195EC;
	sub_82A22660(ctx, base);
	// 832195EC: 893F0024  lbz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832195F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832195F4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832195F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832195FC: 7D270034  cntlzw r7, r9
	ctx.r[7].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83219600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219604: 54E6DFFE  rlwinm r6, r7, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 83219608: 68CB0001  xori r11, r6, 1
	ctx.r[11].u64 = ctx.r[6].u64 ^ 1;
	// 8321960C: 81480010  lwz r10, 0x10(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 83219610: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 83219614: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219618: 4E800421  bctrl
	ctx.lr = 0x8321961C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321961C: 57A9063E  clrlwi r9, r29, 0x18
	ctx.r[9].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83219620: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83219624: 419A000C  beq cr6, 0x83219630
	if ctx.cr[6].eq {
	pc = 0x83219630; continue 'dispatch;
	}
	// 83219628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321962C: 4BFFEF95  bl 0x832185c0
	ctx.lr = 0x83219630;
	sub_832185C0(ctx, base);
	// 83219630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83219634: 4BA8FE28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219638 size=368
    let mut pc: u32 = 0x83219638;
    'dispatch: loop {
        match pc {
            0x83219638 => {
    //   block [0x83219638..0x832197A8)
	// 83219638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321963C: 4BA8FDD1  bl 0x82ca940c
	ctx.lr = 0x83219640;
	sub_82CA93D0(ctx, base);
	// 83219640: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219644: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219648: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321964C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83219650: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219654: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83219658: 4198004C  blt cr6, 0x832196a4
	if ctx.cr[6].lt {
	pc = 0x832196A4; continue 'dispatch;
	}
	// 8321965C: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83219660: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219664: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 83219668: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321966C: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83219670: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83219674: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219678: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321967C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219680: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83219684: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219688: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321968C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219690: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83219694: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219698: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321969C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832196A0: 48000014  b 0x832196b4
	pc = 0x832196B4; continue 'dispatch;
	// 832196A4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832196A8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832196AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832196B0: 4B807839  bl 0x82a20ee8
	ctx.lr = 0x832196B4;
	sub_82A20EE8(ctx, base);
	// 832196B4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832196B8: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832196BC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 832196C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832196C4: 419A0024  beq cr6, 0x832196e8
	if ctx.cr[6].eq {
	pc = 0x832196E8; continue 'dispatch;
	}
	// 832196C8: 89610058  lbz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 832196CC: 89410059  lbz r10, 0x59(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 832196D0: 8921005B  lbz r9, 0x5b(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 832196D4: 8901005A  lbz r8, 0x5a(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 832196D8: 9961005B  stb r11, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[11].u8 ) };
	// 832196DC: 9941005A  stb r10, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[10].u8 ) };
	// 832196E0: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 832196E4: 99010059  stb r8, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[8].u8 ) };
	// 832196E8: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832196EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832196F0: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832196F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832196F8: 4B8080A1  bl 0x82a21798
	ctx.lr = 0x832196FC;
	sub_82A21798(ctx, base);
	// 832196FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83219700: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83219704: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 83219708: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321970C: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83219710: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219714: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 83219718: 4B808081  bl 0x82a21798
	ctx.lr = 0x8321971C;
	sub_82A21798(ctx, base);
	// 8321971C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219720: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 83219724: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219728: 90FE0020  stw r7, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 8321972C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219730: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83219734: 41980034  blt cr6, 0x83219768
	if ctx.cr[6].lt {
	pc = 0x83219768; continue 'dispatch;
	}
	// 83219738: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321973C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219740: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219744: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 83219748: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321974C: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 83219750: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219754: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219758: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321975C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83219760: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83219764: 48000014  b 0x83219778
	pc = 0x83219778; continue 'dispatch;
	// 83219768: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321976C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83219770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219774: 4B807775  bl 0x82a20ee8
	ctx.lr = 0x83219778;
	sub_82A20EE8(ctx, base);
	// 83219778: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321977C: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83219780: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83219784: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219788: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321978C: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 83219790: 98DE0024  stb r6, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[6].u8 ) };
	// 83219794: 419A000C  beq cr6, 0x832197a0
	if ctx.cr[6].eq {
	pc = 0x832197A0; continue 'dispatch;
	}
	// 83219798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321979C: 4BFFEE25  bl 0x832185c0
	ctx.lr = 0x832197A0;
	sub_832185C0(ctx, base);
	// 832197A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832197A4: 4BA8FCB8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832197A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832197A8 size=300
    let mut pc: u32 = 0x832197A8;
    'dispatch: loop {
        match pc {
            0x832197A8 => {
    //   block [0x832197A8..0x832198D4)
	// 832197A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832197AC: 4BA8FC5D  bl 0x82ca9408
	ctx.lr = 0x832197B0;
	sub_82CA93D0(ctx, base);
	// 832197B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832197B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832197B8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832197BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832197C0: 3BCB9E34  addi r30, r11, -0x61cc
	ctx.r[30].s64 = ctx.r[11].s64 + -25036;
	// 832197C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 832197C8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832197CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832197D0: 389EFFBC  addi r4, r30, -0x44
	ctx.r[4].s64 = ctx.r[30].s64 + -68;
	// 832197D4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832197D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 832197DC: 4E800421  bctrl
	ctx.lr = 0x832197E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832197E0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 832197E4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832197E8: 419A00E4  beq cr6, 0x832198cc
	if ctx.cr[6].eq {
	pc = 0x832198CC; continue 'dispatch;
	}
	// 832197EC: 389EFFD0  addi r4, r30, -0x30
	ctx.r[4].s64 = ctx.r[30].s64 + -48;
	// 832197F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832197F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832197F8: 4B0136D9  bl 0x8222ced0
	ctx.lr = 0x832197FC;
	sub_8222CED0(ctx, base);
	// 832197FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219804: 409A0010  bne cr6, 0x83219814
	if !ctx.cr[6].eq {
	pc = 0x83219814; continue 'dispatch;
	}
	// 83219808: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321980C: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 83219810: 48000008  b 0x83219818
	pc = 0x83219818; continue 'dispatch;
	// 83219814: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219818: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321981C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219820: 4B267031  bl 0x82480850
	ctx.lr = 0x83219824;
	sub_82480850(ctx, base);
	// 83219824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219828: 4AFFB5B1  bl 0x82214dd8
	ctx.lr = 0x8321982C;
	sub_82214DD8(ctx, base);
	// 8321982C: 389EFFE8  addi r4, r30, -0x18
	ctx.r[4].s64 = ctx.r[30].s64 + -24;
	// 83219830: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219838: 4B013699  bl 0x8222ced0
	ctx.lr = 0x8321983C;
	sub_8222CED0(ctx, base);
	// 8321983C: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 83219840: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83219844: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219848: 4801BC19  bl 0x83235460
	ctx.lr = 0x8321984C;
	sub_83235460(ctx, base);
	// 8321984C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219850: 4AFFB589  bl 0x82214dd8
	ctx.lr = 0x83219854;
	sub_82214DD8(ctx, base);
	// 83219854: 389EFFF4  addi r4, r30, -0xc
	ctx.r[4].s64 = ctx.r[30].s64 + -12;
	// 83219858: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321985C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219860: 4B013671  bl 0x8222ced0
	ctx.lr = 0x83219864;
	sub_8222CED0(ctx, base);
	// 83219864: 38BD001C  addi r5, r29, 0x1c
	ctx.r[5].s64 = ctx.r[29].s64 + 28;
	// 83219868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321986C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219870: 4801BBF1  bl 0x83235460
	ctx.lr = 0x83219874;
	sub_83235460(ctx, base);
	// 83219874: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219878: 4AFFB561  bl 0x82214dd8
	ctx.lr = 0x8321987C;
	sub_82214DD8(ctx, base);
	// 8321987C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83219880: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219888: 4B013649  bl 0x8222ced0
	ctx.lr = 0x8321988C;
	sub_8222CED0(ctx, base);
	// 8321988C: 38BD0024  addi r5, r29, 0x24
	ctx.r[5].s64 = ctx.r[29].s64 + 36;
	// 83219890: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83219894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219898: 4801BD69  bl 0x83235600
	ctx.lr = 0x8321989C;
	sub_83235600(ctx, base);
	// 8321989C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832198A0: 4AFFB539  bl 0x82214dd8
	ctx.lr = 0x832198A4;
	sub_82214DD8(ctx, base);
	// 832198A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832198A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832198AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832198B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832198B4: 4E800421  bctrl
	ctx.lr = 0x832198B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832198B8: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 832198BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832198C0: 419A000C  beq cr6, 0x832198cc
	if ctx.cr[6].eq {
	pc = 0x832198CC; continue 'dispatch;
	}
	// 832198C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832198C8: 4BFFECF9  bl 0x832185c0
	ctx.lr = 0x832198CC;
	sub_832185C0(ctx, base);
	// 832198CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832198D0: 4BA8FB88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832198D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832198D8 size=136
    let mut pc: u32 = 0x832198D8;
    'dispatch: loop {
        match pc {
            0x832198D8 => {
    //   block [0x832198D8..0x83219960)
	// 832198D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832198DC: 4BA8FB31  bl 0x82ca940c
	ctx.lr = 0x832198E0;
	sub_82CA93D0(ctx, base);
	// 832198E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832198E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832198E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832198EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832198F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832198F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832198F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832198FC: 4E800421  bctrl
	ctx.lr = 0x83219900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219900: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219904: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321990C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219910: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83219914: 4E800421  bctrl
	ctx.lr = 0x83219918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219918: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321991C: 409A003C  bne cr6, 0x83219958
	if !ctx.cr[6].eq {
	pc = 0x83219958; continue 'dispatch;
	}
	// 83219920: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83219924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219928: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321992C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219930: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83219934: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219938: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8321993C: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83219940: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83219944: 811E0020  lwz r8, 0x20(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83219948: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8321994C: 88FE0024  lbz r7, 0x24(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83219950: 98FF0024  stb r7, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 83219954: 4BFFEC6D  bl 0x832185c0
	ctx.lr = 0x83219958;
	sub_832185C0(ctx, base);
	// 83219958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321995C: 4BA8FB00  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219960 size=104
    let mut pc: u32 = 0x83219960;
    'dispatch: loop {
        match pc {
            0x83219960 => {
    //   block [0x83219960..0x832199C8)
	// 83219960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83219968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321996C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219970: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83219974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219978: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321997C: 388B9DD4  addi r4, r11, -0x622c
	ctx.r[4].s64 = ctx.r[11].s64 + -25132;
	// 83219980: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83219984: 4B01354D  bl 0x8222ced0
	ctx.lr = 0x83219988;
	sub_8222CED0(ctx, base);
	// 83219988: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321998C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219990: 4AFD51A9  bl 0x821eeb38
	ctx.lr = 0x83219994;
	sub_821EEB38(ctx, base);
	// 83219994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219998: 4B9E9E59  bl 0x82c037f0
	ctx.lr = 0x8321999C;
	sub_82C037F0(ctx, base);
	// 8321999C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 832199A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832199A4: 4AFFB435  bl 0x82214dd8
	ctx.lr = 0x832199A8;
	sub_82214DD8(ctx, base);
	// 832199A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832199AC: 4AFFB42D  bl 0x82214dd8
	ctx.lr = 0x832199B0;
	sub_82214DD8(ctx, base);
	// 832199B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832199B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832199B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832199BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832199C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832199C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832199C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832199C8 size=512
    let mut pc: u32 = 0x832199C8;
    'dispatch: loop {
        match pc {
            0x832199C8 => {
    //   block [0x832199C8..0x83219BC8)
	// 832199C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832199CC: 4BA8FA3D  bl 0x82ca9408
	ctx.lr = 0x832199D0;
	sub_82CA93D0(ctx, base);
	// 832199D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832199D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832199D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832199DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832199E0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 832199E4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 832199E8: 4198004C  blt cr6, 0x83219a34
	if ctx.cr[6].lt {
	pc = 0x83219A34; continue 'dispatch;
	}
	// 832199EC: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 832199F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832199F4: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 832199F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832199FC: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83219A00: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83219A04: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219A08: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83219A0C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219A10: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83219A14: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219A18: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 83219A1C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219A20: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83219A24: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219A28: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83219A2C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83219A30: 48000014  b 0x83219a44
	pc = 0x83219A44; continue 'dispatch;
	// 83219A34: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83219A38: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 83219A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219A40: 4B8074A9  bl 0x82a20ee8
	ctx.lr = 0x83219A44;
	sub_82A20EE8(ctx, base);
	// 83219A44: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83219A48: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219A4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83219A54: 419A0024  beq cr6, 0x83219a78
	if ctx.cr[6].eq {
	pc = 0x83219A78; continue 'dispatch;
	}
	// 83219A58: 89610055  lbz r11, 0x55(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 83219A5C: 89410056  lbz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83219A60: 8921005F  lbz r9, 0x5f(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(95 as u32) ) } as u64;
	// 83219A64: 8901005C  lbz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83219A68: 99610056  stb r11, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u8 ) };
	// 83219A6C: 99410055  stb r10, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[10].u8 ) };
	// 83219A70: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 83219A74: 99010057  stb r8, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[8].u8 ) };
	// 83219A78: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219A7C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83219A80: 4198004C  blt cr6, 0x83219acc
	if ctx.cr[6].lt {
	pc = 0x83219ACC; continue 'dispatch;
	}
	// 83219A84: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 83219A88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219A8C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 83219A90: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219A94: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 83219A98: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83219A9C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219AA0: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83219AA4: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219AA8: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 83219AAC: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219AB0: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 83219AB4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219AB8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83219ABC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219AC0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83219AC4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83219AC8: 48000014  b 0x83219adc
	pc = 0x83219ADC; continue 'dispatch;
	// 83219ACC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83219AD0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83219AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219AD8: 4B807411  bl 0x82a20ee8
	ctx.lr = 0x83219ADC;
	sub_82A20EE8(ctx, base);
	// 83219ADC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83219AE0: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219AE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219AE8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83219AEC: 419A0024  beq cr6, 0x83219b10
	if ctx.cr[6].eq {
	pc = 0x83219B10; continue 'dispatch;
	}
	// 83219AF0: 89610059  lbz r11, 0x59(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 83219AF4: 8941005A  lbz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 83219AF8: 89210063  lbz r9, 0x63(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(99 as u32) ) } as u64;
	// 83219AFC: 89010060  lbz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83219B00: 9961005A  stb r11, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 83219B04: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 83219B08: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 83219B0C: 9901005B  stb r8, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[8].u8 ) };
	// 83219B10: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83219B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219B18: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83219B1C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219B20: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 83219B24: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83219B28: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83219B2C: 4E800421  bctrl
	ctx.lr = 0x83219B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219B30: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83219B34: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219B38: 7D634214  add r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[8].u64;
	// 83219B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219B40: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 83219B44: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 83219B48: 4E800421  bctrl
	ctx.lr = 0x83219B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219B4C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219B50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83219B54: 41980034  blt cr6, 0x83219b88
	if ctx.cr[6].lt {
	pc = 0x83219B88; continue 'dispatch;
	}
	// 83219B58: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 83219B5C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219B60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219B64: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 83219B68: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 83219B6C: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 83219B70: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219B74: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83219B78: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83219B7C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83219B80: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83219B84: 48000014  b 0x83219b98
	pc = 0x83219B98; continue 'dispatch;
	// 83219B88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83219B8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83219B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219B94: 4B807355  bl 0x82a20ee8
	ctx.lr = 0x83219B98;
	sub_82A20EE8(ctx, base);
	// 83219B98: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219B9C: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83219BA0: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83219BA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219BA8: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 83219BAC: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 83219BB0: 98DE0014  stb r6, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[6].u8 ) };
	// 83219BB4: 419A000C  beq cr6, 0x83219bc0
	if ctx.cr[6].eq {
	pc = 0x83219BC0; continue 'dispatch;
	}
	// 83219BB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219BBC: 4BFFEA05  bl 0x832185c0
	ctx.lr = 0x83219BC0;
	sub_832185C0(ctx, base);
	// 83219BC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83219BC4: 4BA8F894  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219BC8 size=96
    let mut pc: u32 = 0x83219BC8;
    'dispatch: loop {
        match pc {
            0x83219BC8 => {
    //   block [0x83219BC8..0x83219C28)
	// 83219BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219BCC: 4BA8F841  bl 0x82ca940c
	ctx.lr = 0x83219BD0;
	sub_82CA93D0(ctx, base);
	// 83219BD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219BD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83219BD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219BDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219BE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219BE4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219BE8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219BEC: 4E800421  bctrl
	ctx.lr = 0x83219BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219BF0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219BF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219BFC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219C00: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83219C04: 4E800421  bctrl
	ctx.lr = 0x83219C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219C08: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83219C0C: 409A0014  bne cr6, 0x83219c20
	if !ctx.cr[6].eq {
	pc = 0x83219C20; continue 'dispatch;
	}
	// 83219C10: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83219C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219C18: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83219C1C: 4BFFE9A5  bl 0x832185c0
	ctx.lr = 0x83219C20;
	sub_832185C0(ctx, base);
	// 83219C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83219C24: 4BA8F838  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219C28 size=84
    let mut pc: u32 = 0x83219C28;
    'dispatch: loop {
        match pc {
            0x83219C28 => {
    //   block [0x83219C28..0x83219C7C)
	// 83219C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83219C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83219C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83219C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219C40: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83219C44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83219C48: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83219C4C: 4B8089AD  bl 0x82a225f8
	ctx.lr = 0x83219C50;
	sub_82A225F8(ctx, base);
	// 83219C50: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 83219C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219C58: 419A000C  beq cr6, 0x83219c64
	if ctx.cr[6].eq {
	pc = 0x83219C64; continue 'dispatch;
	}
	// 83219C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219C60: 4BFFE961  bl 0x832185c0
	ctx.lr = 0x83219C64;
	sub_832185C0(ctx, base);
	// 83219C64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83219C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83219C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83219C70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83219C74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83219C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219C80 size=212
    let mut pc: u32 = 0x83219C80;
    'dispatch: loop {
        match pc {
            0x83219C80 => {
    //   block [0x83219C80..0x83219D54)
	// 83219C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219C84: 4BA8F789  bl 0x82ca940c
	ctx.lr = 0x83219C88;
	sub_82CA93D0(ctx, base);
	// 83219C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219C8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219C90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83219C94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83219C98: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219C9C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83219CA0: 4198004C  blt cr6, 0x83219cec
	if ctx.cr[6].lt {
	pc = 0x83219CEC; continue 'dispatch;
	}
	// 83219CA4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219CA8: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83219CAC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219CB0: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 83219CB4: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 83219CB8: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 83219CBC: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219CC0: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83219CC4: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219CC8: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 83219CCC: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219CD0: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 83219CD4: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219CD8: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 83219CDC: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83219CE0: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83219CE4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83219CE8: 48000014  b 0x83219cfc
	pc = 0x83219CFC; continue 'dispatch;
	// 83219CEC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83219CF0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83219CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219CF8: 4B8071F1  bl 0x82a20ee8
	ctx.lr = 0x83219CFC;
	sub_82A20EE8(ctx, base);
	// 83219CFC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83219D00: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219D04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219D08: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83219D0C: 419A0024  beq cr6, 0x83219d30
	if ctx.cr[6].eq {
	pc = 0x83219D30; continue 'dispatch;
	}
	// 83219D10: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83219D14: 89410052  lbz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83219D18: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 83219D1C: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83219D20: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 83219D24: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 83219D28: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83219D2C: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 83219D30: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219D34: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83219D38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83219D3C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83219D40: 419A000C  beq cr6, 0x83219d4c
	if ctx.cr[6].eq {
	pc = 0x83219D4C; continue 'dispatch;
	}
	// 83219D44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219D48: 4BFFE879  bl 0x832185c0
	ctx.lr = 0x83219D4C;
	sub_832185C0(ctx, base);
	// 83219D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83219D50: 4BA8F70C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219D58 size=180
    let mut pc: u32 = 0x83219D58;
    'dispatch: loop {
        match pc {
            0x83219D58 => {
    //   block [0x83219D58..0x83219E0C)
	// 83219D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219D5C: 4BA8F6AD  bl 0x82ca9408
	ctx.lr = 0x83219D60;
	sub_82CA93D0(ctx, base);
	// 83219D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219D64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219D68: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83219D6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219D70: 3BCB9D74  addi r30, r11, -0x628c
	ctx.r[30].s64 = ctx.r[11].s64 + -25228;
	// 83219D74: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83219D78: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219D80: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 83219D84: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219D88: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83219D8C: 4E800421  bctrl
	ctx.lr = 0x83219D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219D90: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83219D94: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83219D98: 419A006C  beq cr6, 0x83219e04
	if ctx.cr[6].eq {
	pc = 0x83219E04; continue 'dispatch;
	}
	// 83219D9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83219DA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219DA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219DA8: 4B013129  bl 0x8222ced0
	ctx.lr = 0x83219DAC;
	sub_8222CED0(ctx, base);
	// 83219DAC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219DB4: 409A0010  bne cr6, 0x83219dc4
	if !ctx.cr[6].eq {
	pc = 0x83219DC4; continue 'dispatch;
	}
	// 83219DB8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83219DBC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 83219DC0: 48000008  b 0x83219dc8
	pc = 0x83219DC8; continue 'dispatch;
	// 83219DC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219DC8: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 83219DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219DD0: 4B2669F9  bl 0x824807c8
	ctx.lr = 0x83219DD4;
	sub_824807C8(ctx, base);
	// 83219DD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219DD8: 4AFFB001  bl 0x82214dd8
	ctx.lr = 0x83219DDC;
	sub_82214DD8(ctx, base);
	// 83219DDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219DE4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219DE8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219DEC: 4E800421  bctrl
	ctx.lr = 0x83219DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219DF0: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 83219DF4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83219DF8: 419A000C  beq cr6, 0x83219e04
	if ctx.cr[6].eq {
	pc = 0x83219E04; continue 'dispatch;
	}
	// 83219DFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83219E00: 4BFFE7C1  bl 0x832185c0
	ctx.lr = 0x83219E04;
	sub_832185C0(ctx, base);
	// 83219E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83219E08: 4BA8F650  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219E10 size=96
    let mut pc: u32 = 0x83219E10;
    'dispatch: loop {
        match pc {
            0x83219E10 => {
    //   block [0x83219E10..0x83219E70)
	// 83219E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219E14: 4BA8F5F9  bl 0x82ca940c
	ctx.lr = 0x83219E18;
	sub_82CA93D0(ctx, base);
	// 83219E18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219E1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83219E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219E24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219E28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219E2C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219E30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83219E34: 4E800421  bctrl
	ctx.lr = 0x83219E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219E38: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219E3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219E44: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219E48: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83219E4C: 4E800421  bctrl
	ctx.lr = 0x83219E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219E50: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83219E54: 409A0014  bne cr6, 0x83219e68
	if !ctx.cr[6].eq {
	pc = 0x83219E68; continue 'dispatch;
	}
	// 83219E58: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83219E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219E60: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83219E64: 4BFFE75D  bl 0x832185c0
	ctx.lr = 0x83219E68;
	sub_832185C0(ctx, base);
	// 83219E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83219E6C: 4BA8F5F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219E70 size=84
    let mut pc: u32 = 0x83219E70;
    'dispatch: loop {
        match pc {
            0x83219E70 => {
    //   block [0x83219E70..0x83219EC4)
	// 83219E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83219E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83219E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83219E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219E84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83219E88: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83219E8C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83219E90: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83219E94: 4B8087CD  bl 0x82a22660
	ctx.lr = 0x83219E98;
	sub_82A22660(ctx, base);
	// 83219E98: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 83219E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219EA0: 419A000C  beq cr6, 0x83219eac
	if ctx.cr[6].eq {
	pc = 0x83219EAC; continue 'dispatch;
	}
	// 83219EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219EA8: 4BFFE719  bl 0x832185c0
	ctx.lr = 0x83219EAC;
	sub_832185C0(ctx, base);
	// 83219EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83219EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83219EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83219EB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83219EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83219EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83219EC8 size=212
    let mut pc: u32 = 0x83219EC8;
    'dispatch: loop {
        match pc {
            0x83219EC8 => {
    //   block [0x83219EC8..0x83219F9C)
	// 83219EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219ECC: 4BA8F541  bl 0x82ca940c
	ctx.lr = 0x83219ED0;
	sub_82CA93D0(ctx, base);
	// 83219ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219ED4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219ED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83219EDC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83219EE0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83219EE4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83219EE8: 4198004C  blt cr6, 0x83219f34
	if ctx.cr[6].lt {
	pc = 0x83219F34; continue 'dispatch;
	}
	// 83219EEC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83219EF0: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83219EF4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219EF8: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 83219EFC: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 83219F00: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 83219F04: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219F08: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83219F0C: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83219F10: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 83219F14: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83219F18: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 83219F1C: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83219F20: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 83219F24: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83219F28: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83219F2C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83219F30: 48000014  b 0x83219f44
	pc = 0x83219F44; continue 'dispatch;
	// 83219F34: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83219F38: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83219F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219F40: 4B806FA9  bl 0x82a20ee8
	ctx.lr = 0x83219F44;
	sub_82A20EE8(ctx, base);
	// 83219F44: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83219F48: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83219F4C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83219F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219F54: 419A0024  beq cr6, 0x83219f78
	if ctx.cr[6].eq {
	pc = 0x83219F78; continue 'dispatch;
	}
	// 83219F58: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219F5C: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 83219F60: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 83219F64: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83219F68: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 83219F6C: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 83219F70: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83219F74: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 83219F78: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 83219F7C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83219F80: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83219F84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219F88: 419A000C  beq cr6, 0x83219f94
	if ctx.cr[6].eq {
	pc = 0x83219F94; continue 'dispatch;
	}
	// 83219F8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83219F90: 4BFFE631  bl 0x832185c0
	ctx.lr = 0x83219F94;
	sub_832185C0(ctx, base);
	// 83219F94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83219F98: 4BA8F4C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83219FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83219FA0 size=332
    let mut pc: u32 = 0x83219FA0;
    'dispatch: loop {
        match pc {
            0x83219FA0 => {
    //   block [0x83219FA0..0x8321A0EC)
	// 83219FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83219FA4: 4BA8F465  bl 0x82ca9408
	ctx.lr = 0x83219FA8;
	sub_82CA93D0(ctx, base);
	// 83219FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83219FAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83219FB0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83219FB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83219FB8: 3BCB9D4C  addi r30, r11, -0x62b4
	ctx.r[30].s64 = ctx.r[11].s64 + -25268;
	// 83219FBC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83219FC0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83219FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83219FC8: 389EFFA4  addi r4, r30, -0x5c
	ctx.r[4].s64 = ctx.r[30].s64 + -92;
	// 83219FCC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83219FD0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83219FD4: 4E800421  bctrl
	ctx.lr = 0x83219FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83219FD8: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83219FDC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83219FE0: 419A002C  beq cr6, 0x8321a00c
	if ctx.cr[6].eq {
	pc = 0x8321A00C; continue 'dispatch;
	}
	// 83219FE4: 389EFFBC  addi r4, r30, -0x44
	ctx.r[4].s64 = ctx.r[30].s64 + -68;
	// 83219FE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83219FEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83219FF0: 4B012EE1  bl 0x8222ced0
	ctx.lr = 0x83219FF4;
	sub_8222CED0(ctx, base);
	// 83219FF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83219FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83219FFC: 409A00A8  bne cr6, 0x8321a0a4
	if !ctx.cr[6].eq {
	pc = 0x8321A0A4; continue 'dispatch;
	}
	// 8321A000: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A004: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321A008: 480000A0  b 0x8321a0a8
	pc = 0x8321A0A8; continue 'dispatch;
	// 8321A00C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A010: 389EFFC4  addi r4, r30, -0x3c
	ctx.r[4].s64 = ctx.r[30].s64 + -60;
	// 8321A014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A018: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A01C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A020: 4E800421  bctrl
	ctx.lr = 0x8321A024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A024: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321A028: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321A02C: 419A002C  beq cr6, 0x8321a058
	if ctx.cr[6].eq {
	pc = 0x8321A058; continue 'dispatch;
	}
	// 8321A030: 389EFFDC  addi r4, r30, -0x24
	ctx.r[4].s64 = ctx.r[30].s64 + -36;
	// 8321A034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A03C: 4B012E95  bl 0x8222ced0
	ctx.lr = 0x8321A040;
	sub_8222CED0(ctx, base);
	// 8321A040: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A048: 409A005C  bne cr6, 0x8321a0a4
	if !ctx.cr[6].eq {
	pc = 0x8321A0A4; continue 'dispatch;
	}
	// 8321A04C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A050: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321A054: 48000054  b 0x8321a0a8
	pc = 0x8321A0A8; continue 'dispatch;
	// 8321A058: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A05C: 389EFFE8  addi r4, r30, -0x18
	ctx.r[4].s64 = ctx.r[30].s64 + -24;
	// 8321A060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A064: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A068: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A06C: 4E800421  bctrl
	ctx.lr = 0x8321A070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A070: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321A074: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321A078: 419A0058  beq cr6, 0x8321a0d0
	if ctx.cr[6].eq {
	pc = 0x8321A0D0; continue 'dispatch;
	}
	// 8321A07C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321A080: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A084: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A088: 4B012E49  bl 0x8222ced0
	ctx.lr = 0x8321A08C;
	sub_8222CED0(ctx, base);
	// 8321A08C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A094: 409A0010  bne cr6, 0x8321a0a4
	if !ctx.cr[6].eq {
	pc = 0x8321A0A4; continue 'dispatch;
	}
	// 8321A098: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A09C: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321A0A0: 48000008  b 0x8321a0a8
	pc = 0x8321A0A8; continue 'dispatch;
	// 8321A0A4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A0A8: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321A0AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A0B0: 4B2667A1  bl 0x82480850
	ctx.lr = 0x8321A0B4;
	sub_82480850(ctx, base);
	// 8321A0B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A0B8: 4AFFAD21  bl 0x82214dd8
	ctx.lr = 0x8321A0BC;
	sub_82214DD8(ctx, base);
	// 8321A0BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A0C4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A0C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A0CC: 4E800421  bctrl
	ctx.lr = 0x8321A0D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A0D0: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321A0D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A0D8: 419A000C  beq cr6, 0x8321a0e4
	if ctx.cr[6].eq {
	pc = 0x8321A0E4; continue 'dispatch;
	}
	// 8321A0DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321A0E0: 4BFFE4E1  bl 0x832185c0
	ctx.lr = 0x8321A0E4;
	sub_832185C0(ctx, base);
	// 8321A0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A0E8: 4BA8F370  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321A0F0 size=96
    let mut pc: u32 = 0x8321A0F0;
    'dispatch: loop {
        match pc {
            0x8321A0F0 => {
    //   block [0x8321A0F0..0x8321A150)
	// 8321A0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A0F4: 4BA8F319  bl 0x82ca940c
	ctx.lr = 0x8321A0F8;
	sub_82CA93D0(ctx, base);
	// 8321A0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A0FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321A100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A108: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A10C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A110: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A114: 4E800421  bctrl
	ctx.lr = 0x8321A118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A118: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A11C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A124: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A128: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321A12C: 4E800421  bctrl
	ctx.lr = 0x8321A130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A130: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321A134: 409A0014  bne cr6, 0x8321a148
	if !ctx.cr[6].eq {
	pc = 0x8321A148; continue 'dispatch;
	}
	// 8321A138: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A13C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A140: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321A144: 4BFFE47D  bl 0x832185c0
	ctx.lr = 0x8321A148;
	sub_832185C0(ctx, base);
	// 8321A148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A14C: 4BA8F310  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321A150 size=124
    let mut pc: u32 = 0x8321A150;
    'dispatch: loop {
        match pc {
            0x8321A150 => {
    //   block [0x8321A150..0x8321A1CC)
	// 8321A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A154: 4BA8F2B9  bl 0x82ca940c
	ctx.lr = 0x8321A158;
	sub_82CA93D0(ctx, base);
	// 8321A158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A15C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A160: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321A164: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321A168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A16C: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321A170: 4B8084F1  bl 0x82a22660
	ctx.lr = 0x8321A174;
	sub_82A22660(ctx, base);
	// 8321A174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A178: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321A17C: 4B8084E5  bl 0x82a22660
	ctx.lr = 0x8321A180;
	sub_82A22660(ctx, base);
	// 8321A180: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A184: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A188: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321A18C: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8321A190: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321A194: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321A198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A19C: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321A1A0: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A1A4: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 8321A1A8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8321A1AC: 4E800421  bctrl
	ctx.lr = 0x8321A1B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A1B0: 57A5063E  clrlwi r5, r29, 0x18
	ctx.r[5].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321A1B4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8321A1B8: 419A000C  beq cr6, 0x8321a1c4
	if ctx.cr[6].eq {
	pc = 0x8321A1C4; continue 'dispatch;
	}
	// 8321A1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A1C0: 4BFFE401  bl 0x832185c0
	ctx.lr = 0x8321A1C4;
	sub_832185C0(ctx, base);
	// 8321A1C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A1C8: 4BA8F294  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321A1D0 size=468
    let mut pc: u32 = 0x8321A1D0;
    'dispatch: loop {
        match pc {
            0x8321A1D0 => {
    //   block [0x8321A1D0..0x8321A3A4)
	// 8321A1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A1D4: 4BA8F239  bl 0x82ca940c
	ctx.lr = 0x8321A1D8;
	sub_82CA93D0(ctx, base);
	// 8321A1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A1DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A1E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321A1E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321A1E8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321A1EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321A1F0: 4198004C  blt cr6, 0x8321a23c
	if ctx.cr[6].lt {
	pc = 0x8321A23C; continue 'dispatch;
	}
	// 8321A1F4: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321A1F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A1FC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 8321A200: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A204: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321A208: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321A20C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A210: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321A214: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321A218: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321A21C: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321A220: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321A224: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321A228: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321A22C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321A230: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321A234: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321A238: 48000014  b 0x8321a24c
	pc = 0x8321A24C; continue 'dispatch;
	// 8321A23C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321A240: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321A244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A248: 4B806CA1  bl 0x82a20ee8
	ctx.lr = 0x8321A24C;
	sub_82A20EE8(ctx, base);
	// 8321A24C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A250: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A254: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8321A258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A25C: 419A0024  beq cr6, 0x8321a280
	if ctx.cr[6].eq {
	pc = 0x8321A280; continue 'dispatch;
	}
	// 8321A260: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A264: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8321A268: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321A26C: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8321A270: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8321A274: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8321A278: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 8321A27C: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 8321A280: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A284: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321A288: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321A28C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321A290: 4198004C  blt cr6, 0x8321a2dc
	if ctx.cr[6].lt {
	pc = 0x8321A2DC; continue 'dispatch;
	}
	// 8321A294: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321A298: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A29C: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 8321A2A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A2A4: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321A2A8: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321A2AC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A2B0: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321A2B4: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321A2B8: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321A2BC: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321A2C0: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321A2C4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321A2C8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321A2CC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321A2D0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321A2D4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321A2D8: 48000014  b 0x8321a2ec
	pc = 0x8321A2EC; continue 'dispatch;
	// 8321A2DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321A2E0: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8321A2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A2E8: 4B806C01  bl 0x82a20ee8
	ctx.lr = 0x8321A2EC;
	sub_82A20EE8(ctx, base);
	// 8321A2EC: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A2F0: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A2F4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8321A2F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A2FC: 419A0024  beq cr6, 0x8321a320
	if ctx.cr[6].eq {
	pc = 0x8321A320; continue 'dispatch;
	}
	// 8321A300: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A304: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8321A308: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321A30C: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8321A310: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8321A314: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8321A318: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 8321A31C: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 8321A320: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A324: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321A328: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321A32C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321A330: 41980034  blt cr6, 0x8321a364
	if ctx.cr[6].lt {
	pc = 0x8321A364; continue 'dispatch;
	}
	// 8321A334: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321A338: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A33C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A340: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8321A344: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321A348: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321A34C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A350: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321A354: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321A358: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321A35C: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321A360: 48000014  b 0x8321a374
	pc = 0x8321A374; continue 'dispatch;
	// 8321A364: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321A368: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321A36C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A370: 4B806B79  bl 0x82a20ee8
	ctx.lr = 0x8321A374;
	sub_82A20EE8(ctx, base);
	// 8321A374: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A378: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321A37C: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8321A380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321A384: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321A388: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 8321A38C: 98DE0018  stb r6, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[6].u8 ) };
	// 8321A390: 419A000C  beq cr6, 0x8321a39c
	if ctx.cr[6].eq {
	pc = 0x8321A39C; continue 'dispatch;
	}
	// 8321A394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A398: 4BFFE229  bl 0x832185c0
	ctx.lr = 0x8321A39C;
	sub_832185C0(ctx, base);
	// 8321A39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A3A0: 4BA8F0BC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A3A8 size=276
    let mut pc: u32 = 0x8321A3A8;
    'dispatch: loop {
        match pc {
            0x8321A3A8 => {
    //   block [0x8321A3A8..0x8321A4BC)
	// 8321A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A3AC: 4BA8F059  bl 0x82ca9404
	ctx.lr = 0x8321A3B0;
	sub_82CA93D0(ctx, base);
	// 8321A3B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A3B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A3B8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321A3BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A3C0: 3BCB9CDC  addi r30, r11, -0x6324
	ctx.r[30].s64 = ctx.r[11].s64 + -25380;
	// 8321A3C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8321A3C8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A3D0: 389EFFB4  addi r4, r30, -0x4c
	ctx.r[4].s64 = ctx.r[30].s64 + -76;
	// 8321A3D4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A3D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321A3DC: 4E800421  bctrl
	ctx.lr = 0x8321A3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A3E0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321A3E4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321A3E8: 419A00CC  beq cr6, 0x8321a4b4
	if ctx.cr[6].eq {
	pc = 0x8321A4B4; continue 'dispatch;
	}
	// 8321A3EC: 389EFFD0  addi r4, r30, -0x30
	ctx.r[4].s64 = ctx.r[30].s64 + -48;
	// 8321A3F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A3F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A3F8: 4B012AD9  bl 0x8222ced0
	ctx.lr = 0x8321A3FC;
	sub_8222CED0(ctx, base);
	// 8321A3FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A400: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 8321A404: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321A408: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A40C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A410: 419A0008  beq cr6, 0x8321a418
	if ctx.cr[6].eq {
	pc = 0x8321A418; continue 'dispatch;
	}
	// 8321A414: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A418: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321A41C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A420: 4B266431  bl 0x82480850
	ctx.lr = 0x8321A424;
	sub_82480850(ctx, base);
	// 8321A424: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A428: 4AFFA9B1  bl 0x82214dd8
	ctx.lr = 0x8321A42C;
	sub_82214DD8(ctx, base);
	// 8321A42C: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 8321A430: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A434: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A438: 4B012A99  bl 0x8222ced0
	ctx.lr = 0x8321A43C;
	sub_8222CED0(ctx, base);
	// 8321A43C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A440: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321A444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A448: 419A0008  beq cr6, 0x8321a450
	if ctx.cr[6].eq {
	pc = 0x8321A450; continue 'dispatch;
	}
	// 8321A44C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A450: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 8321A454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A458: 4B2663F9  bl 0x82480850
	ctx.lr = 0x8321A45C;
	sub_82480850(ctx, base);
	// 8321A45C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A460: 4AFFA979  bl 0x82214dd8
	ctx.lr = 0x8321A464;
	sub_82214DD8(ctx, base);
	// 8321A464: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321A468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A46C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A470: 4B012A61  bl 0x8222ced0
	ctx.lr = 0x8321A474;
	sub_8222CED0(ctx, base);
	// 8321A474: 38BD0018  addi r5, r29, 0x18
	ctx.r[5].s64 = ctx.r[29].s64 + 24;
	// 8321A478: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321A47C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A480: 4801B181  bl 0x83235600
	ctx.lr = 0x8321A484;
	sub_83235600(ctx, base);
	// 8321A484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A488: 4AFFA951  bl 0x82214dd8
	ctx.lr = 0x8321A48C;
	sub_82214DD8(ctx, base);
	// 8321A48C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A494: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A498: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A49C: 4E800421  bctrl
	ctx.lr = 0x8321A4A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A4A0: 5769063E  clrlwi r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 8321A4A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321A4A8: 419A000C  beq cr6, 0x8321a4b4
	if ctx.cr[6].eq {
	pc = 0x8321A4B4; continue 'dispatch;
	}
	// 8321A4AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321A4B0: 4BFFE111  bl 0x832185c0
	ctx.lr = 0x8321A4B4;
	sub_832185C0(ctx, base);
	// 8321A4B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321A4B8: 4BA8EF9C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321A4C0 size=112
    let mut pc: u32 = 0x8321A4C0;
    'dispatch: loop {
        match pc {
            0x8321A4C0 => {
    //   block [0x8321A4C0..0x8321A530)
	// 8321A4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A4C4: 4BA8EF49  bl 0x82ca940c
	ctx.lr = 0x8321A4C8;
	sub_82CA93D0(ctx, base);
	// 8321A4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A4CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321A4D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A4D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A4D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A4DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A4E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A4E4: 4E800421  bctrl
	ctx.lr = 0x8321A4E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A4E8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A4EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A4F4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A4F8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321A4FC: 4E800421  bctrl
	ctx.lr = 0x8321A500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A500: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321A504: 409A0024  bne cr6, 0x8321a528
	if !ctx.cr[6].eq {
	pc = 0x8321A528; continue 'dispatch;
	}
	// 8321A508: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321A50C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A510: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321A514: C1BE0014  lfs f13, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321A518: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321A51C: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A520: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8321A524: 4BFFE09D  bl 0x832185c0
	ctx.lr = 0x8321A528;
	sub_832185C0(ctx, base);
	// 8321A528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A52C: 4BA8EF30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A530 size=84
    let mut pc: u32 = 0x8321A530;
    'dispatch: loop {
        match pc {
            0x8321A530 => {
    //   block [0x8321A530..0x8321A584)
	// 8321A530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321A538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321A53C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321A540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A548: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321A54C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321A550: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A554: 4B8080A5  bl 0x82a225f8
	ctx.lr = 0x8321A558;
	sub_82A225F8(ctx, base);
	// 8321A558: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321A55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A560: 419A000C  beq cr6, 0x8321a56c
	if ctx.cr[6].eq {
	pc = 0x8321A56C; continue 'dispatch;
	}
	// 8321A564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A568: 4BFFE059  bl 0x832185c0
	ctx.lr = 0x8321A56C;
	sub_832185C0(ctx, base);
	// 8321A56C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321A578: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321A57C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321A580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A588 size=212
    let mut pc: u32 = 0x8321A588;
    'dispatch: loop {
        match pc {
            0x8321A588 => {
    //   block [0x8321A588..0x8321A65C)
	// 8321A588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A58C: 4BA8EE81  bl 0x82ca940c
	ctx.lr = 0x8321A590;
	sub_82CA93D0(ctx, base);
	// 8321A590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A594: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A598: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321A59C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321A5A0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321A5A4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8321A5A8: 4198004C  blt cr6, 0x8321a5f4
	if ctx.cr[6].lt {
	pc = 0x8321A5F4; continue 'dispatch;
	}
	// 8321A5AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A5B0: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321A5B4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A5B8: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 8321A5BC: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321A5C0: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 8321A5C4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A5C8: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321A5CC: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321A5D0: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321A5D4: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321A5D8: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321A5DC: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321A5E0: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 8321A5E4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321A5E8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321A5EC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321A5F0: 48000014  b 0x8321a604
	pc = 0x8321A604; continue 'dispatch;
	// 8321A5F4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321A5F8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321A5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A600: 4B8068E9  bl 0x82a20ee8
	ctx.lr = 0x8321A604;
	sub_82A20EE8(ctx, base);
	// 8321A604: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A608: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A60C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321A610: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8321A614: 419A0024  beq cr6, 0x8321a638
	if ctx.cr[6].eq {
	pc = 0x8321A638; continue 'dispatch;
	}
	// 8321A618: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321A61C: 89410052  lbz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321A620: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321A624: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A628: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 8321A62C: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8321A630: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321A634: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8321A638: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A63C: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321A640: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321A644: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A648: 419A000C  beq cr6, 0x8321a654
	if ctx.cr[6].eq {
	pc = 0x8321A654; continue 'dispatch;
	}
	// 8321A64C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A650: 4BFFDF71  bl 0x832185c0
	ctx.lr = 0x8321A654;
	sub_832185C0(ctx, base);
	// 8321A654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A658: 4BA8EE04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A660 size=196
    let mut pc: u32 = 0x8321A660;
    'dispatch: loop {
        match pc {
            0x8321A660 => {
    //   block [0x8321A660..0x8321A724)
	// 8321A660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A664: 4BA8EDA5  bl 0x82ca9408
	ctx.lr = 0x8321A668;
	sub_82CA93D0(ctx, base);
	// 8321A668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A66C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A670: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321A674: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A678: 3BCB9C88  addi r30, r11, -0x6378
	ctx.r[30].s64 = ctx.r[11].s64 + -25464;
	// 8321A67C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321A680: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A688: 389EFFE8  addi r4, r30, -0x18
	ctx.r[4].s64 = ctx.r[30].s64 + -24;
	// 8321A68C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A690: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321A694: 4E800421  bctrl
	ctx.lr = 0x8321A698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A698: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321A69C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321A6A0: 419A007C  beq cr6, 0x8321a71c
	if ctx.cr[6].eq {
	pc = 0x8321A71C; continue 'dispatch;
	}
	// 8321A6A4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A6A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321A6AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A6B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A6B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8321A6B8: 4B012819  bl 0x8222ced0
	ctx.lr = 0x8321A6BC;
	sub_8222CED0(ctx, base);
	// 8321A6BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A6C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A6C4: 409A0010  bne cr6, 0x8321a6d4
	if !ctx.cr[6].eq {
	pc = 0x8321A6D4; continue 'dispatch;
	}
	// 8321A6C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A6CC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321A6D0: 48000008  b 0x8321a6d8
	pc = 0x8321A6D8; continue 'dispatch;
	// 8321A6D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A6D8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8321A6DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A6E0: 4B2660E9  bl 0x824807c8
	ctx.lr = 0x8321A6E4;
	sub_824807C8(ctx, base);
	// 8321A6E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A6E8: 4AFFA6F1  bl 0x82214dd8
	ctx.lr = 0x8321A6EC;
	sub_82214DD8(ctx, base);
	// 8321A6EC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A6F4: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A6F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A6FC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A700: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321A704: 4E800421  bctrl
	ctx.lr = 0x8321A708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A708: 5788063E  clrlwi r8, r28, 0x18
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321A70C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321A710: 419A000C  beq cr6, 0x8321a71c
	if ctx.cr[6].eq {
	pc = 0x8321A71C; continue 'dispatch;
	}
	// 8321A714: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321A718: 4BFFDEA9  bl 0x832185c0
	ctx.lr = 0x8321A71C;
	sub_832185C0(ctx, base);
	// 8321A71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A720: 4BA8ED38  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A728 size=96
    let mut pc: u32 = 0x8321A728;
    'dispatch: loop {
        match pc {
            0x8321A728 => {
    //   block [0x8321A728..0x8321A788)
	// 8321A728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A72C: 4BA8ECE1  bl 0x82ca940c
	ctx.lr = 0x8321A730;
	sub_82CA93D0(ctx, base);
	// 8321A730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A734: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321A738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A73C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A740: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A744: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A748: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A74C: 4E800421  bctrl
	ctx.lr = 0x8321A750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A750: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A754: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A75C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A760: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321A764: 4E800421  bctrl
	ctx.lr = 0x8321A768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A768: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321A76C: 409A0014  bne cr6, 0x8321a780
	if !ctx.cr[6].eq {
	pc = 0x8321A780; continue 'dispatch;
	}
	// 8321A770: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A778: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A77C: 4BFFDE45  bl 0x832185c0
	ctx.lr = 0x8321A780;
	sub_832185C0(ctx, base);
	// 8321A780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A784: 4BA8ECD8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A788 size=84
    let mut pc: u32 = 0x8321A788;
    'dispatch: loop {
        match pc {
            0x8321A788 => {
    //   block [0x8321A788..0x8321A7DC)
	// 8321A788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321A790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321A794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321A798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A79C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A7A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321A7A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321A7A8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A7AC: 4B807E4D  bl 0x82a225f8
	ctx.lr = 0x8321A7B0;
	sub_82A225F8(ctx, base);
	// 8321A7B0: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321A7B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A7B8: 419A000C  beq cr6, 0x8321a7c4
	if ctx.cr[6].eq {
	pc = 0x8321A7C4; continue 'dispatch;
	}
	// 8321A7BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A7C0: 4BFFDE01  bl 0x832185c0
	ctx.lr = 0x8321A7C4;
	sub_832185C0(ctx, base);
	// 8321A7C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321A7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321A7D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321A7D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321A7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A7E0 size=212
    let mut pc: u32 = 0x8321A7E0;
    'dispatch: loop {
        match pc {
            0x8321A7E0 => {
    //   block [0x8321A7E0..0x8321A8B4)
	// 8321A7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A7E4: 4BA8EC29  bl 0x82ca940c
	ctx.lr = 0x8321A7E8;
	sub_82CA93D0(ctx, base);
	// 8321A7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A7EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A7F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321A7F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321A7F8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321A7FC: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8321A800: 4198004C  blt cr6, 0x8321a84c
	if ctx.cr[6].lt {
	pc = 0x8321A84C; continue 'dispatch;
	}
	// 8321A804: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A808: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321A80C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A810: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 8321A814: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321A818: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 8321A81C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A820: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321A824: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321A828: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321A82C: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321A830: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321A834: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321A838: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 8321A83C: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321A840: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321A844: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321A848: 48000014  b 0x8321a85c
	pc = 0x8321A85C; continue 'dispatch;
	// 8321A84C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321A850: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321A854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A858: 4B806691  bl 0x82a20ee8
	ctx.lr = 0x8321A85C;
	sub_82A20EE8(ctx, base);
	// 8321A85C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A860: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321A864: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321A868: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8321A86C: 419A0024  beq cr6, 0x8321a890
	if ctx.cr[6].eq {
	pc = 0x8321A890; continue 'dispatch;
	}
	// 8321A870: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321A874: 89410052  lbz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321A878: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321A87C: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A880: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 8321A884: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8321A888: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321A88C: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8321A890: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A894: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321A898: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321A89C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A8A0: 419A000C  beq cr6, 0x8321a8ac
	if ctx.cr[6].eq {
	pc = 0x8321A8AC; continue 'dispatch;
	}
	// 8321A8A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A8A8: 4BFFDD19  bl 0x832185c0
	ctx.lr = 0x8321A8AC;
	sub_832185C0(ctx, base);
	// 8321A8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A8B0: 4BA8EBAC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A8B8 size=196
    let mut pc: u32 = 0x8321A8B8;
    'dispatch: loop {
        match pc {
            0x8321A8B8 => {
    //   block [0x8321A8B8..0x8321A97C)
	// 8321A8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A8BC: 4BA8EB4D  bl 0x82ca9408
	ctx.lr = 0x8321A8C0;
	sub_82CA93D0(ctx, base);
	// 8321A8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A8C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321A8C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321A8CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A8D0: 3BCB9C64  addi r30, r11, -0x639c
	ctx.r[30].s64 = ctx.r[11].s64 + -25500;
	// 8321A8D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321A8D8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A8E0: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 8321A8E4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A8E8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321A8EC: 4E800421  bctrl
	ctx.lr = 0x8321A8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A8F0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321A8F4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321A8F8: 419A007C  beq cr6, 0x8321a974
	if ctx.cr[6].eq {
	pc = 0x8321A974; continue 'dispatch;
	}
	// 8321A8FC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A900: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321A904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321A908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A90C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8321A910: 4B0125C1  bl 0x8222ced0
	ctx.lr = 0x8321A914;
	sub_8222CED0(ctx, base);
	// 8321A914: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321A918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321A91C: 409A0010  bne cr6, 0x8321a92c
	if !ctx.cr[6].eq {
	pc = 0x8321A92C; continue 'dispatch;
	}
	// 8321A920: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321A924: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321A928: 48000008  b 0x8321a930
	pc = 0x8321A930; continue 'dispatch;
	// 8321A92C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A930: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8321A934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A938: 4B265E91  bl 0x824807c8
	ctx.lr = 0x8321A93C;
	sub_824807C8(ctx, base);
	// 8321A93C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321A940: 4AFFA499  bl 0x82214dd8
	ctx.lr = 0x8321A944;
	sub_82214DD8(ctx, base);
	// 8321A944: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321A948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A94C: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A950: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A954: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321A958: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321A95C: 4E800421  bctrl
	ctx.lr = 0x8321A960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A960: 5788063E  clrlwi r8, r28, 0x18
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321A964: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321A968: 419A000C  beq cr6, 0x8321a974
	if ctx.cr[6].eq {
	pc = 0x8321A974; continue 'dispatch;
	}
	// 8321A96C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321A970: 4BFFDC51  bl 0x832185c0
	ctx.lr = 0x8321A974;
	sub_832185C0(ctx, base);
	// 8321A974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321A978: 4BA8EAE0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A980 size=96
    let mut pc: u32 = 0x8321A980;
    'dispatch: loop {
        match pc {
            0x8321A980 => {
    //   block [0x8321A980..0x8321A9E0)
	// 8321A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A984: 4BA8EA89  bl 0x82ca940c
	ctx.lr = 0x8321A988;
	sub_82CA93D0(ctx, base);
	// 8321A988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A98C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321A990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321A998: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A99C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A9A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321A9A4: 4E800421  bctrl
	ctx.lr = 0x8321A9A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A9A8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321A9AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321A9B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A9B4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321A9B8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321A9BC: 4E800421  bctrl
	ctx.lr = 0x8321A9C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321A9C0: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321A9C4: 409A0014  bne cr6, 0x8321a9d8
	if !ctx.cr[6].eq {
	pc = 0x8321A9D8; continue 'dispatch;
	}
	// 8321A9C8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321A9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321A9D0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321A9D4: 4BFFDBED  bl 0x832185c0
	ctx.lr = 0x8321A9D8;
	sub_832185C0(ctx, base);
	// 8321A9D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321A9DC: 4BA8EA80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321A9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321A9E0 size=84
    let mut pc: u32 = 0x8321A9E0;
    'dispatch: loop {
        match pc {
            0x8321A9E0 => {
    //   block [0x8321A9E0..0x8321AA34)
	// 8321A9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321A9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321A9E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321A9EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321A9F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321A9F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321A9F8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321A9FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321AA00: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321AA04: 4B807BF5  bl 0x82a225f8
	ctx.lr = 0x8321AA08;
	sub_82A225F8(ctx, base);
	// 8321AA08: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321AA0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321AA10: 419A000C  beq cr6, 0x8321aa1c
	if ctx.cr[6].eq {
	pc = 0x8321AA1C; continue 'dispatch;
	}
	// 8321AA14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AA18: 4BFFDBA9  bl 0x832185c0
	ctx.lr = 0x8321AA1C;
	sub_832185C0(ctx, base);
	// 8321AA1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321AA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321AA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321AA28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321AA2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321AA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AA38 size=212
    let mut pc: u32 = 0x8321AA38;
    'dispatch: loop {
        match pc {
            0x8321AA38 => {
    //   block [0x8321AA38..0x8321AB0C)
	// 8321AA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AA3C: 4BA8E9D1  bl 0x82ca940c
	ctx.lr = 0x8321AA40;
	sub_82CA93D0(ctx, base);
	// 8321AA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AA44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321AA48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321AA4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321AA50: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321AA54: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8321AA58: 4198004C  blt cr6, 0x8321aaa4
	if ctx.cr[6].lt {
	pc = 0x8321AAA4; continue 'dispatch;
	}
	// 8321AA5C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321AA60: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321AA64: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AA68: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 8321AA6C: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321AA70: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 8321AA74: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AA78: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321AA7C: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321AA80: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321AA84: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321AA88: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321AA8C: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321AA90: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 8321AA94: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321AA98: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321AA9C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321AAA0: 48000014  b 0x8321aab4
	pc = 0x8321AAB4; continue 'dispatch;
	// 8321AAA4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321AAA8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321AAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AAB0: 4B806439  bl 0x82a20ee8
	ctx.lr = 0x8321AAB4;
	sub_82A20EE8(ctx, base);
	// 8321AAB4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321AAB8: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321AABC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321AAC0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8321AAC4: 419A0024  beq cr6, 0x8321aae8
	if ctx.cr[6].eq {
	pc = 0x8321AAE8; continue 'dispatch;
	}
	// 8321AAC8: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321AACC: 89410052  lbz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321AAD0: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321AAD4: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321AAD8: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 8321AADC: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8321AAE0: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321AAE4: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8321AAE8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321AAEC: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321AAF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321AAF4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321AAF8: 419A000C  beq cr6, 0x8321ab04
	if ctx.cr[6].eq {
	pc = 0x8321AB04; continue 'dispatch;
	}
	// 8321AAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321AB00: 4BFFDAC1  bl 0x832185c0
	ctx.lr = 0x8321AB04;
	sub_832185C0(ctx, base);
	// 8321AB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321AB08: 4BA8E954  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AB10 size=196
    let mut pc: u32 = 0x8321AB10;
    'dispatch: loop {
        match pc {
            0x8321AB10 => {
    //   block [0x8321AB10..0x8321ABD4)
	// 8321AB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AB14: 4BA8E8F5  bl 0x82ca9408
	ctx.lr = 0x8321AB18;
	sub_82CA93D0(ctx, base);
	// 8321AB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AB1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321AB20: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321AB24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321AB28: 3BCB9C3C  addi r30, r11, -0x63c4
	ctx.r[30].s64 = ctx.r[11].s64 + -25540;
	// 8321AB2C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321AB30: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AB34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AB38: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 8321AB3C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AB40: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321AB44: 4E800421  bctrl
	ctx.lr = 0x8321AB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AB48: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321AB4C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321AB50: 419A007C  beq cr6, 0x8321abcc
	if ctx.cr[6].eq {
	pc = 0x8321ABCC; continue 'dispatch;
	}
	// 8321AB54: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321AB58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321AB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321AB60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AB64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8321AB68: 4B012369  bl 0x8222ced0
	ctx.lr = 0x8321AB6C;
	sub_8222CED0(ctx, base);
	// 8321AB6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321AB70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321AB74: 409A0010  bne cr6, 0x8321ab84
	if !ctx.cr[6].eq {
	pc = 0x8321AB84; continue 'dispatch;
	}
	// 8321AB78: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321AB7C: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321AB80: 48000008  b 0x8321ab88
	pc = 0x8321AB88; continue 'dispatch;
	// 8321AB84: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AB88: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8321AB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AB90: 4B265C39  bl 0x824807c8
	ctx.lr = 0x8321AB94;
	sub_824807C8(ctx, base);
	// 8321AB94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AB98: 4AFFA241  bl 0x82214dd8
	ctx.lr = 0x8321AB9C;
	sub_82214DD8(ctx, base);
	// 8321AB9C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321ABA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321ABA4: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321ABA8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321ABAC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321ABB0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321ABB4: 4E800421  bctrl
	ctx.lr = 0x8321ABB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321ABB8: 5788063E  clrlwi r8, r28, 0x18
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321ABBC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321ABC0: 419A000C  beq cr6, 0x8321abcc
	if ctx.cr[6].eq {
	pc = 0x8321ABCC; continue 'dispatch;
	}
	// 8321ABC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321ABC8: 4BFFD9F9  bl 0x832185c0
	ctx.lr = 0x8321ABCC;
	sub_832185C0(ctx, base);
	// 8321ABCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321ABD0: 4BA8E888  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321ABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321ABD8 size=96
    let mut pc: u32 = 0x8321ABD8;
    'dispatch: loop {
        match pc {
            0x8321ABD8 => {
    //   block [0x8321ABD8..0x8321AC38)
	// 8321ABD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321ABDC: 4BA8E831  bl 0x82ca940c
	ctx.lr = 0x8321ABE0;
	sub_82CA93D0(ctx, base);
	// 8321ABE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321ABE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321ABE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321ABEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321ABF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321ABF4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321ABF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321ABFC: 4E800421  bctrl
	ctx.lr = 0x8321AC00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AC00: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AC04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321AC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AC0C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AC10: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321AC14: 4E800421  bctrl
	ctx.lr = 0x8321AC18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AC18: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321AC1C: 409A0014  bne cr6, 0x8321ac30
	if !ctx.cr[6].eq {
	pc = 0x8321AC30; continue 'dispatch;
	}
	// 8321AC20: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321AC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AC28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321AC2C: 4BFFD995  bl 0x832185c0
	ctx.lr = 0x8321AC30;
	sub_832185C0(ctx, base);
	// 8321AC30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321AC34: 4BA8E828  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AC38 size=88
    let mut pc: u32 = 0x8321AC38;
    'dispatch: loop {
        match pc {
            0x8321AC38 => {
    //   block [0x8321AC38..0x8321AC90)
	// 8321AC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321AC40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321AC44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321AC48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AC4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321AC50: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 8321AC54: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321AC58: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AC90 size=104
    let mut pc: u32 = 0x8321AC90;
    'dispatch: loop {
        match pc {
            0x8321AC90 => {
    //   block [0x8321AC90..0x8321ACF8)
	// 8321AC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321AC98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321AC9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321ACA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321ACA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321ACA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321ACAC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321ACB0: 4B806C61  bl 0x82a21910
	ctx.lr = 0x8321ACB4;
	sub_82A21910(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321ACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321ACF8 size=156
    let mut pc: u32 = 0x8321ACF8;
    'dispatch: loop {
        match pc {
            0x8321ACF8 => {
    //   block [0x8321ACF8..0x8321AD94)
	// 8321ACF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321ACFC: 4BA8E70D  bl 0x82ca9408
	ctx.lr = 0x8321AD00;
	sub_82CA93D0(ctx, base);
	// 8321AD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AD04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321AD08: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321AD0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321AD10: 3BCB9C18  addi r30, r11, -0x63e8
	ctx.r[30].s64 = ctx.r[11].s64 + -25576;
	// 8321AD14: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321AD18: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AD1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AD20: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 8321AD24: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AD28: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321AD2C: 4E800421  bctrl
	ctx.lr = 0x8321AD30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AD30: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321AD34: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321AD38: 419A0054  beq cr6, 0x8321ad8c
	if ctx.cr[6].eq {
	pc = 0x8321AD8C; continue 'dispatch;
	}
	// 8321AD3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321AD40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321AD44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AD48: 4B012189  bl 0x8222ced0
	ctx.lr = 0x8321AD4C;
	sub_8222CED0(ctx, base);
	// 8321AD4C: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321AD50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321AD54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AD58: 4801A7B9  bl 0x83235510
	ctx.lr = 0x8321AD5C;
	sub_83235510(ctx, base);
	// 8321AD5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AD60: 4AFFA079  bl 0x82214dd8
	ctx.lr = 0x8321AD64;
	sub_82214DD8(ctx, base);
	// 8321AD64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AD68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AD6C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321AD70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321AD74: 4E800421  bctrl
	ctx.lr = 0x8321AD78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AD78: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321AD7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321AD80: 419A000C  beq cr6, 0x8321ad8c
	if ctx.cr[6].eq {
	pc = 0x8321AD8C; continue 'dispatch;
	}
	// 8321AD84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321AD88: 4BFFD839  bl 0x832185c0
	ctx.lr = 0x8321AD8C;
	sub_832185C0(ctx, base);
	// 8321AD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321AD90: 4BA8E6C8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AD98 size=100
    let mut pc: u32 = 0x8321AD98;
    'dispatch: loop {
        match pc {
            0x8321AD98 => {
    //   block [0x8321AD98..0x8321ADFC)
	// 8321AD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AD9C: 4BA8E671  bl 0x82ca940c
	ctx.lr = 0x8321ADA0;
	sub_82CA93D0(ctx, base);
	// 8321ADA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321ADA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321ADA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321ADAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321ADB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321ADB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321ADB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321ADBC: 4E800421  bctrl
	ctx.lr = 0x8321ADC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321ADC0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321ADC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321ADC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321ADCC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321ADD0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321ADD4: 4E800421  bctrl
	ctx.lr = 0x8321ADD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321ADD8: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321ADDC: 409A0018  bne cr6, 0x8321adf4
	if !ctx.cr[6].eq {
	pc = 0x8321ADF4; continue 'dispatch;
	}
	// 8321ADE0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 8321ADE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AE00 size=84
    let mut pc: u32 = 0x8321AE00;
    'dispatch: loop {
        match pc {
            0x8321AE00 => {
    //   block [0x8321AE00..0x8321AE54)
	// 8321AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321AE08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321AE0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321AE10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AE14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321AE18: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321AE1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321AE20: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321AE24: 4B8077D5  bl 0x82a225f8
	ctx.lr = 0x8321AE28;
	sub_82A225F8(ctx, base);
	// 8321AE28: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321AE2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321AE30: 419A000C  beq cr6, 0x8321ae3c
	if ctx.cr[6].eq {
	pc = 0x8321AE3C; continue 'dispatch;
	}
	// 8321AE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AE38: 4BFFD789  bl 0x832185c0
	ctx.lr = 0x8321AE3C;
	sub_832185C0(ctx, base);
	// 8321AE3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321AE40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321AE44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321AE48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321AE4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321AE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AE58 size=212
    let mut pc: u32 = 0x8321AE58;
    'dispatch: loop {
        match pc {
            0x8321AE58 => {
    //   block [0x8321AE58..0x8321AF2C)
	// 8321AE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AE5C: 4BA8E5B1  bl 0x82ca940c
	ctx.lr = 0x8321AE60;
	sub_82CA93D0(ctx, base);
	// 8321AE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AE64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321AE68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321AE6C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321AE70: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321AE74: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8321AE78: 4198004C  blt cr6, 0x8321aec4
	if ctx.cr[6].lt {
	pc = 0x8321AEC4; continue 'dispatch;
	}
	// 8321AE7C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321AE80: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321AE84: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AE88: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 8321AE8C: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321AE90: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 8321AE94: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AE98: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321AE9C: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321AEA0: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321AEA4: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321AEA8: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321AEAC: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321AEB0: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 8321AEB4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321AEB8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321AEBC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321AEC0: 48000014  b 0x8321aed4
	pc = 0x8321AED4; continue 'dispatch;
	// 8321AEC4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321AEC8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321AECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AED0: 4B806019  bl 0x82a20ee8
	ctx.lr = 0x8321AED4;
	sub_82A20EE8(ctx, base);
	// 8321AED4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321AED8: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321AEDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321AEE0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8321AEE4: 419A0024  beq cr6, 0x8321af08
	if ctx.cr[6].eq {
	pc = 0x8321AF08; continue 'dispatch;
	}
	// 8321AEE8: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321AEEC: 89410052  lbz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321AEF0: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321AEF4: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321AEF8: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 8321AEFC: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8321AF00: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321AF04: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8321AF08: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321AF0C: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321AF10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321AF14: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321AF18: 419A000C  beq cr6, 0x8321af24
	if ctx.cr[6].eq {
	pc = 0x8321AF24; continue 'dispatch;
	}
	// 8321AF1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321AF20: 4BFFD6A1  bl 0x832185c0
	ctx.lr = 0x8321AF24;
	sub_832185C0(ctx, base);
	// 8321AF24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321AF28: 4BA8E534  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AF30 size=180
    let mut pc: u32 = 0x8321AF30;
    'dispatch: loop {
        match pc {
            0x8321AF30 => {
    //   block [0x8321AF30..0x8321AFE4)
	// 8321AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AF34: 4BA8E4D5  bl 0x82ca9408
	ctx.lr = 0x8321AF38;
	sub_82CA93D0(ctx, base);
	// 8321AF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AF3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321AF40: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321AF44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321AF48: 3BCB9BEC  addi r30, r11, -0x6414
	ctx.r[30].s64 = ctx.r[11].s64 + -25620;
	// 8321AF4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321AF50: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AF58: 389EFFDC  addi r4, r30, -0x24
	ctx.r[4].s64 = ctx.r[30].s64 + -36;
	// 8321AF5C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321AF60: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321AF64: 4E800421  bctrl
	ctx.lr = 0x8321AF68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AF68: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321AF6C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321AF70: 419A006C  beq cr6, 0x8321afdc
	if ctx.cr[6].eq {
	pc = 0x8321AFDC; continue 'dispatch;
	}
	// 8321AF74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321AF78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321AF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AF80: 4B011F51  bl 0x8222ced0
	ctx.lr = 0x8321AF84;
	sub_8222CED0(ctx, base);
	// 8321AF84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321AF88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321AF8C: 409A0010  bne cr6, 0x8321af9c
	if !ctx.cr[6].eq {
	pc = 0x8321AF9C; continue 'dispatch;
	}
	// 8321AF90: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321AF94: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321AF98: 48000008  b 0x8321afa0
	pc = 0x8321AFA0; continue 'dispatch;
	// 8321AF9C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AFA0: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321AFA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AFA8: 4B265821  bl 0x824807c8
	ctx.lr = 0x8321AFAC;
	sub_824807C8(ctx, base);
	// 8321AFAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321AFB0: 4AFF9E29  bl 0x82214dd8
	ctx.lr = 0x8321AFB4;
	sub_82214DD8(ctx, base);
	// 8321AFB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321AFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321AFBC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321AFC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321AFC4: 4E800421  bctrl
	ctx.lr = 0x8321AFC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321AFC8: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321AFCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321AFD0: 419A000C  beq cr6, 0x8321afdc
	if ctx.cr[6].eq {
	pc = 0x8321AFDC; continue 'dispatch;
	}
	// 8321AFD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321AFD8: 4BFFD5E9  bl 0x832185c0
	ctx.lr = 0x8321AFDC;
	sub_832185C0(ctx, base);
	// 8321AFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321AFE0: 4BA8E478  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321AFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321AFE8 size=96
    let mut pc: u32 = 0x8321AFE8;
    'dispatch: loop {
        match pc {
            0x8321AFE8 => {
    //   block [0x8321AFE8..0x8321B048)
	// 8321AFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321AFEC: 4BA8E421  bl 0x82ca940c
	ctx.lr = 0x8321AFF0;
	sub_82CA93D0(ctx, base);
	// 8321AFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321AFF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321AFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321AFFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B000: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B004: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B008: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321B00C: 4E800421  bctrl
	ctx.lr = 0x8321B010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B010: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B014: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B01C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B020: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321B024: 4E800421  bctrl
	ctx.lr = 0x8321B028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B028: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321B02C: 409A0014  bne cr6, 0x8321b040
	if !ctx.cr[6].eq {
	pc = 0x8321B040; continue 'dispatch;
	}
	// 8321B030: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321B034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B038: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321B03C: 4BFFD585  bl 0x832185c0
	ctx.lr = 0x8321B040;
	sub_832185C0(ctx, base);
	// 8321B040: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321B044: 4BA8E418  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321B048 size=84
    let mut pc: u32 = 0x8321B048;
    'dispatch: loop {
        match pc {
            0x8321B048 => {
    //   block [0x8321B048..0x8321B09C)
	// 8321B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B04C: 4BA8E3C1  bl 0x82ca940c
	ctx.lr = 0x8321B050;
	sub_82CA93D0(ctx, base);
	// 8321B050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321B058: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321B05C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321B060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B064: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8321B068: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8321B06C: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321B070: 4B8075F1  bl 0x82a22660
	ctx.lr = 0x8321B074;
	sub_82A22660(ctx, base);
	// 8321B074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B078: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321B07C: 4B8075E5  bl 0x82a22660
	ctx.lr = 0x8321B080;
	sub_82A22660(ctx, base);
	// 8321B080: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321B084: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B088: 419A000C  beq cr6, 0x8321b094
	if ctx.cr[6].eq {
	pc = 0x8321B094; continue 'dispatch;
	}
	// 8321B08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B090: 4BFFD531  bl 0x832185c0
	ctx.lr = 0x8321B094;
	sub_832185C0(ctx, base);
	// 8321B094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321B098: 4BA8E3C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321B0A0 size=96
    let mut pc: u32 = 0x8321B0A0;
    'dispatch: loop {
        match pc {
            0x8321B0A0 => {
    //   block [0x8321B0A0..0x8321B100)
	// 8321B0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321B0A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321B0AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321B0B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B0B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321B0B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B0BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321B0C0: 4B8066D9  bl 0x82a21798
	ctx.lr = 0x8321B0C4;
	sub_82A21798(ctx, base);
	// 8321B0C4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B0C8: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321B0CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B0D0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321B0D4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B0D8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B0DC: 419A000C  beq cr6, 0x8321b0e8
	if ctx.cr[6].eq {
	pc = 0x8321B0E8; continue 'dispatch;
	}
	// 8321B0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B0E4: 4BFFD4DD  bl 0x832185c0
	ctx.lr = 0x8321B0E8;
	sub_832185C0(ctx, base);
	// 8321B0E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321B0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321B0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321B0F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321B0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321B0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321B100 size=156
    let mut pc: u32 = 0x8321B100;
    'dispatch: loop {
        match pc {
            0x8321B100 => {
    //   block [0x8321B100..0x8321B19C)
	// 8321B100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B104: 4BA8E305  bl 0x82ca9408
	ctx.lr = 0x8321B108;
	sub_82CA93D0(ctx, base);
	// 8321B108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B10C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321B110: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321B114: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321B118: 3BCB9BBC  addi r30, r11, -0x6444
	ctx.r[30].s64 = ctx.r[11].s64 + -25668;
	// 8321B11C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321B120: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B128: 389EFFEC  addi r4, r30, -0x14
	ctx.r[4].s64 = ctx.r[30].s64 + -20;
	// 8321B12C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B130: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321B134: 4E800421  bctrl
	ctx.lr = 0x8321B138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B138: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321B13C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321B140: 419A0054  beq cr6, 0x8321b194
	if ctx.cr[6].eq {
	pc = 0x8321B194; continue 'dispatch;
	}
	// 8321B144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321B148: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321B14C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B150: 4B011D81  bl 0x8222ced0
	ctx.lr = 0x8321B154;
	sub_8222CED0(ctx, base);
	// 8321B154: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321B158: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321B15C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B160: 4801A301  bl 0x83235460
	ctx.lr = 0x8321B164;
	sub_83235460(ctx, base);
	// 8321B164: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B168: 4AFF9C71  bl 0x82214dd8
	ctx.lr = 0x8321B16C;
	sub_82214DD8(ctx, base);
	// 8321B16C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B174: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B178: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321B17C: 4E800421  bctrl
	ctx.lr = 0x8321B180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B180: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321B184: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321B188: 419A000C  beq cr6, 0x8321b194
	if ctx.cr[6].eq {
	pc = 0x8321B194; continue 'dispatch;
	}
	// 8321B18C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321B190: 4BFFD431  bl 0x832185c0
	ctx.lr = 0x8321B194;
	sub_832185C0(ctx, base);
	// 8321B194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321B198: 4BA8E2C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321B1A0 size=104
    let mut pc: u32 = 0x8321B1A0;
    'dispatch: loop {
        match pc {
            0x8321B1A0 => {
    //   block [0x8321B1A0..0x8321B208)
	// 8321B1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B1A4: 4BA8E269  bl 0x82ca940c
	ctx.lr = 0x8321B1A8;
	sub_82CA93D0(ctx, base);
	// 8321B1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B1AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321B1B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321B1B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B1B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B1BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B1C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321B1C4: 4E800421  bctrl
	ctx.lr = 0x8321B1C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B1C8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B1CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321B1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B1D4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B1D8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321B1DC: 4E800421  bctrl
	ctx.lr = 0x8321B1E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B1E0: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321B1E4: 409A001C  bne cr6, 0x8321b200
	if !ctx.cr[6].eq {
	pc = 0x8321B200; continue 'dispatch;
	}
	// 8321B1E8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321B1EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B1F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321B1F4: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B1F8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8321B1FC: 4BFFD3C5  bl 0x832185c0
	ctx.lr = 0x8321B200;
	sub_832185C0(ctx, base);
	// 8321B200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321B204: 4BA8E258  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321B208 size=1468
    let mut pc: u32 = 0x8321B208;
    'dispatch: loop {
        match pc {
            0x8321B208 => {
    //   block [0x8321B208..0x8321B7C4)
	// 8321B208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B20C: 4BA8E1FD  bl 0x82ca9408
	ctx.lr = 0x8321B210;
	sub_82CA93D0(ctx, base);
	// 8321B210: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B214: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321B218: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321B21C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321B220: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B224: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B228: 4198004C  blt cr6, 0x8321b274
	if ctx.cr[6].lt {
	pc = 0x8321B274; continue 'dispatch;
	}
	// 8321B22C: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B230: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B234: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 8321B238: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B23C: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B240: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B244: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B248: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B24C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B250: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B254: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B258: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B25C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B260: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B264: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B268: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B26C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B270: 48000014  b 0x8321b284
	pc = 0x8321B284; continue 'dispatch;
	// 8321B274: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B278: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8321B27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B280: 4B805C69  bl 0x82a20ee8
	ctx.lr = 0x8321B284;
	sub_82A20EE8(ctx, base);
	// 8321B284: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8321B288: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B28C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B290: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8321B294: 419A0024  beq cr6, 0x8321b2b8
	if ctx.cr[6].eq {
	pc = 0x8321B2B8; continue 'dispatch;
	}
	// 8321B298: 89610055  lbz r11, 0x55(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8321B29C: 89410056  lbz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8321B2A0: 8921005F  lbz r9, 0x5f(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(95 as u32) ) } as u64;
	// 8321B2A4: 8901005C  lbz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8321B2A8: 99610056  stb r11, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u8 ) };
	// 8321B2AC: 99410055  stb r10, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[10].u8 ) };
	// 8321B2B0: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 8321B2B4: 99010057  stb r8, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[8].u8 ) };
	// 8321B2B8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B2BC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B2C0: 4198004C  blt cr6, 0x8321b30c
	if ctx.cr[6].lt {
	pc = 0x8321B30C; continue 'dispatch;
	}
	// 8321B2C4: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B2C8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B2CC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8321B2D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B2D4: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B2D8: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B2DC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B2E0: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B2E4: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B2E8: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B2EC: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B2F0: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B2F4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B2F8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B2FC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B300: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B304: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B308: 48000014  b 0x8321b31c
	pc = 0x8321B31C; continue 'dispatch;
	// 8321B30C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B310: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8321B314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B318: 4B805BD1  bl 0x82a20ee8
	ctx.lr = 0x8321B31C;
	sub_82A20EE8(ctx, base);
	// 8321B31C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8321B320: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B324: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B328: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8321B32C: 419A0024  beq cr6, 0x8321b350
	if ctx.cr[6].eq {
	pc = 0x8321B350; continue 'dispatch;
	}
	// 8321B330: 89610059  lbz r11, 0x59(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 8321B334: 8941005A  lbz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8321B338: 89210063  lbz r9, 0x63(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(99 as u32) ) } as u64;
	// 8321B33C: 89010060  lbz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8321B340: 9961005A  stb r11, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 8321B344: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 8321B348: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 8321B34C: 9901005B  stb r8, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[8].u8 ) };
	// 8321B350: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321B354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B358: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321B35C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B360: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 8321B364: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321B368: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321B36C: 4E800421  bctrl
	ctx.lr = 0x8321B370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B370: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321B374: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B378: 7D634214  add r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[8].u64;
	// 8321B37C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B380: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 8321B384: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8321B388: 4E800421  bctrl
	ctx.lr = 0x8321B38C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B38C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B390: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B394: 4198004C  blt cr6, 0x8321b3e0
	if ctx.cr[6].lt {
	pc = 0x8321B3E0; continue 'dispatch;
	}
	// 8321B398: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B39C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B3A0: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 8321B3A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B3A8: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B3AC: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B3B0: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B3B4: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B3B8: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B3BC: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B3C0: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B3C4: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B3C8: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B3CC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B3D0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B3D4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B3D8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B3DC: 48000014  b 0x8321b3f0
	pc = 0x8321B3F0; continue 'dispatch;
	// 8321B3E0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B3E4: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 8321B3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B3EC: 4B805AFD  bl 0x82a20ee8
	ctx.lr = 0x8321B3F0;
	sub_82A20EE8(ctx, base);
	// 8321B3F0: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8321B3F4: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B3F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B3FC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8321B400: 419A0024  beq cr6, 0x8321b424
	if ctx.cr[6].eq {
	pc = 0x8321B424; continue 'dispatch;
	}
	// 8321B404: 89610059  lbz r11, 0x59(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 8321B408: 8941005A  lbz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8321B40C: 89210067  lbz r9, 0x67(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(103 as u32) ) } as u64;
	// 8321B410: 89010064  lbz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8321B414: 9961005A  stb r11, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 8321B418: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 8321B41C: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 8321B420: 9901005B  stb r8, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[8].u8 ) };
	// 8321B424: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321B428: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8321B42C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B430: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B434: 4198004C  blt cr6, 0x8321b480
	if ctx.cr[6].lt {
	pc = 0x8321B480; continue 'dispatch;
	}
	// 8321B438: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B43C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B440: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 8321B444: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B448: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B44C: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B450: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B454: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B458: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B45C: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B460: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B464: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B468: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B46C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B470: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B474: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B478: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B47C: 48000014  b 0x8321b490
	pc = 0x8321B490; continue 'dispatch;
	// 8321B480: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B484: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8321B488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B48C: 4B805A5D  bl 0x82a20ee8
	ctx.lr = 0x8321B490;
	sub_82A20EE8(ctx, base);
	// 8321B490: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8321B494: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B498: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B49C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8321B4A0: 419A0024  beq cr6, 0x8321b4c4
	if ctx.cr[6].eq {
	pc = 0x8321B4C4; continue 'dispatch;
	}
	// 8321B4A4: 89610059  lbz r11, 0x59(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 8321B4A8: 8941005A  lbz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8321B4AC: 8921006B  lbz r9, 0x6b(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(107 as u32) ) } as u64;
	// 8321B4B0: 89010068  lbz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8321B4B4: 9961005A  stb r11, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 8321B4B8: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 8321B4BC: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 8321B4C0: 9901005B  stb r8, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[8].u8 ) };
	// 8321B4C4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321B4C8: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8321B4CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B4D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321B4D4: 41980034  blt cr6, 0x8321b508
	if ctx.cr[6].lt {
	pc = 0x8321B508; continue 'dispatch;
	}
	// 8321B4D8: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321B4DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B4E0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B4E4: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8321B4E8: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321B4EC: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321B4F0: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B4F4: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B4F8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B4FC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B500: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B504: 48000014  b 0x8321b518
	pc = 0x8321B518; continue 'dispatch;
	// 8321B508: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321B50C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321B510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B514: 4B8059D5  bl 0x82a20ee8
	ctx.lr = 0x8321B518;
	sub_82A20EE8(ctx, base);
	// 8321B518: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321B51C: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8321B520: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321B524: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321B528: 98FE001C  stb r7, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[7].u8 ) };
	// 8321B52C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B530: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B534: 4198004C  blt cr6, 0x8321b580
	if ctx.cr[6].lt {
	pc = 0x8321B580; continue 'dispatch;
	}
	// 8321B538: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B53C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B540: 3901006C  addi r8, r1, 0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + 108;
	// 8321B544: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B548: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B54C: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B550: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B554: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B558: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B55C: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B560: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B564: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B568: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B56C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B570: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B574: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B578: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B57C: 48000014  b 0x8321b590
	pc = 0x8321B590; continue 'dispatch;
	// 8321B580: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B584: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 8321B588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B58C: 4B80595D  bl 0x82a20ee8
	ctx.lr = 0x8321B590;
	sub_82A20EE8(ctx, base);
	// 8321B590: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B594: C001006C  lfs f0, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B598: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8321B59C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B5A0: 419A0024  beq cr6, 0x8321b5c4
	if ctx.cr[6].eq {
	pc = 0x8321B5C4; continue 'dispatch;
	}
	// 8321B5A4: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321B5A8: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8321B5AC: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321B5B0: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8321B5B4: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8321B5B8: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8321B5BC: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 8321B5C0: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 8321B5C4: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B5C8: D01E0020  stfs f0, 0x20(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8321B5CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B5D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321B5D4: 41980034  blt cr6, 0x8321b608
	if ctx.cr[6].lt {
	pc = 0x8321B608; continue 'dispatch;
	}
	// 8321B5D8: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321B5DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B5E0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B5E4: 39010051  addi r8, r1, 0x51
	ctx.r[8].s64 = ctx.r[1].s64 + 81;
	// 8321B5E8: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321B5EC: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321B5F0: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B5F4: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B5F8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B5FC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B600: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B604: 48000014  b 0x8321b618
	pc = 0x8321B618; continue 'dispatch;
	// 8321B608: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321B60C: 38810051  addi r4, r1, 0x51
	ctx.r[4].s64 = ctx.r[1].s64 + 81;
	// 8321B610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B614: 4B8058D5  bl 0x82a20ee8
	ctx.lr = 0x8321B618;
	sub_82A20EE8(ctx, base);
	// 8321B618: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321B61C: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8321B620: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321B624: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321B628: 98FE0024  stb r7, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 8321B62C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B630: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321B634: 41980034  blt cr6, 0x8321b668
	if ctx.cr[6].lt {
	pc = 0x8321B668; continue 'dispatch;
	}
	// 8321B638: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321B63C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B640: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B644: 39010052  addi r8, r1, 0x52
	ctx.r[8].s64 = ctx.r[1].s64 + 82;
	// 8321B648: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321B64C: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321B650: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B654: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B658: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B65C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B660: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B664: 48000014  b 0x8321b678
	pc = 0x8321B678; continue 'dispatch;
	// 8321B668: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321B66C: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 8321B670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B674: 4B805875  bl 0x82a20ee8
	ctx.lr = 0x8321B678;
	sub_82A20EE8(ctx, base);
	// 8321B678: 89210052  lbz r9, 0x52(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321B67C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8321B680: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8321B684: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321B688: 816AA91C  lwz r11, -0x56e4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-22244 as u32) ) } as u64;
	// 8321B68C: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 8321B690: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8321B694: 98DE0025  stb r6, 0x25(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(37 as u32), ctx.r[6].u8 ) };
	// 8321B698: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B69C: 419800A0  blt cr6, 0x8321b73c
	if ctx.cr[6].lt {
	pc = 0x8321B73C; continue 'dispatch;
	}
	// 8321B6A0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321B6A4: 4198004C  blt cr6, 0x8321b6f0
	if ctx.cr[6].lt {
	pc = 0x8321B6F0; continue 'dispatch;
	}
	// 8321B6A8: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321B6AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B6B0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8321B6B4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B6B8: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321B6BC: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321B6C0: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B6C4: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B6C8: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B6CC: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321B6D0: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B6D4: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321B6D8: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B6DC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B6E0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B6E4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B6E8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321B6EC: 48000014  b 0x8321b700
	pc = 0x8321B700; continue 'dispatch;
	// 8321B6F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B6F4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8321B6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B6FC: 4B8057ED  bl 0x82a20ee8
	ctx.lr = 0x8321B700;
	sub_82A20EE8(ctx, base);
	// 8321B700: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B704: C0010070  lfs f0, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B708: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8321B70C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B710: 419A0024  beq cr6, 0x8321b734
	if ctx.cr[6].eq {
	pc = 0x8321B734; continue 'dispatch;
	}
	// 8321B714: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321B718: 89410055  lbz r10, 0x55(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8321B71C: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321B720: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8321B724: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8321B728: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8321B72C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 8321B730: 99010055  stb r8, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[8].u8 ) };
	// 8321B734: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B738: 4800006C  b 0x8321b7a4
	pc = 0x8321B7A4; continue 'dispatch;
	// 8321B73C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321B740: 41980034  blt cr6, 0x8321b774
	if ctx.cr[6].lt {
	pc = 0x8321B774; continue 'dispatch;
	}
	// 8321B744: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321B748: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B74C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B750: 39010053  addi r8, r1, 0x53
	ctx.r[8].s64 = ctx.r[1].s64 + 83;
	// 8321B754: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321B758: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321B75C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B760: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321B764: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321B768: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321B76C: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321B770: 48000014  b 0x8321b784
	pc = 0x8321B784; continue 'dispatch;
	// 8321B774: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321B778: 38810053  addi r4, r1, 0x53
	ctx.r[4].s64 = ctx.r[1].s64 + 83;
	// 8321B77C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B780: 4B805769  bl 0x82a20ee8
	ctx.lr = 0x8321B784;
	sub_82A20EE8(ctx, base);
	// 8321B784: 89410053  lbz r10, 0x53(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 8321B788: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321B78C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321B790: 419A0010  beq cr6, 0x8321b7a0
	if ctx.cr[6].eq {
	pc = 0x8321B7A0; continue 'dispatch;
	}
	// 8321B794: 394B9484  addi r10, r11, -0x6b7c
	ctx.r[10].s64 = ctx.r[11].s64 + -27516;
	// 8321B798: C00AFE50  lfs f0, -0x1b0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B79C: 48000008  b 0x8321b7a4
	pc = 0x8321B7A4; continue 'dispatch;
	// 8321B7A0: C00B9484  lfs f0, -0x6b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B7A4: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321B7A8: D01E0028  stfs f0, 0x28(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8321B7AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B7B0: 419A000C  beq cr6, 0x8321b7bc
	if ctx.cr[6].eq {
	pc = 0x8321B7BC; continue 'dispatch;
	}
	// 8321B7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B7B8: 4BFFCE09  bl 0x832185c0
	ctx.lr = 0x8321B7BC;
	sub_832185C0(ctx, base);
	// 8321B7BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8321B7C0: 4BA8DC98  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321B7C8 size=112
    let mut pc: u32 = 0x8321B7C8;
    'dispatch: loop {
        match pc {
            0x8321B7C8 => {
    //   block [0x8321B7C8..0x8321B838)
	// 8321B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B7CC: 4BA8DC41  bl 0x82ca940c
	ctx.lr = 0x8321B7D0;
	sub_82CA93D0(ctx, base);
	// 8321B7D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B7D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321B7D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321B7DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B7E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B7E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B7E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321B7EC: 4E800421  bctrl
	ctx.lr = 0x8321B7F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B7F0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B7F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321B7F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B7FC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B800: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321B804: 4E800421  bctrl
	ctx.lr = 0x8321B808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B808: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321B80C: 409A0024  bne cr6, 0x8321b830
	if !ctx.cr[6].eq {
	pc = 0x8321B830; continue 'dispatch;
	}
	// 8321B810: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321B814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B818: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321B81C: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B820: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8321B824: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B828: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8321B82C: 4BFFCD95  bl 0x832185c0
	ctx.lr = 0x8321B830;
	sub_832185C0(ctx, base);
	// 8321B830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321B834: 4BA8DC28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321B838 size=84
    let mut pc: u32 = 0x8321B838;
    'dispatch: loop {
        match pc {
            0x8321B838 => {
    //   block [0x8321B838..0x8321B88C)
	// 8321B838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321B840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321B844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321B848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B84C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321B850: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8321B854: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321B858: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321B85C: 4B806E05  bl 0x82a22660
	ctx.lr = 0x8321B860;
	sub_82A22660(ctx, base);
	// 8321B860: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8321B864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B868: 419A000C  beq cr6, 0x8321b874
	if ctx.cr[6].eq {
	pc = 0x8321B874; continue 'dispatch;
	}
	// 8321B86C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B870: 4BFFCD51  bl 0x832185c0
	ctx.lr = 0x8321B874;
	sub_832185C0(ctx, base);
	// 8321B874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321B878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321B87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321B880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321B884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321B888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321B890 size=212
    let mut pc: u32 = 0x8321B890;
    'dispatch: loop {
        match pc {
            0x8321B890 => {
    //   block [0x8321B890..0x8321B964)
	// 8321B890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B894: 4BA8DB79  bl 0x82ca940c
	ctx.lr = 0x8321B898;
	sub_82CA93D0(ctx, base);
	// 8321B898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B89C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321B8A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321B8A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321B8A8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321B8AC: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8321B8B0: 4198004C  blt cr6, 0x8321b8fc
	if ctx.cr[6].lt {
	pc = 0x8321B8FC; continue 'dispatch;
	}
	// 8321B8B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B8B8: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321B8BC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B8C0: 38EAFFFC  addi r7, r10, -4
	ctx.r[7].s64 = ctx.r[10].s64 + -4;
	// 8321B8C4: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321B8C8: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 8321B8CC: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B8D0: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321B8D4: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321B8D8: 98680001  stb r3, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321B8DC: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321B8E0: 99480002  stb r10, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321B8E4: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321B8E8: 99280003  stb r9, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 8321B8EC: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321B8F0: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321B8F4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321B8F8: 48000014  b 0x8321b90c
	pc = 0x8321B90C; continue 'dispatch;
	// 8321B8FC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321B900: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321B904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B908: 4B8055E1  bl 0x82a20ee8
	ctx.lr = 0x8321B90C;
	sub_82A20EE8(ctx, base);
	// 8321B90C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321B910: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B914: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321B918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B91C: 419A0024  beq cr6, 0x8321b940
	if ctx.cr[6].eq {
	pc = 0x8321B940; continue 'dispatch;
	}
	// 8321B920: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321B924: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321B928: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 8321B92C: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321B930: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 8321B934: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 8321B938: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321B93C: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 8321B940: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8321B944: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321B948: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321B94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B950: 419A000C  beq cr6, 0x8321b95c
	if ctx.cr[6].eq {
	pc = 0x8321B95C; continue 'dispatch;
	}
	// 8321B954: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321B958: 4BFFCC69  bl 0x832185c0
	ctx.lr = 0x8321B95C;
	sub_832185C0(ctx, base);
	// 8321B95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321B960: 4BA8DAFC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321B968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321B968 size=180
    let mut pc: u32 = 0x8321B968;
    'dispatch: loop {
        match pc {
            0x8321B968 => {
    //   block [0x8321B968..0x8321BA1C)
	// 8321B968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321B96C: 4BA8DA9D  bl 0x82ca9408
	ctx.lr = 0x8321B970;
	sub_82CA93D0(ctx, base);
	// 8321B970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321B974: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321B978: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321B97C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321B980: 3BCB9AC4  addi r30, r11, -0x653c
	ctx.r[30].s64 = ctx.r[11].s64 + -25916;
	// 8321B984: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321B988: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B98C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B990: 389EFFE4  addi r4, r30, -0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + -28;
	// 8321B994: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321B998: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321B99C: 4E800421  bctrl
	ctx.lr = 0x8321B9A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321B9A0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321B9A4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321B9A8: 419A006C  beq cr6, 0x8321ba14
	if ctx.cr[6].eq {
	pc = 0x8321BA14; continue 'dispatch;
	}
	// 8321B9AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321B9B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321B9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B9B8: 4B011519  bl 0x8222ced0
	ctx.lr = 0x8321B9BC;
	sub_8222CED0(ctx, base);
	// 8321B9BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321B9C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321B9C4: 409A0010  bne cr6, 0x8321b9d4
	if !ctx.cr[6].eq {
	pc = 0x8321B9D4; continue 'dispatch;
	}
	// 8321B9C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321B9CC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321B9D0: 48000008  b 0x8321b9d8
	pc = 0x8321B9D8; continue 'dispatch;
	// 8321B9D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B9D8: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 8321B9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B9E0: 4B264E71  bl 0x82480850
	ctx.lr = 0x8321B9E4;
	sub_82480850(ctx, base);
	// 8321B9E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321B9E8: 4AFF93F1  bl 0x82214dd8
	ctx.lr = 0x8321B9EC;
	sub_82214DD8(ctx, base);
	// 8321B9EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321B9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321B9F4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321B9F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321B9FC: 4E800421  bctrl
	ctx.lr = 0x8321BA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BA00: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321BA04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321BA08: 419A000C  beq cr6, 0x8321ba14
	if ctx.cr[6].eq {
	pc = 0x8321BA14; continue 'dispatch;
	}
	// 8321BA0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321BA10: 4BFFCBB1  bl 0x832185c0
	ctx.lr = 0x8321BA14;
	sub_832185C0(ctx, base);
	// 8321BA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321BA18: 4BA8DA40  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321BA20 size=96
    let mut pc: u32 = 0x8321BA20;
    'dispatch: loop {
        match pc {
            0x8321BA20 => {
    //   block [0x8321BA20..0x8321BA80)
	// 8321BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BA24: 4BA8D9E9  bl 0x82ca940c
	ctx.lr = 0x8321BA28;
	sub_82CA93D0(ctx, base);
	// 8321BA28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BA2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321BA30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321BA34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321BA38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BA3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321BA40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321BA44: 4E800421  bctrl
	ctx.lr = 0x8321BA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BA48: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BA4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321BA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BA54: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321BA58: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321BA5C: 4E800421  bctrl
	ctx.lr = 0x8321BA60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BA60: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8321BA64: 409A0014  bne cr6, 0x8321ba78
	if !ctx.cr[6].eq {
	pc = 0x8321BA78; continue 'dispatch;
	}
	// 8321BA68: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321BA6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BA70: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321BA74: 4BFFCB4D  bl 0x832185c0
	ctx.lr = 0x8321BA78;
	sub_832185C0(ctx, base);
	// 8321BA78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321BA7C: 4BA8D9E0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321BA80 size=308
    let mut pc: u32 = 0x8321BA80;
    'dispatch: loop {
        match pc {
            0x8321BA80 => {
    //   block [0x8321BA80..0x8321BBB4)
	// 8321BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BA84: 4BA8D981  bl 0x82ca9404
	ctx.lr = 0x8321BA88;
	sub_82CA93D0(ctx, base);
	// 8321BA88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BA8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321BA90: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321BA94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321BA98: 3BCB9DC0  addi r30, r11, -0x6240
	ctx.r[30].s64 = ctx.r[11].s64 + -25152;
	// 8321BA9C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321BAA0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BAA8: 389EFFE0  addi r4, r30, -0x20
	ctx.r[4].s64 = ctx.r[30].s64 + -32;
	// 8321BAAC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321BAB0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321BAB4: 4E800421  bctrl
	ctx.lr = 0x8321BAB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BAB8: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321BABC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321BAC0: 419A00EC  beq cr6, 0x8321bbac
	if ctx.cr[6].eq {
	pc = 0x8321BBAC; continue 'dispatch;
	}
	// 8321BAC4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321BAC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8321BACC: 38EB7088  addi r7, r11, 0x7088
	ctx.r[7].s64 = ctx.r[11].s64 + 28808;
	// 8321BAD0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8321BAD4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8321BAD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321BADC: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8321BAE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8321BAE4: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321BAE8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321BAEC: 4082FFE8  bne 0x8321bad4
	if !ctx.cr[0].eq {
	pc = 0x8321BAD4; continue 'dispatch;
	}
	// 8321BAF0: 389EFFF8  addi r4, r30, -8
	ctx.r[4].s64 = ctx.r[30].s64 + -8;
	// 8321BAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BAF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BAFC: 4B0113D5  bl 0x8222ced0
	ctx.lr = 0x8321BB00;
	sub_8222CED0(ctx, base);
	// 8321BB00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321BB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BB08: 409A0010  bne cr6, 0x8321bb18
	if !ctx.cr[6].eq {
	pc = 0x8321BB18; continue 'dispatch;
	}
	// 8321BB0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321BB10: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8321BB14: 48000008  b 0x8321bb1c
	pc = 0x8321BB1C; continue 'dispatch;
	// 8321BB18: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BB1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8321BB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BB24: 4B7E625D  bl 0x82a01d80
	ctx.lr = 0x8321BB28;
	sub_82A01D80(ctx, base);
	// 8321BB28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB2C: 4AFF92AD  bl 0x82214dd8
	ctx.lr = 0x8321BB30;
	sub_82214DD8(ctx, base);
	// 8321BB30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321BB34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB38: 4AFD3001  bl 0x821eeb38
	ctx.lr = 0x8321BB3C;
	sub_821EEB38(ctx, base);
	// 8321BB3C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB40: 4B9E7CB1  bl 0x82c037f0
	ctx.lr = 0x8321BB44;
	sub_82C037F0(ctx, base);
	// 8321BB44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8321BB48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB4C: 4AFF928D  bl 0x82214dd8
	ctx.lr = 0x8321BB50;
	sub_82214DD8(ctx, base);
	// 8321BB50: 937D0010  stw r27, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8321BB54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321BB58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BB5C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB60: 4B011371  bl 0x8222ced0
	ctx.lr = 0x8321BB64;
	sub_8222CED0(ctx, base);
	// 8321BB64: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 8321BB68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321BB6C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB70: 48019A91  bl 0x83235600
	ctx.lr = 0x8321BB74;
	sub_83235600(ctx, base);
	// 8321BB74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BB78: 4AFF9261  bl 0x82214dd8
	ctx.lr = 0x8321BB7C;
	sub_82214DD8(ctx, base);
	// 8321BB7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BB80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BB84: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321BB88: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321BB8C: 4E800421  bctrl
	ctx.lr = 0x8321BB90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BB90: 5789063E  clrlwi r9, r28, 0x18
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8321BB94: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321BB98: 419A000C  beq cr6, 0x8321bba4
	if ctx.cr[6].eq {
	pc = 0x8321BBA4; continue 'dispatch;
	}
	// 8321BB9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321BBA0: 4BFFCA21  bl 0x832185c0
	ctx.lr = 0x8321BBA4;
	sub_832185C0(ctx, base);
	// 8321BBA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321BBA8: 4AFF9231  bl 0x82214dd8
	ctx.lr = 0x8321BBAC;
	sub_82214DD8(ctx, base);
	// 8321BBAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321BBB0: 4BA8D8A4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321BBB8 size=716
    let mut pc: u32 = 0x8321BBB8;
    'dispatch: loop {
        match pc {
            0x8321BBB8 => {
    //   block [0x8321BBB8..0x8321BE84)
	// 8321BBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BBBC: 4BA8D841  bl 0x82ca93fc
	ctx.lr = 0x8321BBC0;
	sub_82CA93D0(ctx, base);
	// 8321BBC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BBC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321BBC8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321BBCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321BBD0: 3BCB9484  addi r30, r11, -0x6b7c
	ctx.r[30].s64 = ctx.r[11].s64 + -27516;
	// 8321BBD4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8321BBD8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BBE0: 389E0648  addi r4, r30, 0x648
	ctx.r[4].s64 = ctx.r[30].s64 + 1608;
	// 8321BBE4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321BBE8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321BBEC: 4E800421  bctrl
	ctx.lr = 0x8321BBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BBF0: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321BBF4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321BBF8: 419A0284  beq cr6, 0x8321be7c
	if ctx.cr[6].eq {
	pc = 0x8321BE7C; continue 'dispatch;
	}
	// 8321BBFC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8321BC00: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8321BC04: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8321BC08: 39097088  addi r8, r9, 0x7088
	ctx.r[8].s64 = ctx.r[9].s64 + 28808;
	// 8321BC0C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321BC10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321BC14: 7D604028  lwarx r11, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321BC18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321BC1C: 7D60412D  stwcx. r11, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321BC20: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321BC24: 4082FFE8  bne 0x8321bc0c
	if !ctx.cr[0].eq {
	pc = 0x8321BC0C; continue 'dispatch;
	}
	// 8321BC28: 389E0668  addi r4, r30, 0x668
	ctx.r[4].s64 = ctx.r[30].s64 + 1640;
	// 8321BC2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BC30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC34: 4B01129D  bl 0x8222ced0
	ctx.lr = 0x8321BC38;
	sub_8222CED0(ctx, base);
	// 8321BC38: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321BC3C: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 8321BC40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321BC44: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321BC48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BC4C: 419A0008  beq cr6, 0x8321bc54
	if ctx.cr[6].eq {
	pc = 0x8321BC54; continue 'dispatch;
	}
	// 8321BC50: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BC54: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8321BC58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BC5C: 4B7E6125  bl 0x82a01d80
	ctx.lr = 0x8321BC60;
	sub_82A01D80(ctx, base);
	// 8321BC60: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC64: 4AFF9175  bl 0x82214dd8
	ctx.lr = 0x8321BC68;
	sub_82214DD8(ctx, base);
	// 8321BC68: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321BC6C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC70: 4AFD2EC9  bl 0x821eeb38
	ctx.lr = 0x8321BC74;
	sub_821EEB38(ctx, base);
	// 8321BC74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC78: 4B9E7B79  bl 0x82c037f0
	ctx.lr = 0x8321BC7C;
	sub_82C037F0(ctx, base);
	// 8321BC7C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8321BC80: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC84: 4AFF9155  bl 0x82214dd8
	ctx.lr = 0x8321BC88;
	sub_82214DD8(ctx, base);
	// 8321BC88: 933D0010  stw r25, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 8321BC8C: 389E0678  addi r4, r30, 0x678
	ctx.r[4].s64 = ctx.r[30].s64 + 1656;
	// 8321BC90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BC94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BC98: 4B011239  bl 0x8222ced0
	ctx.lr = 0x8321BC9C;
	sub_8222CED0(ctx, base);
	// 8321BC9C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321BCA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321BCA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BCA8: 419A0008  beq cr6, 0x8321bcb0
	if ctx.cr[6].eq {
	pc = 0x8321BCB0; continue 'dispatch;
	}
	// 8321BCAC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BCB0: 38BD0014  addi r5, r29, 0x14
	ctx.r[5].s64 = ctx.r[29].s64 + 20;
	// 8321BCB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BCB8: 4B264B11  bl 0x824807c8
	ctx.lr = 0x8321BCBC;
	sub_824807C8(ctx, base);
	// 8321BCBC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BCC0: 4AFF9119  bl 0x82214dd8
	ctx.lr = 0x8321BCC4;
	sub_82214DD8(ctx, base);
	// 8321BCC4: 389E0688  addi r4, r30, 0x688
	ctx.r[4].s64 = ctx.r[30].s64 + 1672;
	// 8321BCC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BCCC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BCD0: 4B011201  bl 0x8222ced0
	ctx.lr = 0x8321BCD4;
	sub_8222CED0(ctx, base);
	// 8321BCD4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321BCD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321BCDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BCE0: 419A0008  beq cr6, 0x8321bce8
	if ctx.cr[6].eq {
	pc = 0x8321BCE8; continue 'dispatch;
	}
	// 8321BCE4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BCE8: 38BD0018  addi r5, r29, 0x18
	ctx.r[5].s64 = ctx.r[29].s64 + 24;
	// 8321BCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BCF0: 4B264AD9  bl 0x824807c8
	ctx.lr = 0x8321BCF4;
	sub_824807C8(ctx, base);
	// 8321BCF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BCF8: 4AFF90E1  bl 0x82214dd8
	ctx.lr = 0x8321BCFC;
	sub_82214DD8(ctx, base);
	// 8321BCFC: 389E0698  addi r4, r30, 0x698
	ctx.r[4].s64 = ctx.r[30].s64 + 1688;
	// 8321BD00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BD04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD08: 4B0111C9  bl 0x8222ced0
	ctx.lr = 0x8321BD0C;
	sub_8222CED0(ctx, base);
	// 8321BD0C: 38BD001C  addi r5, r29, 0x1c
	ctx.r[5].s64 = ctx.r[29].s64 + 28;
	// 8321BD10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321BD14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD18: 480198E9  bl 0x83235600
	ctx.lr = 0x8321BD1C;
	sub_83235600(ctx, base);
	// 8321BD1C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD20: 4AFF90B9  bl 0x82214dd8
	ctx.lr = 0x8321BD24;
	sub_82214DD8(ctx, base);
	// 8321BD24: 389E06B4  addi r4, r30, 0x6b4
	ctx.r[4].s64 = ctx.r[30].s64 + 1716;
	// 8321BD28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BD2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD30: 4B0111A1  bl 0x8222ced0
	ctx.lr = 0x8321BD34;
	sub_8222CED0(ctx, base);
	// 8321BD34: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321BD38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321BD3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BD40: 419A0008  beq cr6, 0x8321bd48
	if ctx.cr[6].eq {
	pc = 0x8321BD48; continue 'dispatch;
	}
	// 8321BD44: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BD48: 38BD0020  addi r5, r29, 0x20
	ctx.r[5].s64 = ctx.r[29].s64 + 32;
	// 8321BD4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BD50: 4B264B01  bl 0x82480850
	ctx.lr = 0x8321BD54;
	sub_82480850(ctx, base);
	// 8321BD54: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD58: 4AFF9081  bl 0x82214dd8
	ctx.lr = 0x8321BD5C;
	sub_82214DD8(ctx, base);
	// 8321BD5C: 389E06CC  addi r4, r30, 0x6cc
	ctx.r[4].s64 = ctx.r[30].s64 + 1740;
	// 8321BD60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BD64: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD68: 4B011169  bl 0x8222ced0
	ctx.lr = 0x8321BD6C;
	sub_8222CED0(ctx, base);
	// 8321BD6C: 38BD0024  addi r5, r29, 0x24
	ctx.r[5].s64 = ctx.r[29].s64 + 36;
	// 8321BD70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321BD74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD78: 48019889  bl 0x83235600
	ctx.lr = 0x8321BD7C;
	sub_83235600(ctx, base);
	// 8321BD7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD80: 4AFF9059  bl 0x82214dd8
	ctx.lr = 0x8321BD84;
	sub_82214DD8(ctx, base);
	// 8321BD84: 389E06E4  addi r4, r30, 0x6e4
	ctx.r[4].s64 = ctx.r[30].s64 + 1764;
	// 8321BD88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BD8C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BD90: 4B011141  bl 0x8222ced0
	ctx.lr = 0x8321BD94;
	sub_8222CED0(ctx, base);
	// 8321BD94: 38BD0025  addi r5, r29, 0x25
	ctx.r[5].s64 = ctx.r[29].s64 + 37;
	// 8321BD98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321BD9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BDA0: 48019861  bl 0x83235600
	ctx.lr = 0x8321BDA4;
	sub_83235600(ctx, base);
	// 8321BDA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BDA8: 4AFF9031  bl 0x82214dd8
	ctx.lr = 0x8321BDAC;
	sub_82214DD8(ctx, base);
	// 8321BDAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BDB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BDB4: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8321BDB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321BDBC: 4E800421  bctrl
	ctx.lr = 0x8321BDC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BDC0: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321BDC4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321BDC8: 419A004C  beq cr6, 0x8321be14
	if ctx.cr[6].eq {
	pc = 0x8321BE14; continue 'dispatch;
	}
	// 8321BDCC: 9B610050  stb r27, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u8 ) };
	// 8321BDD0: 389E06F8  addi r4, r30, 0x6f8
	ctx.r[4].s64 = ctx.r[30].s64 + 1784;
	// 8321BDD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BDD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BDDC: 4B0110F5  bl 0x8222ced0
	ctx.lr = 0x8321BDE0;
	sub_8222CED0(ctx, base);
	// 8321BDE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8321BDE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321BDE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BDEC: 48019815  bl 0x83235600
	ctx.lr = 0x8321BDF0;
	sub_83235600(ctx, base);
	// 8321BDF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BDF4: 4AFF8FE5  bl 0x82214dd8
	ctx.lr = 0x8321BDF8;
	sub_82214DD8(ctx, base);
	// 8321BDF8: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321BDFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321BE00: 419A000C  beq cr6, 0x8321be0c
	if ctx.cr[6].eq {
	pc = 0x8321BE0C; continue 'dispatch;
	}
	// 8321BE04: C01EFE50  lfs f0, -0x1b0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321BE08: 48000008  b 0x8321be10
	pc = 0x8321BE10; continue 'dispatch;
	// 8321BE0C: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321BE10: D01D0028  stfs f0, 0x28(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8321BE14: 389E0708  addi r4, r30, 0x708
	ctx.r[4].s64 = ctx.r[30].s64 + 1800;
	// 8321BE18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321BE1C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BE20: 4B0110B1  bl 0x8222ced0
	ctx.lr = 0x8321BE24;
	sub_8222CED0(ctx, base);
	// 8321BE24: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8321BE28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321BE2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BE30: 419A0008  beq cr6, 0x8321be38
	if ctx.cr[6].eq {
	pc = 0x8321BE38; continue 'dispatch;
	}
	// 8321BE34: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BE38: 38BD0028  addi r5, r29, 0x28
	ctx.r[5].s64 = ctx.r[29].s64 + 40;
	// 8321BE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BE40: 4B264A11  bl 0x82480850
	ctx.lr = 0x8321BE44;
	sub_82480850(ctx, base);
	// 8321BE44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321BE48: 4AFF8F91  bl 0x82214dd8
	ctx.lr = 0x8321BE4C;
	sub_82214DD8(ctx, base);
	// 8321BE4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321BE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BE54: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321BE58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321BE5C: 4E800421  bctrl
	ctx.lr = 0x8321BE60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321BE60: 5749063E  clrlwi r9, r26, 0x18
	ctx.r[9].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 8321BE64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321BE68: 419A000C  beq cr6, 0x8321be74
	if ctx.cr[6].eq {
	pc = 0x8321BE74; continue 'dispatch;
	}
	// 8321BE6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321BE70: 4BFFC751  bl 0x832185c0
	ctx.lr = 0x8321BE74;
	sub_832185C0(ctx, base);
	// 8321BE74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321BE78: 4AFF8F61  bl 0x82214dd8
	ctx.lr = 0x8321BE7C;
	sub_82214DD8(ctx, base);
	// 8321BE7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8321BE80: 4BA8D5CC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321BE88 size=44
    let mut pc: u32 = 0x8321BE88;
    'dispatch: loop {
        match pc {
            0x8321BE88 => {
    //   block [0x8321BE88..0x8321BEB4)
	// 8321BE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321BE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BE94: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8321BE98: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8321BE9C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8321BEA0: 4B7FEB59  bl 0x82a1a9f8
	ctx.lr = 0x8321BEA4;
	sub_82A1A9F8(ctx, base);
	// 8321BEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321BEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321BEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321BEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321BEB8 size=156
    let mut pc: u32 = 0x8321BEB8;
    'dispatch: loop {
        match pc {
            0x8321BEB8 => {
    //   block [0x8321BEB8..0x8321BF54)
	// 8321BEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321BEC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321BEC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BEC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321BECC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321BED0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8321BED4: 394B2848  addi r10, r11, 0x2848
	ctx.r[10].s64 = ctx.r[11].s64 + 10312;
	// 8321BED8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BEDC: 4B00337D  bl 0x8221f258
	ctx.lr = 0x8321BEE0;
	sub_8221F258(ctx, base);
	// 8321BEE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8321BEE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8321BEE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321BEEC: 419A0008  beq cr6, 0x8321bef4
	if ctx.cr[6].eq {
	pc = 0x8321BEF4; continue 'dispatch;
	}
	// 8321BEF0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BEF4: 352B0004  addic. r9, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8321BEF8: 41820008  beq 0x8321bf00
	if ctx.cr[0].eq {
	pc = 0x8321BF00; continue 'dispatch;
	}
	// 8321BEFC: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BF00: 352B0008  addic. r9, r11, 8
	ctx.xer.ca = (ctx.r[11].u32 > (!(8 as u32)));
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8321BF04: 41820008  beq 0x8321bf0c
	if ctx.cr[0].eq {
	pc = 0x8321BF0C; continue 'dispatch;
	}
	// 8321BF08: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BF0C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8321BF10: 994B0011  stb r10, 0x11(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 8321BF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321BF18: 992B0010  stb r9, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 8321BF1C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8321BF20: 992B0011  stb r9, 0x11(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(17 as u32), ctx.r[9].u8 ) };
	// 8321BF24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BF28: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321BF2C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BF30: 91290000  stw r9, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321BF34: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BF38: 91080008  stw r8, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8321BF3C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8321BF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321BF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321BF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321BF4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321BF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321BF58 size=120
    let mut pc: u32 = 0x8321BF58;
    'dispatch: loop {
        match pc {
            0x8321BF58 => {
    //   block [0x8321BF58..0x8321BFD0)
	// 8321BF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321BF60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321BF64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321BF68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BF6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321BF70: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321BF74: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 8321BF78: 394B2848  addi r10, r11, 0x2848
	ctx.r[10].s64 = ctx.r[11].s64 + 10312;
	// 8321BF7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321BF80: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BF84: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BF88: 80890004  lwz r4, 4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321BF8C: 4B7FEB75  bl 0x82a1ab00
	ctx.lr = 0x8321BF90;
	sub_82A1AB00(ctx, base);
	// 8321BF90: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8321BF98: 90E70004  stw r7, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8321BF9C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321BFA4: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8321BFA8: 90C60000  stw r6, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8321BFAC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321BFB0: 90A50008  stw r5, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8321BFB4: 4B5B6FBD  bl 0x827d2f70
	ctx.lr = 0x8321BFB8;
	sub_827D2F70(ctx, base);
	// 8321BFB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321BFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321BFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321BFC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321BFC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321BFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321BFD0 size=16
    let mut pc: u32 = 0x8321BFD0;
    'dispatch: loop {
        match pc {
            0x8321BFD0 => {
    //   block [0x8321BFD0..0x8321BFE0)
	// 8321BFD0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321BFD4: 394B2980  addi r10, r11, 0x2980
	ctx.r[10].s64 = ctx.r[11].s64 + 10624;
	// 8321BFD8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321BFDC: 4BFFFF7C  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321BFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321BFE0 size=144
    let mut pc: u32 = 0x8321BFE0;
    'dispatch: loop {
        match pc {
            0x8321BFE0 => {
    //   block [0x8321BFE0..0x8321C070)
	// 8321BFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321BFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321BFE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321BFEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321BFF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321BFF4: 4BFFFEC5  bl 0x8321beb8
	ctx.lr = 0x8321BFF8;
	sub_8321BEB8(ctx, base);
	// 8321BFF8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321BFFC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8321C000: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8321C004: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8321C008: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8321C00C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C010: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8321C014: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321C018: 3CA0820F  lis r5, -0x7df1
	ctx.r[5].s64 = -2112946176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C070 size=16
    let mut pc: u32 = 0x8321C070;
    'dispatch: loop {
        match pc {
            0x8321C070 => {
    //   block [0x8321C070..0x8321C080)
	// 8321C070: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C074: 394B2968  addi r10, r11, 0x2968
	ctx.r[10].s64 = ctx.r[11].s64 + 10600;
	// 8321C078: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C07C: 4BFFFEDC  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C080 size=76
    let mut pc: u32 = 0x8321C080;
    'dispatch: loop {
        match pc {
            0x8321C080 => {
    //   block [0x8321C080..0x8321C0CC)
	// 8321C080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C08C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C090: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C094: 4BFFFE25  bl 0x8321beb8
	ctx.lr = 0x8321C098;
	sub_8321BEB8(ctx, base);
	// 8321C098: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C09C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8321C0A0: 392B2968  addi r9, r11, 0x2968
	ctx.r[9].s64 = ctx.r[11].s64 + 10600;
	// 8321C0A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C0A8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C0AC: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C0B0: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C0B4: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321C0B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C0C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C0D0 size=88
    let mut pc: u32 = 0x8321C0D0;
    'dispatch: loop {
        match pc {
            0x8321C0D0 => {
    //   block [0x8321C0D0..0x8321C128)
	// 8321C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C0D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C0DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C0E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C0E4: 4BFFFDD5  bl 0x8321beb8
	ctx.lr = 0x8321C0E8;
	sub_8321BEB8(ctx, base);
	// 8321C0E8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C0EC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C0F0: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8321C0F4: 390A2950  addi r8, r10, 0x2950
	ctx.r[8].s64 = ctx.r[10].s64 + 10576;
	// 8321C0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C0FC: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C100: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321C104: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321C108: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C10C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C110: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8321C114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C128 size=72
    let mut pc: u32 = 0x8321C128;
    'dispatch: loop {
        match pc {
            0x8321C128 => {
    //   block [0x8321C128..0x8321C170)
	// 8321C128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C13C: 4BFFFD7D  bl 0x8321beb8
	ctx.lr = 0x8321C140;
	sub_8321BEB8(ctx, base);
	// 8321C140: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C144: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C14C: 392A2708  addi r9, r10, 0x2708
	ctx.r[9].s64 = ctx.r[10].s64 + 9992;
	// 8321C150: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C154: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C158: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C15C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C170 size=104
    let mut pc: u32 = 0x8321C170;
    'dispatch: loop {
        match pc {
            0x8321C170 => {
    //   block [0x8321C170..0x8321C1D8)
	// 8321C170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C17C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C184: 4BFFFD35  bl 0x8321beb8
	ctx.lr = 0x8321C188;
	sub_8321BEB8(ctx, base);
	// 8321C188: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C18C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C190: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8321C194: 390A26F0  addi r8, r10, 0x26f0
	ctx.r[8].s64 = ctx.r[10].s64 + 9968;
	// 8321C198: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8321C19C: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C1A0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321C1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C1A8: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C1AC: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C1B0: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321C1B4: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8321C1B8: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8321C1BC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8321C1C0: 98FF0024  stb r7, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 8321C1C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C1D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C1D8 size=128
    let mut pc: u32 = 0x8321C1D8;
    'dispatch: loop {
        match pc {
            0x8321C1D8 => {
    //   block [0x8321C1D8..0x8321C258)
	// 8321C1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C1E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C1E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C1EC: 4BFFFCCD  bl 0x8321beb8
	ctx.lr = 0x8321C1F0;
	sub_8321BEB8(ctx, base);
	// 8321C1F0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C1F4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8321C1F8: 392B26D8  addi r9, r11, 0x26d8
	ctx.r[9].s64 = ctx.r[11].s64 + 9944;
	// 8321C1FC: 388A9D80  addi r4, r10, -0x6280
	ctx.r[4].s64 = ctx.r[10].s64 + -25216;
	// 8321C200: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321C208: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321C20C: 4B010CC5  bl 0x8222ced0
	ctx.lr = 0x8321C210;
	sub_8222CED0(ctx, base);
	// 8321C210: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321C214: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321C218: 4AFD2921  bl 0x821eeb38
	ctx.lr = 0x8321C21C;
	sub_821EEB38(ctx, base);
	// 8321C21C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321C220: 4B9E75D1  bl 0x82c037f0
	ctx.lr = 0x8321C224;
	sub_82C037F0(ctx, base);
	// 8321C224: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8321C228: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321C22C: 4AFF8BAD  bl 0x82214dd8
	ctx.lr = 0x8321C230;
	sub_82214DD8(ctx, base);
	// 8321C230: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321C234: 4AFF8BA5  bl 0x82214dd8
	ctx.lr = 0x8321C238;
	sub_82214DD8(ctx, base);
	// 8321C238: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8321C23C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C240: 991F0014  stb r8, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 8321C244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321C248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C258 size=68
    let mut pc: u32 = 0x8321C258;
    'dispatch: loop {
        match pc {
            0x8321C258 => {
    //   block [0x8321C258..0x8321C29C)
	// 8321C258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C26C: 4BFFFC4D  bl 0x8321beb8
	ctx.lr = 0x8321C270;
	sub_8321BEB8(ctx, base);
	// 8321C270: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8321C278: 392B26C0  addi r9, r11, 0x26c0
	ctx.r[9].s64 = ctx.r[11].s64 + 9920;
	// 8321C27C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321C280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C284: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C2A0 size=72
    let mut pc: u32 = 0x8321C2A0;
    'dispatch: loop {
        match pc {
            0x8321C2A0 => {
    //   block [0x8321C2A0..0x8321C2E8)
	// 8321C2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C2A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C2AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C2B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C2B4: 4BFFFC05  bl 0x8321beb8
	ctx.lr = 0x8321C2B8;
	sub_8321BEB8(ctx, base);
	// 8321C2B8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C2BC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C2C4: 392A26A8  addi r9, r10, 0x26a8
	ctx.r[9].s64 = ctx.r[10].s64 + 9896;
	// 8321C2C8: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C2CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C2D0: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C2D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C2E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C2E8 size=16
    let mut pc: u32 = 0x8321C2E8;
    'dispatch: loop {
        match pc {
            0x8321C2E8 => {
    //   block [0x8321C2E8..0x8321C2F8)
	// 8321C2E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C2EC: 394B2920  addi r10, r11, 0x2920
	ctx.r[10].s64 = ctx.r[11].s64 + 10528;
	// 8321C2F0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C2F4: 4BFFFC64  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C2F8 size=84
    let mut pc: u32 = 0x8321C2F8;
    'dispatch: loop {
        match pc {
            0x8321C2F8 => {
    //   block [0x8321C2F8..0x8321C34C)
	// 8321C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C30C: 4BFFFBAD  bl 0x8321beb8
	ctx.lr = 0x8321C310;
	sub_8321BEB8(ctx, base);
	// 8321C310: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C314: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C318: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8321C31C: 390A2920  addi r8, r10, 0x2920
	ctx.r[8].s64 = ctx.r[10].s64 + 10528;
	// 8321C320: 993F0018  stb r9, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 8321C324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C328: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C32C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321C330: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C334: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321C338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C350 size=16
    let mut pc: u32 = 0x8321C350;
    'dispatch: loop {
        match pc {
            0x8321C350 => {
    //   block [0x8321C350..0x8321C360)
	// 8321C350: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C354: 394B2908  addi r10, r11, 0x2908
	ctx.r[10].s64 = ctx.r[11].s64 + 10504;
	// 8321C358: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C35C: 4BFFFBFC  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C360 size=68
    let mut pc: u32 = 0x8321C360;
    'dispatch: loop {
        match pc {
            0x8321C360 => {
    //   block [0x8321C360..0x8321C3A4)
	// 8321C360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C36C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C374: 4BFFFB45  bl 0x8321beb8
	ctx.lr = 0x8321C378;
	sub_8321BEB8(ctx, base);
	// 8321C378: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C37C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8321C380: 392B2908  addi r9, r11, 0x2908
	ctx.r[9].s64 = ctx.r[11].s64 + 10504;
	// 8321C384: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321C388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C38C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C39C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C3A8 size=16
    let mut pc: u32 = 0x8321C3A8;
    'dispatch: loop {
        match pc {
            0x8321C3A8 => {
    //   block [0x8321C3A8..0x8321C3B8)
	// 8321C3A8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C3AC: 394B28F0  addi r10, r11, 0x28f0
	ctx.r[10].s64 = ctx.r[11].s64 + 10480;
	// 8321C3B0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C3B4: 4BFFFBA4  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C3B8 size=68
    let mut pc: u32 = 0x8321C3B8;
    'dispatch: loop {
        match pc {
            0x8321C3B8 => {
    //   block [0x8321C3B8..0x8321C3FC)
	// 8321C3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C3C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C3C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C3C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C3CC: 4BFFFAED  bl 0x8321beb8
	ctx.lr = 0x8321C3D0;
	sub_8321BEB8(ctx, base);
	// 8321C3D0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C3D4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8321C3D8: 392B28F0  addi r9, r11, 0x28f0
	ctx.r[9].s64 = ctx.r[11].s64 + 10480;
	// 8321C3DC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321C3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C3E4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C3F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C400 size=16
    let mut pc: u32 = 0x8321C400;
    'dispatch: loop {
        match pc {
            0x8321C400 => {
    //   block [0x8321C400..0x8321C410)
	// 8321C400: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C404: 394B28D8  addi r10, r11, 0x28d8
	ctx.r[10].s64 = ctx.r[11].s64 + 10456;
	// 8321C408: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C40C: 4BFFFB4C  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C410 size=68
    let mut pc: u32 = 0x8321C410;
    'dispatch: loop {
        match pc {
            0x8321C410 => {
    //   block [0x8321C410..0x8321C454)
	// 8321C410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C41C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C424: 4BFFFA95  bl 0x8321beb8
	ctx.lr = 0x8321C428;
	sub_8321BEB8(ctx, base);
	// 8321C428: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C42C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8321C430: 392B28D8  addi r9, r11, 0x28d8
	ctx.r[9].s64 = ctx.r[11].s64 + 10456;
	// 8321C434: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321C438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C43C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C44C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C458 size=16
    let mut pc: u32 = 0x8321C458;
    'dispatch: loop {
        match pc {
            0x8321C458 => {
    //   block [0x8321C458..0x8321C468)
	// 8321C458: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C45C: 394B28C0  addi r10, r11, 0x28c0
	ctx.r[10].s64 = ctx.r[11].s64 + 10432;
	// 8321C460: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C464: 4BFFFAF4  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C468 size=92
    let mut pc: u32 = 0x8321C468;
    'dispatch: loop {
        match pc {
            0x8321C468 => {
    //   block [0x8321C468..0x8321C4C4)
	// 8321C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C474: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C47C: 4BFFFA3D  bl 0x8321beb8
	ctx.lr = 0x8321C480;
	sub_8321BEB8(ctx, base);
	// 8321C480: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C484: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8321C488: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8321C48C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8321C490: 38E928C0  addi r7, r9, 0x28c0
	ctx.r[7].s64 = ctx.r[9].s64 + 10432;
	// 8321C494: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C49C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321C4A0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C4C8 size=16
    let mut pc: u32 = 0x8321C4C8;
    'dispatch: loop {
        match pc {
            0x8321C4C8 => {
    //   block [0x8321C4C8..0x8321C4D8)
	// 8321C4C8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C4CC: 394B28A8  addi r10, r11, 0x28a8
	ctx.r[10].s64 = ctx.r[11].s64 + 10408;
	// 8321C4D0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C4D4: 4BFFFA84  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321C4D8 size=68
    let mut pc: u32 = 0x8321C4D8;
    'dispatch: loop {
        match pc {
            0x8321C4D8 => {
    //   block [0x8321C4D8..0x8321C51C)
	// 8321C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C4E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C4E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C4E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C4EC: 4BFFF9CD  bl 0x8321beb8
	ctx.lr = 0x8321C4F0;
	sub_8321BEB8(ctx, base);
	// 8321C4F0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C4F4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8321C4F8: 392B28A8  addi r9, r11, 0x28a8
	ctx.r[9].s64 = ctx.r[11].s64 + 10408;
	// 8321C4FC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8321C500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C504: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C520 size=16
    let mut pc: u32 = 0x8321C520;
    'dispatch: loop {
        match pc {
            0x8321C520 => {
    //   block [0x8321C520..0x8321C530)
	// 8321C520: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C524: 394B2890  addi r10, r11, 0x2890
	ctx.r[10].s64 = ctx.r[11].s64 + 10384;
	// 8321C528: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C52C: 4BFFFA2C  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C530 size=76
    let mut pc: u32 = 0x8321C530;
    'dispatch: loop {
        match pc {
            0x8321C530 => {
    //   block [0x8321C530..0x8321C57C)
	// 8321C530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C53C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C544: 4BFFF975  bl 0x8321beb8
	ctx.lr = 0x8321C548;
	sub_8321BEB8(ctx, base);
	// 8321C548: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C54C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8321C550: 392B2890  addi r9, r11, 0x2890
	ctx.r[9].s64 = ctx.r[11].s64 + 10384;
	// 8321C554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C558: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C55C: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C560: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C564: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8321C568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C580 size=16
    let mut pc: u32 = 0x8321C580;
    'dispatch: loop {
        match pc {
            0x8321C580 => {
    //   block [0x8321C580..0x8321C590)
	// 8321C580: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C584: 394B2878  addi r10, r11, 0x2878
	ctx.r[10].s64 = ctx.r[11].s64 + 10360;
	// 8321C588: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C58C: 4BFFF9CC  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C590 size=108
    let mut pc: u32 = 0x8321C590;
    'dispatch: loop {
        match pc {
            0x8321C590 => {
    //   block [0x8321C590..0x8321C5FC)
	// 8321C590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C59C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C5A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C5A4: 4BFFF915  bl 0x8321beb8
	ctx.lr = 0x8321C5A8;
	sub_8321BEB8(ctx, base);
	// 8321C5A8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C5AC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 8321C5B0: 390B2878  addi r8, r11, 0x2878
	ctx.r[8].s64 = ctx.r[11].s64 + 10360;
	// 8321C5B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8321C5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8321C5BC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321C5C0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8321C5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C5C8: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C5CC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8321C5D0: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8321C5D4: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8321C5D8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8321C5DC: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8321C5E0: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 8321C5E4: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 8321C5E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C5F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C600 size=16
    let mut pc: u32 = 0x8321C600;
    'dispatch: loop {
        match pc {
            0x8321C600 => {
    //   block [0x8321C600..0x8321C610)
	// 8321C600: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C604: 394B2860  addi r10, r11, 0x2860
	ctx.r[10].s64 = ctx.r[11].s64 + 10336;
	// 8321C608: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C60C: 4BFFF94C  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C610 size=72
    let mut pc: u32 = 0x8321C610;
    'dispatch: loop {
        match pc {
            0x8321C610 => {
    //   block [0x8321C610..0x8321C658)
	// 8321C610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C618: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C61C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C620: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C624: 4BFFF895  bl 0x8321beb8
	ctx.lr = 0x8321C628;
	sub_8321BEB8(ctx, base);
	// 8321C628: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C62C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8321C630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C634: 392A2860  addi r9, r10, 0x2860
	ctx.r[9].s64 = ctx.r[10].s64 + 10336;
	// 8321C638: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C63C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321C640: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8321C644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321C648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321C64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321C650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321C654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321C658 size=16
    let mut pc: u32 = 0x8321C658;
    'dispatch: loop {
        match pc {
            0x8321C658 => {
    //   block [0x8321C658..0x8321C668)
	// 8321C658: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321C65C: 394B2920  addi r10, r11, 0x2920
	ctx.r[10].s64 = ctx.r[11].s64 + 10528;
	// 8321C660: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321C664: 4BFFF8F4  b 0x8321bf58
	sub_8321BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C668 size=168
    let mut pc: u32 = 0x8321C668;
    'dispatch: loop {
        match pc {
            0x8321C668 => {
    //   block [0x8321C668..0x8321C710)
	// 8321C668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321C670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321C674: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C67C: 4BFFF83D  bl 0x8321beb8
	ctx.lr = 0x8321C680;
	sub_8321BEB8(ctx, base);
	// 8321C680: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C684: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 8321C688: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8321C68C: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8321C690: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8321C694: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C698: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8321C69C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321C6A0: 3CA0820F  lis r5, -0x7df1
	ctx.r[5].s64 = -2112946176;
	// 8321C6A4: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C6A8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8321C6AC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8321C6B0: 38652938  addi r3, r5, 0x2938
	ctx.r[3].s64 = ctx.r[5].s64 + 10552;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C710 size=16
    let mut pc: u32 = 0x8321C710;
    'dispatch: loop {
        match pc {
            0x8321C710 => {
    //   block [0x8321C710..0x8321C720)
	// 8321C710: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C714: D0230004  stfs f1, 4(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C718: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8321C71C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C720 size=20
    let mut pc: u32 = 0x8321C720;
    'dispatch: loop {
        match pc {
            0x8321C720 => {
    //   block [0x8321C720..0x8321C734)
	// 8321C720: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C724: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C728: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321C72C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C738 size=28
    let mut pc: u32 = 0x8321C738;
    'dispatch: loop {
        match pc {
            0x8321C738 => {
    //   block [0x8321C738..0x8321C754)
	// 8321C738: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C73C: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C740: C00B9490  lfs f0, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C744: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321C748: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C74C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8321C750: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C754 size=8
    let mut pc: u32 = 0x8321C754;
    'dispatch: loop {
        match pc {
            0x8321C754 => {
    //   block [0x8321C754..0x8321C75C)
	// 8321C754: D0030004  stfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C760 size=28
    let mut pc: u32 = 0x8321C760;
    'dispatch: loop {
        match pc {
            0x8321C760 => {
    //   block [0x8321C760..0x8321C77C)
	// 8321C760: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C764: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8321C768: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8321C76C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C770: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8321C774: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C780 size=36
    let mut pc: u32 = 0x8321C780;
    'dispatch: loop {
        match pc {
            0x8321C780 => {
    //   block [0x8321C780..0x8321C7A4)
	// 8321C780: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321C784: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C788: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321C78C: 392AF1BC  addi r9, r10, -0xe44
	ctx.r[9].s64 = ctx.r[10].s64 + -3652;
	// 8321C790: 816B6DD4  lwz r11, 0x6dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C794: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321C798: 7DA84C2E  lfsx f13, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C79C: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8321C7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C7A8 size=44
    let mut pc: u32 = 0x8321C7A8;
    'dispatch: loop {
        match pc {
            0x8321C7A8 => {
    //   block [0x8321C7A8..0x8321C7D4)
	// 8321C7A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321C7AC: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C7B0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321C7B4: 392AF1BC  addi r9, r10, -0xe44
	ctx.r[9].s64 = ctx.r[10].s64 + -3652;
	// 8321C7B8: 816B6DD4  lwz r11, 0x6dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C7BC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321C7C0: 7C084C2E  lfsx f0, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C7C4: ED800072  fmuls f12, f0, f1
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8321C7C8: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C7CC: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8321C7D0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C7D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C7D4 size=20
    let mut pc: u32 = 0x8321C7D4;
    'dispatch: loop {
        match pc {
            0x8321C7D4 => {
    //   block [0x8321C7D4..0x8321C7E8)
	// 8321C7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C7D8: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C7DC: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321C7E0: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C7E8 size=36
    let mut pc: u32 = 0x8321C7E8;
    'dispatch: loop {
        match pc {
            0x8321C7E8 => {
    //   block [0x8321C7E8..0x8321C80C)
	// 8321C7E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321C7EC: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C7F0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321C7F4: 392AF1BC  addi r9, r10, -0xe44
	ctx.r[9].s64 = ctx.r[10].s64 + -3652;
	// 8321C7F8: 816B6DD4  lwz r11, 0x6dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C7FC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321C800: 7DA84C2E  lfsx f13, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C804: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8321C808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C810 size=44
    let mut pc: u32 = 0x8321C810;
    'dispatch: loop {
        match pc {
            0x8321C810 => {
    //   block [0x8321C810..0x8321C83C)
	// 8321C810: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321C814: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C818: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321C81C: 392AF1BC  addi r9, r10, -0xe44
	ctx.r[9].s64 = ctx.r[10].s64 + -3652;
	// 8321C820: 816B6DD4  lwz r11, 0x6dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C824: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321C828: 7DA84C2E  lfsx f13, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C82C: ED8D0072  fmuls f12, f13, f1
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 8321C830: D1830000  stfs f12, 0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8321C834: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8321C838: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C83C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C83C size=28
    let mut pc: u32 = 0x8321C83C;
    'dispatch: loop {
        match pc {
            0x8321C83C => {
    //   block [0x8321C83C..0x8321C858)
	// 8321C83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321C840: FDA06090  fmr f13, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[12].f64;
	// 8321C844: C00B9490  lfs f0, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C848: ED8C002A  fadds f12, f12, f0
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321C84C: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C850: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8321C854: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8321C858 size=12
    let mut pc: u32 = 0x8321C858;
    'dispatch: loop {
        match pc {
            0x8321C858 => {
    //   block [0x8321C858..0x8321C864)
	// 8321C858: EC0D002A  fadds f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321C85C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C868 size=108
    let mut pc: u32 = 0x8321C868;
    'dispatch: loop {
        match pc {
            0x8321C868 => {
    //   block [0x8321C868..0x8321C8D4)
	// 8321C868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C86C: 4BA8CB9D  bl 0x82ca9408
	ctx.lr = 0x8321C870;
	sub_82CA93D0(ctx, base);
	// 8321C870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C874: 3FA08349  lis r29, -0x7cb7
	ctx.r[29].s64 = -2092367872;
	// 8321C878: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8321C87C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321C880: 3B8BF1BC  addi r28, r11, -0xe44
	ctx.r[28].s64 = ctx.r[11].s64 + -3652;
	// 8321C884: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321C888: 817D6DD4  lwz r11, 0x6dd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C88C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321C890: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8321C894: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C898: 7DAAE42E  lfsx f13, r10, r28
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C89C: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8321C8A0: 4B805DC1  bl 0x82a22660
	ctx.lr = 0x8321C8A4;
	sub_82A22660(ctx, base);
	// 8321C8A4: 817D6DD4  lwz r11, 0x6dd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C8A8: C19F0004  lfs f12, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321C8AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321C8B0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8321C8B4: 7D69E42E  lfsx f11, r9, r28
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8321C8B8: EC2C5824  fdivs f1, f12, f11
	ctx.f[1].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 8321C8BC: 4B805DA5  bl 0x82a22660
	ctx.lr = 0x8321C8C0;
	sub_82A22660(ctx, base);
	// 8321C8C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321C8C4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321C8C8: 4B805D31  bl 0x82a225f8
	ctx.lr = 0x8321C8CC;
	sub_82A225F8(ctx, base);
	// 8321C8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321C8D0: 4BA8CB88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321C8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321C8D8 size=640
    let mut pc: u32 = 0x8321C8D8;
    'dispatch: loop {
        match pc {
            0x8321C8D8 => {
    //   block [0x8321C8D8..0x8321CB58)
	// 8321C8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321C8DC: 4BA8CB2D  bl 0x82ca9408
	ctx.lr = 0x8321C8E0;
	sub_82CA93D0(ctx, base);
	// 8321C8E0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8321C8E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321C8E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321C8EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8321C8F0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321C8F4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321C8F8: 4198004C  blt cr6, 0x8321c944
	if ctx.cr[6].lt {
	pc = 0x8321C944; continue 'dispatch;
	}
	// 8321C8FC: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321C900: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321C904: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 8321C908: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321C90C: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321C910: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321C914: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321C918: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321C91C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321C920: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321C924: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321C928: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321C92C: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321C930: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321C934: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321C938: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321C93C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321C940: 48000014  b 0x8321c954
	pc = 0x8321C954; continue 'dispatch;
	// 8321C944: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321C948: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321C94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321C950: 4B804599  bl 0x82a20ee8
	ctx.lr = 0x8321C954;
	sub_82A20EE8(ctx, base);
	// 8321C954: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321C958: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C95C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321C960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321C964: 419A0024  beq cr6, 0x8321c988
	if ctx.cr[6].eq {
	pc = 0x8321C988; continue 'dispatch;
	}
	// 8321C968: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321C96C: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321C970: 89210053  lbz r9, 0x53(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 8321C974: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321C978: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 8321C97C: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 8321C980: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8321C984: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 8321C988: 3FC08349  lis r30, -0x7cb7
	ctx.r[30].s64 = -2092367872;
	// 8321C98C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321C990: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8321C994: C1BC0004  lfs f13, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321C998: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8321C99C: 3BABF1BC  addi r29, r11, -0xe44
	ctx.r[29].s64 = ctx.r[11].s64 + -3652;
	// 8321C9A0: 813E6DD4  lwz r9, 0x6dd4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321C9A4: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321C9A8: C3EA9490  lfs f31, -0x6b70(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8321C9AC: 7D88EC2E  lfsx f12, r8, r29
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321C9B0: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8321C9B4: D01C0000  stfs f0, 0(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8321C9B8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8321C9BC: 4098001C  bge cr6, 0x8321c9d8
	if !ctx.cr[6].lt {
	pc = 0x8321C9D8; continue 'dispatch;
	}
	// 8321C9C0: EDA0F82A  fadds f13, f0, f31
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8321C9C4: D1BC0004  stfs f13, 4(r28)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C9C8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8321C9CC: 4098000C  bge cr6, 0x8321c9d8
	if !ctx.cr[6].lt {
	pc = 0x8321C9D8; continue 'dispatch;
	}
	// 8321C9D0: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8321C9D4: D01C0004  stfs f0, 4(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321C9D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321C9DC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321C9E0: 4198004C  blt cr6, 0x8321ca2c
	if ctx.cr[6].lt {
	pc = 0x8321CA2C; continue 'dispatch;
	}
	// 8321C9E4: 390BFFFC  addi r8, r11, -4
	ctx.r[8].s64 = ctx.r[11].s64 + -4;
	// 8321C9E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321C9EC: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 8321C9F0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321C9F4: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 8321C9F8: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 8321C9FC: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CA00: 98870000  stb r4, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321CA04: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321CA08: 98670001  stb r3, 1(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 8321CA0C: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321CA10: 99470002  stb r10, 2(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8321CA14: 888B0003  lbz r4, 3(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321CA18: 98870003  stb r4, 3(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(3 as u32), ctx.r[4].u8 ) };
	// 8321CA1C: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321CA20: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 8321CA24: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321CA28: 48000018  b 0x8321ca40
	pc = 0x8321CA40; continue 'dispatch;
	// 8321CA2C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321CA30: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8321CA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CA38: 4B8044B1  bl 0x82a20ee8
	ctx.lr = 0x8321CA3C;
	sub_82A20EE8(ctx, base);
	// 8321CA3C: 813E6DD4  lwz r9, 0x6dd4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321CA40: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321CA44: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CA48: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8321CA4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321CA50: 419A0024  beq cr6, 0x8321ca74
	if ctx.cr[6].eq {
	pc = 0x8321CA74; continue 'dispatch;
	}
	// 8321CA54: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321CA58: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321CA5C: 89010053  lbz r8, 0x53(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 8321CA60: 88E10052  lbz r7, 0x52(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321CA64: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 8321CA68: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 8321CA6C: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 8321CA70: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 8321CA74: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8321CA78: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321CA7C: C01C0000  lfs f0, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CA80: 7D8BEC2E  lfsx f12, r11, r29
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321CA84: ED6C0372  fmuls f11, f12, f13
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8321CA88: D17C0004  stfs f11, 4(r28)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321CA8C: FF0B0000  fcmpu cr6, f11, f0
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[0].f64);
	// 8321CA90: 4098000C  bge cr6, 0x8321ca9c
	if !ctx.cr[6].lt {
	pc = 0x8321CA9C; continue 'dispatch;
	}
	// 8321CA94: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8321CA98: D01C0004  stfs f0, 4(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321CA9C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321CAA0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8321CAA4: 4198004C  blt cr6, 0x8321caf0
	if ctx.cr[6].lt {
	pc = 0x8321CAF0; continue 'dispatch;
	}
	// 8321CAA8: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8321CAAC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321CAB0: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 8321CAB4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321CAB8: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 8321CABC: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 8321CAC0: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CAC4: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321CAC8: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8321CACC: 98880001  stb r4, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 8321CAD0: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8321CAD4: 98680002  stb r3, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 8321CAD8: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8321CADC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321CAE0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321CAE4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321CAE8: 99680003  stb r11, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8321CAEC: 48000014  b 0x8321cb00
	pc = 0x8321CB00; continue 'dispatch;
	// 8321CAF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8321CAF4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321CAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CAFC: 4B8043ED  bl 0x82a20ee8
	ctx.lr = 0x8321CB00;
	sub_82A20EE8(ctx, base);
	// 8321CB00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321CB04: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8321CB08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321CB0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8321CB10: 419A0038  beq cr6, 0x8321cb48
	if ctx.cr[6].eq {
	pc = 0x8321CB48; continue 'dispatch;
	}
	// 8321CB14: 89610052  lbz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8321CB18: 89210051  lbz r9, 0x51(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321CB1C: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321CB20: 89410057  lbz r10, 0x57(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8321CB24: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8321CB28: 99210052  stb r9, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[9].u8 ) };
	// 8321CB2C: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8321CB30: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8321CB34: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321CB38: 90FC0008  stw r7, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8321CB3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321CB40: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321CB44: 4BA8C914  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8321CB48: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8321CB4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321CB50: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321CB54: 4BA8C904  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321CB58 size=372
    let mut pc: u32 = 0x8321CB58;
    'dispatch: loop {
        match pc {
            0x8321CB58 => {
    //   block [0x8321CB58..0x8321CCCC)
	// 8321CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CB5C: 4BA8C8AD  bl 0x82ca9408
	ctx.lr = 0x8321CB60;
	sub_82CA93D0(ctx, base);
	// 8321CB60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CB64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321CB68: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321CB6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321CB70: 3BCB9490  addi r30, r11, -0x6b70
	ctx.r[30].s64 = ctx.r[11].s64 + -27504;
	// 8321CB74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CB78: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CB7C: 389E0C74  addi r4, r30, 0xc74
	ctx.r[4].s64 = ctx.r[30].s64 + 3188;
	// 8321CB80: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321CB84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321CB88: 4E800421  bctrl
	ctx.lr = 0x8321CB8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321CB8C: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321CB90: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8321CB94: 419A0130  beq cr6, 0x8321ccc4
	if ctx.cr[6].eq {
	pc = 0x8321CCC4; continue 'dispatch;
	}
	// 8321CB98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CBA0: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8321CBA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321CBA8: 4E800421  bctrl
	ctx.lr = 0x8321CBAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321CBAC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321CBB0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321CBB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321CBB8: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 8321CBBC: 419A00BC  beq cr6, 0x8321cc78
	if ctx.cr[6].eq {
	pc = 0x8321CC78; continue 'dispatch;
	}
	// 8321CBC0: 389E0C84  addi r4, r30, 0xc84
	ctx.r[4].s64 = ctx.r[30].s64 + 3204;
	// 8321CBC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321CBC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CBCC: 4B010305  bl 0x8222ced0
	ctx.lr = 0x8321CBD0;
	sub_8222CED0(ctx, base);
	// 8321CBD0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321CBD4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321CBD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321CBDC: 419A0008  beq cr6, 0x8321cbe4
	if ctx.cr[6].eq {
	pc = 0x8321CBE4; continue 'dispatch;
	}
	// 8321CBE0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CBE4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8321CBE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CBEC: 4B263C65  bl 0x82480850
	ctx.lr = 0x8321CBF0;
	sub_82480850(ctx, base);
	// 8321CBF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CBF4: 4AFF81E5  bl 0x82214dd8
	ctx.lr = 0x8321CBF8;
	sub_82214DD8(ctx, base);
	// 8321CBF8: 389E0C94  addi r4, r30, 0xc94
	ctx.r[4].s64 = ctx.r[30].s64 + 3220;
	// 8321CBFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321CC00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CC04: 4B0102CD  bl 0x8222ced0
	ctx.lr = 0x8321CC08;
	sub_8222CED0(ctx, base);
	// 8321CC08: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321CC0C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321CC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321CC14: 419A0008  beq cr6, 0x8321cc1c
	if ctx.cr[6].eq {
	pc = 0x8321CC1C; continue 'dispatch;
	}
	// 8321CC18: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CC1C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8321CC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CC24: 4B263C2D  bl 0x82480850
	ctx.lr = 0x8321CC28;
	sub_82480850(ctx, base);
	// 8321CC28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CC2C: 4AFF81AD  bl 0x82214dd8
	ctx.lr = 0x8321CC30;
	sub_82214DD8(ctx, base);
	// 8321CC30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321CC34: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8321CC38: 4BFFFBD9  bl 0x8321c810
	ctx.lr = 0x8321CC3C;
	sub_8321C810(ctx, base);
	// 8321CC3C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8321CC40: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321CC44: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CC48: C1BD0000  lfs f13, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321CC4C: 392AF1BC  addi r9, r10, -0xe44
	ctx.r[9].s64 = ctx.r[10].s64 + -3652;
	// 8321CC50: 816B6DD4  lwz r11, 0x6dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28116 as u32) ) } as u64;
	// 8321CC54: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8321CC58: 7D884C2E  lfsx f12, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321CC5C: ED6C0032  fmuls f11, f12, f0
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8321CC60: D17D0004  stfs f11, 4(r29)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321CC64: FF0B6800  fcmpu cr6, f11, f13
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[13].f64);
	// 8321CC68: 40980010  bge cr6, 0x8321cc78
	if !ctx.cr[6].lt {
	pc = 0x8321CC78; continue 'dispatch;
	}
	// 8321CC6C: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CC70: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8321CC74: D01D0004  stfs f0, 4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8321CC78: 389E0CA4  addi r4, r30, 0xca4
	ctx.r[4].s64 = ctx.r[30].s64 + 3236;
	// 8321CC7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321CC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CC84: 4B01024D  bl 0x8222ced0
	ctx.lr = 0x8321CC88;
	sub_8222CED0(ctx, base);
	// 8321CC88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321CC8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321CC90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321CC94: 419A0008  beq cr6, 0x8321cc9c
	if ctx.cr[6].eq {
	pc = 0x8321CC9C; continue 'dispatch;
	}
	// 8321CC98: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CC9C: 38BD0008  addi r5, r29, 8
	ctx.r[5].s64 = ctx.r[29].s64 + 8;
	// 8321CCA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CCA4: 4B263B25  bl 0x824807c8
	ctx.lr = 0x8321CCA8;
	sub_824807C8(ctx, base);
	// 8321CCA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CCAC: 4AFF812D  bl 0x82214dd8
	ctx.lr = 0x8321CCB0;
	sub_82214DD8(ctx, base);
	// 8321CCB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CCB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CCB8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321CCBC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321CCC0: 4E800421  bctrl
	ctx.lr = 0x8321CCC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321CCC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321CCC8: 4BA8C790  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321CCD0 size=4
    let mut pc: u32 = 0x8321CCD0;
    'dispatch: loop {
        match pc {
            0x8321CCD0 => {
    //   block [0x8321CCD0..0x8321CCD4)
	// 8321CCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321CCD8 size=184
    let mut pc: u32 = 0x8321CCD8;
    'dispatch: loop {
        match pc {
            0x8321CCD8 => {
    //   block [0x8321CCD8..0x8321CD90)
	// 8321CCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321CCE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321CCE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321CCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CCEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321CCF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321CCF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321CCF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321CCFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321CD00: 897F00C0  lbz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321CD04: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CD08: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8321CD0C: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321CD10: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321CD14: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321CD18: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 8321CD1C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8321CD20: 4E800421  bctrl
	ctx.lr = 0x8321CD24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321CD24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321CD28: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321CD2C: 4B8058CD  bl 0x82a225f8
	ctx.lr = 0x8321CD30;
	sub_82A225F8(ctx, base);
	// 8321CD30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321CD34: 809F00C8  lwz r4, 0xc8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 8321CD38: 4B8058C1  bl 0x82a225f8
	ctx.lr = 0x8321CD3C;
	sub_82A225F8(ctx, base);
	// 8321CD3C: 897F00CC  lbz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8321CD40: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321CD44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321CD48: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8321CD4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321CD50: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321CD54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321CD58: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321CD5C: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321CD60: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 8321CD64: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8321CD68: 4E800421  bctrl
	ctx.lr = 0x8321CD6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321CD6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321CD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CD74: 48008735  bl 0x832254a8
	ctx.lr = 0x8321CD78;
	sub_832254A8(ctx, base);
	// 8321CD78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321CD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321CD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321CD84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321CD88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321CD90 size=80
    let mut pc: u32 = 0x8321CD90;
    'dispatch: loop {
        match pc {
            0x8321CD90 => {
    //   block [0x8321CD90..0x8321CDE0)
	// 8321CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321CD98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321CD9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CDA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321CDA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8321CDA8: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 8321CDAC: 394B2640  addi r10, r11, 0x2640
	ctx.r[10].s64 = ctx.r[11].s64 + 9792;
	// 8321CDB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321CDB4: 4B7F41B5  bl 0x82a10f68
	ctx.lr = 0x8321CDB8;
	sub_82A10F68(ctx, base);
	// 8321CDB8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8321CDBC: 4B7F40CD  bl 0x82a10e88
	ctx.lr = 0x8321CDC0;
	sub_82A10E88(ctx, base);
	// 8321CDC0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8321CDC4: 39092720  addi r8, r9, 0x2720
	ctx.r[8].s64 = ctx.r[9].s64 + 10016;
	// 8321CDC8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321CDCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321CDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321CDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321CDD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321CDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321CDE0 size=92
    let mut pc: u32 = 0x8321CDE0;
    'dispatch: loop {
        match pc {
            0x8321CDE0 => {
    //   block [0x8321CDE0..0x8321CE3C)
	// 8321CDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321CDE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321CDEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321CDF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CDF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321CDF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321CDFC: 480090E5  bl 0x83225ee0
	ctx.lr = 0x8321CE00;
	sub_83225EE0(ctx, base);
	// 8321CE00: 897E00C0  lbz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321CE04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CE08: 997F00C0  stb r11, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 8321CE0C: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321CE10: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8321CE14: 813E00C8  lwz r9, 0xc8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 8321CE18: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 8321CE1C: 891E00CC  lbz r8, 0xcc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8321CE20: 991F00CC  stb r8, 0xcc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[8].u8 ) };
	// 8321CE24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321CE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321CE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321CE30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321CE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321CE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321CE40 size=236
    let mut pc: u32 = 0x8321CE40;
    'dispatch: loop {
        match pc {
            0x8321CE40 => {
    //   block [0x8321CE40..0x8321CF2C)
	// 8321CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CE44: 4BA8C5C9  bl 0x82ca940c
	ctx.lr = 0x8321CE48;
	sub_82CA93D0(ctx, base);
	// 8321CE48: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 8321CE4C: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8321CE50: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8321CE54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CE58: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321CE5C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8321CE60: 3BAB92D4  addi r29, r11, -0x6d2c
	ctx.r[29].s64 = ctx.r[11].s64 + -27948;
	// 8321CE64: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 8321CE68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321CE6C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321CE70: C3FD01B0  lfs f31, 0x1b0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(432 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8321CE74: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8321CE78: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321CE7C: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321CE80: C3BD01BC  lfs f29, 0x1bc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(444 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8321CE84: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 8321CE88: 7CE43378  or r4, r7, r6
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8321CE8C: 7C09242E  lfsx f0, r9, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CE90: FDA0F86E  fsel f13, f0, f1, f31
	ctx.f[13].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 8321CE94: ED8DE828  fsubs f12, f13, f29
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[29].f64) as f32) as f64);
	// 8321CE98: FF0CF800  fcmpu cr6, f12, f31
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[31].f64);
	// 8321CE9C: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321CEA0: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8321CEA4: 5568F77A  rlwinm r8, r11, 0x1e, 0x1d, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8321CEA8: 7D474378  or r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8321CEAC: 7D693C2E  lfsx f11, r9, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8321CEB0: FFCB6F6E  fsel f30, f11, f29, f13
	ctx.f[30].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[29].f64 } else { ctx.f[13].f64 };
	// 8321CEB4: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8321CEB8: 48024349  bl 0x83241200
	ctx.lr = 0x8321CEBC;
	sub_83241200(ctx, base);
	// 8321CEBC: FF1EF800  fcmpu cr6, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 8321CEC0: 40990058  ble cr6, 0x8321cf18
	if !ctx.cr[6].gt {
	pc = 0x8321CF18; continue 'dispatch;
	}
	// 8321CEC4: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 8321CEC8: 41980024  blt cr6, 0x8321ceec
	if ctx.cr[6].lt {
	pc = 0x8321CEEC; continue 'dispatch;
	}
	// 8321CECC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321CED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CED4: 4BFFFF0D  bl 0x8321cde0
	ctx.lr = 0x8321CED8;
	sub_8321CDE0(ctx, base);
	// 8321CED8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321CEDC: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8321CEE0: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321CEE4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8321CEE8: 4BA8C574  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8321CEEC: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321CEF0: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8321CEF4: 40990024  ble cr6, 0x8321cf18
	if !ctx.cr[6].gt {
	pc = 0x8321CF18; continue 'dispatch;
	}
	// 8321CEF8: 897E00C0  lbz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321CEFC: 997F00C0  stb r11, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 8321CF00: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321CF04: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8321CF08: 813E00C8  lwz r9, 0xc8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 8321CF0C: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 8321CF10: 891E00CC  lbz r8, 0xcc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8321CF14: 991F00CC  stb r8, 0xcc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[8].u8 ) };
	// 8321CF18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321CF1C: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8321CF20: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321CF24: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8321CF28: 4BA8C534  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321CF30 size=104
    let mut pc: u32 = 0x8321CF30;
    'dispatch: loop {
        match pc {
            0x8321CF30 => {
    //   block [0x8321CF30..0x8321CF98)
	// 8321CF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321CF38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321CF3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321CF40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CF44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321CF48: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321CF4C: 48009295  bl 0x832261e0
	ctx.lr = 0x8321CF50;
	sub_832261E0(ctx, base);
	// 8321CF50: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321CF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321CF58: 394B27B8  addi r10, r11, 0x27b8
	ctx.r[10].s64 = ctx.r[11].s64 + 10168;
	// 8321CF5C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321CF60: 893E00C0  lbz r9, 0xc0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321CF64: 993F00C0  stb r9, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[9].u8 ) };
	// 8321CF68: 811E00C4  lwz r8, 0xc4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321CF6C: 911F00C4  stw r8, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[8].u32 ) };
	// 8321CF70: 80FE00C8  lwz r7, 0xc8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 8321CF74: 90FF00C8  stw r7, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[7].u32 ) };
	// 8321CF78: 88DE00CC  lbz r6, 0xcc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8321CF7C: 98DF00CC  stb r6, 0xcc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[6].u8 ) };
	// 8321CF80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321CF8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321CF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321CF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321CF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321CF98 size=176
    let mut pc: u32 = 0x8321CF98;
    'dispatch: loop {
        match pc {
            0x8321CF98 => {
    //   block [0x8321CF98..0x8321D048)
	// 8321CF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321CF9C: 4BA8C471  bl 0x82ca940c
	ctx.lr = 0x8321CFA0;
	sub_82CA93D0(ctx, base);
	// 8321CFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321CFA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321CFA8: 48009429  bl 0x832263d0
	ctx.lr = 0x8321CFAC;
	sub_832263D0(ctx, base);
	// 8321CFAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321CFB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8321CFB4: 394B27B8  addi r10, r11, 0x27b8
	ctx.r[10].s64 = ctx.r[11].s64 + 10168;
	// 8321CFB8: 9BDD00C0  stb r30, 0xc0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(192 as u32), ctx.r[30].u8 ) };
	// 8321CFBC: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 8321CFC0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321CFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321CFC8: 3BE908D5  addi r31, r9, 0x8d5
	ctx.r[31].s64 = ctx.r[9].s64 + 2261;
	// 8321CFCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321CFD0: 389FF7CE  addi r4, r31, -0x832
	ctx.r[4].s64 = ctx.r[31].s64 + -2098;
	// 8321CFD4: 4B00FEFD  bl 0x8222ced0
	ctx.lr = 0x8321CFD8;
	sub_8222CED0(ctx, base);
	// 8321CFD8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321CFDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CFE0: 4AFD1B59  bl 0x821eeb38
	ctx.lr = 0x8321CFE4;
	sub_821EEB38(ctx, base);
	// 8321CFE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CFE8: 4B9E6809  bl 0x82c037f0
	ctx.lr = 0x8321CFEC;
	sub_82C037F0(ctx, base);
	// 8321CFEC: 907D00C4  stw r3, 0xc4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(196 as u32), ctx.r[3].u32 ) };
	// 8321CFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321CFF4: 4AFF7DE5  bl 0x82214dd8
	ctx.lr = 0x8321CFF8;
	sub_82214DD8(ctx, base);
	// 8321CFF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321CFFC: 4AFF7DDD  bl 0x82214dd8
	ctx.lr = 0x8321D000;
	sub_82214DD8(ctx, base);
	// 8321D000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321D008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D00C: 4B00FEC5  bl 0x8222ced0
	ctx.lr = 0x8321D010;
	sub_8222CED0(ctx, base);
	// 8321D010: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D014: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D018: 4AFD1B21  bl 0x821eeb38
	ctx.lr = 0x8321D01C;
	sub_821EEB38(ctx, base);
	// 8321D01C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D020: 4B9E67D1  bl 0x82c037f0
	ctx.lr = 0x8321D024;
	sub_82C037F0(ctx, base);
	// 8321D024: 907D00C8  stw r3, 0xc8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(200 as u32), ctx.r[3].u32 ) };
	// 8321D028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D02C: 4AFF7DAD  bl 0x82214dd8
	ctx.lr = 0x8321D030;
	sub_82214DD8(ctx, base);
	// 8321D030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D034: 4AFF7DA5  bl 0x82214dd8
	ctx.lr = 0x8321D038;
	sub_82214DD8(ctx, base);
	// 8321D038: 9BDD00CC  stb r30, 0xcc(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(204 as u32), ctx.r[30].u8 ) };
	// 8321D03C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321D040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321D044: 4BA8C418  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D048 size=260
    let mut pc: u32 = 0x8321D048;
    'dispatch: loop {
        match pc {
            0x8321D048 => {
    //   block [0x8321D048..0x8321D14C)
	// 8321D048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D04C: 4BA8C3C1  bl 0x82ca940c
	ctx.lr = 0x8321D050;
	sub_82CA93D0(ctx, base);
	// 8321D050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D054: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321D058: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321D05C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8321D060: 386BE688  addi r3, r11, -0x1978
	ctx.r[3].s64 = ctx.r[11].s64 + -6520;
	// 8321D064: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321D068: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8321D06C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8321D070: 48023CC1  bl 0x83240d30
	ctx.lr = 0x8321D074;
	sub_83240D30(ctx, base);
	// 8321D074: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321D078: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321D07C: 419A0020  beq cr6, 0x8321d09c
	if ctx.cr[6].eq {
	pc = 0x8321D09C; continue 'dispatch;
	}
	// 8321D080: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8321D084: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D088: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8321D08C: 4AF9AA8D  bl 0x821b7b18
	ctx.lr = 0x8321D090;
	sub_821B7B18(ctx, base);
	// 8321D090: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321D094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321D098: 4BA8C3C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8321D09C: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 8321D0A0: 4B0021B9  bl 0x8221f258
	ctx.lr = 0x8321D0A4;
	sub_8221F258(ctx, base);
	// 8321D0A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321D0A8: 419A0014  beq cr6, 0x8321d0bc
	if ctx.cr[6].eq {
	pc = 0x8321D0BC; continue 'dispatch;
	}
	// 8321D0AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321D0B0: 4BFFFEE9  bl 0x8321cf98
	ctx.lr = 0x8321D0B4;
	sub_8321CF98(ctx, base);
	// 8321D0B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321D0B8: 48000008  b 0x8321d0c0
	pc = 0x8321D0C0; continue 'dispatch;
	// 8321D0BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D0C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D0C4: 4B055F95  bl 0x82273058
	ctx.lr = 0x8321D0C8;
	sub_82273058(ctx, base);
	// 8321D0C8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321D0CC: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321D0D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321D0D4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8321D0D8: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8321D0DC: 419A0020  beq cr6, 0x8321d0fc
	if ctx.cr[6].eq {
	pc = 0x8321D0FC; continue 'dispatch;
	}
	// 8321D0E0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321D0E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D0E8: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321D0EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321D0F0: 7D60F92D  stwcx. r11, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321D0F4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D0F8: 4082FFE8  bne 0x8321d0e0
	if !ctx.cr[0].eq {
	pc = 0x8321D0E0; continue 'dispatch;
	}
	// 8321D0FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321D100: 4800ECB1  bl 0x8322bdb0
	ctx.lr = 0x8321D104;
	sub_8322BDB0(ctx, base);
	// 8321D104: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321D108: 4AF9AA11  bl 0x821b7b18
	ctx.lr = 0x8321D10C;
	sub_821B7B18(ctx, base);
	// 8321D10C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8321D110: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8321D114: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321D118: 419A0020  beq cr6, 0x8321d138
	if ctx.cr[6].eq {
	pc = 0x8321D138; continue 'dispatch;
	}
	// 8321D11C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321D120: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D124: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321D128: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321D12C: 7D60F92D  stwcx. r11, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321D130: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D134: 4082FFE8  bne 0x8321d11c
	if !ctx.cr[0].eq {
	pc = 0x8321D11C; continue 'dispatch;
	}
	// 8321D138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D13C: 4AF9A9DD  bl 0x821b7b18
	ctx.lr = 0x8321D140;
	sub_821B7B18(ctx, base);
	// 8321D140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321D148: 4BA8C314  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321D150 size=728
    let mut pc: u32 = 0x8321D150;
    'dispatch: loop {
        match pc {
            0x8321D150 => {
    //   block [0x8321D150..0x8321D428)
	// 8321D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D154: 4BA8C2A5  bl 0x82ca93f8
	ctx.lr = 0x8321D158;
	sub_82CA93D0(ctx, base);
	// 8321D158: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8321D15C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D160: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321D164: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8321D168: 3BEB9490  addi r31, r11, -0x6b70
	ctx.r[31].s64 = ctx.r[11].s64 + -27504;
	// 8321D16C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8321D170: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321D174: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 8321D178: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321D17C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8321D180: C01FFFF4  lfs f0, -0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321D184: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8321D188: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321D18C: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321D190: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 8321D194: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8321D198: 7D892C2E  lfsx f12, r9, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321D19C: FD6C006E  fsel f11, f12, f1, f0
	ctx.f[11].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[0].f64 };
	// 8321D1A0: ED4B6828  fsubs f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8321D1A4: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 8321D1A8: 7C800026  mfcr r4
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[4].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321D1AC: 5483DF7A  rlwinm r3, r4, 0x1b, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 8321D1B0: 548BF77A  rlwinm r11, r4, 0x1e, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000003u64;
	// 8321D1B4: 7C6A5B78  or r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 8321D1B8: 7D29542E  lfsx f9, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8321D1BC: FFE95B6E  fsel f31, f9, f13, f11
	ctx.f[31].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[11].f64 };
	// 8321D1C0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8321D1C4: 41990048  bgt cr6, 0x8321d20c
	if ctx.cr[6].gt {
	pc = 0x8321D20C; continue 'dispatch;
	}
	// 8321D1C8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D1CC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D1D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321D1D4: 91780004  stw r11, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321D1D8: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321D1DC: 419A023C  beq cr6, 0x8321d418
	if ctx.cr[6].eq {
	pc = 0x8321D418; continue 'dispatch;
	}
	// 8321D1E0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8321D1E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D1E8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8321D1EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8321D1F0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321D1F4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D1F8: 4082FFE8  bne 0x8321d1e0
	if !ctx.cr[0].eq {
	pc = 0x8321D1E0; continue 'dispatch;
	}
	// 8321D1FC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321D200: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321D204: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321D208: 4BA8C240  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 8321D20C: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D210: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 8321D214: 41980044  blt cr6, 0x8321d258
	if ctx.cr[6].lt {
	pc = 0x8321D258; continue 'dispatch;
	}
	// 8321D218: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D21C: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321D220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321D224: 91780004  stw r11, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321D228: 419A01F0  beq cr6, 0x8321d418
	if ctx.cr[6].eq {
	pc = 0x8321D418; continue 'dispatch;
	}
	// 8321D22C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8321D230: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D234: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8321D238: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8321D23C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321D240: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D244: 4082FFE8  bne 0x8321d22c
	if !ctx.cr[0].eq {
	pc = 0x8321D22C; continue 'dispatch;
	}
	// 8321D248: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321D24C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321D250: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321D254: 4BA8C1F4  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 8321D258: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D25C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8321D260: 83CA0004  lwz r30, 4(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D264: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D268: 4B131A59  bl 0x8234ecc0
	ctx.lr = 0x8321D26C;
	sub_8234ECC0(ctx, base);
	// 8321D26C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321D270: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D274: 4AFD2FCD  bl 0x821f0240
	ctx.lr = 0x8321D278;
	sub_821F0240(ctx, base);
	// 8321D278: 389F0D54  addi r4, r31, 0xd54
	ctx.r[4].s64 = ctx.r[31].s64 + 3412;
	// 8321D27C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D280: 4AFBD741  bl 0x821da9c0
	ctx.lr = 0x8321D284;
	sub_821DA9C0(ctx, base);
	// 8321D284: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8321D288: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8321D28C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8321D290: 4AFD1701  bl 0x821ee990
	ctx.lr = 0x8321D294;
	sub_821EE990(ctx, base);
	// 8321D294: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8321D298: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321D29C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8321D2A0: 4AFC5E69  bl 0x821e3108
	ctx.lr = 0x8321D2A4;
	sub_821E3108(ctx, base);
	// 8321D2A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321D2A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D2AC: 4AFD2F95  bl 0x821f0240
	ctx.lr = 0x8321D2B0;
	sub_821F0240(ctx, base);
	// 8321D2B0: 389F0D50  addi r4, r31, 0xd50
	ctx.r[4].s64 = ctx.r[31].s64 + 3408;
	// 8321D2B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D2B8: 4AFBD709  bl 0x821da9c0
	ctx.lr = 0x8321D2BC;
	sub_821DA9C0(ctx, base);
	// 8321D2BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321D2C0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321D2C4: 4B1319FD  bl 0x8234ecc0
	ctx.lr = 0x8321D2C8;
	sub_8234ECC0(ctx, base);
	// 8321D2C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8321D2CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D2D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321D2D4: 4AFC5E35  bl 0x821e3108
	ctx.lr = 0x8321D2D8;
	sub_821E3108(ctx, base);
	// 8321D2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D2DC: 4AFF7AFD  bl 0x82214dd8
	ctx.lr = 0x8321D2E0;
	sub_82214DD8(ctx, base);
	// 8321D2E0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8321D2E4: 4AFF7AF5  bl 0x82214dd8
	ctx.lr = 0x8321D2E8;
	sub_82214DD8(ctx, base);
	// 8321D2E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D2EC: 4AFF7AED  bl 0x82214dd8
	ctx.lr = 0x8321D2F0;
	sub_82214DD8(ctx, base);
	// 8321D2F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8321D2F4: 4AFF7AE5  bl 0x82214dd8
	ctx.lr = 0x8321D2F8;
	sub_82214DD8(ctx, base);
	// 8321D2F8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8321D2FC: 4AFF7ADD  bl 0x82214dd8
	ctx.lr = 0x8321D300;
	sub_82214DD8(ctx, base);
	// 8321D300: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321D304: 4AFF7AD5  bl 0x82214dd8
	ctx.lr = 0x8321D308;
	sub_82214DD8(ctx, base);
	// 8321D308: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321D30C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D310: 4AFD1829  bl 0x821eeb38
	ctx.lr = 0x8321D314;
	sub_821EEB38(ctx, base);
	// 8321D314: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D318: 4B9E64D9  bl 0x82c037f0
	ctx.lr = 0x8321D31C;
	sub_82C037F0(ctx, base);
	// 8321D31C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8321D320: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D324: 4AFF7AB5  bl 0x82214dd8
	ctx.lr = 0x8321D328;
	sub_82214DD8(ctx, base);
	// 8321D328: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8321D32C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8321D330: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8321D334: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8321D338: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8321D33C: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8321D340: 4800E9F1  bl 0x8322bd30
	ctx.lr = 0x8321D344;
	sub_8322BD30(ctx, base);
	// 8321D344: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D348: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D34C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321D350: 419A0028  beq cr6, 0x8321d378
	if ctx.cr[6].eq {
	pc = 0x8321D378; continue 'dispatch;
	}
	// 8321D354: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321D358: 4AF9A7C1  bl 0x821b7b18
	ctx.lr = 0x8321D35C;
	sub_821B7B18(ctx, base);
	// 8321D35C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D360: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8321D364: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 8321D368: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321D36C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8321D370: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8321D374: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8321D378: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8321D37C: 4AF9A79D  bl 0x821b7b18
	ctx.lr = 0x8321D380;
	sub_821B7B18(ctx, base);
	// 8321D380: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8321D384: 409A0058  bne cr6, 0x8321d3dc
	if !ctx.cr[6].eq {
	pc = 0x8321D3DC; continue 'dispatch;
	}
	// 8321D388: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 8321D38C: 4B001ECD  bl 0x8221f258
	ctx.lr = 0x8321D390;
	sub_8221F258(ctx, base);
	// 8321D390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321D394: 419A0018  beq cr6, 0x8321d3ac
	if ctx.cr[6].eq {
	pc = 0x8321D3AC; continue 'dispatch;
	}
	// 8321D398: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8321D39C: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D3A0: 4BFFFB91  bl 0x8321cf30
	ctx.lr = 0x8321D3A4;
	sub_8321CF30(ctx, base);
	// 8321D3A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321D3A8: 48000008  b 0x8321d3b0
	pc = 0x8321D3B0; continue 'dispatch;
	// 8321D3AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8321D3B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321D3B4: 4B055CA5  bl 0x82273058
	ctx.lr = 0x8321D3B8;
	sub_82273058(ctx, base);
	// 8321D3B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321D3BC: 83C10068  lwz r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8321D3C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D3C4: 4801EA55  bl 0x8323be18
	ctx.lr = 0x8321D3C8;
	sub_8323BE18(ctx, base);
	// 8321D3C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D3CC: 80B90000  lwz r5, 0(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D3D0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8321D3D4: 4BFFFA6D  bl 0x8321ce40
	ctx.lr = 0x8321D3D8;
	sub_8321CE40(ctx, base);
	// 8321D3D8: 83A1006C  lwz r29, 0x6c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8321D3DC: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8321D3E0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8321D3E4: 93B80004  stw r29, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8321D3E8: 419A0020  beq cr6, 0x8321d408
	if ctx.cr[6].eq {
	pc = 0x8321D408; continue 'dispatch;
	}
	// 8321D3EC: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321D3F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D3F4: 7D60E828  lwarx r11, 0, r29
	// lwarx
	let ea = ctx.r[29].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321D3F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321D3FC: 7D60E92D  stwcx. r11, 0, r29
	// stwcx.
	let addr = ctx.r[29].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321D400: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321D404: 4082FFE8  bne 0x8321d3ec
	if !ctx.cr[0].eq {
	pc = 0x8321D3EC; continue 'dispatch;
	}
	// 8321D408: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321D40C: 4AF9A70D  bl 0x821b7b18
	ctx.lr = 0x8321D410;
	sub_821B7B18(ctx, base);
	// 8321D410: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321D414: 4AFF79C5  bl 0x82214dd8
	ctx.lr = 0x8321D418;
	sub_82214DD8(ctx, base);
	// 8321D418: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321D41C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321D420: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321D424: 4BA8C024  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D428 size=508
    let mut pc: u32 = 0x8321D428;
    'dispatch: loop {
        match pc {
            0x8321D428 => {
    //   block [0x8321D428..0x8321D624)
	// 8321D428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D42C: 4BA8BFDD  bl 0x82ca9408
	ctx.lr = 0x8321D430;
	sub_82CA93D0(ctx, base);
	// 8321D430: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321D438: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321D43C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321D440: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8321D444: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321D448: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321D44C: 41980034  blt cr6, 0x8321d480
	if ctx.cr[6].lt {
	pc = 0x8321D480; continue 'dispatch;
	}
	// 8321D450: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321D454: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321D458: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D45C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8321D460: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321D464: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321D468: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D46C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321D470: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321D474: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321D478: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321D47C: 48000014  b 0x8321d490
	pc = 0x8321D490; continue 'dispatch;
	// 8321D480: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321D484: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D48C: 4B803A5D  bl 0x82a20ee8
	ctx.lr = 0x8321D490;
	sub_82A20EE8(ctx, base);
	// 8321D490: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321D494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8321D498: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D49C: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8321D4A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D4A4: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321D4A8: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321D4AC: 98FE00C0  stb r7, 0xc0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[7].u8 ) };
	// 8321D4B0: 4B804DA9  bl 0x82a22258
	ctx.lr = 0x8321D4B4;
	sub_82A22258(ctx, base);
	// 8321D4B4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8321D4B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8321D4BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D4C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321D4C4: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D4C8: 917E00C4  stw r11, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8321D4CC: 4B804D8D  bl 0x82a22258
	ctx.lr = 0x8321D4D0;
	sub_82A22258(ctx, base);
	// 8321D4D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D4D4: 915E00C8  stw r10, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 8321D4D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321D4DC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321D4E0: 41980034  blt cr6, 0x8321d514
	if ctx.cr[6].lt {
	pc = 0x8321D514; continue 'dispatch;
	}
	// 8321D4E4: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8321D4E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321D4EC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D4F0: 39010051  addi r8, r1, 0x51
	ctx.r[8].s64 = ctx.r[1].s64 + 81;
	// 8321D4F4: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 8321D4F8: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321D4FC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D500: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8321D504: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8321D508: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8321D50C: 98A80000  stb r5, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8321D510: 48000014  b 0x8321d524
	pc = 0x8321D524; continue 'dispatch;
	// 8321D514: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321D518: 38810051  addi r4, r1, 0x51
	ctx.r[4].s64 = ctx.r[1].s64 + 81;
	// 8321D51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D520: 4B8039C9  bl 0x82a20ee8
	ctx.lr = 0x8321D524;
	sub_82A20EE8(ctx, base);
	// 8321D524: 89410051  lbz r10, 0x51(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8321D528: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8321D52C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D530: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8321D534: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D538: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321D53C: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321D540: 98FE00CC  stb r7, 0xcc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[7].u8 ) };
	// 8321D544: 4800987D  bl 0x83226dc0
	ctx.lr = 0x8321D548;
	sub_83226DC0(ctx, base);
	// 8321D548: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8321D54C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D554: 48009A0D  bl 0x83226f60
	ctx.lr = 0x8321D558;
	sub_83226F60(ctx, base);
	// 8321D558: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8321D55C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D564: 480099FD  bl 0x83226f60
	ctx.lr = 0x8321D568;
	sub_83226F60(ctx, base);
	// 8321D568: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8321D56C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D574: 480099ED  bl 0x83226f60
	ctx.lr = 0x8321D578;
	sub_83226F60(ctx, base);
	// 8321D578: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 8321D57C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D580: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D584: 480099DD  bl 0x83226f60
	ctx.lr = 0x8321D588;
	sub_83226F60(ctx, base);
	// 8321D588: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 8321D58C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D594: 480099CD  bl 0x83226f60
	ctx.lr = 0x8321D598;
	sub_83226F60(ctx, base);
	// 8321D598: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 8321D59C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5A4: 480099BD  bl 0x83226f60
	ctx.lr = 0x8321D5A8;
	sub_83226F60(ctx, base);
	// 8321D5A8: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8321D5AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5B4: 480099AD  bl 0x83226f60
	ctx.lr = 0x8321D5B8;
	sub_83226F60(ctx, base);
	// 8321D5B8: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8321D5BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5C4: 4800999D  bl 0x83226f60
	ctx.lr = 0x8321D5C8;
	sub_83226F60(ctx, base);
	// 8321D5C8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8321D5CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5D4: 4800998D  bl 0x83226f60
	ctx.lr = 0x8321D5D8;
	sub_83226F60(ctx, base);
	// 8321D5D8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8321D5DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5E4: 4800997D  bl 0x83226f60
	ctx.lr = 0x8321D5E8;
	sub_83226F60(ctx, base);
	// 8321D5E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8321D5EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D5F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D5F4: 4800996D  bl 0x83226f60
	ctx.lr = 0x8321D5F8;
	sub_83226F60(ctx, base);
	// 8321D5F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321D5FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D604: 4800995D  bl 0x83226f60
	ctx.lr = 0x8321D608;
	sub_83226F60(ctx, base);
	// 8321D608: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8321D60C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8321D610: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D618: 48008E99  bl 0x832264b0
	ctx.lr = 0x8321D61C;
	sub_832264B0(ctx, base);
	// 8321D61C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321D620: 4BA8BE38  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D628 size=544
    let mut pc: u32 = 0x8321D628;
    'dispatch: loop {
        match pc {
            0x8321D628 => {
    //   block [0x8321D628..0x8321D848)
	// 8321D628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D62C: 4BA8BDD1  bl 0x82ca93fc
	ctx.lr = 0x8321D630;
	sub_82CA93D0(ctx, base);
	// 8321D630: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D634: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8321D638: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321D63C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321D640: 3B8BFFDF  addi r28, r11, -0x21
	ctx.r[28].s64 = ctx.r[11].s64 + -33;
	// 8321D644: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321D648: 816AE9B4  lwz r11, -0x164c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5708 as u32) ) } as u64;
	// 8321D64C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8321D650: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8321D654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321D658: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321D65C: 419A0008  beq cr6, 0x8321d664
	if ctx.cr[6].eq {
	pc = 0x8321D664; continue 'dispatch;
	}
	// 8321D660: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D664: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D66C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321D670: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321D674: 4E800421  bctrl
	ctx.lr = 0x8321D678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321D678: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321D67C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321D680: 419A01C0  beq cr6, 0x8321d840
	if ctx.cr[6].eq {
	pc = 0x8321D840; continue 'dispatch;
	}
	// 8321D684: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321D688: 38BE00C0  addi r5, r30, 0xc0
	ctx.r[5].s64 = ctx.r[30].s64 + 192;
	// 8321D68C: 386BE9B8  addi r3, r11, -0x1648
	ctx.r[3].s64 = ctx.r[11].s64 + -5704;
	// 8321D690: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D694: 48017F6D  bl 0x83235600
	ctx.lr = 0x8321D698;
	sub_83235600(ctx, base);
	// 8321D698: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321D69C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321D6A0: 3BAB08D7  addi r29, r11, 0x8d7
	ctx.r[29].s64 = ctx.r[11].s64 + 2263;
	// 8321D6A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D6A8: 389DFFFF  addi r4, r29, -1
	ctx.r[4].s64 = ctx.r[29].s64 + -1;
	// 8321D6AC: 4B00F825  bl 0x8222ced0
	ctx.lr = 0x8321D6B0;
	sub_8222CED0(ctx, base);
	// 8321D6B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8321D6B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321D6B8: 816AE9BC  lwz r11, -0x1644(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5700 as u32) ) } as u64;
	// 8321D6BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321D6C0: 419A0008  beq cr6, 0x8321d6c8
	if ctx.cr[6].eq {
	pc = 0x8321D6C8; continue 'dispatch;
	}
	// 8321D6C4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D6C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8321D6CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D6D0: 4B7E46B1  bl 0x82a01d80
	ctx.lr = 0x8321D6D4;
	sub_82A01D80(ctx, base);
	// 8321D6D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D6D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D6DC: 4AFD145D  bl 0x821eeb38
	ctx.lr = 0x8321D6E0;
	sub_821EEB38(ctx, base);
	// 8321D6E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D6E4: 4B9E610D  bl 0x82c037f0
	ctx.lr = 0x8321D6E8;
	sub_82C037F0(ctx, base);
	// 8321D6E8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8321D6EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D6F0: 4AFF76E9  bl 0x82214dd8
	ctx.lr = 0x8321D6F4;
	sub_82214DD8(ctx, base);
	// 8321D6F4: 933E00C4  stw r25, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[25].u32 ) };
	// 8321D6F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8321D6FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D700: 4B057D39  bl 0x82275438
	ctx.lr = 0x8321D704;
	sub_82275438(ctx, base);
	// 8321D704: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321D708: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8321D70C: 816BE9C0  lwz r11, -0x1640(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5696 as u32) ) } as u64;
	// 8321D710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321D714: 419A0008  beq cr6, 0x8321d71c
	if ctx.cr[6].eq {
	pc = 0x8321D71C; continue 'dispatch;
	}
	// 8321D718: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D71C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8321D720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D724: 4B7E465D  bl 0x82a01d80
	ctx.lr = 0x8321D728;
	sub_82A01D80(ctx, base);
	// 8321D728: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D72C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D730: 4AFD1409  bl 0x821eeb38
	ctx.lr = 0x8321D734;
	sub_821EEB38(ctx, base);
	// 8321D734: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D738: 4B9E60B9  bl 0x82c037f0
	ctx.lr = 0x8321D73C;
	sub_82C037F0(ctx, base);
	// 8321D73C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321D740: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321D744: 4AFF7695  bl 0x82214dd8
	ctx.lr = 0x8321D748;
	sub_82214DD8(ctx, base);
	// 8321D748: 93BE00C8  stw r29, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[29].u32 ) };
	// 8321D74C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321D750: 38BE00CC  addi r5, r30, 0xcc
	ctx.r[5].s64 = ctx.r[30].s64 + 204;
	// 8321D754: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 8321D758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D75C: 48017EA5  bl 0x83235600
	ctx.lr = 0x8321D760;
	sub_83235600(ctx, base);
	// 8321D760: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8321D764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D76C: 48009725  bl 0x83226e90
	ctx.lr = 0x8321D770;
	sub_83226E90(ctx, base);
	// 8321D770: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8321D774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D77C: 480098B5  bl 0x83227030
	ctx.lr = 0x8321D780;
	sub_83227030(ctx, base);
	// 8321D780: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8321D784: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D788: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D78C: 480098A5  bl 0x83227030
	ctx.lr = 0x8321D790;
	sub_83227030(ctx, base);
	// 8321D790: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8321D794: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D79C: 48009895  bl 0x83227030
	ctx.lr = 0x8321D7A0;
	sub_83227030(ctx, base);
	// 8321D7A0: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 8321D7A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7AC: 48009885  bl 0x83227030
	ctx.lr = 0x8321D7B0;
	sub_83227030(ctx, base);
	// 8321D7B0: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 8321D7B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7BC: 48009875  bl 0x83227030
	ctx.lr = 0x8321D7C0;
	sub_83227030(ctx, base);
	// 8321D7C0: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 8321D7C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7CC: 48009865  bl 0x83227030
	ctx.lr = 0x8321D7D0;
	sub_83227030(ctx, base);
	// 8321D7D0: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8321D7D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7DC: 48009855  bl 0x83227030
	ctx.lr = 0x8321D7E0;
	sub_83227030(ctx, base);
	// 8321D7E0: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8321D7E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7EC: 48009845  bl 0x83227030
	ctx.lr = 0x8321D7F0;
	sub_83227030(ctx, base);
	// 8321D7F0: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8321D7F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D7F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D7FC: 48009835  bl 0x83227030
	ctx.lr = 0x8321D800;
	sub_83227030(ctx, base);
	// 8321D800: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8321D804: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D80C: 48009825  bl 0x83227030
	ctx.lr = 0x8321D810;
	sub_83227030(ctx, base);
	// 8321D810: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8321D814: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8321D818: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321D81C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D820: 48023EA9  bl 0x832416c8
	ctx.lr = 0x8321D824;
	sub_832416C8(ctx, base);
	// 8321D824: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D82C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321D830: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8321D834: 4E800421  bctrl
	ctx.lr = 0x8321D838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321D838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321D83C: 4AFF759D  bl 0x82214dd8
	ctx.lr = 0x8321D840;
	sub_82214DD8(ctx, base);
	// 8321D840: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8321D844: 4BA8BC08  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321D848 size=4
    let mut pc: u32 = 0x8321D848;
    'dispatch: loop {
        match pc {
            0x8321D848 => {
    //   block [0x8321D848..0x8321D84C)
	// 8321D848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D850 size=124
    let mut pc: u32 = 0x8321D850;
    'dispatch: loop {
        match pc {
            0x8321D850 => {
    //   block [0x8321D850..0x8321D8CC)
	// 8321D850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321D858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321D85C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321D860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321D868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321D86C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321D870: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D874: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321D878: 897F00C0  lbz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321D87C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321D880: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8321D884: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321D888: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321D88C: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8321D890: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 8321D894: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8321D898: 4E800421  bctrl
	ctx.lr = 0x8321D89C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321D89C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321D8A0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321D8A4: 4B804D55  bl 0x82a225f8
	ctx.lr = 0x8321D8A8;
	sub_82A225F8(ctx, base);
	// 8321D8A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321D8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D8B0: 48007BF9  bl 0x832254a8
	ctx.lr = 0x8321D8B4;
	sub_832254A8(ctx, base);
	// 8321D8B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321D8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321D8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321D8C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321D8C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321D8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D8D0 size=80
    let mut pc: u32 = 0x8321D8D0;
    'dispatch: loop {
        match pc {
            0x8321D8D0 => {
    //   block [0x8321D8D0..0x8321D920)
	// 8321D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321D8D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321D8DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D8E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321D8E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8321D8E8: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 8321D8EC: 394B2640  addi r10, r11, 0x2640
	ctx.r[10].s64 = ctx.r[11].s64 + 9792;
	// 8321D8F0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321D8F4: 4B7F3675  bl 0x82a10f68
	ctx.lr = 0x8321D8F8;
	sub_82A10F68(ctx, base);
	// 8321D8F8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8321D8FC: 4B7F358D  bl 0x82a10e88
	ctx.lr = 0x8321D900;
	sub_82A10E88(ctx, base);
	// 8321D900: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8321D904: 39092720  addi r8, r9, 0x2720
	ctx.r[8].s64 = ctx.r[9].s64 + 10016;
	// 8321D908: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8321D90C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321D910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321D914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321D918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321D91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321D920 size=76
    let mut pc: u32 = 0x8321D920;
    'dispatch: loop {
        match pc {
            0x8321D920 => {
    //   block [0x8321D920..0x8321D96C)
	// 8321D920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321D928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321D92C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321D930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321D938: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321D93C: 480085A5  bl 0x83225ee0
	ctx.lr = 0x8321D940;
	sub_83225EE0(ctx, base);
	// 8321D940: 897E00C0  lbz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321D944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321D948: 997F00C0  stb r11, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 8321D94C: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321D950: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8321D954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321D958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321D95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321D960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321D964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321D968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321D970 size=212
    let mut pc: u32 = 0x8321D970;
    'dispatch: loop {
        match pc {
            0x8321D970 => {
    //   block [0x8321D970..0x8321DA44)
	// 8321D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321D974: 4BA8BA99  bl 0x82ca940c
	ctx.lr = 0x8321D978;
	sub_82CA93D0(ctx, base);
	// 8321D978: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 8321D97C: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8321D980: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8321D984: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321D988: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321D98C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8321D990: 3BEB92D4  addi r31, r11, -0x6d2c
	ctx.r[31].s64 = ctx.r[11].s64 + -27948;
	// 8321D994: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 8321D998: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321D99C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321D9A0: C3FF01B0  lfs f31, 0x1b0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8321D9A4: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8321D9A8: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321D9AC: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321D9B0: C3BF01BC  lfs f29, 0x1bc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8321D9B4: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 8321D9B8: 7CE43378  or r4, r7, r6
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8321D9BC: 7C09242E  lfsx f0, r9, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321D9C0: FDA0F86E  fsel f13, f0, f1, f31
	ctx.f[13].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 8321D9C4: ED8DE828  fsubs f12, f13, f29
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[29].f64) as f32) as f64);
	// 8321D9C8: FF0CF800  fcmpu cr6, f12, f31
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[31].f64);
	// 8321D9CC: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321D9D0: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8321D9D4: 5568F77A  rlwinm r8, r11, 0x1e, 0x1d, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8321D9D8: 7D474378  or r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8321D9DC: 7D693C2E  lfsx f11, r9, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8321D9E0: FFCB6F6E  fsel f30, f11, f29, f13
	ctx.f[30].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[29].f64 } else { ctx.f[13].f64 };
	// 8321D9E4: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8321D9E8: 48023819  bl 0x83241200
	ctx.lr = 0x8321D9EC;
	sub_83241200(ctx, base);
	// 8321D9EC: FF1EF800  fcmpu cr6, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 8321D9F0: 40990040  ble cr6, 0x8321da30
	if !ctx.cr[6].gt {
	pc = 0x8321DA30; continue 'dispatch;
	}
	// 8321D9F4: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 8321D9F8: 41980024  blt cr6, 0x8321da1c
	if ctx.cr[6].lt {
	pc = 0x8321DA1C; continue 'dispatch;
	}
	// 8321D9FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8321DA00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DA04: 4BFFFF1D  bl 0x8321d920
	ctx.lr = 0x8321DA08;
	sub_8321D920(ctx, base);
	// 8321DA08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321DA0C: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8321DA10: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321DA14: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8321DA18: 4BA8BA44  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8321DA1C: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321DA20: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8321DA24: 4099000C  ble cr6, 0x8321da30
	if !ctx.cr[6].gt {
	pc = 0x8321DA30; continue 'dispatch;
	}
	// 8321DA28: 897D00C0  lbz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321DA2C: 997E00C0  stb r11, 0xc0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 8321DA30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321DA34: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8321DA38: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8321DA3C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8321DA40: 4BA8BA1C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321DA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321DA48 size=88
    let mut pc: u32 = 0x8321DA48;
    'dispatch: loop {
        match pc {
            0x8321DA48 => {
    //   block [0x8321DA48..0x8321DAA0)
	// 8321DA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321DA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321DA50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8321DA54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321DA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321DA5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321DA60: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8321DA64: 4800877D  bl 0x832261e0
	ctx.lr = 0x8321DA68;
	sub_832261E0(ctx, base);
	// 8321DA68: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321DA6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321DA70: 394B278C  addi r10, r11, 0x278c
	ctx.r[10].s64 = ctx.r[11].s64 + 10124;
	// 8321DA74: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321DA78: 893E00C0  lbz r9, 0xc0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8321DA7C: 993F00C0  stb r9, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[9].u8 ) };
	// 8321DA80: 811E00C4  lwz r8, 0xc4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8321DA84: 911F00C4  stw r8, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[8].u32 ) };
	// 8321DA88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321DA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321DA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321DA94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8321DA98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321DA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321DAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321DAA0 size=128
    let mut pc: u32 = 0x8321DAA0;
    'dispatch: loop {
        match pc {
            0x8321DAA0 => {
    //   block [0x8321DAA0..0x8321DB20)
	// 8321DAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321DAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321DAA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321DAAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321DAB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321DAB4: 4800891D  bl 0x832263d0
	ctx.lr = 0x8321DAB8;
	sub_832263D0(ctx, base);
	// 8321DAB8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8321DABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8321DAC0: 392B278C  addi r9, r11, 0x278c
	ctx.r[9].s64 = ctx.r[11].s64 + 10124;
	// 8321DAC4: 995F00C0  stb r10, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[10].u8 ) };
	// 8321DAC8: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 8321DACC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8321DAD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321DAD4: 388861EF  addi r4, r8, 0x61ef
	ctx.r[4].s64 = ctx.r[8].s64 + 25071;
	// 8321DAD8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DADC: 4B00F3F5  bl 0x8222ced0
	ctx.lr = 0x8321DAE0;
	sub_8222CED0(ctx, base);
	// 8321DAE0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321DAE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DAE8: 4AFD1051  bl 0x821eeb38
	ctx.lr = 0x8321DAEC;
	sub_821EEB38(ctx, base);
	// 8321DAEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DAF0: 4B9E5D01  bl 0x82c037f0
	ctx.lr = 0x8321DAF4;
	sub_82C037F0(ctx, base);
	// 8321DAF4: 907F00C4  stw r3, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[3].u32 ) };
	// 8321DAF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DAFC: 4AFF72DD  bl 0x82214dd8
	ctx.lr = 0x8321DB00;
	sub_82214DD8(ctx, base);
	// 8321DB00: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DB04: 4AFF72D5  bl 0x82214dd8
	ctx.lr = 0x8321DB08;
	sub_82214DD8(ctx, base);
	// 8321DB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321DB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321DB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321DB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321DB18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321DB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321DB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321DB20 size=260
    let mut pc: u32 = 0x8321DB20;
    'dispatch: loop {
        match pc {
            0x8321DB20 => {
    //   block [0x8321DB20..0x8321DC24)
	// 8321DB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321DB24: 4BA8B8E9  bl 0x82ca940c
	ctx.lr = 0x8321DB28;
	sub_82CA93D0(ctx, base);
	// 8321DB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321DB2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321DB30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321DB34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8321DB38: 386BE688  addi r3, r11, -0x1978
	ctx.r[3].s64 = ctx.r[11].s64 + -6520;
	// 8321DB3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321DB40: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8321DB44: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8321DB48: 480231E9  bl 0x83240d30
	ctx.lr = 0x8321DB4C;
	sub_83240D30(ctx, base);
	// 8321DB4C: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321DB50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321DB54: 419A0020  beq cr6, 0x8321db74
	if ctx.cr[6].eq {
	pc = 0x8321DB74; continue 'dispatch;
	}
	// 8321DB58: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8321DB5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DB60: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8321DB64: 4AF99FB5  bl 0x821b7b18
	ctx.lr = 0x8321DB68;
	sub_821B7B18(ctx, base);
	// 8321DB68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321DB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321DB70: 4BA8B8EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8321DB74: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 8321DB78: 4B0016E1  bl 0x8221f258
	ctx.lr = 0x8321DB7C;
	sub_8221F258(ctx, base);
	// 8321DB7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321DB80: 419A0014  beq cr6, 0x8321db94
	if ctx.cr[6].eq {
	pc = 0x8321DB94; continue 'dispatch;
	}
	// 8321DB84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321DB88: 4BFFFF19  bl 0x8321daa0
	ctx.lr = 0x8321DB8C;
	sub_8321DAA0(ctx, base);
	// 8321DB8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321DB90: 48000008  b 0x8321db98
	pc = 0x8321DB98; continue 'dispatch;
	// 8321DB94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DB98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DB9C: 4B0554BD  bl 0x82273058
	ctx.lr = 0x8321DBA0;
	sub_82273058(ctx, base);
	// 8321DBA0: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8321DBA4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321DBA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321DBAC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8321DBB0: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8321DBB4: 419A0020  beq cr6, 0x8321dbd4
	if ctx.cr[6].eq {
	pc = 0x8321DBD4; continue 'dispatch;
	}
	// 8321DBB8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321DBBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DBC0: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321DBC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321DBC8: 7D60F92D  stwcx. r11, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321DBCC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DBD0: 4082FFE8  bne 0x8321dbb8
	if !ctx.cr[0].eq {
	pc = 0x8321DBB8; continue 'dispatch;
	}
	// 8321DBD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321DBD8: 4800E1D9  bl 0x8322bdb0
	ctx.lr = 0x8321DBDC;
	sub_8322BDB0(ctx, base);
	// 8321DBDC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321DBE0: 4AF99F39  bl 0x821b7b18
	ctx.lr = 0x8321DBE4;
	sub_821B7B18(ctx, base);
	// 8321DBE4: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8321DBE8: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8321DBEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321DBF0: 419A0020  beq cr6, 0x8321dc10
	if ctx.cr[6].eq {
	pc = 0x8321DC10; continue 'dispatch;
	}
	// 8321DBF4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321DBF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DBFC: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321DC00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321DC04: 7D60F92D  stwcx. r11, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321DC08: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DC0C: 4082FFE8  bne 0x8321dbf4
	if !ctx.cr[0].eq {
	pc = 0x8321DBF4; continue 'dispatch;
	}
	// 8321DC10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DC14: 4AF99F05  bl 0x821b7b18
	ctx.lr = 0x8321DC18;
	sub_821B7B18(ctx, base);
	// 8321DC18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8321DC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321DC20: 4BA8B83C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8321DC28 size=728
    let mut pc: u32 = 0x8321DC28;
    'dispatch: loop {
        match pc {
            0x8321DC28 => {
    //   block [0x8321DC28..0x8321DF00)
	// 8321DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321DC2C: 4BA8B7CD  bl 0x82ca93f8
	ctx.lr = 0x8321DC30;
	sub_82CA93D0(ctx, base);
	// 8321DC30: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8321DC34: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321DC38: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8321DC3C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8321DC40: 3BEB9490  addi r31, r11, -0x6b70
	ctx.r[31].s64 = ctx.r[11].s64 + -27504;
	// 8321DC44: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8321DC48: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321DC4C: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 8321DC50: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8321DC54: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8321DC58: C01FFFF4  lfs f0, -0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8321DC5C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8321DC60: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321DC64: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8321DC68: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 8321DC6C: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8321DC70: 7D892C2E  lfsx f12, r9, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8321DC74: FD6C006E  fsel f11, f12, f1, f0
	ctx.f[11].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[0].f64 };
	// 8321DC78: ED4B6828  fsubs f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8321DC7C: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 8321DC80: 7C800026  mfcr r4
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[4].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8321DC84: 5483DF7A  rlwinm r3, r4, 0x1b, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 8321DC88: 548BF77A  rlwinm r11, r4, 0x1e, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000003u64;
	// 8321DC8C: 7C6A5B78  or r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 8321DC90: 7D29542E  lfsx f9, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8321DC94: FFE95B6E  fsel f31, f9, f13, f11
	ctx.f[31].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[11].f64 };
	// 8321DC98: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8321DC9C: 41990048  bgt cr6, 0x8321dce4
	if ctx.cr[6].gt {
	pc = 0x8321DCE4; continue 'dispatch;
	}
	// 8321DCA0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DCA4: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DCA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321DCAC: 91780004  stw r11, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321DCB0: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321DCB4: 419A023C  beq cr6, 0x8321def0
	if ctx.cr[6].eq {
	pc = 0x8321DEF0; continue 'dispatch;
	}
	// 8321DCB8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8321DCBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DCC0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8321DCC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8321DCC8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321DCCC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DCD0: 4082FFE8  bne 0x8321dcb8
	if !ctx.cr[0].eq {
	pc = 0x8321DCB8; continue 'dispatch;
	}
	// 8321DCD4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321DCD8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321DCDC: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321DCE0: 4BA8B768  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 8321DCE4: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DCE8: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 8321DCEC: 41980044  blt cr6, 0x8321dd30
	if ctx.cr[6].lt {
	pc = 0x8321DD30; continue 'dispatch;
	}
	// 8321DCF0: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DCF4: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8321DCF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321DCFC: 91780004  stw r11, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321DD00: 419A01F0  beq cr6, 0x8321def0
	if ctx.cr[6].eq {
	pc = 0x8321DEF0; continue 'dispatch;
	}
	// 8321DD04: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8321DD08: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DD0C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8321DD10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8321DD14: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321DD18: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DD1C: 4082FFE8  bne 0x8321dd04
	if !ctx.cr[0].eq {
	pc = 0x8321DD04; continue 'dispatch;
	}
	// 8321DD20: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321DD24: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321DD28: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321DD2C: 4BA8B71C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 8321DD30: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DD34: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8321DD38: 83CA0004  lwz r30, 4(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DD3C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DD40: 4B130F81  bl 0x8234ecc0
	ctx.lr = 0x8321DD44;
	sub_8234ECC0(ctx, base);
	// 8321DD44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321DD48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DD4C: 4AFD24F5  bl 0x821f0240
	ctx.lr = 0x8321DD50;
	sub_821F0240(ctx, base);
	// 8321DD50: 389F0DD0  addi r4, r31, 0xdd0
	ctx.r[4].s64 = ctx.r[31].s64 + 3536;
	// 8321DD54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DD58: 4AFBCC69  bl 0x821da9c0
	ctx.lr = 0x8321DD5C;
	sub_821DA9C0(ctx, base);
	// 8321DD5C: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8321DD60: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8321DD64: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8321DD68: 4AFD0C29  bl 0x821ee990
	ctx.lr = 0x8321DD6C;
	sub_821EE990(ctx, base);
	// 8321DD6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8321DD70: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8321DD74: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8321DD78: 4AFC5391  bl 0x821e3108
	ctx.lr = 0x8321DD7C;
	sub_821E3108(ctx, base);
	// 8321DD7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321DD80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DD84: 4AFD24BD  bl 0x821f0240
	ctx.lr = 0x8321DD88;
	sub_821F0240(ctx, base);
	// 8321DD88: 389F0DCC  addi r4, r31, 0xdcc
	ctx.r[4].s64 = ctx.r[31].s64 + 3532;
	// 8321DD8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DD90: 4AFBCC31  bl 0x821da9c0
	ctx.lr = 0x8321DD94;
	sub_821DA9C0(ctx, base);
	// 8321DD94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8321DD98: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321DD9C: 4B130F25  bl 0x8234ecc0
	ctx.lr = 0x8321DDA0;
	sub_8234ECC0(ctx, base);
	// 8321DDA0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8321DDA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321DDA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321DDAC: 4AFC535D  bl 0x821e3108
	ctx.lr = 0x8321DDB0;
	sub_821E3108(ctx, base);
	// 8321DDB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321DDB4: 4AFF7025  bl 0x82214dd8
	ctx.lr = 0x8321DDB8;
	sub_82214DD8(ctx, base);
	// 8321DDB8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8321DDBC: 4AFF701D  bl 0x82214dd8
	ctx.lr = 0x8321DDC0;
	sub_82214DD8(ctx, base);
	// 8321DDC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DDC4: 4AFF7015  bl 0x82214dd8
	ctx.lr = 0x8321DDC8;
	sub_82214DD8(ctx, base);
	// 8321DDC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8321DDCC: 4AFF700D  bl 0x82214dd8
	ctx.lr = 0x8321DDD0;
	sub_82214DD8(ctx, base);
	// 8321DDD0: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8321DDD4: 4AFF7005  bl 0x82214dd8
	ctx.lr = 0x8321DDD8;
	sub_82214DD8(ctx, base);
	// 8321DDD8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321DDDC: 4AFF6FFD  bl 0x82214dd8
	ctx.lr = 0x8321DDE0;
	sub_82214DD8(ctx, base);
	// 8321DDE0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321DDE4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DDE8: 4AFD0D51  bl 0x821eeb38
	ctx.lr = 0x8321DDEC;
	sub_821EEB38(ctx, base);
	// 8321DDEC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DDF0: 4B9E5A01  bl 0x82c037f0
	ctx.lr = 0x8321DDF4;
	sub_82C037F0(ctx, base);
	// 8321DDF4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8321DDF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DDFC: 4AFF6FDD  bl 0x82214dd8
	ctx.lr = 0x8321DE00;
	sub_82214DD8(ctx, base);
	// 8321DE00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8321DE04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8321DE08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8321DE0C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8321DE10: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8321DE14: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8321DE18: 4800DF19  bl 0x8322bd30
	ctx.lr = 0x8321DE1C;
	sub_8322BD30(ctx, base);
	// 8321DE1C: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DE20: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DE24: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8321DE28: 419A0028  beq cr6, 0x8321de50
	if ctx.cr[6].eq {
	pc = 0x8321DE50; continue 'dispatch;
	}
	// 8321DE2C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321DE30: 4AF99CE9  bl 0x821b7b18
	ctx.lr = 0x8321DE34;
	sub_821B7B18(ctx, base);
	// 8321DE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DE38: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8321DE3C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 8321DE40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321DE44: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8321DE48: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8321DE4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8321DE50: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8321DE54: 4AF99CC5  bl 0x821b7b18
	ctx.lr = 0x8321DE58;
	sub_821B7B18(ctx, base);
	// 8321DE58: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8321DE5C: 409A0058  bne cr6, 0x8321deb4
	if !ctx.cr[6].eq {
	pc = 0x8321DEB4; continue 'dispatch;
	}
	// 8321DE60: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 8321DE64: 4B0013F5  bl 0x8221f258
	ctx.lr = 0x8321DE68;
	sub_8221F258(ctx, base);
	// 8321DE68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321DE6C: 419A0018  beq cr6, 0x8321de84
	if ctx.cr[6].eq {
	pc = 0x8321DE84; continue 'dispatch;
	}
	// 8321DE70: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8321DE74: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DE78: 4BFFFBD1  bl 0x8321da48
	ctx.lr = 0x8321DE7C;
	sub_8321DA48(ctx, base);
	// 8321DE7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8321DE80: 48000008  b 0x8321de88
	pc = 0x8321DE88; continue 'dispatch;
	// 8321DE84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8321DE88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321DE8C: 4B0551CD  bl 0x82273058
	ctx.lr = 0x8321DE90;
	sub_82273058(ctx, base);
	// 8321DE90: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8321DE94: 83C10068  lwz r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8321DE98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DE9C: 4801DF7D  bl 0x8323be18
	ctx.lr = 0x8321DEA0;
	sub_8323BE18(ctx, base);
	// 8321DEA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DEA4: 80B90000  lwz r5, 0(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DEA8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8321DEAC: 4BFFFAC5  bl 0x8321d970
	ctx.lr = 0x8321DEB0;
	sub_8321D970(ctx, base);
	// 8321DEB0: 83A1006C  lwz r29, 0x6c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8321DEB4: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8321DEB8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8321DEBC: 93B80004  stw r29, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8321DEC0: 419A0020  beq cr6, 0x8321dee0
	if ctx.cr[6].eq {
	pc = 0x8321DEE0; continue 'dispatch;
	}
	// 8321DEC4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8321DEC8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DECC: 7D60E828  lwarx r11, 0, r29
	// lwarx
	let ea = ctx.r[29].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8321DED0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8321DED4: 7D60E92D  stwcx. r11, 0, r29
	// stwcx.
	let addr = ctx.r[29].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8321DED8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8321DEDC: 4082FFE8  bne 0x8321dec4
	if !ctx.cr[0].eq {
	pc = 0x8321DEC4; continue 'dispatch;
	}
	// 8321DEE0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8321DEE4: 4AF99C35  bl 0x821b7b18
	ctx.lr = 0x8321DEE8;
	sub_821B7B18(ctx, base);
	// 8321DEE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8321DEEC: 4AFF6EED  bl 0x82214dd8
	ctx.lr = 0x8321DEF0;
	sub_82214DD8(ctx, base);
	// 8321DEF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8321DEF4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8321DEF8: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8321DEFC: 4BA8B54C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321DF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321DF00 size=372
    let mut pc: u32 = 0x8321DF00;
    'dispatch: loop {
        match pc {
            0x8321DF00 => {
    //   block [0x8321DF00..0x8321E074)
	// 8321DF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321DF04: 4BA8B505  bl 0x82ca9408
	ctx.lr = 0x8321DF08;
	sub_82CA93D0(ctx, base);
	// 8321DF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321DF0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321DF10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321DF14: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321DF18: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8321DF1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8321DF20: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8321DF24: 41980034  blt cr6, 0x8321df58
	if ctx.cr[6].lt {
	pc = 0x8321DF58; continue 'dispatch;
	}
	// 8321DF28: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321DF2C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8321DF30: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321DF34: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 8321DF38: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 8321DF3C: 38A90001  addi r5, r9, 1
	ctx.r[5].s64 = ctx.r[9].s64 + 1;
	// 8321DF40: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DF44: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8321DF48: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8321DF4C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8321DF50: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8321DF54: 48000014  b 0x8321df68
	pc = 0x8321DF68; continue 'dispatch;
	// 8321DF58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321DF5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321DF60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321DF64: 4B802F85  bl 0x82a20ee8
	ctx.lr = 0x8321DF68;
	sub_82A20EE8(ctx, base);
	// 8321DF68: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8321DF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8321DF70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DF74: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8321DF78: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321DF7C: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8321DF80: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 8321DF84: 98FE00C0  stb r7, 0xc0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[7].u8 ) };
	// 8321DF88: 4B8042D1  bl 0x82a22258
	ctx.lr = 0x8321DF8C;
	sub_82A22258(ctx, base);
	// 8321DF8C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8321DF90: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8321DF94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DF98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DF9C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321DFA0: 917E00C4  stw r11, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8321DFA4: 48008E1D  bl 0x83226dc0
	ctx.lr = 0x8321DFA8;
	sub_83226DC0(ctx, base);
	// 8321DFA8: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8321DFAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DFB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DFB4: 48008FAD  bl 0x83226f60
	ctx.lr = 0x8321DFB8;
	sub_83226F60(ctx, base);
	// 8321DFB8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8321DFBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DFC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DFC4: 48008F9D  bl 0x83226f60
	ctx.lr = 0x8321DFC8;
	sub_83226F60(ctx, base);
	// 8321DFC8: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 8321DFCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DFD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DFD4: 48008F8D  bl 0x83226f60
	ctx.lr = 0x8321DFD8;
	sub_83226F60(ctx, base);
	// 8321DFD8: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 8321DFDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DFE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DFE4: 48008F7D  bl 0x83226f60
	ctx.lr = 0x8321DFE8;
	sub_83226F60(ctx, base);
	// 8321DFE8: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 8321DFEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321DFF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321DFF4: 48008F6D  bl 0x83226f60
	ctx.lr = 0x8321DFF8;
	sub_83226F60(ctx, base);
	// 8321DFF8: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8321DFFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E004: 48008F5D  bl 0x83226f60
	ctx.lr = 0x8321E008;
	sub_83226F60(ctx, base);
	// 8321E008: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8321E00C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E014: 48008F4D  bl 0x83226f60
	ctx.lr = 0x8321E018;
	sub_83226F60(ctx, base);
	// 8321E018: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8321E01C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E024: 48008F3D  bl 0x83226f60
	ctx.lr = 0x8321E028;
	sub_83226F60(ctx, base);
	// 8321E028: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8321E02C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E034: 48008F2D  bl 0x83226f60
	ctx.lr = 0x8321E038;
	sub_83226F60(ctx, base);
	// 8321E038: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8321E03C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E044: 48008F1D  bl 0x83226f60
	ctx.lr = 0x8321E048;
	sub_83226F60(ctx, base);
	// 8321E048: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8321E04C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E050: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E054: 48008F0D  bl 0x83226f60
	ctx.lr = 0x8321E058;
	sub_83226F60(ctx, base);
	// 8321E058: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8321E05C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8321E060: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E064: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E068: 48008449  bl 0x832264b0
	ctx.lr = 0x8321E06C;
	sub_832264B0(ctx, base);
	// 8321E06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8321E070: 4BA8B3E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321E078 size=420
    let mut pc: u32 = 0x8321E078;
    'dispatch: loop {
        match pc {
            0x8321E078 => {
    //   block [0x8321E078..0x8321E21C)
	// 8321E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321E07C: 4BA8B389  bl 0x82ca9404
	ctx.lr = 0x8321E080;
	sub_82CA93D0(ctx, base);
	// 8321E080: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321E084: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8321E088: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8321E08C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8321E090: 3BABFFDF  addi r29, r11, -0x21
	ctx.r[29].s64 = ctx.r[11].s64 + -33;
	// 8321E094: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8321E098: 816AE99C  lwz r11, -0x1664(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5732 as u32) ) } as u64;
	// 8321E09C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8321E0A0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8321E0A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321E0A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8321E0AC: 419A0008  beq cr6, 0x8321e0b4
	if ctx.cr[6].eq {
	pc = 0x8321E0B4; continue 'dispatch;
	}
	// 8321E0B0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E0B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321E0BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321E0C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321E0C4: 4E800421  bctrl
	ctx.lr = 0x8321E0C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321E0C8: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8321E0CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8321E0D0: 419A0144  beq cr6, 0x8321e214
	if ctx.cr[6].eq {
	pc = 0x8321E214; continue 'dispatch;
	}
	// 8321E0D4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8321E0D8: 38BE00C0  addi r5, r30, 0xc0
	ctx.r[5].s64 = ctx.r[30].s64 + 192;
	// 8321E0DC: 386BE9A0  addi r3, r11, -0x1660
	ctx.r[3].s64 = ctx.r[11].s64 + -5728;
	// 8321E0E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E0E4: 4801751D  bl 0x83235600
	ctx.lr = 0x8321E0E8;
	sub_83235600(ctx, base);
	// 8321E0E8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8321E0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8321E0F0: 388A9217  addi r4, r10, -0x6de9
	ctx.r[4].s64 = ctx.r[10].s64 + -28137;
	// 8321E0F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321E0F8: 4B00EDD9  bl 0x8222ced0
	ctx.lr = 0x8321E0FC;
	sub_8222CED0(ctx, base);
	// 8321E0FC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8321E100: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8321E104: 8169E9A4  lwz r11, -0x165c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-5724 as u32) ) } as u64;
	// 8321E108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321E10C: 419A0008  beq cr6, 0x8321e114
	if ctx.cr[6].eq {
	pc = 0x8321E114; continue 'dispatch;
	}
	// 8321E110: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E114: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8321E118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321E11C: 4B7E3C65  bl 0x82a01d80
	ctx.lr = 0x8321E120;
	sub_82A01D80(ctx, base);
	// 8321E120: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8321E124: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321E128: 4AFD0A11  bl 0x821eeb38
	ctx.lr = 0x8321E12C;
	sub_821EEB38(ctx, base);
	// 8321E12C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321E130: 4B9E56C1  bl 0x82c037f0
	ctx.lr = 0x8321E134;
	sub_82C037F0(ctx, base);
	// 8321E134: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8321E138: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8321E13C: 4AFF6C9D  bl 0x82214dd8
	ctx.lr = 0x8321E140;
	sub_82214DD8(ctx, base);
	// 8321E140: 93BE00C4  stw r29, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[29].u32 ) };
	// 8321E144: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8321E148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E14C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E150: 48008D41  bl 0x83226e90
	ctx.lr = 0x8321E154;
	sub_83226E90(ctx, base);
	// 8321E154: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8321E158: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E15C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E160: 48008ED1  bl 0x83227030
	ctx.lr = 0x8321E164;
	sub_83227030(ctx, base);
	// 8321E164: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8321E168: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E16C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E170: 48008EC1  bl 0x83227030
	ctx.lr = 0x8321E174;
	sub_83227030(ctx, base);
	// 8321E174: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 8321E178: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E17C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E180: 48008EB1  bl 0x83227030
	ctx.lr = 0x8321E184;
	sub_83227030(ctx, base);
	// 8321E184: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 8321E188: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E18C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E190: 48008EA1  bl 0x83227030
	ctx.lr = 0x8321E194;
	sub_83227030(ctx, base);
	// 8321E194: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 8321E198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E19C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1A0: 48008E91  bl 0x83227030
	ctx.lr = 0x8321E1A4;
	sub_83227030(ctx, base);
	// 8321E1A4: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8321E1A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E1AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1B0: 48008E81  bl 0x83227030
	ctx.lr = 0x8321E1B4;
	sub_83227030(ctx, base);
	// 8321E1B4: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8321E1B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E1BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1C0: 48008E71  bl 0x83227030
	ctx.lr = 0x8321E1C4;
	sub_83227030(ctx, base);
	// 8321E1C4: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8321E1C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E1CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1D0: 48008E61  bl 0x83227030
	ctx.lr = 0x8321E1D4;
	sub_83227030(ctx, base);
	// 8321E1D4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8321E1D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E1DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1E0: 48008E51  bl 0x83227030
	ctx.lr = 0x8321E1E4;
	sub_83227030(ctx, base);
	// 8321E1E4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8321E1E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8321E1EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8321E1F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8321E1F4: 480234D5  bl 0x832416c8
	ctx.lr = 0x8321E1F8;
	sub_832416C8(ctx, base);
	// 8321E1F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E1FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8321E200: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8321E204: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8321E208: 4E800421  bctrl
	ctx.lr = 0x8321E20C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321E20C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8321E210: 4AFF6BC9  bl 0x82214dd8
	ctx.lr = 0x8321E214;
	sub_82214DD8(ctx, base);
	// 8321E214: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8321E218: 4BA8B23C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321E220 size=20
    let mut pc: u32 = 0x8321E220;
    'dispatch: loop {
        match pc {
            0x8321E220 => {
    //   block [0x8321E220..0x8321E234)
	// 8321E220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8321E224: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8321E228: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8321E22C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8321E230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321E238 size=116
    let mut pc: u32 = 0x8321E238;
    'dispatch: loop {
        match pc {
            0x8321E238 => {
    //   block [0x8321E238..0x8321E2AC)
	// 8321E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321E240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321E244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321E248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321E24C: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321E250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321E254: 419A0044  beq cr6, 0x8321e298
	if ctx.cr[6].eq {
	pc = 0x8321E298; continue 'dispatch;
	}
	// 8321E258: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E25C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321E260: 419A0038  beq cr6, 0x8321e298
	if ctx.cr[6].eq {
	pc = 0x8321E298; continue 'dispatch;
	}
	// 8321E264: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321E268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321E26C: 419A002C  beq cr6, 0x8321e298
	if ctx.cr[6].eq {
	pc = 0x8321E298; continue 'dispatch;
	}
	// 8321E270: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8321E274: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E278: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8321E27C: 390AF1BC  addi r8, r10, -0xe44
	ctx.r[8].s64 = ctx.r[10].s64 + -3652;
	// 8321E280: 80E90008  lwz r7, 8(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321E284: 80A8002C  lwz r5, 0x2c(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) } as u64;
	// 8321E288: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8321E28C: 4E800421  bctrl
	ctx.lr = 0x8321E290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321E290: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8321E294: 98DF0008  stb r6, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u8 ) };
	// 8321E298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321E29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321E2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321E2A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321E2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321E2B0 size=124
    let mut pc: u32 = 0x8321E2B0;
    'dispatch: loop {
        match pc {
            0x8321E2B0 => {
    //   block [0x8321E2B0..0x8321E32C)
	// 8321E2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321E2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8321E2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8321E2BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321E2C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321E2C4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E2C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8321E2CC: 419A004C  beq cr6, 0x8321e318
	if ctx.cr[6].eq {
	pc = 0x8321E318; continue 'dispatch;
	}
	// 8321E2D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321E2D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8321E2D8: 419A0040  beq cr6, 0x8321e318
	if ctx.cr[6].eq {
	pc = 0x8321E318; continue 'dispatch;
	}
	// 8321E2DC: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8321E2E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8321E2E4: 409A0034  bne cr6, 0x8321e318
	if !ctx.cr[6].eq {
	pc = 0x8321E318; continue 'dispatch;
	}
	// 8321E2E8: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8321E2EC: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 8321E2F0: 419A0028  beq cr6, 0x8321e318
	if ctx.cr[6].eq {
	pc = 0x8321E318; continue 'dispatch;
	}
	// 8321E2F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8321E2F8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8321E2FC: 392BF1BC  addi r9, r11, -0xe44
	ctx.r[9].s64 = ctx.r[11].s64 + -3652;
	// 8321E300: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8321E304: 80A9002C  lwz r5, 0x2c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 8321E308: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8321E30C: 4E800421  bctrl
	ctx.lr = 0x8321E310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8321E310: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8321E314: 98FF0008  stb r7, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 8321E318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8321E31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8321E320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8321E324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8321E328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8321E330 size=44
    let mut pc: u32 = 0x8321E330;
    'dispatch: loop {
        match pc {
            0x8321E330 => {
    //   block [0x8321E330..0x8321E35C)
	// 8321E330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8321E334: 4BA8B0D9  bl 0x82ca940c
	ctx.lr = 0x8321E338;
	sub_82CA93D0(ctx, base);
	// 8321E338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8321E33C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8321E340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8321E344: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8321E348: 4BFFFEF1  bl 0x8321e238
	ctx.lr = 0x8321E34C;
	sub_8321E238(ctx, base);
	// 8321E34C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8321E350: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8321E354: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8321E358: 4BA8B104  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321E360 size=8
    let mut pc: u32 = 0x8321E360;
    'dispatch: loop {
        match pc {
            0x8321E360 => {
    //   block [0x8321E360..0x8321E368)
	// 8321E360: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8321E364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8321E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8321E368 size=4
    let mut pc: u32 = 0x8321E368;
    'dispatch: loop {
        match pc {
            0x8321E368 => {
    //   block [0x8321E368..0x8321E36C)
	// 8321E368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


