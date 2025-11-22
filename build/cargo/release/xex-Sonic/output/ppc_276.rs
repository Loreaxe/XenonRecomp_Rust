pub fn sub_83160028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160028 size=8
    let mut pc: u32 = 0x83160028;
    'dispatch: loop {
        match pc {
            0x83160028 => {
    //   block [0x83160028..0x83160030)
	// 83160028: 886A0003  lbz r3, 3(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 8316002C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160030 size=48
    let mut pc: u32 = 0x83160030;
    'dispatch: loop {
        match pc {
            0x83160030 => {
    //   block [0x83160030..0x83160060)
	// 83160030: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83160034: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160038: 419A0060  beq cr6, 0x83160098
	if ctx.cr[6].eq {
		sub_83160098(ctx, base);
		return;
	}
	// 8316003C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160040: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 83160044: 41990054  bgt cr6, 0x83160098
	if ctx.cr[6].gt {
		sub_83160098(ctx, base);
		return;
	}
	// 83160048: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 8316004C: 398C0060  addi r12, r12, 0x60
	ctx.r[12].s64 = ctx.r[12].s64 + 96;
	// 83160050: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83160054: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160058: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8316005C: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
			// ERROR: 0x83160078
			return;
		},
		1 => {
			// ERROR: 0x83160078
			return;
		},
		2 => {
			// ERROR: 0x83160080
			return;
		},
		3 => {
			// ERROR: 0x83160080
			return;
		},
		4 => {
			// ERROR: 0x8316008C
			return;
		},
		5 => {
			// ERROR: 0x8316008C
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160060 size=32
    let mut pc: u32 = 0x83160060;
    'dispatch: loop {
        match pc {
            0x83160060 => {
    //   block [0x83160060..0x83160080)
	// 83160060: 83160078  lwz r24, 0x78(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(120 as u32) ) } as u64;
	// 83160064: 83160078  lwz r24, 0x78(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(120 as u32) ) } as u64;
	// 83160068: 83160080  lwz r24, 0x80(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(128 as u32) ) } as u64;
	// 8316006C: 83160080  lwz r24, 0x80(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(128 as u32) ) } as u64;
	// 83160070: 8316008C  lwz r24, 0x8c(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(140 as u32) ) } as u64;
	// 83160074: 8316008C  lwz r24, 0x8c(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(140 as u32) ) } as u64;
	// 83160078: 886B0018  lbz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316007C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160080 size=12
    let mut pc: u32 = 0x83160080;
    'dispatch: loop {
        match pc {
            0x83160080 => {
    //   block [0x83160080..0x8316008C)
	// 83160080: A16B0018  lhz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160084: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83160088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316008C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316008C size=12
    let mut pc: u32 = 0x8316008C;
    'dispatch: loop {
        match pc {
            0x8316008C => {
    //   block [0x8316008C..0x83160098)
	// 8316008C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160090: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83160094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160098 size=8
    let mut pc: u32 = 0x83160098;
    'dispatch: loop {
        match pc {
            0x83160098 => {
    //   block [0x83160098..0x831600A0)
	// 83160098: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316009C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831600A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831600A0 size=4
    let mut pc: u32 = 0x831600A0;
    'dispatch: loop {
        match pc {
            0x831600A0 => {
    //   block [0x831600A0..0x831600A4)
	// 831600A0: 4BFFFF00  b 0x8315ffa0
	sub_8315FFA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831600A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831600A8 size=96
    let mut pc: u32 = 0x831600A8;
    'dispatch: loop {
        match pc {
            0x831600A8 => {
    //   block [0x831600A8..0x83160108)
	// 831600A8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 831600AC: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831600B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831600B4: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 831600B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831600BC: 419A009C  beq cr6, 0x83160158
	if ctx.cr[6].eq {
		sub_83160158(ctx, base);
		return;
	}
	// 831600C0: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 831600C4: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831600C8: 40980090  bge cr6, 0x83160158
	if !ctx.cr[6].lt {
		sub_83160158(ctx, base);
		return;
	}
	// 831600CC: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 831600D0: A10B000A  lhz r8, 0xa(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 831600D4: 7D4A21D6  mullw r10, r10, r4
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831600D8: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831600DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831600E0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 831600E4: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 831600E8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831600EC: 419900D0  bgt cr6, 0x831601bc
	if ctx.cr[6].gt {
		sub_831601BC(ctx, base);
		return;
	}
	// 831600F0: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 831600F4: 398C0108  addi r12, r12, 0x108
	ctx.r[12].s64 = ctx.r[12].s64 + 264;
	// 831600F8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831600FC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160100: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83160104: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x83160120
			return;
		},
		1 => {
			// ERROR: 0x83160120
			return;
		},
		2 => {
			// ERROR: 0x83160128
			return;
		},
		3 => {
			// ERROR: 0x83160128
			return;
		},
		4 => {
			// ERROR: 0x83160140
			return;
		},
		5 => {
			// ERROR: 0x83160140
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160108 size=32
    let mut pc: u32 = 0x83160108;
    'dispatch: loop {
        match pc {
            0x83160108 => {
    //   block [0x83160108..0x83160128)
	// 83160108: 83160120  lwz r24, 0x120(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(288 as u32) ) } as u64;
	// 8316010C: 83160120  lwz r24, 0x120(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(288 as u32) ) } as u64;
	// 83160110: 83160128  lwz r24, 0x128(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(296 as u32) ) } as u64;
	// 83160114: 83160128  lwz r24, 0x128(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(296 as u32) ) } as u64;
	// 83160118: 83160140  lwz r24, 0x140(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(320 as u32) ) } as u64;
	// 8316011C: 83160140  lwz r24, 0x140(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(320 as u32) ) } as u64;
	// 83160120: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160128 size=24
    let mut pc: u32 = 0x83160128;
    'dispatch: loop {
        match pc {
            0x83160128 => {
    //   block [0x83160128..0x83160140)
	// 83160128: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316012C: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160130: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83160134: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160138: 5503043E  clrlwi r3, r8, 0x10
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 8316013C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160140 size=24
    let mut pc: u32 = 0x83160140;
    'dispatch: loop {
        match pc {
            0x83160140 => {
    //   block [0x83160140..0x83160158)
	// 83160140: 896A0002  lbz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160144: 894A0003  lbz r10, 3(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160148: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8316014C: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160150: 5503043E  clrlwi r3, r8, 0x10
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 83160154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160158 size=48
    let mut pc: u32 = 0x83160158;
    'dispatch: loop {
        match pc {
            0x83160158 => {
    //   block [0x83160158..0x83160188)
	// 83160158: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316015C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160160: 419A005C  beq cr6, 0x831601bc
	if ctx.cr[6].eq {
		sub_831601BC(ctx, base);
		return;
	}
	// 83160164: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160168: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 8316016C: 41990050  bgt cr6, 0x831601bc
	if ctx.cr[6].gt {
		sub_831601BC(ctx, base);
		return;
	}
	// 83160170: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 83160174: 398C0188  addi r12, r12, 0x188
	ctx.r[12].s64 = ctx.r[12].s64 + 392;
	// 83160178: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8316017C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160180: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83160184: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
			// ERROR: 0x831601A0
			return;
		},
		1 => {
			// ERROR: 0x831601A0
			return;
		},
		2 => {
			// ERROR: 0x831601A8
			return;
		},
		3 => {
			// ERROR: 0x831601A8
			return;
		},
		4 => {
			// ERROR: 0x831601B0
			return;
		},
		5 => {
			// ERROR: 0x831601B0
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160188 size=32
    let mut pc: u32 = 0x83160188;
    'dispatch: loop {
        match pc {
            0x83160188 => {
    //   block [0x83160188..0x831601A8)
	// 83160188: 831601A0  lwz r24, 0x1a0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(416 as u32) ) } as u64;
	// 8316018C: 831601A0  lwz r24, 0x1a0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(416 as u32) ) } as u64;
	// 83160190: 831601A8  lwz r24, 0x1a8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(424 as u32) ) } as u64;
	// 83160194: 831601A8  lwz r24, 0x1a8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(424 as u32) ) } as u64;
	// 83160198: 831601B0  lwz r24, 0x1b0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(432 as u32) ) } as u64;
	// 8316019C: 831601B0  lwz r24, 0x1b0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(432 as u32) ) } as u64;
	// 831601A0: 886B0018  lbz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831601A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831601A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831601A8 size=8
    let mut pc: u32 = 0x831601A8;
    'dispatch: loop {
        match pc {
            0x831601A8 => {
    //   block [0x831601A8..0x831601B0)
	// 831601A8: A06B0018  lhz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831601AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831601B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831601B0 size=12
    let mut pc: u32 = 0x831601B0;
    'dispatch: loop {
        match pc {
            0x831601B0 => {
    //   block [0x831601B0..0x831601BC)
	// 831601B0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831601B4: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831601B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831601BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831601BC size=8
    let mut pc: u32 = 0x831601BC;
    'dispatch: loop {
        match pc {
            0x831601BC => {
    //   block [0x831601BC..0x831601C4)
	// 831601BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831601C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831601C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831601C8 size=4
    let mut pc: u32 = 0x831601C8;
    'dispatch: loop {
        match pc {
            0x831601C8 => {
    //   block [0x831601C8..0x831601CC)
	// 831601C8: 4BFFFEE0  b 0x831600a8
	sub_831600A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831601D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831601D0 size=96
    let mut pc: u32 = 0x831601D0;
    'dispatch: loop {
        match pc {
            0x831601D0 => {
    //   block [0x831601D0..0x83160230)
	// 831601D0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 831601D4: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831601D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831601DC: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 831601E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831601E4: 419A00AC  beq cr6, 0x83160290
	if ctx.cr[6].eq {
		sub_83160290(ctx, base);
		return;
	}
	// 831601E8: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 831601EC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831601F0: 409800A0  bge cr6, 0x83160290
	if !ctx.cr[6].lt {
		sub_83160290(ctx, base);
		return;
	}
	// 831601F4: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 831601F8: A0EB000A  lhz r7, 0xa(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 831601FC: 7D0A21D6  mullw r8, r10, r4
	ctx.r[8].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 83160200: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160204: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160208: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 8316020C: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 83160210: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83160214: 419900DC  bgt cr6, 0x831602f0
	if ctx.cr[6].gt {
		sub_831602F0(ctx, base);
		return;
	}
	// 83160218: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 8316021C: 398C0230  addi r12, r12, 0x230
	ctx.r[12].s64 = ctx.r[12].s64 + 560;
	// 83160220: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83160224: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160228: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8316022C: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
			// ERROR: 0x83160248
			return;
		},
		1 => {
			// ERROR: 0x83160248
			return;
		},
		2 => {
			// ERROR: 0x83160250
			return;
		},
		3 => {
			// ERROR: 0x83160250
			return;
		},
		4 => {
			// ERROR: 0x83160264
			return;
		},
		5 => {
			// ERROR: 0x83160264
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160230 size=32
    let mut pc: u32 = 0x83160230;
    'dispatch: loop {
        match pc {
            0x83160230 => {
    //   block [0x83160230..0x83160250)
	// 83160230: 83160248  lwz r24, 0x248(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(584 as u32) ) } as u64;
	// 83160234: 83160248  lwz r24, 0x248(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(584 as u32) ) } as u64;
	// 83160238: 83160250  lwz r24, 0x250(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(592 as u32) ) } as u64;
	// 8316023C: 83160250  lwz r24, 0x250(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(592 as u32) ) } as u64;
	// 83160240: 83160264  lwz r24, 0x264(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(612 as u32) ) } as u64;
	// 83160244: 83160264  lwz r24, 0x264(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(612 as u32) ) } as u64;
	// 83160248: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160250 size=20
    let mut pc: u32 = 0x83160250;
    'dispatch: loop {
        match pc {
            0x83160250 => {
    //   block [0x83160250..0x83160264)
	// 83160250: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160254: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160258: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8316025C: 7D034B78  or r3, r8, r9
	ctx.r[3].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 83160260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160264 size=44
    let mut pc: u32 = 0x83160264;
    'dispatch: loop {
        match pc {
            0x83160264 => {
    //   block [0x83160264..0x83160290)
	// 83160264: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160268: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8316026C: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83160270: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160274: 88CB0003  lbz r6, 3(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160278: 7D054B78  or r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8316027C: 54A4402E  slwi r4, r5, 8
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83160280: 7C833B78  or r3, r4, r7
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[7].u64;
	// 83160284: 546B402E  slwi r11, r3, 8
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83160288: 7D633378  or r3, r11, r6
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 8316028C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160290 size=48
    let mut pc: u32 = 0x83160290;
    'dispatch: loop {
        match pc {
            0x83160290 => {
    //   block [0x83160290..0x831602C0)
	// 83160290: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83160294: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160298: 419A0058  beq cr6, 0x831602f0
	if ctx.cr[6].eq {
		sub_831602F0(ctx, base);
		return;
	}
	// 8316029C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831602A0: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 831602A4: 4199004C  bgt cr6, 0x831602f0
	if ctx.cr[6].gt {
		sub_831602F0(ctx, base);
		return;
	}
	// 831602A8: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 831602AC: 398C02C0  addi r12, r12, 0x2c0
	ctx.r[12].s64 = ctx.r[12].s64 + 704;
	// 831602B0: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831602B4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831602B8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831602BC: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
			// ERROR: 0x831602D8
			return;
		},
		1 => {
			// ERROR: 0x831602D8
			return;
		},
		2 => {
			// ERROR: 0x831602E0
			return;
		},
		3 => {
			// ERROR: 0x831602E0
			return;
		},
		4 => {
			// ERROR: 0x831602E8
			return;
		},
		5 => {
			// ERROR: 0x831602E8
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831602C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831602C0 size=32
    let mut pc: u32 = 0x831602C0;
    'dispatch: loop {
        match pc {
            0x831602C0 => {
    //   block [0x831602C0..0x831602E0)
	// 831602C0: 831602D8  lwz r24, 0x2d8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(728 as u32) ) } as u64;
	// 831602C4: 831602D8  lwz r24, 0x2d8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(728 as u32) ) } as u64;
	// 831602C8: 831602E0  lwz r24, 0x2e0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(736 as u32) ) } as u64;
	// 831602CC: 831602E0  lwz r24, 0x2e0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(736 as u32) ) } as u64;
	// 831602D0: 831602E8  lwz r24, 0x2e8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(744 as u32) ) } as u64;
	// 831602D4: 831602E8  lwz r24, 0x2e8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(744 as u32) ) } as u64;
	// 831602D8: 886B0018  lbz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831602DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831602E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831602E0 size=8
    let mut pc: u32 = 0x831602E0;
    'dispatch: loop {
        match pc {
            0x831602E0 => {
    //   block [0x831602E0..0x831602E8)
	// 831602E0: A06B0018  lhz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831602E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831602E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831602E8 size=8
    let mut pc: u32 = 0x831602E8;
    'dispatch: loop {
        match pc {
            0x831602E8 => {
    //   block [0x831602E8..0x831602F0)
	// 831602E8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831602EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831602F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831602F0 size=8
    let mut pc: u32 = 0x831602F0;
    'dispatch: loop {
        match pc {
            0x831602F0 => {
    //   block [0x831602F0..0x831602F8)
	// 831602F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831602F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831602F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831602F8 size=4
    let mut pc: u32 = 0x831602F8;
    'dispatch: loop {
        match pc {
            0x831602F8 => {
    //   block [0x831602F8..0x831602FC)
	// 831602F8: 4BFFFED8  b 0x831601d0
	sub_831601D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160300 size=152
    let mut pc: u32 = 0x83160300;
    'dispatch: loop {
        match pc {
            0x83160300 => {
    //   block [0x83160300..0x83160398)
	// 83160300: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83160304: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160308: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316030C: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 83160310: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160314: 419A0084  beq cr6, 0x83160398
	if ctx.cr[6].eq {
		sub_83160398(ctx, base);
		return;
	}
	// 83160318: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316031C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83160320: 40980078  bge cr6, 0x83160398
	if !ctx.cr[6].lt {
		sub_83160398(ctx, base);
		return;
	}
	// 83160324: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 83160328: A12B000A  lhz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 8316032C: 7D6A21D6  mullw r11, r10, r4
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 83160330: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160334: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83160338: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316033C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160340: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160344: 792747E4  rldicr r7, r9, 8, 0x3f
	ctx.r[7].u64 = (ctx.r[9].u64).rotate_left(8) & 0xFFFFFFFFFFFFFFFF;
	// 83160348: 88CB0002  lbz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8316034C: 88AB0003  lbz r5, 3(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160350: 7CE44378  or r4, r7, r8
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 83160354: 886B0004  lbz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83160358: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 8316035C: 788945E4  sldi r9, r4, 8
	ctx.r[9].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 83160360: 88EB0007  lbz r7, 7(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83160364: 890B0006  lbz r8, 6(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 83160368: 7D263378  or r6, r9, r6
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[6].u64;
	// 8316036C: 78C445E4  sldi r4, r6, 8
	ctx.r[4].u64 = ctx.r[6].u64.wrapping_shl(8);
	ctx.r[4].u32 = ctx.r[4].u64 as u32;
	// 83160370: 7C8B2B78  or r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[5].u64;
	// 83160374: 796945E4  sldi r9, r11, 8
	ctx.r[9].u64 = ctx.r[11].u64.wrapping_shl(8);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 83160378: 7D261B78  or r6, r9, r3
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[3].u64;
	// 8316037C: 78C545E4  sldi r5, r6, 8
	ctx.r[5].u64 = ctx.r[6].u64.wrapping_shl(8);
	ctx.r[5].u32 = ctx.r[5].u64 as u32;
	// 83160380: 7CA45378  or r4, r5, r10
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 83160384: 788345E4  sldi r3, r4, 8
	ctx.r[3].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[3].u32 = ctx.r[3].u64 as u32;
	// 83160388: 7C6B4378  or r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[8].u64;
	// 8316038C: 796A45E4  sldi r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 83160390: 7D433B78  or r3, r10, r7
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83160394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160398 size=20
    let mut pc: u32 = 0x83160398;
    'dispatch: loop {
        match pc {
            0x83160398 => {
    //   block [0x83160398..0x831603AC)
	// 83160398: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316039C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831603A0: 419A000C  beq cr6, 0x831603ac
	if ctx.cr[6].eq {
		sub_831603AC(ctx, base);
		return;
	}
	// 831603A4: E86B0018  ld r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 831603A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831603AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831603AC size=8
    let mut pc: u32 = 0x831603AC;
    'dispatch: loop {
        match pc {
            0x831603AC => {
    //   block [0x831603AC..0x831603B4)
	// 831603AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831603B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831603B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831603B8 size=112
    let mut pc: u32 = 0x831603B8;
    'dispatch: loop {
        match pc {
            0x831603B8 => {
    //   block [0x831603B8..0x83160428)
	// 831603B8: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 831603BC: 54AB2834  slwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831603C0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831603C4: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 831603C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831603CC: 419A005C  beq cr6, 0x83160428
	if ctx.cr[6].eq {
		sub_83160428(ctx, base);
		return;
	}
	// 831603D0: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 831603D4: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831603D8: 40980050  bge cr6, 0x83160428
	if !ctx.cr[6].lt {
		sub_83160428(ctx, base);
		return;
	}
	// 831603DC: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 831603E0: A12B000A  lhz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 831603E4: 7D6A21D6  mullw r11, r10, r4
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831603E8: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831603EC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831603F0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831603F4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831603F8: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 831603FC: 5527403E  rotlwi r7, r9, 8
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83160400: 88CB0002  lbz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160404: 88AB0003  lbz r5, 3(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160408: 7CE44378  or r4, r7, r8
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8316040C: 5483402E  slwi r3, r4, 8
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83160410: 7C6B3378  or r11, r3, r6
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[6].u64;
	// 83160414: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160418: 7D492B78  or r9, r10, r5
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 8316041C: 9121FFF0  stw r9, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u32 ) };
	// 83160420: C021FFF0  lfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83160424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83160428 size=20
    let mut pc: u32 = 0x83160428;
    'dispatch: loop {
        match pc {
            0x83160428 => {
    //   block [0x83160428..0x8316043C)
	// 83160428: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316042C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160430: 419A000C  beq cr6, 0x8316043c
	if ctx.cr[6].eq {
		sub_8316043C(ctx, base);
		return;
	}
	// 83160434: C02B0018  lfs f1, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83160438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316043C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8316043C size=12
    let mut pc: u32 = 0x8316043C;
    'dispatch: loop {
        match pc {
            0x8316043C => {
    //   block [0x8316043C..0x83160448)
	// 8316043C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83160440: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83160444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160448 size=120
    let mut pc: u32 = 0x83160448;
    'dispatch: loop {
        match pc {
            0x83160448 => {
    //   block [0x83160448..0x831604C0)
	// 83160448: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316044C: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160450: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160454: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 83160458: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8316045C: 419A0064  beq cr6, 0x831604c0
	if ctx.cr[6].eq {
		sub_831604C0(ctx, base);
		return;
	}
	// 83160460: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83160464: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83160468: 40980058  bge cr6, 0x831604c0
	if !ctx.cr[6].lt {
		sub_831604C0(ctx, base);
		return;
	}
	// 8316046C: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 83160470: A12B000A  lhz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83160474: 7D6A21D6  mullw r11, r10, r4
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 83160478: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316047C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83160480: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160484: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160488: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8316048C: 5527403E  rotlwi r7, r9, 8
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83160490: 88CB0002  lbz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160494: 88AB0003  lbz r5, 3(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160498: 7CE44378  or r4, r7, r8
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8316049C: 548B402E  slwi r11, r4, 8
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831604A0: 7D6A3378  or r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 831604A4: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831604A8: 7D2B2B78  or r11, r9, r5
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 831604AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831604B0: 419A0024  beq cr6, 0x831604d4
	if ctx.cr[6].eq {
		sub_831604D4(ctx, base);
		return;
	}
	// 831604B4: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831604B8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831604BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831604C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831604C0 size=20
    let mut pc: u32 = 0x831604C0;
    'dispatch: loop {
        match pc {
            0x831604C0 => {
    //   block [0x831604C0..0x831604D4)
	// 831604C0: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831604C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831604C8: 419A000C  beq cr6, 0x831604d4
	if ctx.cr[6].eq {
		sub_831604D4(ctx, base);
		return;
	}
	// 831604CC: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831604D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831604D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831604D4 size=8
    let mut pc: u32 = 0x831604D4;
    'dispatch: loop {
        match pc {
            0x831604D4 => {
    //   block [0x831604D4..0x831604DC)
	// 831604D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831604D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831604E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831604E0 size=56
    let mut pc: u32 = 0x831604E0;
    'dispatch: loop {
        match pc {
            0x831604E0 => {
    //   block [0x831604E0..0x83160518)
	// 831604E0: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831604E4: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831604E8: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831604EC: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 831604F0: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 831604F4: 7D265378  or r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831604F8: 54C5402E  slwi r5, r6, 8
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831604FC: 7CA44378  or r4, r5, r8
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[8].u64;
	// 83160500: 548B402E  slwi r11, r4, 8
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83160504: 7D6B3B78  or r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 83160508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316050C: 409A000C  bne cr6, 0x83160518
	if !ctx.cr[6].eq {
		sub_83160518(ctx, base);
		return;
	}
	// 83160510: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160518 size=12
    let mut pc: u32 = 0x83160518;
    'dispatch: loop {
        match pc {
            0x83160518 => {
    //   block [0x83160518..0x83160524)
	// 83160518: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316051C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83160520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160528 size=80
    let mut pc: u32 = 0x83160528;
    'dispatch: loop {
        match pc {
            0x83160528 => {
    //   block [0x83160528..0x83160578)
	// 83160528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316052C: 48047C41  bl 0x831a816c
	ctx.lr = 0x83160530;
	sub_831A8130(ctx, base);
	// 83160530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160534: 83E30028  lwz r31, 0x28(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83160538: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316053C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83160540: 419A002C  beq cr6, 0x8316056c
	if ctx.cr[6].eq {
	pc = 0x8316056C; continue 'dispatch;
	}
	// 83160544: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83160548: 897F0009  lbz r11, 9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 8316054C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83160550: 419A0010  beq cr6, 0x83160560
	if ctx.cr[6].eq {
	pc = 0x83160560; continue 'dispatch;
	}
	// 83160554: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160558: 480037A1  bl 0x83163cf8
	ctx.lr = 0x8316055C;
	sub_83163CF8(ctx, base);
	// 8316055C: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83160560: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83160564: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 83160568: 4082FFE0  bne 0x83160548
	if !ctx.cr[0].eq {
	pc = 0x83160548; continue 'dispatch;
	}
	// 8316056C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83160570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83160574: 48047C48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160578 size=12
    let mut pc: u32 = 0x83160578;
    'dispatch: loop {
        match pc {
            0x83160578 => {
    //   block [0x83160578..0x83160584)
	// 83160578: 2B05000B  cmplwi cr6, r5, 0xb
	ctx.cr[6].compare_u32(ctx.r[5].u32, 11 as u32, &mut ctx.xer);
	// 8316057C: 98A60000  stb r5, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83160580: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160584 size=24
    let mut pc: u32 = 0x83160584;
    'dispatch: loop {
        match pc {
            0x83160584 => {
    //   block [0x83160584..0x8316059C)
	// 83160584: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 83160588: 398C059C  addi r12, r12, 0x59c
	ctx.r[12].s64 = ctx.r[12].s64 + 1436;
	// 8316058C: 54A0103A  slwi r0, r5, 2
	ctx.r[0].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83160590: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160594: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83160598: 4E800420  bctr
	match ctx.r[5].u64 {
		0 => {
			// ERROR: 0x831605CC
			return;
		},
		1 => {
			// ERROR: 0x831605CC
			return;
		},
		2 => {
			// ERROR: 0x831605D8
			return;
		},
		3 => {
			// ERROR: 0x831605D8
			return;
		},
		4 => {
			// ERROR: 0x831605F0
			return;
		},
		5 => {
			// ERROR: 0x831605F0
			return;
		},
		6 => {
			// ERROR: 0x83160620
			return;
		},
		7 => {
			// ERROR: 0x83160620
			return;
		},
		8 => {
			// ERROR: 0x83160680
			return;
		},
		9 => {
			// ERROR: 0x831606B8
			return;
		},
		10 => {
			// ERROR: 0x83160720
			return;
		},
		11 => {
			// ERROR: 0x83160768
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316059C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316059C size=60
    let mut pc: u32 = 0x8316059C;
    'dispatch: loop {
        match pc {
            0x8316059C => {
    //   block [0x8316059C..0x831605D8)
	// 8316059C: 831605CC  lwz r24, 0x5cc(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1484 as u32) ) } as u64;
	// 831605A0: 831605CC  lwz r24, 0x5cc(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1484 as u32) ) } as u64;
	// 831605A4: 831605D8  lwz r24, 0x5d8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1496 as u32) ) } as u64;
	// 831605A8: 831605D8  lwz r24, 0x5d8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1496 as u32) ) } as u64;
	// 831605AC: 831605F0  lwz r24, 0x5f0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1520 as u32) ) } as u64;
	// 831605B0: 831605F0  lwz r24, 0x5f0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1520 as u32) ) } as u64;
	// 831605B4: 83160620  lwz r24, 0x620(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1568 as u32) ) } as u64;
	// 831605B8: 83160620  lwz r24, 0x620(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1568 as u32) ) } as u64;
	// 831605BC: 83160680  lwz r24, 0x680(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1664 as u32) ) } as u64;
	// 831605C0: 831606B8  lwz r24, 0x6b8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1720 as u32) ) } as u64;
	// 831605C4: 83160720  lwz r24, 0x720(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831605C8: 83160768  lwz r24, 0x768(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1896 as u32) ) } as u64;
	// 831605CC: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831605D0: 99660008  stb r11, 8(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 831605D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831605D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831605D8 size=24
    let mut pc: u32 = 0x831605D8;
    'dispatch: loop {
        match pc {
            0x831605D8 => {
    //   block [0x831605D8..0x831605F0)
	// 831605D8: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831605DC: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831605E0: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831605E4: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831605E8: B1060008  sth r8, 8(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[8].u16 ) };
	// 831605EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831605F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831605F0 size=48
    let mut pc: u32 = 0x831605F0;
    'dispatch: loop {
        match pc {
            0x831605F0 => {
    //   block [0x831605F0..0x83160620)
	// 831605F0: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831605F4: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831605F8: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831605FC: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160600: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160604: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160608: 54A4402E  slwi r4, r5, 8
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8316060C: 7C834378  or r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 83160610: 546B402E  slwi r11, r3, 8
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83160614: 7D6A3B78  or r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 83160618: 91460008  stw r10, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8316061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160620 size=96
    let mut pc: u32 = 0x83160620;
    'dispatch: loop {
        match pc {
            0x83160620 => {
    //   block [0x83160620..0x83160680)
	// 83160620: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160624: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160628: 796947E4  rldicr r9, r11, 8, 0x3f
	ctx.r[9].u64 = (ctx.r[11].u64).rotate_left(8) & 0xFFFFFFFFFFFFFFFF;
	// 8316062C: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160630: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160634: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160638: 88640004  lbz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316063C: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 83160640: 78AA45E4  sldi r10, r5, 8
	ctx.r[10].u64 = ctx.r[5].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 83160644: 89240006  lbz r9, 6(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 83160648: 88A40007  lbz r5, 7(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(7 as u32) ) } as u64;
	// 8316064C: 7D444378  or r4, r10, r8
	ctx.r[4].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 83160650: 788A45E4  sldi r10, r4, 8
	ctx.r[10].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 83160654: 7D483B78  or r8, r10, r7
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83160658: 790745E4  sldi r7, r8, 8
	ctx.r[7].u64 = ctx.r[8].u64.wrapping_shl(8);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8316065C: 7CE41B78  or r4, r7, r3
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[3].u64;
	// 83160660: 788345E4  sldi r3, r4, 8
	ctx.r[3].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[3].u32 = ctx.r[3].u64 as u32;
	// 83160664: 7C6B5B78  or r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 83160668: 796A45E4  sldi r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8316066C: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83160670: 792845E4  sldi r8, r9, 8
	ctx.r[8].u64 = ctx.r[9].u64.wrapping_shl(8);
	ctx.r[8].u32 = ctx.r[8].u64 as u32;
	// 83160674: 7D072B78  or r7, r8, r5
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 83160678: F8E60008  std r7, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 8316067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83160680 size=56
    let mut pc: u32 = 0x83160680;
    'dispatch: loop {
        match pc {
            0x83160680 => {
    //   block [0x83160680..0x831606B8)
	// 83160680: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160684: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160688: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8316068C: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160690: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160694: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160698: 54A4402E  slwi r4, r5, 8
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8316069C: 7C834378  or r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 831606A0: 546B402E  slwi r11, r3, 8
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831606A4: 7D6A3B78  or r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 831606A8: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 831606AC: C001FFF0  lfs f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831606B0: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 831606B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831606B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831606B8 size=104
    let mut pc: u32 = 0x831606B8;
    'dispatch: loop {
        match pc {
            0x831606B8 => {
    //   block [0x831606B8..0x83160720)
	// 831606B8: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831606BC: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831606C0: 796947E4  rldicr r9, r11, 8, 0x3f
	ctx.r[9].u64 = (ctx.r[11].u64).rotate_left(8) & 0xFFFFFFFFFFFFFFFF;
	// 831606C4: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 831606C8: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 831606CC: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831606D0: 88640004  lbz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831606D4: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 831606D8: 78AA45E4  sldi r10, r5, 8
	ctx.r[10].u64 = ctx.r[5].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 831606DC: 89240006  lbz r9, 6(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 831606E0: 88A40007  lbz r5, 7(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(7 as u32) ) } as u64;
	// 831606E4: 7D444378  or r4, r10, r8
	ctx.r[4].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 831606E8: 788A45E4  sldi r10, r4, 8
	ctx.r[10].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 831606EC: 7D483B78  or r8, r10, r7
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 831606F0: 790745E4  sldi r7, r8, 8
	ctx.r[7].u64 = ctx.r[8].u64.wrapping_shl(8);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 831606F4: 7CE41B78  or r4, r7, r3
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[3].u64;
	// 831606F8: 788345E4  sldi r3, r4, 8
	ctx.r[3].u64 = ctx.r[4].u64.wrapping_shl(8);
	ctx.r[3].u32 = ctx.r[3].u64 as u32;
	// 831606FC: 7C6B5B78  or r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 83160700: 796A45E4  sldi r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64.wrapping_shl(8);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 83160704: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83160708: 792845E4  sldi r8, r9, 8
	ctx.r[8].u64 = ctx.r[9].u64.wrapping_shl(8);
	ctx.r[8].u32 = ctx.r[8].u64 as u32;
	// 8316070C: 7D072B78  or r7, r8, r5
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 83160710: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 83160714: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83160718: D8060008  stfd f0, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 8316071C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160720 size=56
    let mut pc: u32 = 0x83160720;
    'dispatch: loop {
        match pc {
            0x83160720 => {
    //   block [0x83160720..0x83160758)
	// 83160720: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160724: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160728: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8316072C: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160730: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 83160734: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160738: 54A4402E  slwi r4, r5, 8
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8316073C: 7C8B4378  or r11, r4, r8
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 83160740: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160744: 7D4B3B78  or r11, r10, r7
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83160748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316074C: 409A000C  bne cr6, 0x83160758
	if !ctx.cr[6].eq {
		sub_83160758(ctx, base);
		return;
	}
	// 83160750: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83160754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160758 size=16
    let mut pc: u32 = 0x83160758;
    'dispatch: loop {
        match pc {
            0x83160758 => {
    //   block [0x83160758..0x83160768)
	// 83160758: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316075C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83160760: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83160764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83160768 size=100
    let mut pc: u32 = 0x83160768;
    'dispatch: loop {
        match pc {
            0x83160768 => {
    //   block [0x83160768..0x831607CC)
	// 83160768: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316076C: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83160770: 5569403E  rotlwi r9, r11, 8
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83160774: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83160778: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 8316077C: 7D255378  or r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83160780: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83160784: 54A3402E  slwi r3, r5, 8
	ctx.r[3].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83160788: 7C6B4378  or r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[8].u64;
	// 8316078C: 5569402E  slwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83160790: 7D2B3B78  or r11, r9, r7
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 83160794: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160798: 91060008  stw r8, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8316079C: 88E40005  lbz r7, 5(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 831607A0: 88A40006  lbz r5, 6(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 831607A4: 88640007  lbz r3, 7(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(7 as u32) ) } as u64;
	// 831607A8: 89640004  lbz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831607AC: 556A403E  rotlwi r10, r11, 8
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831607B0: 7D493B78  or r9, r10, r7
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 831607B4: 5528402E  slwi r8, r9, 8
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831607B8: 7D072B78  or r7, r8, r5
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 831607BC: 54E5402E  slwi r5, r7, 8
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831607C0: 7CA41B78  or r4, r5, r3
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 831607C4: 9086000C  stw r4, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 831607C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831607D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831607D0 size=136
    let mut pc: u32 = 0x831607D0;
    'dispatch: loop {
        match pc {
            0x831607D0 => {
    //   block [0x831607D0..0x83160858)
	// 831607D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831607D4: 48047999  bl 0x831a816c
	ctx.lr = 0x831607D8;
	sub_831A8130(ctx, base);
	// 831607D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831607DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831607E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831607E4: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 831607E8: 4198001C  blt cr6, 0x83160804
	if ctx.cr[6].lt {
	pc = 0x83160804; continue 'dispatch;
	}
	// 831607EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831607F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831607F4: 388B6478  addi r4, r11, 0x6478
	ctx.r[4].s64 = ctx.r[11].s64 + 25720;
	// 831607F8: 4BFFF321  bl 0x8315fb18
	ctx.lr = 0x831607FC;
	sub_8315FB18(ctx, base);
	// 831607FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83160800: 480479BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83160804: 397E0006  addi r11, r30, 6
	ctx.r[11].s64 = ctx.r[30].s64 + 6;
	// 83160808: 557D1838  slwi r29, r11, 3
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8316080C: 7C9DF82E  lwzx r4, r29, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83160810: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83160814: 409A001C  bne cr6, 0x83160830
	if !ctx.cr[6].eq {
	pc = 0x83160830; continue 'dispatch;
	}
	// 83160818: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316081C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160820: 388B645C  addi r4, r11, 0x645c
	ctx.r[4].s64 = ctx.r[11].s64 + 25692;
	// 83160824: 4BFFF2F5  bl 0x8315fb18
	ctx.lr = 0x83160828;
	sub_8315FB18(ctx, base);
	// 83160828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316082C: 48047990  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83160830: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160834: 4BFFEF35  bl 0x8315f768
	ctx.lr = 0x83160838;
	sub_8315F768(ctx, base);
	// 83160838: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316083C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83160840: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83160844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83160848: 7D1DF92E  stwx r8, r29, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 8316084C: 9149002C  stw r10, 0x2c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83160850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83160854: 48047968  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160858 size=268
    let mut pc: u32 = 0x83160858;
    'dispatch: loop {
        match pc {
            0x83160858 => {
    //   block [0x83160858..0x83160964)
	// 83160858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316085C: 480478FD  bl 0x831a8158
	ctx.lr = 0x83160860;
	sub_831A8130(ctx, base);
	// 83160860: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160864: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83160868: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316086C: 39440006  addi r10, r4, 6
	ctx.r[10].s64 = ctx.r[4].s64 + 6;
	// 83160870: 7D2BEA14  add r9, r11, r29
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83160874: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83160878: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8316087C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83160880: 8309002C  lwz r24, 0x2c(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 83160884: 7F48E82E  lwzx r26, r8, r29
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83160888: 4BEF37B1  bl 0x83054038
	ctx.lr = 0x8316088C;
	sub_83054038(ctx, base);
	// 8316088C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83160890: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83160894: 38FCFFFF  addi r7, r28, -1
	ctx.r[7].s64 = ctx.r[28].s64 + -1;
	// 83160898: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316089C: 54FEF87E  srwi r30, r7, 1
	ctx.r[30].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 831608A0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 831608A4: 57C6103A  slwi r6, r30, 2
	ctx.r[6].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831608A8: 7C86D02E  lwzx r4, r6, r26
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 831608AC: 4BFFFB9D  bl 0x83160448
	ctx.lr = 0x831608B0;
	sub_83160448(ctx, base);
	// 831608B0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831608B4: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831608B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831608BC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831608C0: 419A0014  beq cr6, 0x831608d4
	if ctx.cr[6].eq {
	pc = 0x831608D4; continue 'dispatch;
	}
	// 831608C4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831608C8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831608CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831608D0: 419AFFE0  beq cr6, 0x831608b0
	if ctx.cr[6].eq {
	pc = 0x831608B0; continue 'dispatch;
	}
	// 831608D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831608D8: 419A0070  beq cr6, 0x83160948
	if ctx.cr[6].eq {
	pc = 0x83160948; continue 'dispatch;
	}
	// 831608DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831608E0: 4099000C  ble cr6, 0x831608ec
	if !ctx.cr[6].gt {
	pc = 0x831608EC; continue 'dispatch;
	}
	// 831608E4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 831608E8: 48000008  b 0x831608f0
	pc = 0x831608F0; continue 'dispatch;
	// 831608EC: 3B7E0001  addi r27, r30, 1
	ctx.r[27].s64 = ctx.r[30].s64 + 1;
	// 831608F0: 7F1BE040  cmplw cr6, r27, r28
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831608F4: 419A0064  beq cr6, 0x83160958
	if ctx.cr[6].eq {
	pc = 0x83160958; continue 'dispatch;
	}
	// 831608F8: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 831608FC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83160900: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83160904: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83160908: 557EF87E  srwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8316090C: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 83160910: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160914: 7C8AD02E  lwzx r4, r10, r26
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83160918: 4BFFFB31  bl 0x83160448
	ctx.lr = 0x8316091C;
	sub_83160448(ctx, base);
	// 8316091C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160920: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83160928: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8316092C: 419A0014  beq cr6, 0x83160940
	if ctx.cr[6].eq {
	pc = 0x83160940; continue 'dispatch;
	}
	// 83160930: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83160934: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83160938: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316093C: 419AFFE0  beq cr6, 0x8316091c
	if ctx.cr[6].eq {
	pc = 0x8316091C; continue 'dispatch;
	}
	// 83160940: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83160944: 409AFF9C  bne cr6, 0x831608e0
	if !ctx.cr[6].eq {
	pc = 0x831608E0; continue 'dispatch;
	}
	// 83160948: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316094C: 7C6BD02E  lwzx r3, r11, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83160950: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160954: 48047854  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83160958: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316095C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160960: 48047848  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160968 size=108
    let mut pc: u32 = 0x83160968;
    'dispatch: loop {
        match pc {
            0x83160968 => {
    //   block [0x83160968..0x831609D4)
	// 83160968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316096C: 480477FD  bl 0x831a8168
	ctx.lr = 0x83160970;
	sub_831A8130(ctx, base);
	// 83160970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160974: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83160978: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316097C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160980: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83160984: 419A000C  beq cr6, 0x83160990
	if ctx.cr[6].eq {
	pc = 0x83160990; continue 'dispatch;
	}
	// 83160988: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316098C: 4BFFEDDD  bl 0x8315f768
	ctx.lr = 0x83160990;
	sub_8315F768(ctx, base);
	// 83160990: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83160994: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 83160998: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316099C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831609A0: 419A0010  beq cr6, 0x831609b0
	if ctx.cr[6].eq {
	pc = 0x831609B0; continue 'dispatch;
	}
	// 831609A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831609A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831609AC: 4BFFFE25  bl 0x831607d0
	ctx.lr = 0x831609B0;
	sub_831607D0(ctx, base);
	// 831609B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831609B4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 831609B8: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 831609BC: 4198FFDC  blt cr6, 0x83160998
	if ctx.cr[6].lt {
	pc = 0x83160998; continue 'dispatch;
	}
	// 831609C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831609C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831609C8: 4BFFEDA1  bl 0x8315f768
	ctx.lr = 0x831609CC;
	sub_8315F768(ctx, base);
	// 831609CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831609D0: 480477E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831609D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831609D8 size=140
    let mut pc: u32 = 0x831609D8;
    'dispatch: loop {
        match pc {
            0x831609D8 => {
    //   block [0x831609D8..0x83160A64)
	// 831609D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831609DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831609E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831609E4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 831609E8: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831609EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831609F0: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 831609F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831609F8: 419A0044  beq cr6, 0x83160a3c
	if ctx.cr[6].eq {
	pc = 0x83160A3C; continue 'dispatch;
	}
	// 831609FC: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83160A00: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83160A04: 40980038  bge cr6, 0x83160a3c
	if !ctx.cr[6].lt {
	pc = 0x83160A3C; continue 'dispatch;
	}
	// 83160A08: A1430022  lhz r10, 0x22(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 83160A0C: A10B000A  lhz r8, 0xa(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83160A10: 7D2A21D6  mullw r9, r10, r4
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 83160A14: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160A18: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160A1C: 7D694214  add r11, r9, r8
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83160A20: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160A24: 4BFFFB55  bl 0x83160578
	ctx.lr = 0x83160A28;
	sub_83160578(ctx, base);
	// 83160A28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83160A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83160A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83160A38: 4E800020  blr
	return;
	// 83160A3C: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 83160A40: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 83160A44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160A48: F9460000  std r10, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83160A4C: E90B0018  ld r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 83160A50: F9060008  std r8, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 83160A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83160A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83160A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83160A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160A68 size=208
    let mut pc: u32 = 0x83160A68;
    'dispatch: loop {
        match pc {
            0x83160A68 => {
    //   block [0x83160A68..0x83160B38)
	// 83160A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83160A6C: 480476F1  bl 0x831a815c
	ctx.lr = 0x83160A70;
	sub_831A8130(ctx, base);
	// 83160A70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160A74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83160A78: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83160A7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83160A80: 3BDD0001  addi r30, r29, 1
	ctx.r[30].s64 = ctx.r[29].s64 + 1;
	// 83160A84: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83160A88: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160A8C: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83160A90: 5569DFFE  rlwinm r9, r11, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83160A94: 556806F6  rlwinm r8, r11, 0, 0x1b, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83160A98: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83160A9C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 83160AA0: 993F0008  stb r9, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 83160AA4: 557B06B4  rlwinm r27, r11, 0, 0x1a, 0x1a
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83160AA8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83160AAC: 419A0020  beq cr6, 0x83160acc
	if ctx.cr[6].eq {
	pc = 0x83160ACC; continue 'dispatch;
	}
	// 83160AB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83160AB4: 4BFFFA2D  bl 0x831604e0
	ctx.lr = 0x83160AB8;
	sub_831604E0(ctx, base);
	// 83160AB8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83160ABC: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 83160AC0: 48003239  bl 0x83163cf8
	ctx.lr = 0x83160AC4;
	sub_83163CF8(ctx, base);
	// 83160AC4: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83160AC8: 48000008  b 0x83160ad0
	pc = 0x83160AD0; continue 'dispatch;
	// 83160ACC: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83160AD0: 576B0634  rlwinm r11, r27, 0, 0x18, 0x1a
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 83160AD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83160AD8: 419A0028  beq cr6, 0x83160b00
	if ctx.cr[6].eq {
	pc = 0x83160B00; continue 'dispatch;
	}
	// 83160ADC: 38DF0010  addi r6, r31, 0x10
	ctx.r[6].s64 = ctx.r[31].s64 + 16;
	// 83160AE0: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160AE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83160AE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83160AEC: 4BFFFA8D  bl 0x83160578
	ctx.lr = 0x83160AF0;
	sub_83160578(ctx, base);
	// 83160AF0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160AF4: 48003205  bl 0x83163cf8
	ctx.lr = 0x83160AF8;
	sub_83163CF8(ctx, base);
	// 83160AF8: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83160AFC: 48000010  b 0x83160b0c
	pc = 0x83160B0C; continue 'dispatch;
	// 83160B00: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 83160B04: FB5F0018  std r26, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u64 ) };
	// 83160B08: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 83160B0C: 572B0672  rlwinm r11, r25, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	// 83160B10: 7C7DF050  subf r3, r29, r30
	ctx.r[3].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 83160B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83160B18: 419A0014  beq cr6, 0x83160b2c
	if ctx.cr[6].eq {
	pc = 0x83160B2C; continue 'dispatch;
	}
	// 83160B1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83160B20: 997F0009  stb r11, 9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 83160B24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83160B28: 48047684  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83160B2C: 9B5F0009  stb r26, 9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(9 as u32), ctx.r[26].u8 ) };
	// 83160B30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83160B34: 48047678  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160B38 size=272
    let mut pc: u32 = 0x83160B38;
    'dispatch: loop {
        match pc {
            0x83160B38 => {
    //   block [0x83160B38..0x83160C48)
	// 83160B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83160B3C: 48047625  bl 0x831a8160
	ctx.lr = 0x83160B40;
	sub_831A8130(ctx, base);
	// 83160B40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83160B48: 548A2834  slwi r10, r4, 5
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160B4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83160B50: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83160B54: A35F0022  lhz r26, 0x22(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 83160B58: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160B5C: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160B60: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 83160B64: A10B000A  lhz r8, 0xa(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83160B68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160B6C: 419A00CC  beq cr6, 0x83160c38
	if ctx.cr[6].eq {
	pc = 0x83160C38; continue 'dispatch;
	}
	// 83160B70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160B74: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83160B78: 409A00C0  bne cr6, 0x83160c38
	if !ctx.cr[6].eq {
	pc = 0x83160C38; continue 'dispatch;
	}
	// 83160B7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83160B80: 395F002C  addi r10, r31, 0x2c
	ctx.r[10].s64 = ctx.r[31].s64 + 44;
	// 83160B84: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160B88: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83160B8C: 419A008C  beq cr6, 0x83160c18
	if ctx.cr[6].eq {
	pc = 0x83160C18; continue 'dispatch;
	}
	// 83160B90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83160B94: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83160B98: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83160B9C: 4198FFE8  blt cr6, 0x83160b84
	if ctx.cr[6].lt {
	pc = 0x83160B84; continue 'dispatch;
	}
	// 83160BA0: 839F0024  lwz r28, 0x24(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83160BA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83160BA8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83160BAC: 419A0090  beq cr6, 0x83160c3c
	if ctx.cr[6].eq {
	pc = 0x83160C3C; continue 'dispatch;
	}
	// 83160BB0: 7FC94214  add r30, r9, r8
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83160BB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83160BB8: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 83160BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83160BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83160BC4: 4BFFF9B5  bl 0x83160578
	ctx.lr = 0x83160BC8;
	sub_83160578(ctx, base);
	// 83160BC8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160BCC: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 83160BD0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160BD4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160BD8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83160BDC: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83160BE0: 419A0014  beq cr6, 0x83160bf4
	if ctx.cr[6].eq {
	pc = 0x83160BF4; continue 'dispatch;
	}
	// 83160BE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83160BE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83160BEC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83160BF0: 419AFFE0  beq cr6, 0x83160bd0
	if ctx.cr[6].eq {
	pc = 0x83160BD0; continue 'dispatch;
	}
	// 83160BF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83160BF8: 419A0044  beq cr6, 0x83160c3c
	if ctx.cr[6].eq {
	pc = 0x83160C3C; continue 'dispatch;
	}
	// 83160BFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83160C00: 7FDED214  add r30, r30, r26
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[26].u64;
	// 83160C04: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83160C08: 4198FFAC  blt cr6, 0x83160bb4
	if ctx.cr[6].lt {
	pc = 0x83160BB4; continue 'dispatch;
	}
	// 83160C0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83160C10: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160C14: 4804759C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83160C18: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83160C1C: 4098FF84  bge cr6, 0x83160ba0
	if !ctx.cr[6].lt {
	pc = 0x83160BA0; continue 'dispatch;
	}
	// 83160C20: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83160C24: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83160C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83160C2C: 4BFFFC2D  bl 0x83160858
	ctx.lr = 0x83160C30;
	sub_83160858(ctx, base);
	// 83160C30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160C34: 4804757C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83160C38: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 83160C3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83160C40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160C44: 4804756C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160C48 size=160
    let mut pc: u32 = 0x83160C48;
    'dispatch: loop {
        match pc {
            0x83160C48 => {
    //   block [0x83160C48..0x83160CE8)
	// 83160C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83160C4C: 48047515  bl 0x831a8160
	ctx.lr = 0x83160C50;
	sub_831A8130(ctx, base);
	// 83160C50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83160C58: 548A2834  slwi r10, r4, 5
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83160C5C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83160C60: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83160C64: A37F0022  lhz r27, 0x22(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 83160C68: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83160C6C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83160C70: 890B0009  lbz r8, 9(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 83160C74: A12B000A  lhz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83160C78: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83160C7C: 419A0060  beq cr6, 0x83160cdc
	if ctx.cr[6].eq {
	pc = 0x83160CDC; continue 'dispatch;
	}
	// 83160C80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160C84: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83160C88: 409A0054  bne cr6, 0x83160cdc
	if !ctx.cr[6].eq {
	pc = 0x83160CDC; continue 'dispatch;
	}
	// 83160C8C: 839F0024  lwz r28, 0x24(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83160C90: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83160C94: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83160C98: 419A0038  beq cr6, 0x83160cd0
	if ctx.cr[6].eq {
	pc = 0x83160CD0; continue 'dispatch;
	}
	// 83160C9C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83160CA0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83160CA4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83160CA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83160CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83160CB0: 4BFFF8C9  bl 0x83160578
	ctx.lr = 0x83160CB4;
	sub_83160578(ctx, base);
	// 83160CB4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160CB8: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83160CBC: 419A0014  beq cr6, 0x83160cd0
	if ctx.cr[6].eq {
	pc = 0x83160CD0; continue 'dispatch;
	}
	// 83160CC0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83160CC4: 7FDEDA14  add r30, r30, r27
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 83160CC8: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83160CCC: 4198FFD4  blt cr6, 0x83160ca0
	if ctx.cr[6].lt {
	pc = 0x83160CA0; continue 'dispatch;
	}
	// 83160CD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83160CD4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160CD8: 480474D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83160CDC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83160CE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160CE4: 480474CC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83160CE8 size=524
    let mut pc: u32 = 0x83160CE8;
    'dispatch: loop {
        match pc {
            0x83160CE8 => {
    //   block [0x83160CE8..0x83160D84)
	// 83160CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83160CEC: 4804747D  bl 0x831a8168
	ctx.lr = 0x83160CF0;
	sub_831A8130(ctx, base);
	// 83160CF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160CF4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83160CF8: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160CFC: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160D00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83160D04: 394B8308  addi r10, r11, -0x7cf8
	ctx.r[10].s64 = ctx.r[11].s64 + -31992;
	// 83160D08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83160D0C: 83CB8308  lwz r30, -0x7cf8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31992 as u32) ) } as u64;
	// 83160D10: 83EA0004  lwz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83160D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83160D18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83160D1C: 4BFFFCBD  bl 0x831609d8
	ctx.lr = 0x83160D20;
	sub_831609D8(ctx, base);
	// 83160D20: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83160D24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83160D28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83160D2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83160D30: 4BFFFCA9  bl 0x831609d8
	ctx.lr = 0x83160D34;
	sub_831609D8(ctx, base);
	// 83160D34: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83160D38: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 83160D3C: 41990140  bgt cr6, 0x83160e7c
	if ctx.cr[6].gt {
	pc = 0x83160E7C; continue 'dispatch;
	}
	// 83160D40: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 83160D44: 398C0D58  addi r12, r12, 0xd58
	ctx.r[12].s64 = ctx.r[12].s64 + 3416;
	// 83160D48: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83160D4C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83160D50: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83160D54: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83160D84; continue 'dispatch;
		},
		1 => {
	pc = 0x83160DA4; continue 'dispatch;
		},
		2 => {
	pc = 0x83160DC8; continue 'dispatch;
		},
		3 => {
	pc = 0x83160DE4; continue 'dispatch;
		},
		4 => {
	pc = 0x83160E08; continue 'dispatch;
		},
		5 => {
	pc = 0x83160E28; continue 'dispatch;
		},
		6 => {
	pc = 0x83160E44; continue 'dispatch;
		},
		7 => {
	pc = 0x83160E44; continue 'dispatch;
		},
		8 => {
	pc = 0x83160E60; continue 'dispatch;
		},
		9 => {
	pc = 0x83160E88; continue 'dispatch;
		},
		10 => {
	pc = 0x83160EA0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83160D58: 83160D84  lwz r24, 0xd84(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3460 as u32) ) } as u64;
	// 83160D5C: 83160DA4  lwz r24, 0xda4(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3492 as u32) ) } as u64;
	// 83160D60: 83160DC8  lwz r24, 0xdc8(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3528 as u32) ) } as u64;
	// 83160D64: 83160DE4  lwz r24, 0xde4(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3556 as u32) ) } as u64;
	// 83160D68: 83160E08  lwz r24, 0xe08(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3592 as u32) ) } as u64;
	// 83160D6C: 83160E28  lwz r24, 0xe28(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3624 as u32) ) } as u64;
	// 83160D70: 83160E44  lwz r24, 0xe44(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3652 as u32) ) } as u64;
	// 83160D74: 83160E44  lwz r24, 0xe44(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3652 as u32) ) } as u64;
	// 83160D78: 83160E60  lwz r24, 0xe60(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3680 as u32) ) } as u64;
	// 83160D7C: 83160E88  lwz r24, 0xe88(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3720 as u32) ) } as u64;
	// 83160D80: 83160EA0  lwz r24, 0xea0(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3744 as u32) ) } as u64;
            }
            0x83160D84 => {
    //   block [0x83160D84..0x83160DA4)
	// 83160D84: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160D88: 89410058  lbz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160D8C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83160D90: 4198002C  blt cr6, 0x83160dbc
	if ctx.cr[6].lt {
	pc = 0x83160DBC; continue 'dispatch;
	}
	// 83160D94: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83160D98: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83160D9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160DA0: 48047418  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160DA4 => {
    //   block [0x83160DA4..0x83160DC8)
	// 83160DA4: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160DA8: 89410058  lbz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160DAC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83160DB0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83160DB4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83160DB8: 409800BC  bge cr6, 0x83160e74
	if !ctx.cr[6].lt {
	pc = 0x83160E74; continue 'dispatch;
	}
	// 83160DBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160DC0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160DC4: 480473F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160DC8 => {
    //   block [0x83160DC8..0x83160DE4)
	// 83160DC8: A1610068  lhz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160DCC: A1410058  lhz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160DD0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83160DD4: 4098FFC0  bge cr6, 0x83160d94
	if !ctx.cr[6].lt {
	pc = 0x83160D94; continue 'dispatch;
	}
	// 83160DD8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160DDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160DE0: 480473D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160DE4 => {
    //   block [0x83160DE4..0x83160E08)
	// 83160DE4: A1610068  lhz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160DE8: A1410058  lhz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160DEC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83160DF0: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83160DF4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83160DF8: 4098007C  bge cr6, 0x83160e74
	if !ctx.cr[6].lt {
	pc = 0x83160E74; continue 'dispatch;
	}
	// 83160DFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160E00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160E04: 480473B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160E08 => {
    //   block [0x83160E08..0x83160E28)
	// 83160E08: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160E0C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160E10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83160E14: 4198FFA8  blt cr6, 0x83160dbc
	if ctx.cr[6].lt {
	pc = 0x83160DBC; continue 'dispatch;
	}
	// 83160E18: 7D6B5010  subfc r11, r11, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83160E1C: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83160E20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160E24: 48047394  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160E28 => {
    //   block [0x83160E28..0x83160E44)
	// 83160E28: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83160E2C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83160E30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83160E34: 40980040  bge cr6, 0x83160e74
	if !ctx.cr[6].lt {
	pc = 0x83160E74; continue 'dispatch;
	}
	// 83160E38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160E3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160E40: 48047378  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160E44 => {
    //   block [0x83160E44..0x83160E60)
	// 83160E44: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83160E48: E9410068  ld r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83160E4C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83160E50: 40980024  bge cr6, 0x83160e74
	if !ctx.cr[6].lt {
	pc = 0x83160E74; continue 'dispatch;
	}
	// 83160E54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83160E58: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160E5C: 4804735C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160E60 => {
    //   block [0x83160E60..0x83160E88)
	// 83160E60: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83160E64: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83160E68: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83160E6C: 4198FF50  blt cr6, 0x83160dbc
	if ctx.cr[6].lt {
	pc = 0x83160DBC; continue 'dispatch;
	}
	// 83160E70: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83160E74: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83160E78: 41990008  bgt cr6, 0x83160e80
	if ctx.cr[6].gt {
	pc = 0x83160E80; continue 'dispatch;
	}
	// 83160E7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160E80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160E84: 48047334  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83160E88 => {
    //   block [0x83160E88..0x83160EA0)
	// 83160E88: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83160E8C: C9A10068  lfd f13, 0x68(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83160E90: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83160E94: 4198FF28  blt cr6, 0x83160dbc
	if ctx.cr[6].lt {
	pc = 0x83160DBC; continue 'dispatch;
	}
	// 83160E98: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83160E9C: 4BFFFFD8  b 0x83160e74
	pc = 0x83160E74; continue 'dispatch;
            }
            0x83160EA0 => {
    //   block [0x83160EA0..0x83160EF4)
	// 83160EA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83160EA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83160EA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83160EAC: 4BFFF59D  bl 0x83160448
	ctx.lr = 0x83160EB0;
	sub_83160448(ctx, base);
	// 83160EB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83160EB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83160EB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83160EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83160EC0: 4BFFF589  bl 0x83160448
	ctx.lr = 0x83160EC4;
	sub_83160448(ctx, base);
	// 83160EC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83160EC8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160ECC: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160ED0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83160ED4: 7C695050  subf r3, r9, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83160ED8: 419AFFA8  beq cr6, 0x83160e80
	if ctx.cr[6].eq {
	pc = 0x83160E80; continue 'dispatch;
	}
	// 83160EDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83160EE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83160EE4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83160EE8: 419AFFE0  beq cr6, 0x83160ec8
	if ctx.cr[6].eq {
	pc = 0x83160EC8; continue 'dispatch;
	}
	// 83160EEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83160EF0: 480472C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83160EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83160EF8 size=328
    let mut pc: u32 = 0x83160EF8;
    'dispatch: loop {
        match pc {
            0x83160EF8 => {
    //   block [0x83160EF8..0x83161040)
	// 83160EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83160EFC: 48047269  bl 0x831a8164
	ctx.lr = 0x83160F00;
	sub_831A8130(ctx, base);
	// 83160F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83160F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83160F08: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83160F0C: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 83160F10: 41980020  blt cr6, 0x83160f30
	if ctx.cr[6].lt {
	pc = 0x83160F30; continue 'dispatch;
	}
	// 83160F14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83160F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F1C: 388B64EC  addi r4, r11, 0x64ec
	ctx.r[4].s64 = ctx.r[11].s64 + 25836;
	// 83160F20: 4BFFEBF9  bl 0x8315fb18
	ctx.lr = 0x83160F24;
	sub_8315FB18(ctx, base);
	// 83160F24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83160F2C: 48047288  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83160F30: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83160F34: 7F8BFA14  add r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83160F38: 817C002C  lwz r11, 0x2c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 83160F3C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83160F40: 419A0020  beq cr6, 0x83160f60
	if ctx.cr[6].eq {
	pc = 0x83160F60; continue 'dispatch;
	}
	// 83160F44: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83160F48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F4C: 388B64CC  addi r4, r11, 0x64cc
	ctx.r[4].s64 = ctx.r[11].s64 + 25804;
	// 83160F50: 4BFFEBC9  bl 0x8315fb18
	ctx.lr = 0x83160F54;
	sub_8315FB18(ctx, base);
	// 83160F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83160F5C: 48047258  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83160F60: 39640006  addi r11, r4, 6
	ctx.r[11].s64 = ctx.r[4].s64 + 6;
	// 83160F64: 557D1838  slwi r29, r11, 3
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83160F68: 7D5DF82E  lwzx r10, r29, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83160F6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83160F70: 419A0020  beq cr6, 0x83160f90
	if ctx.cr[6].eq {
	pc = 0x83160F90; continue 'dispatch;
	}
	// 83160F74: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83160F78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F7C: 388B64B0  addi r4, r11, 0x64b0
	ctx.r[4].s64 = ctx.r[11].s64 + 25776;
	// 83160F80: 4BFFEB99  bl 0x8315fb18
	ctx.lr = 0x83160F84;
	sub_8315FB18(ctx, base);
	// 83160F84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83160F8C: 48047228  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83160F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83160F94: 4BEF30A5  bl 0x83054038
	ctx.lr = 0x83160F98;
	sub_83054038(ctx, base);
	// 83160F98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83160F9C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83160FA0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83160FA4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83160FA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83160FAC: 38AB64A0  addi r5, r11, 0x64a0
	ctx.r[5].s64 = ctx.r[11].s64 + 25760;
	// 83160FB0: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83160FB4: 4BFFE74D  bl 0x8315f700
	ctx.lr = 0x83160FB8;
	sub_8315F700(ctx, base);
	// 83160FB8: 7C7DF92E  stwx r3, r29, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[3].u32) };
	// 83160FBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83160FC0: 409A0020  bne cr6, 0x83160fe0
	if !ctx.cr[6].eq {
	pc = 0x83160FE0; continue 'dispatch;
	}
	// 83160FC4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83160FC8: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83160FCC: 388B6494  addi r4, r11, 0x6494
	ctx.r[4].s64 = ctx.r[11].s64 + 25748;
	// 83160FD0: 4BFFEB71  bl 0x8315fb40
	ctx.lr = 0x83160FD4;
	sub_8315FB40(ctx, base);
	// 83160FD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83160FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83160FDC: 480471D8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83160FE0: 937C002C  stw r27, 0x2c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 83160FE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83160FE8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83160FEC: 419A0020  beq cr6, 0x8316100c
	if ctx.cr[6].eq {
	pc = 0x8316100C; continue 'dispatch;
	}
	// 83160FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83160FF4: 7D3DF82E  lwzx r9, r29, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83160FF8: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 83160FFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83161000: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83161004: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83161008: 4198FFEC  blt cr6, 0x83160ff4
	if ctx.cr[6].lt {
	pc = 0x83160FF4; continue 'dispatch;
	}
	// 8316100C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83161010: 7C7DF82E  lwzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83161014: 3D408316  lis r10, -0x7cea
	ctx.r[10].s64 = -2095710208;
	// 83161018: 392B830C  addi r9, r11, -0x7cf4
	ctx.r[9].s64 = ctx.r[11].s64 + -31988;
	// 8316101C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83161020: 38CA0CE8  addi r6, r10, 0xce8
	ctx.r[6].s64 = ctx.r[10].s64 + 3304;
	// 83161024: 936B830C  stw r27, -0x7cf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-31988 as u32), ctx.r[27].u32 ) };
	// 83161028: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316102C: 93E9FFFC  stw r31, -4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-4 as u32), ctx.r[31].u32 ) };
	// 83161030: 4804C3E1  bl 0x831ad410
	ctx.lr = 0x83161034;
	sub_831AD410(ctx, base);
	// 83161034: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83161038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316103C: 48047178  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161040 size=584
    let mut pc: u32 = 0x83161040;
    'dispatch: loop {
        match pc {
            0x83161040 => {
    //   block [0x83161040..0x83161288)
	// 83161040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161044: 48047121  bl 0x831a8164
	ctx.lr = 0x83161048;
	sub_831A8130(ctx, base);
	// 83161048: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316104C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161050: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83161054: 392B6400  addi r9, r11, 0x6400
	ctx.r[9].s64 = ctx.r[11].s64 + 25600;
	// 83161058: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8316105C: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 83161060: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83161064: 912A8304  stw r9, -0x7cfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31996 as u32), ctx.r[9].u32 ) };
	// 83161068: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316106C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83161070: 38A86568  addi r5, r8, 0x6568
	ctx.r[5].s64 = ctx.r[8].s64 + 25960;
	// 83161074: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83161078: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 8316107C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83161080: 4BFFE681  bl 0x8315f700
	ctx.lr = 0x83161084;
	sub_8315F700(ctx, base);
	// 83161084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161088: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316108C: 409A0020  bne cr6, 0x831610ac
	if !ctx.cr[6].eq {
	pc = 0x831610AC; continue 'dispatch;
	}
	// 83161090: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161094: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83161098: 388B6494  addi r4, r11, 0x6494
	ctx.r[4].s64 = ctx.r[11].s64 + 25748;
	// 8316109C: 4BFFEAA5  bl 0x8315fb40
	ctx.lr = 0x831610A0;
	sub_8315FB40(ctx, base);
	// 831610A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831610A4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831610A8: 4804710C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831610AC: 38A0004C  li r5, 0x4c
	ctx.r[5].s64 = 76;
	// 831610B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831610B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831610B8: 48047129  bl 0x831a81e0
	ctx.lr = 0x831610BC;
	sub_831A81E0(ctx, base);
	// 831610BC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831610C0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 831610C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831610C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831610CC: 48002CA5  bl 0x83163d70
	ctx.lr = 0x831610D0;
	sub_83163D70(ctx, base);
	// 831610D0: 3D604055  lis r11, 0x4055
	ctx.r[11].s64 = 1079312384;
	// 831610D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831610D8: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 831610DC: 61695446  ori r9, r11, 0x5446
	ctx.r[9].u64 = ctx.r[11].u64 | 21574;
	// 831610E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831610E4: 419A0028  beq cr6, 0x8316110c
	if ctx.cr[6].eq {
	pc = 0x8316110C; continue 'dispatch;
	}
	// 831610E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831610EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831610F0: 388B6534  addi r4, r11, 0x6534
	ctx.r[4].s64 = ctx.r[11].s64 + 25908;
	// 831610F4: 4BFFEA25  bl 0x8315fb18
	ctx.lr = 0x831610F8;
	sub_8315FB18(ctx, base);
	// 831610F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831610FC: 4BFFF86D  bl 0x83160968
	ctx.lr = 0x83161100;
	sub_83160968(ctx, base);
	// 83161100: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161104: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83161108: 480470AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8316110C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83161110: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 83161114: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83161118: 40990028  ble cr6, 0x83161140
	if !ctx.cr[6].gt {
	pc = 0x83161140; continue 'dispatch;
	}
	// 8316111C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161124: 388B651C  addi r4, r11, 0x651c
	ctx.r[4].s64 = ctx.r[11].s64 + 25884;
	// 83161128: 4BFFE9F1  bl 0x8315fb18
	ctx.lr = 0x8316112C;
	sub_8315FB18(ctx, base);
	// 8316112C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161130: 4BFFF839  bl 0x83160968
	ctx.lr = 0x83161134;
	sub_83160968(ctx, base);
	// 83161134: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161138: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8316113C: 48047078  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83161140: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83161144: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83161148: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316114C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161150: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83161154: 48002C85  bl 0x83163dd8
	ctx.lr = 0x83161158;
	sub_83163DD8(ctx, base);
	// 83161158: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8316115C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83161160: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83161164: 7F83F214  add r28, r3, r30
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83161168: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316116C: 38A96514  addi r5, r9, 0x6514
	ctx.r[5].s64 = ctx.r[9].s64 + 25876;
	// 83161170: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 83161174: 5509003E  slwi r9, r8, 0
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83161178: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316117C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83161180: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 83161184: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83161188: A1410062  lhz r10, 0x62(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 8316118C: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83161190: 911F0018  stw r8, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 83161194: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83161198: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316119C: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831611A0: 909F001C  stw r4, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 831611A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831611A8: A1610070  lhz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 831611AC: 556A043E  clrlwi r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831611B0: B17F0020  sth r11, 0x20(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 831611B4: 5544283E  rotlwi r4, r10, 5
	ctx.r[4].u64 = ((ctx.r[10].u32).rotate_left(5)) as u64;
	// 831611B8: A1210072  lhz r9, 0x72(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(114 as u32) ) } as u64;
	// 831611BC: B13F0022  sth r9, 0x22(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(34 as u32), ctx.r[9].u16 ) };
	// 831611C0: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 831611C4: 911F0024  stw r8, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 831611C8: 4BFFE539  bl 0x8315f700
	ctx.lr = 0x831611CC;
	sub_8315F700(ctx, base);
	// 831611CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831611D0: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 831611D4: 409A002C  bne cr6, 0x83161200
	if !ctx.cr[6].eq {
	pc = 0x83161200; continue 'dispatch;
	}
	// 831611D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831611DC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 831611E0: 388B6508  addi r4, r11, 0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + 25864;
	// 831611E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831611E8: 4BFFE959  bl 0x8315fb40
	ctx.lr = 0x831611EC;
	sub_8315FB40(ctx, base);
	// 831611EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831611F0: 4BFFF779  bl 0x83160968
	ctx.lr = 0x831611F4;
	sub_83160968(ctx, base);
	// 831611F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831611F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831611FC: 48046FB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83161200: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83161204: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83161208: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8316120C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161210: 419A004C  beq cr6, 0x8316125c
	if ctx.cr[6].eq {
	pc = 0x8316125C; continue 'dispatch;
	}
	// 83161214: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 83161218: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316121C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83161220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161224: 7CBE5A14  add r5, r30, r11
	ctx.r[5].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83161228: 4BFFF841  bl 0x83160a68
	ctx.lr = 0x8316122C;
	sub_83160A68(ctx, base);
	// 8316122C: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 83161230: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83161234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161238: 4BFFF2F1  bl 0x83160528
	ctx.lr = 0x8316123C;
	sub_83160528(ctx, base);
	// 8316123C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83161240: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83161244: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83161248: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8316124C: B069000A  sth r3, 0xa(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(10 as u32), ctx.r[3].u16 ) };
	// 83161250: A11F0020  lhz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83161254: 7F1D4040  cmplw cr6, r29, r8
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83161258: 4198FFC0  blt cr6, 0x83161218
	if ctx.cr[6].lt {
	pc = 0x83161218; continue 'dispatch;
	}
	// 8316125C: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 83161260: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83161264: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 83161268: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8316126C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83161270: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83161274: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83161278: 4082FFF0  bne 0x83161268
	if !ctx.cr[0].eq {
	pc = 0x83161268; continue 'dispatch;
	}
	// 8316127C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161280: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83161284: 48046F30  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161288 size=16
    let mut pc: u32 = 0x83161288;
    'dispatch: loop {
        match pc {
            0x83161288 => {
    //   block [0x83161288..0x83161298)
	// 83161288: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8316128C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83161290: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83161294: 4BFFFDAC  b 0x83161040
	sub_83161040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161298 size=124
    let mut pc: u32 = 0x83161298;
    'dispatch: loop {
        match pc {
            0x83161298 => {
    //   block [0x83161298..0x83161314)
	// 83161298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831612A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831612A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831612A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831612AC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 831612B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831612B4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831612B8: 4BFFE2D1  bl 0x8315f588
	ctx.lr = 0x831612BC;
	sub_8315F588(ctx, base);
	// 831612BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831612C0: 409A001C  bne cr6, 0x831612dc
	if !ctx.cr[6].eq {
	pc = 0x831612DC; continue 'dispatch;
	}
	// 831612C4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831612C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831612CC: 388B6570  addi r4, r11, 0x6570
	ctx.r[4].s64 = ctx.r[11].s64 + 25968;
	// 831612D0: 4BFFE849  bl 0x8315fb18
	ctx.lr = 0x831612D4;
	sub_8315FB18(ctx, base);
	// 831612D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831612D8: 48000024  b 0x831612fc
	pc = 0x831612FC; continue 'dispatch;
	// 831612DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831612E0: 3963002C  addi r11, r3, 0x2c
	ctx.r[11].s64 = ctx.r[3].s64 + 44;
	// 831612E4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831612E8: 409A0008  bne cr6, 0x831612f0
	if !ctx.cr[6].eq {
	pc = 0x831612F0; continue 'dispatch;
	}
	// 831612EC: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 831612F0: 57CA2036  slwi r10, r30, 4
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831612F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831612F8: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831612FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161308: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316130C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83161310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161318 size=152
    let mut pc: u32 = 0x83161318;
    'dispatch: loop {
        match pc {
            0x83161318 => {
    //   block [0x83161318..0x831613B0)
	// 83161318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316131C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83161324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83161328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316132C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83161330: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83161334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161338: 409A001C  bne cr6, 0x83161354
	if !ctx.cr[6].eq {
	pc = 0x83161354; continue 'dispatch;
	}
	// 8316133C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161344: 388B658C  addi r4, r11, 0x658c
	ctx.r[4].s64 = ctx.r[11].s64 + 25996;
	// 83161348: 4BFFE7D1  bl 0x8315fb18
	ctx.lr = 0x8316134C;
	sub_8315FB18(ctx, base);
	// 8316134C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161350: 48000048  b 0x83161398
	pc = 0x83161398; continue 'dispatch;
	// 83161354: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316135C: 419A0008  beq cr6, 0x83161364
	if ctx.cr[6].eq {
	pc = 0x83161364; continue 'dispatch;
	}
	// 83161360: 4BFFEA69  bl 0x8315fdc8
	ctx.lr = 0x83161364;
	sub_8315FDC8(ctx, base);
	// 83161364: 83FE0028  lwz r31, 0x28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83161368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316136C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161370: 915E0028  stw r10, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 83161374: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83161378: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316137C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83161380: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83161384: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316138C: 419A0008  beq cr6, 0x83161394
	if ctx.cr[6].eq {
	pc = 0x83161394; continue 'dispatch;
	}
	// 83161390: 4BFFEA51  bl 0x8315fde0
	ctx.lr = 0x83161394;
	sub_8315FDE0(ctx, base);
	// 83161394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161398: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316139C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831613A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831613A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831613A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831613AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831613B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831613B0 size=116
    let mut pc: u32 = 0x831613B0;
    'dispatch: loop {
        match pc {
            0x831613B0 => {
    //   block [0x831613B0..0x83161424)
	// 831613B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831613B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831613B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831613BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831613C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831613C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831613C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831613CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831613D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831613D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831613D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831613DC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831613E0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831613E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831613E8: 419A0008  beq cr6, 0x831613f0
	if ctx.cr[6].eq {
	pc = 0x831613F0; continue 'dispatch;
	}
	// 831613EC: 4BFFE9DD  bl 0x8315fdc8
	ctx.lr = 0x831613F0;
	sub_8315FDC8(ctx, base);
	// 831613F0: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 831613F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831613F8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831613FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161400: 93FE0028  stw r31, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 83161404: 419A0008  beq cr6, 0x8316140c
	if ctx.cr[6].eq {
	pc = 0x8316140C; continue 'dispatch;
	}
	// 83161408: 4BFFE9D9  bl 0x8315fde0
	ctx.lr = 0x8316140C;
	sub_8315FDE0(ctx, base);
	// 8316140C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161418: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316141C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83161420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161428 size=16
    let mut pc: u32 = 0x83161428;
    'dispatch: loop {
        match pc {
            0x83161428 => {
    //   block [0x83161428..0x83161438)
	// 83161428: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316142C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161434: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161438 size=20
    let mut pc: u32 = 0x83161438;
    'dispatch: loop {
        match pc {
            0x83161438 => {
    //   block [0x83161438..0x8316144C)
	// 83161438: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316143C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83161440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161444: 409AFFF4  bne cr6, 0x83161438
	if !ctx.cr[6].eq {
	pc = 0x83161438; continue 'dispatch;
	}
	// 83161448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161450 size=144
    let mut pc: u32 = 0x83161450;
    'dispatch: loop {
        match pc {
            0x83161450 => {
    //   block [0x83161450..0x831614E0)
	// 83161450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161454: 48046D15  bl 0x831a8168
	ctx.lr = 0x83161458;
	sub_831A8130(ctx, base);
	// 83161458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316145C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83161460: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161464: 409A0010  bne cr6, 0x83161474
	if !ctx.cr[6].eq {
	pc = 0x83161474; continue 'dispatch;
	}
	// 83161468: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316146C: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 83161470: 4BFFE6A9  bl 0x8315fb18
	ctx.lr = 0x83161474;
	sub_8315FB18(ctx, base);
	// 83161474: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316147C: 419A0008  beq cr6, 0x83161484
	if ctx.cr[6].eq {
	pc = 0x83161484; continue 'dispatch;
	}
	// 83161480: 4BFFE949  bl 0x8315fdc8
	ctx.lr = 0x83161484;
	sub_8315FDC8(ctx, base);
	// 83161484: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 83161488: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 8316148C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83161490: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161494: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83161498: 419A0020  beq cr6, 0x831614b8
	if ctx.cr[6].eq {
	pc = 0x831614B8; continue 'dispatch;
	}
	// 8316149C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831614A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831614A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831614A8: 4BFFFF09  bl 0x831613b0
	ctx.lr = 0x831614AC;
	sub_831613B0(ctx, base);
	// 831614AC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831614B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831614B4: 409AFFE8  bne cr6, 0x8316149c
	if !ctx.cr[6].eq {
	pc = 0x8316149C; continue 'dispatch;
	}
	// 831614B8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831614BC: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831614C0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831614C4: 4082FFCC  bne 0x83161490
	if !ctx.cr[0].eq {
	pc = 0x83161490; continue 'dispatch;
	}
	// 831614C8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831614CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831614D0: 419A0008  beq cr6, 0x831614d8
	if ctx.cr[6].eq {
	pc = 0x831614D8; continue 'dispatch;
	}
	// 831614D4: 4BFFE90D  bl 0x8315fde0
	ctx.lr = 0x831614D8;
	sub_8315FDE0(ctx, base);
	// 831614D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831614DC: 48046CDC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831614E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831614E0 size=116
    let mut pc: u32 = 0x831614E0;
    'dispatch: loop {
        match pc {
            0x831614E0 => {
    //   block [0x831614E0..0x83161554)
	// 831614E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831614E4: 48046C89  bl 0x831a816c
	ctx.lr = 0x831614E8;
	sub_831A8130(ctx, base);
	// 831614E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831614EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831614F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831614F4: 419A0058  beq cr6, 0x8316154c
	if ctx.cr[6].eq {
	pc = 0x8316154C; continue 'dispatch;
	}
	// 831614F8: 48000671  bl 0x83161b68
	ctx.lr = 0x831614FC;
	sub_83161B68(ctx, base);
	// 831614FC: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161500: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83161504: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161508: 419A000C  beq cr6, 0x83161514
	if ctx.cr[6].eq {
	pc = 0x83161514; continue 'dispatch;
	}
	// 8316150C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161510: 4BFFE8B9  bl 0x8315fdc8
	ctx.lr = 0x83161514;
	sub_8315FDC8(ctx, base);
	// 83161514: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83161518: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316151C: 419A000C  beq cr6, 0x83161528
	if ctx.cr[6].eq {
	pc = 0x83161528; continue 'dispatch;
	}
	// 83161520: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161524: 4BFFE245  bl 0x8315f768
	ctx.lr = 0x83161528;
	sub_8315F768(ctx, base);
	// 83161528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316152C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161530: 4BFFE239  bl 0x8315f768
	ctx.lr = 0x83161534;
	sub_8315F768(ctx, base);
	// 83161534: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161538: 419A0014  beq cr6, 0x8316154c
	if ctx.cr[6].eq {
	pc = 0x8316154C; continue 'dispatch;
	}
	// 8316153C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161540: 4BFFE8A1  bl 0x8315fde0
	ctx.lr = 0x83161544;
	sub_8315FDE0(ctx, base);
	// 83161544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161548: 4BFFE869  bl 0x8315fdb0
	ctx.lr = 0x8316154C;
	sub_8315FDB0(ctx, base);
	// 8316154C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161550: 48046C6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161558 size=136
    let mut pc: u32 = 0x83161558;
    'dispatch: loop {
        match pc {
            0x83161558 => {
    //   block [0x83161558..0x831615E0)
	// 83161558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161568: 419A0054  beq cr6, 0x831615bc
	if ctx.cr[6].eq {
	pc = 0x831615BC; continue 'dispatch;
	}
	// 8316156C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83161570: 4198004C  blt cr6, 0x831615bc
	if ctx.cr[6].lt {
	pc = 0x831615BC; continue 'dispatch;
	}
	// 83161574: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83161578: 40980044  bge cr6, 0x831615bc
	if !ctx.cr[6].lt {
	pc = 0x831615BC; continue 'dispatch;
	}
	// 8316157C: 39640005  addi r11, r4, 5
	ctx.r[11].s64 = ctx.r[4].s64 + 5;
	// 83161580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83161584: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83161588: 7D69182E  lwzx r11, r9, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8316158C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161590: 419A0018  beq cr6, 0x831615a8
	if ctx.cr[6].eq {
	pc = 0x831615A8; continue 'dispatch;
	}
	// 83161594: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161598: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316159C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831615A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831615A4: 409AFFF0  bne cr6, 0x83161594
	if !ctx.cr[6].eq {
	pc = 0x83161594; continue 'dispatch;
	}
	// 831615A8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 831615AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831615B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831615B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831615B8: 4E800020  blr
	return;
	// 831615BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831615C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831615C4: 388B65A4  addi r4, r11, 0x65a4
	ctx.r[4].s64 = ctx.r[11].s64 + 26020;
	// 831615C8: 4BFFE551  bl 0x8315fb18
	ctx.lr = 0x831615CC;
	sub_8315FB18(ctx, base);
	// 831615CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831615D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831615D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831615D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831615DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831615E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831615E0 size=256
    let mut pc: u32 = 0x831615E0;
    'dispatch: loop {
        match pc {
            0x831615E0 => {
    //   block [0x831615E0..0x831616E0)
	// 831615E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831615E4: 48046B81  bl 0x831a8164
	ctx.lr = 0x831615E8;
	sub_831A8130(ctx, base);
	// 831615E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831615EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831615F0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831615F4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 831615F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831615FC: 419A00C0  beq cr6, 0x831616bc
	if ctx.cr[6].eq {
	pc = 0x831616BC; continue 'dispatch;
	}
	// 83161600: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83161604: 419800B8  blt cr6, 0x831616bc
	if ctx.cr[6].lt {
	pc = 0x831616BC; continue 'dispatch;
	}
	// 83161608: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8316160C: 409800B0  bge cr6, 0x831616bc
	if !ctx.cr[6].lt {
	pc = 0x831616BC; continue 'dispatch;
	}
	// 83161610: 39640005  addi r11, r4, 5
	ctx.r[11].s64 = ctx.r[4].s64 + 5;
	// 83161614: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83161618: 7FFCF02E  lwzx r31, r28, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8316161C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83161620: 419A00AC  beq cr6, 0x831616cc
	if ctx.cr[6].eq {
	pc = 0x831616CC; continue 'dispatch;
	}
	// 83161624: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316162C: 419A0008  beq cr6, 0x83161634
	if ctx.cr[6].eq {
	pc = 0x83161634; continue 'dispatch;
	}
	// 83161630: 4BFFE799  bl 0x8315fdc8
	ctx.lr = 0x83161634;
	sub_8315FDC8(ctx, base);
	// 83161634: E97F0008  ld r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 83161638: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8316163C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83161640: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83161644: 41990020  bgt cr6, 0x83161664
	if ctx.cr[6].gt {
	pc = 0x83161664; continue 'dispatch;
	}
	// 83161648: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8316164C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161650: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161658: 7D7CF12E  stwx r11, r28, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8316165C: 4BFFFD55  bl 0x831613b0
	ctx.lr = 0x83161660;
	sub_831613B0(ctx, base);
	// 83161660: 48000044  b 0x831616a4
	pc = 0x831616A4; continue 'dispatch;
	// 83161664: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83161668: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316166C: 409A002C  bne cr6, 0x83161698
	if !ctx.cr[6].eq {
	pc = 0x83161698; continue 'dispatch;
	}
	// 83161670: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83161674: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83161678: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316167C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83161680: 48000539  bl 0x83161bb8
	ctx.lr = 0x83161684;
	sub_83161BB8(ctx, base);
	// 83161684: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83161688: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8316168C: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83161690: F95F0008  std r10, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83161694: 48000010  b 0x831616a4
	pc = 0x831616A4; continue 'dispatch;
	// 83161698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316169C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831616A0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831616A4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831616A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831616AC: 419A002C  beq cr6, 0x831616d8
	if ctx.cr[6].eq {
	pc = 0x831616D8; continue 'dispatch;
	}
	// 831616B0: 4BFFE731  bl 0x8315fde0
	ctx.lr = 0x831616B4;
	sub_8315FDE0(ctx, base);
	// 831616B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831616B8: 48046AFC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831616BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831616C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831616C4: 388B65A4  addi r4, r11, 0x65a4
	ctx.r[4].s64 = ctx.r[11].s64 + 26020;
	// 831616C8: 4BFFE451  bl 0x8315fb18
	ctx.lr = 0x831616CC;
	sub_8315FB18(ctx, base);
	// 831616CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831616D0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831616D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831616D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831616DC: 48046AD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831616E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831616E0 size=296
    let mut pc: u32 = 0x831616E0;
    'dispatch: loop {
        match pc {
            0x831616E0 => {
    //   block [0x831616E0..0x83161808)
	// 831616E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831616E4: 48046A89  bl 0x831a816c
	ctx.lr = 0x831616E8;
	sub_831A8130(ctx, base);
	// 831616E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831616EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831616F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831616F4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831616F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831616FC: 419A00F4  beq cr6, 0x831617f0
	if ctx.cr[6].eq {
	pc = 0x831617F0; continue 'dispatch;
	}
	// 83161700: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83161704: 419800EC  blt cr6, 0x831617f0
	if ctx.cr[6].lt {
	pc = 0x831617F0; continue 'dispatch;
	}
	// 83161708: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8316170C: 409800E4  bge cr6, 0x831617f0
	if !ctx.cr[6].lt {
	pc = 0x831617F0; continue 'dispatch;
	}
	// 83161710: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161718: 409900E8  ble cr6, 0x83161800
	if !ctx.cr[6].gt {
	pc = 0x83161800; continue 'dispatch;
	}
	// 8316171C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161724: 419A00DC  beq cr6, 0x83161800
	if ctx.cr[6].eq {
	pc = 0x83161800; continue 'dispatch;
	}
	// 83161728: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316172C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161730: 419A0008  beq cr6, 0x83161738
	if ctx.cr[6].eq {
	pc = 0x83161738; continue 'dispatch;
	}
	// 83161734: 4BFFE695  bl 0x8315fdc8
	ctx.lr = 0x83161738;
	sub_8315FDC8(ctx, base);
	// 83161738: 397F0005  addi r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 5;
	// 8316173C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83161740: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83161744: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83161748: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8316174C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161750: 419A0018  beq cr6, 0x83161768
	if ctx.cr[6].eq {
	pc = 0x83161768; continue 'dispatch;
	}
	// 83161754: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83161758: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8316175C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161760: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161764: 409AFFF0  bne cr6, 0x83161754
	if !ctx.cr[6].eq {
	pc = 0x83161754; continue 'dispatch;
	}
	// 83161768: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316176C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83161770: 409A0034  bne cr6, 0x831617a4
	if !ctx.cr[6].eq {
	pc = 0x831617A4; continue 'dispatch;
	}
	// 83161774: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83161778: 419A002C  beq cr6, 0x831617a4
	if ctx.cr[6].eq {
	pc = 0x831617A4; continue 'dispatch;
	}
	// 8316177C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83161780: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161784: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161788: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8316178C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83161790: 409A0014  bne cr6, 0x831617a4
	if !ctx.cr[6].eq {
	pc = 0x831617A4; continue 'dispatch;
	}
	// 83161794: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161798: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8316179C: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831617A0: 48000038  b 0x831617d8
	pc = 0x831617D8; continue 'dispatch;
	// 831617A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831617A8: 4BFFFB71  bl 0x83161318
	ctx.lr = 0x831617AC;
	sub_83161318(ctx, base);
	// 831617AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831617B0: 409A0014  bne cr6, 0x831617c4
	if !ctx.cr[6].eq {
	pc = 0x831617C4; continue 'dispatch;
	}
	// 831617B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831617B8: 388B65B0  addi r4, r11, 0x65b0
	ctx.r[4].s64 = ctx.r[11].s64 + 26032;
	// 831617BC: 4BFFE35D  bl 0x8315fb18
	ctx.lr = 0x831617C0;
	sub_8315FB18(ctx, base);
	// 831617C0: 48000018  b 0x831617d8
	pc = 0x831617D8; continue 'dispatch;
	// 831617C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831617C8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831617CC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831617D0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831617D4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831617D8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 831617DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831617E0: 419A0020  beq cr6, 0x83161800
	if ctx.cr[6].eq {
	pc = 0x83161800; continue 'dispatch;
	}
	// 831617E4: 4BFFE5FD  bl 0x8315fde0
	ctx.lr = 0x831617E8;
	sub_8315FDE0(ctx, base);
	// 831617E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831617EC: 480469D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831617F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831617F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831617F8: 388B65A4  addi r4, r11, 0x65a4
	ctx.r[4].s64 = ctx.r[11].s64 + 26020;
	// 831617FC: 4BFFE31D  bl 0x8315fb18
	ctx.lr = 0x83161800;
	sub_8315FB18(ctx, base);
	// 83161800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161804: 480469B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161808 size=260
    let mut pc: u32 = 0x83161808;
    'dispatch: loop {
        match pc {
            0x83161808 => {
    //   block [0x83161808..0x8316190C)
	// 83161808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316180C: 48046961  bl 0x831a816c
	ctx.lr = 0x83161810;
	sub_831A8130(ctx, base);
	// 83161810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161814: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83161818: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316181C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83161820: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161824: 419A00D0  beq cr6, 0x831618f4
	if ctx.cr[6].eq {
	pc = 0x831618F4; continue 'dispatch;
	}
	// 83161828: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8316182C: 419800C8  blt cr6, 0x831618f4
	if ctx.cr[6].lt {
	pc = 0x831618F4; continue 'dispatch;
	}
	// 83161830: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 83161834: 409800C0  bge cr6, 0x831618f4
	if !ctx.cr[6].lt {
	pc = 0x831618F4; continue 'dispatch;
	}
	// 83161838: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316183C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83161840: 409900C4  ble cr6, 0x83161904
	if !ctx.cr[6].gt {
	pc = 0x83161904; continue 'dispatch;
	}
	// 83161844: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316184C: 419A00B8  beq cr6, 0x83161904
	if ctx.cr[6].eq {
	pc = 0x83161904; continue 'dispatch;
	}
	// 83161850: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161854: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161858: 419A0008  beq cr6, 0x83161860
	if ctx.cr[6].eq {
	pc = 0x83161860; continue 'dispatch;
	}
	// 8316185C: 4BFFE56D  bl 0x8315fdc8
	ctx.lr = 0x83161860;
	sub_8315FDC8(ctx, base);
	// 83161860: 397D0005  addi r11, r29, 5
	ctx.r[11].s64 = ctx.r[29].s64 + 5;
	// 83161864: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83161868: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8316186C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83161870: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83161874: 409A003C  bne cr6, 0x831618b0
	if !ctx.cr[6].eq {
	pc = 0x831618B0; continue 'dispatch;
	}
	// 83161878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316187C: 419A0034  beq cr6, 0x831618b0
	if ctx.cr[6].eq {
	pc = 0x831618B0; continue 'dispatch;
	}
	// 83161880: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161884: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161888: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316188C: 7CE95214  add r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83161890: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83161894: 409A001C  bne cr6, 0x831618b0
	if !ctx.cr[6].eq {
	pc = 0x831618B0; continue 'dispatch;
	}
	// 83161898: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316189C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831618A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831618A4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831618A8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831618AC: 48000030  b 0x831618dc
	pc = 0x831618DC; continue 'dispatch;
	// 831618B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831618B4: 4BFFFA65  bl 0x83161318
	ctx.lr = 0x831618B8;
	sub_83161318(ctx, base);
	// 831618B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831618BC: 419A0048  beq cr6, 0x83161904
	if ctx.cr[6].eq {
	pc = 0x83161904; continue 'dispatch;
	}
	// 831618C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831618C4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831618C8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831618CC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831618D0: 7D3DF02E  lwzx r9, r29, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 831618D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831618D8: 7C7DF12E  stwx r3, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 831618DC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831618E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831618E4: 419A0020  beq cr6, 0x83161904
	if ctx.cr[6].eq {
	pc = 0x83161904; continue 'dispatch;
	}
	// 831618E8: 4BFFE4F9  bl 0x8315fde0
	ctx.lr = 0x831618EC;
	sub_8315FDE0(ctx, base);
	// 831618EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831618F0: 480468CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831618F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831618F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831618FC: 388B65A4  addi r4, r11, 0x65a4
	ctx.r[4].s64 = ctx.r[11].s64 + 26020;
	// 83161900: 4BFFE219  bl 0x8315fb18
	ctx.lr = 0x83161904;
	sub_8315FB18(ctx, base);
	// 83161904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161908: 480468B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161910 size=52
    let mut pc: u32 = 0x83161910;
    'dispatch: loop {
        match pc {
            0x83161910 => {
    //   block [0x83161910..0x83161944)
	// 83161910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316191C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83161920: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83161924: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83161928: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316192C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83161930: 4BFFF969  bl 0x83161298
	ctx.lr = 0x83161934;
	sub_83161298(ctx, base);
	// 83161934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83161938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161948 size=456
    let mut pc: u32 = 0x83161948;
    'dispatch: loop {
        match pc {
            0x83161948 => {
    //   block [0x83161948..0x83161B10)
	// 83161948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316194C: 48046811  bl 0x831a815c
	ctx.lr = 0x83161950;
	sub_831A8130(ctx, base);
	// 83161950: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161954: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83161958: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8316195C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83161960: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83161964: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161968: 409A0020  bne cr6, 0x83161988
	if !ctx.cr[6].eq {
	pc = 0x83161988; continue 'dispatch;
	}
	// 8316196C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161974: 388B665C  addi r4, r11, 0x665c
	ctx.r[4].s64 = ctx.r[11].s64 + 26204;
	// 83161978: 4BFFE1A1  bl 0x8315fb18
	ctx.lr = 0x8316197C;
	sub_8315FB18(ctx, base);
	// 8316197C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161980: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83161984: 48046828  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83161988: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8316198C: 409A0020  bne cr6, 0x831619ac
	if !ctx.cr[6].eq {
	pc = 0x831619AC; continue 'dispatch;
	}
	// 83161990: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161998: 388B6640  addi r4, r11, 0x6640
	ctx.r[4].s64 = ctx.r[11].s64 + 26176;
	// 8316199C: 4BFFE17D  bl 0x8315fb18
	ctx.lr = 0x831619A0;
	sub_8315FB18(ctx, base);
	// 831619A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831619A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831619A8: 48046804  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 831619AC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831619B0: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831619B4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831619B8: 3B4B6634  addi r26, r11, 0x6634
	ctx.r[26].s64 = ctx.r[11].s64 + 26164;
	// 831619BC: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 831619C0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 831619C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831619C8: 4BFFDD39  bl 0x8315f700
	ctx.lr = 0x831619CC;
	sub_8315F700(ctx, base);
	// 831619CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831619D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831619D4: 409A001C  bne cr6, 0x831619f0
	if !ctx.cr[6].eq {
	pc = 0x831619F0; continue 'dispatch;
	}
	// 831619D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831619DC: 388B6628  addi r4, r11, 0x6628
	ctx.r[4].s64 = ctx.r[11].s64 + 26152;
	// 831619E0: 4BFFE139  bl 0x8315fb18
	ctx.lr = 0x831619E4;
	sub_8315FB18(ctx, base);
	// 831619E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831619E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831619EC: 480467C0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 831619F0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 831619F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831619F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831619FC: 480467E5  bl 0x831a81e0
	ctx.lr = 0x83161A00;
	sub_831A81E0(ctx, base);
	// 83161A00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83161A04: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83161A08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83161A0C: 394BBC7C  addi r10, r11, -0x4384
	ctx.r[10].s64 = ctx.r[11].s64 + -17284;
	// 83161A10: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83161A14: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83161A18: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161A1C: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 83161A20: 409A0044  bne cr6, 0x83161a64
	if !ctx.cr[6].eq {
	pc = 0x83161A64; continue 'dispatch;
	}
	// 83161A24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83161A28: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161A2C: 4BFFE4BD  bl 0x8315fee8
	ctx.lr = 0x83161A30;
	sub_8315FEE8(ctx, base);
	// 83161A30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161A34: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83161A38: 409A0030  bne cr6, 0x83161a68
	if !ctx.cr[6].eq {
	pc = 0x83161A68; continue 'dispatch;
	}
	// 83161A3C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161A40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161A44: 388B65F8  addi r4, r11, 0x65f8
	ctx.r[4].s64 = ctx.r[11].s64 + 26104;
	// 83161A48: 4BFFE0D1  bl 0x8315fb18
	ctx.lr = 0x83161A4C;
	sub_8315FB18(ctx, base);
	// 83161A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161A50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83161A54: 4BFFDD15  bl 0x8315f768
	ctx.lr = 0x83161A58;
	sub_8315F768(ctx, base);
	// 83161A58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161A5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83161A60: 4804674C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83161A64: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83161A68: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 83161A6C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161A70: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83161A74: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83161A78: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83161A7C: 38AB65E4  addi r5, r11, 0x65e4
	ctx.r[5].s64 = ctx.r[11].s64 + 26084;
	// 83161A80: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 83161A84: 57C42036  slwi r4, r30, 4
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83161A88: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83161A8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83161A90: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 83161A94: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161A98: 4BFFDC69  bl 0x8315f700
	ctx.lr = 0x83161A9C;
	sub_8315F700(ctx, base);
	// 83161A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161AA0: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83161AA4: 409A003C  bne cr6, 0x83161ae0
	if !ctx.cr[6].eq {
	pc = 0x83161AE0; continue 'dispatch;
	}
	// 83161AA8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161AB0: 388B65BC  addi r4, r11, 0x65bc
	ctx.r[4].s64 = ctx.r[11].s64 + 26044;
	// 83161AB4: 4BFFE065  bl 0x8315fb18
	ctx.lr = 0x83161AB8;
	sub_8315FB18(ctx, base);
	// 83161AB8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161AC0: 419A0008  beq cr6, 0x83161ac8
	if ctx.cr[6].eq {
	pc = 0x83161AC8; continue 'dispatch;
	}
	// 83161AC4: 4BFFE2ED  bl 0x8315fdb0
	ctx.lr = 0x83161AC8;
	sub_8315FDB0(ctx, base);
	// 83161AC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161ACC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83161AD0: 4BFFDC99  bl 0x8315f768
	ctx.lr = 0x83161AD4;
	sub_8315F768(ctx, base);
	// 83161AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161AD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83161ADC: 480466D0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83161AE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161AE4: 419A0020  beq cr6, 0x83161b04
	if ctx.cr[6].eq {
	pc = 0x83161B04; continue 'dispatch;
	}
	// 83161AE8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83161AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161AF0: 7C9D5A14  add r4, r29, r11
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 83161AF4: 4BFFF8BD  bl 0x831613b0
	ctx.lr = 0x83161AF8;
	sub_831613B0(ctx, base);
	// 83161AF8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83161AFC: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 83161B00: 4082FFE8  bne 0x83161ae8
	if !ctx.cr[0].eq {
	pc = 0x83161AE8; continue 'dispatch;
	}
	// 83161B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161B08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83161B0C: 480466A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161B10 size=52
    let mut pc: u32 = 0x83161B10;
    'dispatch: loop {
        match pc {
            0x83161B10 => {
    //   block [0x83161B10..0x83161B44)
	// 83161B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161B1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83161B20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83161B24: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83161B28: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83161B2C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83161B30: 4BFFFE19  bl 0x83161948
	ctx.lr = 0x83161B34;
	sub_83161948(ctx, base);
	// 83161B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83161B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161B48 size=32
    let mut pc: u32 = 0x83161B48;
    'dispatch: loop {
        match pc {
            0x83161B48 => {
    //   block [0x83161B48..0x83161B68)
	// 83161B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B4C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83161B50: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 83161B54: 390A6678  addi r8, r10, 0x6678
	ctx.r[8].s64 = ctx.r[10].s64 + 26232;
	// 83161B58: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B5C: 91098310  stw r8, -0x7cf0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-31984 as u32), ctx.r[8].u32 ) };
	// 83161B60: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 83161B64: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161B68 size=16
    let mut pc: u32 = 0x83161B68;
    'dispatch: loop {
        match pc {
            0x83161B68 => {
    //   block [0x83161B68..0x83161B78)
	// 83161B68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B6C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161B70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83161B74: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161B78 size=16
    let mut pc: u32 = 0x83161B78;
    'dispatch: loop {
        match pc {
            0x83161B78 => {
    //   block [0x83161B78..0x83161B88)
	// 83161B78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B7C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83161B80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83161B84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161B88 size=16
    let mut pc: u32 = 0x83161B88;
    'dispatch: loop {
        match pc {
            0x83161B88 => {
    //   block [0x83161B88..0x83161B98)
	// 83161B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B8C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161B90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83161B94: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161B98 size=16
    let mut pc: u32 = 0x83161B98;
    'dispatch: loop {
        match pc {
            0x83161B98 => {
    //   block [0x83161B98..0x83161BA8)
	// 83161B98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161B9C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83161BA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83161BA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161BA8 size=16
    let mut pc: u32 = 0x83161BA8;
    'dispatch: loop {
        match pc {
            0x83161BA8 => {
    //   block [0x83161BA8..0x83161BB8)
	// 83161BA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161BAC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83161BB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83161BB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161BB8 size=72
    let mut pc: u32 = 0x83161BB8;
    'dispatch: loop {
        match pc {
            0x83161BB8 => {
    //   block [0x83161BB8..0x83161C00)
	// 83161BB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161BBC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83161BC0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161BC4: 5549003E  slwi r9, r10, 0
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83161BC8: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83161BCC: 91260004  stw r9, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83161BD0: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161BD4: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83161BD8: 40990008  ble cr6, 0x83161be0
	if !ctx.cr[6].gt {
	pc = 0x83161BE0; continue 'dispatch;
	}
	// 83161BDC: 90850004  stw r4, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83161BE0: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161BE4: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161BE8: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83161BEC: 91260004  stw r9, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83161BF0: 40820010  bne 0x83161c00
	if !ctx.cr[0].eq {
		sub_83161C00(ctx, base);
		return;
	}
	// 83161BF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83161BF8: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83161BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161C00 size=20
    let mut pc: u32 = 0x83161C00;
    'dispatch: loop {
        match pc {
            0x83161C00 => {
    //   block [0x83161C00..0x83161C14)
	// 83161C00: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161C04: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161C08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83161C0C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83161C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161C18 size=16
    let mut pc: u32 = 0x83161C18;
    'dispatch: loop {
        match pc {
            0x83161C18 => {
    //   block [0x83161C18..0x83161C28)
	// 83161C18: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83161C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83161C20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83161C24: 4BFFFF54  b 0x83161b78
	sub_83161B78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161C28 size=120
    let mut pc: u32 = 0x83161C28;
    'dispatch: loop {
        match pc {
            0x83161C28 => {
    //   block [0x83161C28..0x83161CA0)
	// 83161C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83161C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161C38: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161C3C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83161C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161C44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83161C48: 40990008  ble cr6, 0x83161c50
	if !ctx.cr[6].gt {
	pc = 0x83161C50; continue 'dispatch;
	}
	// 83161C4C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83161C50: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83161C54: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83161C58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83161C5C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83161C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83161C64: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83161C68: 4BFFFF51  bl 0x83161bb8
	ctx.lr = 0x83161C6C;
	sub_83161BB8(ctx, base);
	// 83161C6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83161C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83161C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161C78: 4BFFFF21  bl 0x83161b98
	ctx.lr = 0x83161C7C;
	sub_83161B98(ctx, base);
	// 83161C7C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83161C80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83161C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161C88: 4BFFFF01  bl 0x83161b88
	ctx.lr = 0x83161C8C;
	sub_83161B88(ctx, base);
	// 83161C8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83161C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161CA0 size=8
    let mut pc: u32 = 0x83161CA0;
    'dispatch: loop {
        match pc {
            0x83161CA0 => {
    //   block [0x83161CA0..0x83161CA8)
	// 83161CA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83161CA4: 4BFFFF04  b 0x83161ba8
	sub_83161BA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83161CA8 size=16
    let mut pc: u32 = 0x83161CA8;
    'dispatch: loop {
        match pc {
            0x83161CA8 => {
    //   block [0x83161CA8..0x83161CB8)
	// 83161CA8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83161CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83161CB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83161CB4: 4BFFFEC4  b 0x83161b78
	sub_83161B78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161CB8 size=120
    let mut pc: u32 = 0x83161CB8;
    'dispatch: loop {
        match pc {
            0x83161CB8 => {
    //   block [0x83161CB8..0x83161D30)
	// 83161CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83161CC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161CC8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161CCC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83161CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161CD4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83161CD8: 40990008  ble cr6, 0x83161ce0
	if !ctx.cr[6].gt {
	pc = 0x83161CE0; continue 'dispatch;
	}
	// 83161CDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83161CE0: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83161CE4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83161CE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83161CEC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83161CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83161CF4: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83161CF8: 4BFFFEC1  bl 0x83161bb8
	ctx.lr = 0x83161CFC;
	sub_83161BB8(ctx, base);
	// 83161CFC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83161D00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83161D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161D08: 4BFFFE91  bl 0x83161b98
	ctx.lr = 0x83161D0C;
	sub_83161B98(ctx, base);
	// 83161D0C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83161D10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83161D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161D18: 4BFFFE71  bl 0x83161b88
	ctx.lr = 0x83161D1C;
	sub_83161B88(ctx, base);
	// 83161D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83161D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161D30 size=376
    let mut pc: u32 = 0x83161D30;
    'dispatch: loop {
        match pc {
            0x83161D30 => {
    //   block [0x83161D30..0x83161EA8)
	// 83161D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161D34: 48046425  bl 0x831a8158
	ctx.lr = 0x83161D38;
	sub_831A8130(ctx, base);
	// 83161D38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161D3C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83161D40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83161D44: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83161D48: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83161D4C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83161D50: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83161D54: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161D58: 409A0020  bne cr6, 0x83161d78
	if !ctx.cr[6].eq {
	pc = 0x83161D78; continue 'dispatch;
	}
	// 83161D5C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161D60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161D64: 388B675C  addi r4, r11, 0x675c
	ctx.r[4].s64 = ctx.r[11].s64 + 26460;
	// 83161D68: 4BFFDDB1  bl 0x8315fb18
	ctx.lr = 0x83161D6C;
	sub_8315FB18(ctx, base);
	// 83161D6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161D70: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83161D74: 48046434  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83161D78: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161D7C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161D80: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83161D84: 3B8B6750  addi r28, r11, 0x6750
	ctx.r[28].s64 = ctx.r[11].s64 + 26448;
	// 83161D88: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83161D8C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83161D90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161D94: 4BFFD96D  bl 0x8315f700
	ctx.lr = 0x83161D98;
	sub_8315F700(ctx, base);
	// 83161D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161D9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83161DA0: 409A001C  bne cr6, 0x83161dbc
	if !ctx.cr[6].eq {
	pc = 0x83161DBC; continue 'dispatch;
	}
	// 83161DA4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161DA8: 388B6744  addi r4, r11, 0x6744
	ctx.r[4].s64 = ctx.r[11].s64 + 26436;
	// 83161DAC: 4BFFDD6D  bl 0x8315fb18
	ctx.lr = 0x83161DB0;
	sub_8315FB18(ctx, base);
	// 83161DB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161DB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83161DB8: 480463F0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83161DBC: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 83161DC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83161DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161DC8: 48046419  bl 0x831a81e0
	ctx.lr = 0x83161DCC;
	sub_831A81E0(ctx, base);
	// 83161DCC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83161DD0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83161DD4: 394BBC94  addi r10, r11, -0x436c
	ctx.r[10].s64 = ctx.r[11].s64 + -17260;
	// 83161DD8: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83161DDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83161DE0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161DE4: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 83161DE8: 409A0044  bne cr6, 0x83161e2c
	if !ctx.cr[6].eq {
	pc = 0x83161E2C; continue 'dispatch;
	}
	// 83161DEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161DF0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161DF4: 4BFFE0F5  bl 0x8315fee8
	ctx.lr = 0x83161DF8;
	sub_8315FEE8(ctx, base);
	// 83161DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161DFC: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83161E00: 409A0034  bne cr6, 0x83161e34
	if !ctx.cr[6].eq {
	pc = 0x83161E34; continue 'dispatch;
	}
	// 83161E04: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161E08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161E0C: 388B6714  addi r4, r11, 0x6714
	ctx.r[4].s64 = ctx.r[11].s64 + 26388;
	// 83161E10: 4BFFDD09  bl 0x8315fb18
	ctx.lr = 0x83161E14;
	sub_8315FB18(ctx, base);
	// 83161E14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161E18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161E1C: 4BFFD94D  bl 0x8315f768
	ctx.lr = 0x83161E20;
	sub_8315F768(ctx, base);
	// 83161E20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161E24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83161E28: 48046380  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83161E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83161E30: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83161E34: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83161E38: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83161E3C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83161E40: 7C9BD214  add r4, r27, r26
	ctx.r[4].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 83161E44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161E48: 4BFFD8B9  bl 0x8315f700
	ctx.lr = 0x83161E4C;
	sub_8315F700(ctx, base);
	// 83161E4C: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83161E50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161E54: 409A0038  bne cr6, 0x83161e8c
	if !ctx.cr[6].eq {
	pc = 0x83161E8C; continue 'dispatch;
	}
	// 83161E58: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161E5C: 388B66D4  addi r4, r11, 0x66d4
	ctx.r[4].s64 = ctx.r[11].s64 + 26324;
	// 83161E60: 4BFFDCB9  bl 0x8315fb18
	ctx.lr = 0x83161E64;
	sub_8315FB18(ctx, base);
	// 83161E64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161E68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161E6C: 419A0008  beq cr6, 0x83161e74
	if ctx.cr[6].eq {
	pc = 0x83161E74; continue 'dispatch;
	}
	// 83161E70: 4BFFDF41  bl 0x8315fdb0
	ctx.lr = 0x83161E74;
	sub_8315FDB0(ctx, base);
	// 83161E74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161E78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161E7C: 4BFFD8ED  bl 0x8315f768
	ctx.lr = 0x83161E80;
	sub_8315F768(ctx, base);
	// 83161E80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161E84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83161E88: 48046320  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83161E8C: 937F0024  stw r27, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 83161E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161E94: 935F0028  stw r26, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[26].u32 ) };
	// 83161E98: 4BFFFCD1  bl 0x83161b68
	ctx.lr = 0x83161E9C;
	sub_83161B68(ctx, base);
	// 83161E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83161EA0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83161EA4: 48046304  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161EA8 size=124
    let mut pc: u32 = 0x83161EA8;
    'dispatch: loop {
        match pc {
            0x83161EA8 => {
    //   block [0x83161EA8..0x83161F24)
	// 83161EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161EAC: 480462B9  bl 0x831a8164
	ctx.lr = 0x83161EB0;
	sub_831A8130(ctx, base);
	// 83161EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83161EB8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83161EBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83161EC0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83161EC4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 83161EC8: 4BFFD6C1  bl 0x8315f588
	ctx.lr = 0x83161ECC;
	sub_8315F588(ctx, base);
	// 83161ECC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83161ED0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83161ED4: 4BFFD6B5  bl 0x8315f588
	ctx.lr = 0x83161ED8;
	sub_8315F588(ctx, base);
	// 83161ED8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83161EDC: 409A0020  bne cr6, 0x83161efc
	if !ctx.cr[6].eq {
	pc = 0x83161EFC; continue 'dispatch;
	}
	// 83161EE0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161EE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161EE8: 388B6778  addi r4, r11, 0x6778
	ctx.r[4].s64 = ctx.r[11].s64 + 26488;
	// 83161EEC: 4BFFDC2D  bl 0x8315fb18
	ctx.lr = 0x83161EF0;
	sub_8315FB18(ctx, base);
	// 83161EF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83161EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83161EF8: 480462BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83161EFC: 7D63E214  add r11, r3, r28
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 83161F00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83161F04: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83161F08: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83161F0C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83161F10: 386B003C  addi r3, r11, 0x3c
	ctx.r[3].s64 = ctx.r[11].s64 + 60;
	// 83161F14: 409A0008  bne cr6, 0x83161f1c
	if !ctx.cr[6].eq {
	pc = 0x83161F1C; continue 'dispatch;
	}
	// 83161F18: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 83161F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83161F20: 48046294  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161F28 size=104
    let mut pc: u32 = 0x83161F28;
    'dispatch: loop {
        match pc {
            0x83161F28 => {
    //   block [0x83161F28..0x83161F90)
	// 83161F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161F2C: 48046241  bl 0x831a816c
	ctx.lr = 0x83161F30;
	sub_831A8130(ctx, base);
	// 83161F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161F38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83161F3C: 419A004C  beq cr6, 0x83161f88
	if ctx.cr[6].eq {
	pc = 0x83161F88; continue 'dispatch;
	}
	// 83161F40: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161F44: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83161F48: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161F4C: 419A000C  beq cr6, 0x83161f58
	if ctx.cr[6].eq {
	pc = 0x83161F58; continue 'dispatch;
	}
	// 83161F50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161F54: 4BFFDE75  bl 0x8315fdc8
	ctx.lr = 0x83161F58;
	sub_8315FDC8(ctx, base);
	// 83161F58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161F5C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83161F60: 4BFFD809  bl 0x8315f768
	ctx.lr = 0x83161F64;
	sub_8315F768(ctx, base);
	// 83161F64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83161F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83161F6C: 4BFFD7FD  bl 0x8315f768
	ctx.lr = 0x83161F70;
	sub_8315F768(ctx, base);
	// 83161F70: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83161F74: 419A0014  beq cr6, 0x83161f88
	if ctx.cr[6].eq {
	pc = 0x83161F88; continue 'dispatch;
	}
	// 83161F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161F7C: 4BFFDE65  bl 0x8315fde0
	ctx.lr = 0x83161F80;
	sub_8315FDE0(ctx, base);
	// 83161F80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83161F84: 4BFFDE2D  bl 0x8315fdb0
	ctx.lr = 0x83161F88;
	sub_8315FDB0(ctx, base);
	// 83161F88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83161F8C: 48046230  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83161F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83161F90 size=152
    let mut pc: u32 = 0x83161F90;
    'dispatch: loop {
        match pc {
            0x83161F90 => {
    //   block [0x83161F90..0x83162028)
	// 83161F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83161F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83161F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83161F9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83161FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83161FA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83161FA8: 409A0024  bne cr6, 0x83161fcc
	if !ctx.cr[6].eq {
	pc = 0x83161FCC; continue 'dispatch;
	}
	// 83161FAC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83161FB0: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 83161FB4: 4BFFDB65  bl 0x8315fb18
	ctx.lr = 0x83161FB8;
	sub_8315FB18(ctx, base);
	// 83161FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83161FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83161FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83161FC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83161FC8: 4E800020  blr
	return;
	// 83161FCC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161FD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161FD4: 419A0008  beq cr6, 0x83161fdc
	if ctx.cr[6].eq {
	pc = 0x83161FDC; continue 'dispatch;
	}
	// 83161FD8: 4BFFDDF1  bl 0x8315fdc8
	ctx.lr = 0x83161FDC;
	sub_8315FDC8(ctx, base);
	// 83161FDC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83161FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83161FE4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83161FE8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83161FEC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83161FF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83161FF4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83161FF8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83161FFC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83162000: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83162004: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83162008: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8316200C: 419A0008  beq cr6, 0x83162014
	if ctx.cr[6].eq {
	pc = 0x83162014; continue 'dispatch;
	}
	// 83162010: 4BFFDDD1  bl 0x8315fde0
	ctx.lr = 0x83162014;
	sub_8315FDE0(ctx, base);
	// 83162014: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316201C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162028 size=124
    let mut pc: u32 = 0x83162028;
    'dispatch: loop {
        match pc {
            0x83162028 => {
    //   block [0x83162028..0x831620A4)
	// 83162028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83162034: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83162038: 409A0010  bne cr6, 0x83162048
	if !ctx.cr[6].eq {
	pc = 0x83162048; continue 'dispatch;
	}
	// 8316203C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162040: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 83162044: 48000044  b 0x83162088
	pc = 0x83162088; continue 'dispatch;
	// 83162048: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8316204C: 409A0018  bne cr6, 0x83162064
	if !ctx.cr[6].eq {
	pc = 0x83162064; continue 'dispatch;
	}
	// 83162050: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83162054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316205C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162060: 4E800020  blr
	return;
	// 83162064: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83162068: 409A0018  bne cr6, 0x83162080
	if !ctx.cr[6].eq {
	pc = 0x83162080; continue 'dispatch;
	}
	// 8316206C: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83162070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83162078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316207C: 4E800020  blr
	return;
	// 83162080: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162084: 388B6794  addi r4, r11, 0x6794
	ctx.r[4].s64 = ctx.r[11].s64 + 26516;
	// 83162088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316208C: 4BFFDA8D  bl 0x8315fb18
	ctx.lr = 0x83162090;
	sub_8315FB18(ctx, base);
	// 83162090: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316209C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831620A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831620A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831620A8 size=448
    let mut pc: u32 = 0x831620A8;
    'dispatch: loop {
        match pc {
            0x831620A8 => {
    //   block [0x831620A8..0x83162268)
	// 831620A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831620AC: 480460BD  bl 0x831a8168
	ctx.lr = 0x831620B0;
	sub_831A8130(ctx, base);
	// 831620B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831620B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831620B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831620BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831620C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 831620C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831620C8: 409A0018  bne cr6, 0x831620e0
	if !ctx.cr[6].eq {
	pc = 0x831620E0; continue 'dispatch;
	}
	// 831620CC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831620D0: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 831620D4: 4BFFDA45  bl 0x8315fb18
	ctx.lr = 0x831620D8;
	sub_8315FB18(ctx, base);
	// 831620D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831620DC: 480460DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831620E0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831620E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831620E8: 409A001C  bne cr6, 0x83162104
	if !ctx.cr[6].eq {
	pc = 0x83162104; continue 'dispatch;
	}
	// 831620EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831620F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831620F4: 388B6794  addi r4, r11, 0x6794
	ctx.r[4].s64 = ctx.r[11].s64 + 26516;
	// 831620F8: 4BFFDA21  bl 0x8315fb18
	ctx.lr = 0x831620FC;
	sub_8315FB18(ctx, base);
	// 831620FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83162100: 480460B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83162104: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162108: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316210C: 419A0008  beq cr6, 0x83162114
	if ctx.cr[6].eq {
	pc = 0x83162114; continue 'dispatch;
	}
	// 83162110: 4BFFDCB9  bl 0x8315fdc8
	ctx.lr = 0x83162114;
	sub_8315FDC8(ctx, base);
	// 83162114: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83162118: 409A008C  bne cr6, 0x831621a4
	if !ctx.cr[6].eq {
	pc = 0x831621A4; continue 'dispatch;
	}
	// 8316211C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83162120: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83162124: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162128: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8316212C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83162130: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83162134: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83162138: 41980008  blt cr6, 0x83162140
	if ctx.cr[6].lt {
	pc = 0x83162140; continue 'dispatch;
	}
	// 8316213C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83162140: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83162144: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83162148: 41980008  blt cr6, 0x83162150
	if ctx.cr[6].lt {
	pc = 0x83162150; continue 'dispatch;
	}
	// 8316214C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83162150: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83162154: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83162158: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316215C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83162160: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83162164: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162168: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316216C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83162170: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83162174: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83162178: 7CC74B96  divwu r6, r7, r9
	ctx.r[6].u32 = ctx.r[7].u32 / ctx.r[9].u32;
	// 8316217C: 7CA649D6  mullw r5, r6, r9
	ctx.r[5].s64 = (ctx.r[6].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83162180: 7C853850  subf r4, r5, r7
	ctx.r[4].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 83162184: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83162188: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316218C: 7D634050  subf r11, r3, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[3].s64;
	// 83162190: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83162194: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162198: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316219C: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 831621A0: 480000B0  b 0x83162250
	pc = 0x83162250; continue 'dispatch;
	// 831621A4: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 831621A8: 409A008C  bne cr6, 0x83162234
	if !ctx.cr[6].eq {
	pc = 0x83162234; continue 'dispatch;
	}
	// 831621AC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 831621B0: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831621B4: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831621B8: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831621BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831621C0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831621C4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831621C8: 41980008  blt cr6, 0x831621d0
	if ctx.cr[6].lt {
	pc = 0x831621D0; continue 'dispatch;
	}
	// 831621CC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831621D0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831621D4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831621D8: 41980008  blt cr6, 0x831621e0
	if ctx.cr[6].lt {
	pc = 0x831621E0; continue 'dispatch;
	}
	// 831621DC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 831621E0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831621E4: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831621E8: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831621EC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831621F0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831621F4: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831621F8: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831621FC: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162200: 7CC95A14  add r6, r9, r11
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83162204: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83162208: 7CA63B96  divwu r5, r6, r7
	ctx.r[5].u32 = ctx.r[6].u32 / ctx.r[7].u32;
	// 8316220C: 7C8539D6  mullw r4, r5, r7
	ctx.r[4].s64 = (ctx.r[5].s32 as i64) * (ctx.r[7].s32 as i64);
	// 83162210: 7C643050  subf r3, r4, r6
	ctx.r[3].s64 = ctx.r[6].s64 - ctx.r[4].s64;
	// 83162214: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83162218: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316221C: 7D2B4050  subf r9, r11, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 83162220: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83162224: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162228: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316222C: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 83162230: 48000020  b 0x83162250
	pc = 0x83162250; continue 'dispatch;
	// 83162234: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83162238: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8316223C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83162240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162244: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83162248: 388A6794  addi r4, r10, 0x6794
	ctx.r[4].s64 = ctx.r[10].s64 + 26516;
	// 8316224C: 4BFFD8CD  bl 0x8315fb18
	ctx.lr = 0x83162250;
	sub_8315FB18(ctx, base);
	// 83162250: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162254: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83162258: 419A0008  beq cr6, 0x83162260
	if ctx.cr[6].eq {
	pc = 0x83162260; continue 'dispatch;
	}
	// 8316225C: 4BFFDB85  bl 0x8315fde0
	ctx.lr = 0x83162260;
	sub_8315FDE0(ctx, base);
	// 83162260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83162264: 48045F54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162268 size=416
    let mut pc: u32 = 0x83162268;
    'dispatch: loop {
        match pc {
            0x83162268 => {
    //   block [0x83162268..0x83162408)
	// 83162268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83162274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83162278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316227C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83162280: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83162284: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83162288: 409A0010  bne cr6, 0x83162298
	if !ctx.cr[6].eq {
	pc = 0x83162298; continue 'dispatch;
	}
	// 8316228C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162290: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 83162294: 48000154  b 0x831623e8
	pc = 0x831623E8; continue 'dispatch;
	// 83162298: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316229C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831622A0: 409A0010  bne cr6, 0x831622b0
	if !ctx.cr[6].eq {
	pc = 0x831622B0; continue 'dispatch;
	}
	// 831622A4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831622A8: 388B6794  addi r4, r11, 0x6794
	ctx.r[4].s64 = ctx.r[11].s64 + 26516;
	// 831622AC: 4800013C  b 0x831623e8
	pc = 0x831623E8; continue 'dispatch;
	// 831622B0: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831622B4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831622B8: 419A0138  beq cr6, 0x831623f0
	if ctx.cr[6].eq {
	pc = 0x831623F0; continue 'dispatch;
	}
	// 831622BC: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831622C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831622C4: 419A012C  beq cr6, 0x831623f0
	if ctx.cr[6].eq {
	pc = 0x831623F0; continue 'dispatch;
	}
	// 831622C8: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 831622CC: 409A00BC  bne cr6, 0x83162388
	if !ctx.cr[6].eq {
	pc = 0x83162388; continue 'dispatch;
	}
	// 831622D0: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831622D4: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 831622D8: 7D694050  subf r11, r9, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831622DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831622E0: 40980024  bge cr6, 0x83162304
	if !ctx.cr[6].lt {
	pc = 0x83162304; continue 'dispatch;
	}
	// 831622E4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831622E8: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831622EC: 41980008  blt cr6, 0x831622f4
	if ctx.cr[6].lt {
	pc = 0x831622F4; continue 'dispatch;
	}
	// 831622F0: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 831622F4: 7D493A14  add r10, r9, r7
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 831622F8: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 831622FC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83162300: 48046211  bl 0x831a8510
	ctx.lr = 0x83162304;
	sub_831A8510(ctx, base);
	// 83162304: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162308: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316230C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162310: 7D035850  subf r8, r3, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83162314: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162318: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8316231C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83162320: 40990024  ble cr6, 0x83162344
	if !ctx.cr[6].gt {
	pc = 0x83162344; continue 'dispatch;
	}
	// 83162324: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 83162328: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8316232C: 41980008  blt cr6, 0x83162334
	if ctx.cr[6].lt {
	pc = 0x83162334; continue 'dispatch;
	}
	// 83162330: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83162334: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83162338: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8316233C: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83162340: 480461D1  bl 0x831a8510
	ctx.lr = 0x83162344;
	sub_831A8510(ctx, base);
	// 83162344: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162348: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316234C: 419A0008  beq cr6, 0x83162354
	if ctx.cr[6].eq {
	pc = 0x83162354; continue 'dispatch;
	}
	// 83162350: 4BFFDA79  bl 0x8315fdc8
	ctx.lr = 0x83162354;
	sub_8315FDC8(ctx, base);
	// 83162354: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162358: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316235C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83162360: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83162364: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162368: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8316236C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83162370: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162374: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83162378: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 8316237C: 419A0074  beq cr6, 0x831623f0
	if ctx.cr[6].eq {
	pc = 0x831623F0; continue 'dispatch;
	}
	// 83162380: 4BFFDA61  bl 0x8315fde0
	ctx.lr = 0x83162384;
	sub_8315FDE0(ctx, base);
	// 83162384: 4800006C  b 0x831623f0
	pc = 0x831623F0; continue 'dispatch;
	// 83162388: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316238C: 409A0048  bne cr6, 0x831623d4
	if !ctx.cr[6].eq {
	pc = 0x831623D4; continue 'dispatch;
	}
	// 83162390: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162394: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83162398: 419A0008  beq cr6, 0x831623a0
	if ctx.cr[6].eq {
	pc = 0x831623A0; continue 'dispatch;
	}
	// 8316239C: 4BFFDA2D  bl 0x8315fdc8
	ctx.lr = 0x831623A0;
	sub_8315FDC8(ctx, base);
	// 831623A0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831623A4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831623A8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 831623AC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831623B0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831623B4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 831623B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831623BC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831623C0: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831623C4: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 831623C8: 419A0028  beq cr6, 0x831623f0
	if ctx.cr[6].eq {
	pc = 0x831623F0; continue 'dispatch;
	}
	// 831623CC: 4BFFDA15  bl 0x8315fde0
	ctx.lr = 0x831623D0;
	sub_8315FDE0(ctx, base);
	// 831623D0: 48000020  b 0x831623f0
	pc = 0x831623F0; continue 'dispatch;
	// 831623D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831623D8: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831623DC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831623E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831623E4: 388A6794  addi r4, r10, 0x6794
	ctx.r[4].s64 = ctx.r[10].s64 + 26516;
	// 831623E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831623EC: 4BFFD72D  bl 0x8315fb18
	ctx.lr = 0x831623F0;
	sub_8315FB18(ctx, base);
	// 831623F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831623F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831623F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831623FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83162400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162408 size=476
    let mut pc: u32 = 0x83162408;
    'dispatch: loop {
        match pc {
            0x83162408 => {
    //   block [0x83162408..0x831625E4)
	// 83162408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83162414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83162418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316241C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83162420: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83162424: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83162428: 409A0010  bne cr6, 0x83162438
	if !ctx.cr[6].eq {
	pc = 0x83162438; continue 'dispatch;
	}
	// 8316242C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162430: 388B6598  addi r4, r11, 0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + 26008;
	// 83162434: 48000190  b 0x831625c4
	pc = 0x831625C4; continue 'dispatch;
	// 83162438: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316243C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83162440: 409A0010  bne cr6, 0x83162450
	if !ctx.cr[6].eq {
	pc = 0x83162450; continue 'dispatch;
	}
	// 83162444: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162448: 388B6794  addi r4, r11, 0x6794
	ctx.r[4].s64 = ctx.r[11].s64 + 26516;
	// 8316244C: 48000178  b 0x831625c4
	pc = 0x831625C4; continue 'dispatch;
	// 83162450: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83162458: 40990174  ble cr6, 0x831625cc
	if !ctx.cr[6].gt {
	pc = 0x831625CC; continue 'dispatch;
	}
	// 8316245C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83162464: 419A0168  beq cr6, 0x831625cc
	if ctx.cr[6].eq {
	pc = 0x831625CC; continue 'dispatch;
	}
	// 83162468: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316246C: 409A00A0  bne cr6, 0x8316250c
	if !ctx.cr[6].eq {
	pc = 0x8316250C; continue 'dispatch;
	}
	// 83162470: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162474: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83162478: 419A0008  beq cr6, 0x83162480
	if ctx.cr[6].eq {
	pc = 0x83162480; continue 'dispatch;
	}
	// 8316247C: 4BFFD94D  bl 0x8315fdc8
	ctx.lr = 0x83162480;
	sub_8315FDC8(ctx, base);
	// 83162480: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162484: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162488: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316248C: 7D484850  subf r10, r8, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83162490: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162494: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83162498: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316249C: 7C863850  subf r4, r6, r7
	ctx.r[4].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 831624A0: 7C654B96  divwu r3, r5, r9
	ctx.r[3].u32 = ctx.r[5].u32 / ctx.r[9].u32;
	// 831624A4: 7D644B96  divwu r11, r4, r9
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[9].u32;
	// 831624A8: 7D4349D6  mullw r10, r3, r9
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831624AC: 7D2B49D6  mullw r9, r11, r9
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831624B0: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 831624B4: 7D092050  subf r8, r9, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 831624B8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831624BC: 409A001C  bne cr6, 0x831624d8
	if !ctx.cr[6].eq {
	pc = 0x831624D8; continue 'dispatch;
	}
	// 831624C0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831624C4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831624C8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831624CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831624D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831624D4: 48000014  b 0x831624e8
	pc = 0x831624E8; continue 'dispatch;
	// 831624D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831624DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831624E0: 388B67AC  addi r4, r11, 0x67ac
	ctx.r[4].s64 = ctx.r[11].s64 + 26540;
	// 831624E4: 4BFFD635  bl 0x8315fb18
	ctx.lr = 0x831624E8;
	sub_8315FB18(ctx, base);
	// 831624E8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 831624EC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831624F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831624F4: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831624F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831624FC: 913F002C  stw r9, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 83162500: 419A00CC  beq cr6, 0x831625cc
	if ctx.cr[6].eq {
	pc = 0x831625CC; continue 'dispatch;
	}
	// 83162504: 4BFFD8DD  bl 0x8315fde0
	ctx.lr = 0x83162508;
	sub_8315FDE0(ctx, base);
	// 83162508: 480000C4  b 0x831625cc
	pc = 0x831625CC; continue 'dispatch;
	// 8316250C: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 83162510: 409A00A0  bne cr6, 0x831625b0
	if !ctx.cr[6].eq {
	pc = 0x831625B0; continue 'dispatch;
	}
	// 83162514: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162518: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316251C: 419A0008  beq cr6, 0x83162524
	if ctx.cr[6].eq {
	pc = 0x83162524; continue 'dispatch;
	}
	// 83162520: 4BFFD8A9  bl 0x8315fdc8
	ctx.lr = 0x83162524;
	sub_8315FDC8(ctx, base);
	// 83162524: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83162528: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316252C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83162530: 7D484850  subf r10, r8, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83162534: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162538: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316253C: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83162540: 7C863850  subf r4, r6, r7
	ctx.r[4].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 83162544: 7C654B96  divwu r3, r5, r9
	ctx.r[3].u32 = ctx.r[5].u32 / ctx.r[9].u32;
	// 83162548: 7D644B96  divwu r11, r4, r9
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[9].u32;
	// 8316254C: 7D4349D6  mullw r10, r3, r9
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83162550: 7D2B49D6  mullw r9, r11, r9
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83162554: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 83162558: 7D092050  subf r8, r9, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 8316255C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83162560: 409A001C  bne cr6, 0x8316257c
	if !ctx.cr[6].eq {
	pc = 0x8316257C; continue 'dispatch;
	}
	// 83162564: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83162568: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316256C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162570: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83162574: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83162578: 48000014  b 0x8316258c
	pc = 0x8316258C; continue 'dispatch;
	// 8316257C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83162580: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162584: 388B67A0  addi r4, r11, 0x67a0
	ctx.r[4].s64 = ctx.r[11].s64 + 26528;
	// 83162588: 4BFFD591  bl 0x8315fb18
	ctx.lr = 0x8316258C;
	sub_8315FB18(ctx, base);
	// 8316258C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83162590: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162594: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162598: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8316259C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831625A0: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 831625A4: 419A0028  beq cr6, 0x831625cc
	if ctx.cr[6].eq {
	pc = 0x831625CC; continue 'dispatch;
	}
	// 831625A8: 4BFFD839  bl 0x8315fde0
	ctx.lr = 0x831625AC;
	sub_8315FDE0(ctx, base);
	// 831625AC: 48000020  b 0x831625cc
	pc = 0x831625CC; continue 'dispatch;
	// 831625B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831625B4: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831625B8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831625BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831625C0: 388A6794  addi r4, r10, 0x6794
	ctx.r[4].s64 = ctx.r[10].s64 + 26516;
	// 831625C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831625C8: 4BFFD551  bl 0x8315fb18
	ctx.lr = 0x831625CC;
	sub_8315FB18(ctx, base);
	// 831625CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831625D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831625D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831625D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831625DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831625E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831625E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831625E8 size=60
    let mut pc: u32 = 0x831625E8;
    'dispatch: loop {
        match pc {
            0x831625E8 => {
    //   block [0x831625E8..0x83162624)
	// 831625E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831625EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831625F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831625F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831625F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831625FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83162600: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83162604: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83162608: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8316260C: 38E967B8  addi r7, r9, 0x67b8
	ctx.r[7].s64 = ctx.r[9].s64 + 26552;
	// 83162610: 4BFFF721  bl 0x83161d30
	ctx.lr = 0x83162614;
	sub_83161D30(ctx, base);
	// 83162614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316261C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162628 size=52
    let mut pc: u32 = 0x83162628;
    'dispatch: loop {
        match pc {
            0x83162628 => {
    //   block [0x83162628..0x8316265C)
	// 83162628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316262C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83162634: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83162638: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8316263C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83162640: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83162644: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83162648: 4BFFF861  bl 0x83161ea8
	ctx.lr = 0x8316264C;
	sub_83161EA8(ctx, base);
	// 8316264C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83162650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83162654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83162660 size=1384
    let mut pc: u32 = 0x83162660;
    'dispatch: loop {
        match pc {
            0x83162660 => {
    //   block [0x83162660..0x83162BC8)
	// 83162660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83162664: 48045AE9  bl 0x831a814c
	ctx.lr = 0x83162668;
	sub_831A8130(ctx, base);
	// 83162668: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316266C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162670: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83162674: 419A002C  beq cr6, 0x831626a0
	if ctx.cr[6].eq {
	pc = 0x831626A0; continue 'dispatch;
	}
	// 83162678: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8316267C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83162680: 388A68AC  addi r4, r10, 0x68ac
	ctx.r[4].s64 = ctx.r[10].s64 + 26796;
	// 83162684: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83162688: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8316268C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162690: 4BFFD3B9  bl 0x8315fa48
	ctx.lr = 0x83162694;
	sub_8315FA48(ctx, base);
	// 83162694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162698: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8316269C: 48045B00  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 831626A0: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831626A4: 554AD97E  srwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831626A8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 831626AC: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831626B0: 7F664396  divwu r27, r6, r8
	ctx.r[27].u32 = ctx.r[6].u32 / ctx.r[8].u32;
	// 831626B4: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831626B8: 41980008  blt cr6, 0x831626c0
	if ctx.cr[6].lt {
	pc = 0x831626C0; continue 'dispatch;
	}
	// 831626BC: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 831626C0: 3964001F  addi r11, r4, 0x1f
	ctx.r[11].s64 = ctx.r[4].s64 + 31;
	// 831626C4: 556BD97E  srwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831626C8: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831626CC: 41980008  blt cr6, 0x831626d4
	if ctx.cr[6].lt {
	pc = 0x831626D4; continue 'dispatch;
	}
	// 831626D0: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 831626D4: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831626D8: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 831626DC: A103000A  lhz r8, 0xa(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 831626E0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831626E4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 831626E8: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 831626EC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 831626F0: C9A10050  lfd f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831626F4: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 831626F8: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831626FC: FD406E9C  fcfid f10, f13
	ctx.f[10].f64 = (ctx.f[13].s64 as f64);
	// 83162700: C00AE08C  lfs f0, -0x1f74(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8052 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83162704: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 83162708: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8316270C: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 83162710: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83162714: EDA90032  fmuls f13, f9, f0
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162718: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 8316271C: 419A0488  beq cr6, 0x83162ba4
	if ctx.cr[6].eq {
	pc = 0x83162BA4; continue 'dispatch;
	}
	// 83162720: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 83162724: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83162728: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316272C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83162730: 394B6868  addi r10, r11, 0x6868
	ctx.r[10].s64 = ctx.r[11].s64 + 26728;
	// 83162734: C18868A8  lfs f12, 0x68a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(26792 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83162738: C1667490  lfs f11, 0x7490(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(29840 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8316273C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162740: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83162744: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83162748: 419A044C  beq cr6, 0x83162b94
	if ctx.cr[6].eq {
	pc = 0x83162B94; continue 'dispatch;
	}
	// 8316274C: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 83162750: 38C30030  addi r6, r3, 0x30
	ctx.r[6].s64 = ctx.r[3].s64 + 48;
	// 83162754: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162758: 89050001  lbz r8, 1(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 8316275C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83162760: 7D684378  or r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 83162764: 550B043E  clrlwi r11, r8, 0x10
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 83162768: 55680420  rlwinm r8, r11, 0, 0x10, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8316276C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83162770: 409A0434  bne cr6, 0x83162ba4
	if !ctx.cr[6].eq {
	pc = 0x83162BA4; continue 'dispatch;
	}
	// 83162774: A3C30002  lhz r30, 2(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83162778: 39050002  addi r8, r5, 2
	ctx.r[8].s64 = ctx.r[5].s64 + 2;
	// 8316277C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83162780: A3430004  lhz r26, 4(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162784: 7FC50734  extsh r5, r30
	ctx.r[5].s64 = ctx.r[30].s16 as i64;
	// 83162788: A3230006  lhz r25, 6(r3)
	ctx.r[25].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8316278C: 7FD80734  extsh r24, r30
	ctx.r[24].s64 = ctx.r[30].s16 as i64;
	// 83162790: 7CAB5A78  xor r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 ^ ctx.r[11].u64;
	// 83162794: 7F450734  extsh r5, r26
	ctx.r[5].s64 = ctx.r[26].s16 as i64;
	// 83162798: 557E04FE  clrlwi r30, r11, 0x13
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 8316279C: 7CA5C1D6  mullw r5, r5, r24
	ctx.r[5].s64 = (ctx.r[5].s32 as i64) * (ctx.r[24].s32 as i64);
	// 831627A0: 7F2B0734  extsh r11, r25
	ctx.r[11].s64 = ctx.r[25].s16 as i64;
	// 831627A4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831627A8: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 831627AC: 7FC50734  extsh r5, r30
	ctx.r[5].s64 = ctx.r[30].s16 as i64;
	// 831627B0: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 831627B4: F8A10050  std r5, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u64 ) };
	// 831627B8: C9210050  lfd f9, 0x50(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831627BC: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831627C0: C1060000  lfs f8, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 831627C4: 80A80000  lwz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831627C8: FCC04E9C  fcfid f6, f9
	ctx.f[6].f64 = (ctx.f[9].s64 as f64);
	// 831627CC: C146FFFC  lfs f10, -4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 831627D0: FC803018  frsp f4, f6
	ctx.f[4].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831627D4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831627D8: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 831627DC: 54BE36BA  rlwinm r30, r5, 6, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x03FFFFFFu64;
	// 831627E0: 54BA56BA  rlwinm r26, r5, 0xa, 0x1a, 0x1d
	ctx.r[26].u64 = ctx.r[5].u32 as u64 & 0x003FFFFFu64;
	// 831627E4: 54B976BA  rlwinm r25, r5, 0xe, 0x1a, 0x1d
	ctx.r[25].u64 = ctx.r[5].u32 as u64 & 0x0003FFFFu64;
	// 831627E8: 54B896BA  rlwinm r24, r5, 0x12, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[5].u32 as u64 & 0x00003FFFu64;
	// 831627EC: 54B7D6BA  rlwinm r23, r5, 0x1a, 0x1a, 0x1d
	ctx.r[23].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 831627F0: 7CBE542E  lfsx f5, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831627F4: 54BEB6BA  rlwinm r30, r5, 0x16, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x000003FFu64;
	// 831627F8: EC6402F2  fmuls f3, f4, f11
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[11].f64) as f32) as f64);
	// 831627FC: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 83162800: 54B6F6BA  rlwinm r22, r5, 0x1e, 0x1a, 0x1d
	ctx.r[22].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 83162804: 54A516BA  rlwinm r5, r5, 2, 0x1a, 0x1d
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 83162808: EC4538FA  fmadds f2, f5, f3, f7
	ctx.f[2].f64 = (((ctx.f[5].f64 * ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64);
	// 8316280C: EC2A103A  fmadds f1, f10, f0, f2
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162810: D02B0000  stfs f1, 0(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83162814: 7D1A542E  lfsx f8, r26, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83162818: ECE10032  fmuls f7, f1, f0
	ctx.f[7].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8316281C: FD200890  fmr f9, f1
	ctx.f[9].f64 = ctx.f[1].f64;
	// 83162820: ECC838FA  fmadds f6, f8, f3, f7
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64);
	// 83162824: ECA90372  fmuls f5, f9, f13
	ctx.f[5].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162828: EC8A337A  fmadds f4, f10, f13, f6
	ctx.f[4].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[6].f64) as f32) as f64);
	// 8316282C: D08B0004  stfs f4, 4(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83162830: 7C59542E  lfsx f2, r25, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83162834: ED4228FA  fmadds f10, f2, f3, f5
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162838: ED24503A  fmadds f9, f4, f0, f10
	ctx.f[9].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 8316283C: D12B0008  stfs f9, 8(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83162840: 7D18542E  lfsx f8, r24, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83162844: FC202090  fmr f1, f4
	ctx.f[1].f64 = ctx.f[4].f64;
	// 83162848: ECC90032  fmuls f6, f9, f0
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8316284C: FCE04890  fmr f7, f9
	ctx.f[7].f64 = ctx.f[9].f64;
	// 83162850: ECA830FA  fmadds f5, f8, f3, f6
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162854: EC812B7A  fmadds f4, f1, f13, f5
	ctx.f[4].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162858: D08B000C  stfs f4, 0xc(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8316285C: 7C5E542E  lfsx f2, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83162860: ED440032  fmuls f10, f4, f0
	ctx.f[10].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162864: FC202090  fmr f1, f4
	ctx.f[1].f64 = ctx.f[4].f64;
	// 83162868: ED2250FA  fmadds f9, f2, f3, f10
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[3].f64 + ctx.f[10].f64) as f32) as f64);
	// 8316286C: ED0D49FA  fmadds f8, f13, f7, f9
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162870: D10B0010  stfs f8, 0x10(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83162874: 7CF7542E  lfsx f7, r23, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83162878: ECA80032  fmuls f5, f8, f0
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 8316287C: FCC04090  fmr f6, f8
	ctx.f[6].f64 = ctx.f[8].f64;
	// 83162880: EC8728FA  fmadds f4, f7, f3, f5
	ctx.f[4].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162884: EC41237A  fmadds f2, f1, f13, f4
	ctx.f[2].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 83162888: D04B0014  stfs f2, 0x14(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8316288C: 7C36542E  lfsx f1, r22, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83162890: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162894: FD401090  fmr f10, f2
	ctx.f[10].f64 = ctx.f[2].f64;
	// 83162898: ED0148FA  fmadds f8, f1, f3, f9
	ctx.f[8].f64 = (((ctx.f[1].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 8316289C: ECED41BA  fmadds f7, f13, f6, f8
	ctx.f[7].f64 = (((ctx.f[13].f64 * ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64);
	// 831628A0: D0EB0018  stfs f7, 0x18(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 831628A4: 7C85542E  lfsx f4, r5, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831628A8: ECA70032  fmuls f5, f7, f0
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 831628AC: FCC03890  fmr f6, f7
	ctx.f[6].f64 = ctx.f[7].f64;
	// 831628B0: EC4428FA  fmadds f2, f4, f3, f5
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 831628B4: EC2A137A  fmadds f1, f10, f13, f2
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[2].f64) as f32) as f64);
	// 831628B8: FD400890  fmr f10, f1
	ctx.f[10].f64 = ctx.f[1].f64;
	// 831628BC: D02B001C  stfs f1, 0x1c(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 831628C0: 80A80004  lwz r5, 4(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 831628C4: 54BE36BA  rlwinm r30, r5, 6, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x03FFFFFFu64;
	// 831628C8: ED2A0032  fmuls f9, f10, f0
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 831628CC: 7D1E542E  lfsx f8, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 831628D0: 54BE56BA  rlwinm r30, r5, 0xa, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x003FFFFFu64;
	// 831628D4: ECE848FA  fmadds f7, f8, f3, f9
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 831628D8: 54BA76BA  rlwinm r26, r5, 0xe, 0x1a, 0x1d
	ctx.r[26].u64 = ctx.r[5].u32 as u64 & 0x0003FFFFu64;
	// 831628DC: 54B996BA  rlwinm r25, r5, 0x12, 0x1a, 0x1d
	ctx.r[25].u64 = ctx.r[5].u32 as u64 & 0x00003FFFu64;
	// 831628E0: 54B8B6BA  rlwinm r24, r5, 0x16, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[5].u32 as u64 & 0x000003FFu64;
	// 831628E4: 54B7D6BA  rlwinm r23, r5, 0x1a, 0x1a, 0x1d
	ctx.r[23].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 831628E8: 54B6F6BA  rlwinm r22, r5, 0x1e, 0x1a, 0x1d
	ctx.r[22].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 831628EC: 54A516BA  rlwinm r5, r5, 2, 0x1a, 0x1d
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 831628F0: ECCD39BA  fmadds f6, f13, f6, f7
	ctx.f[6].f64 = (((ctx.f[13].f64 * ctx.f[6].f64 + ctx.f[7].f64) as f32) as f64);
	// 831628F4: D0CB0020  stfs f6, 0x20(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 831628F8: 7CBE542E  lfsx f5, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831628FC: FC803090  fmr f4, f6
	ctx.f[4].f64 = ctx.f[6].f64;
	// 83162900: EC440032  fmuls f2, f4, f0
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162904: EC2510FA  fmadds f1, f5, f3, f2
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[3].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162908: ED4A0B7A  fmadds f10, f10, f13, f1
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64);
	// 8316290C: D14B0024  stfs f10, 0x24(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 83162910: ED0A0032  fmuls f8, f10, f0
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162914: FCC05090  fmr f6, f10
	ctx.f[6].f64 = ctx.f[10].f64;
	// 83162918: 7D3A542E  lfsx f9, r26, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8316291C: ECE940FA  fmadds f7, f9, f3, f8
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[3].f64 + ctx.f[8].f64) as f32) as f64);
	// 83162920: ECA43B7A  fmadds f5, f4, f13, f7
	ctx.f[5].f64 = (((ctx.f[4].f64 * ctx.f[13].f64 + ctx.f[7].f64) as f32) as f64);
	// 83162924: D0AB0028  stfs f5, 0x28(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 83162928: 7C99542E  lfsx f4, r25, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8316292C: EC250032  fmuls f1, f5, f0
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162930: FC402890  fmr f2, f5
	ctx.f[2].f64 = ctx.f[5].f64;
	// 83162934: ED4408FA  fmadds f10, f4, f3, f1
	ctx.f[10].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 83162938: ED26537A  fmadds f9, f6, f13, f10
	ctx.f[9].f64 = (((ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 8316293C: D12B002C  stfs f9, 0x2c(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 83162940: 7D18542E  lfsx f8, r24, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83162944: ECC90032  fmuls f6, f9, f0
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162948: FCE04890  fmr f7, f9
	ctx.f[7].f64 = ctx.f[9].f64;
	// 8316294C: ECA830FA  fmadds f5, f8, f3, f6
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162950: EC822B7A  fmadds f4, f2, f13, f5
	ctx.f[4].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162954: D08B0030  stfs f4, 0x30(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 83162958: 7C37542E  lfsx f1, r23, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8316295C: EC440032  fmuls f2, f4, f0
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162960: FD002090  fmr f8, f4
	ctx.f[8].f64 = ctx.f[4].f64;
	// 83162964: ED4110FA  fmadds f10, f1, f3, f2
	ctx.f[10].f64 = (((ctx.f[1].f64 * ctx.f[3].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162968: ED2D51FA  fmadds f9, f13, f7, f10
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[7].f64 + ctx.f[10].f64) as f32) as f64);
	// 8316296C: D12B0034  stfs f9, 0x34(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 83162970: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162974: FCC04890  fmr f6, f9
	ctx.f[6].f64 = ctx.f[9].f64;
	// 83162978: 7CB6542E  lfsx f5, r22, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8316297C: EC8538FA  fmadds f4, f5, f3, f7
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64);
	// 83162980: EC49203A  fmadds f2, f9, f0, f4
	ctx.f[2].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[4].f64) as f32) as f64);
	// 83162984: D04B0038  stfs f2, 0x38(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 83162988: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 8316298C: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 83162990: 7D45542E  lfsx f10, r5, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83162994: ED0A48FA  fmadds f8, f10, f3, f9
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162998: ECE6437A  fmadds f7, f6, f13, f8
	ctx.f[7].f64 = (((ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[8].f64) as f32) as f64);
	// 8316299C: D0EB003C  stfs f7, 0x3c(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 831629A0: ECA10372  fmuls f5, f1, f13
	ctx.f[5].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 831629A4: 80A80008  lwz r5, 8(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 831629A8: 54BE36BA  rlwinm r30, r5, 6, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x03FFFFFFu64;
	// 831629AC: 7C9E542E  lfsx f4, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831629B0: EC4428FA  fmadds f2, f4, f3, f5
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 831629B4: FCC03890  fmr f6, f7
	ctx.f[6].f64 = ctx.f[7].f64;
	// 831629B8: 54BE56BA  rlwinm r30, r5, 0xa, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x003FFFFFu64;
	// 831629BC: 54BA76BA  rlwinm r26, r5, 0xe, 0x1a, 0x1d
	ctx.r[26].u64 = ctx.r[5].u32 as u64 & 0x0003FFFFu64;
	// 831629C0: 54B996BA  rlwinm r25, r5, 0x12, 0x1a, 0x1d
	ctx.r[25].u64 = ctx.r[5].u32 as u64 & 0x00003FFFu64;
	// 831629C4: 54B8B6BA  rlwinm r24, r5, 0x16, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[5].u32 as u64 & 0x000003FFu64;
	// 831629C8: 54B7D6BA  rlwinm r23, r5, 0x1a, 0x1a, 0x1d
	ctx.r[23].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 831629CC: EC26103A  fmadds f1, f6, f0, f2
	ctx.f[1].f64 = (((ctx.f[6].f64 * ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64);
	// 831629D0: D02B0040  stfs f1, 0x40(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 831629D4: 7D1E542E  lfsx f8, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 831629D8: 54BEF6BA  rlwinm r30, r5, 0x1e, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 831629DC: FD400890  fmr f10, f1
	ctx.f[10].f64 = ctx.f[1].f64;
	// 831629E0: ED2A0032  fmuls f9, f10, f0
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 831629E4: ECE848FA  fmadds f7, f8, f3, f9
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 831629E8: ECC63B7A  fmadds f6, f6, f13, f7
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[7].f64) as f32) as f64);
	// 831629EC: D0CB0044  stfs f6, 0x44(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 831629F0: 7C9A542E  lfsx f4, r26, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831629F4: ECA60032  fmuls f5, f6, f0
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 831629F8: FC403090  fmr f2, f6
	ctx.f[2].f64 = ctx.f[6].f64;
	// 831629FC: EC2428FA  fmadds f1, f4, f3, f5
	ctx.f[1].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162A00: 54A516BA  rlwinm r5, r5, 2, 0x1a, 0x1d
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 83162A04: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83162A08: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83162A0C: ED4A0B7A  fmadds f10, f10, f13, f1
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64);
	// 83162A10: D14B0048  stfs f10, 0x48(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 83162A14: 7D39542E  lfsx f9, r25, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83162A18: ECEA0032  fmuls f7, f10, f0
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162A1C: FD005090  fmr f8, f10
	ctx.f[8].f64 = ctx.f[10].f64;
	// 83162A20: ECC938FA  fmadds f6, f9, f3, f7
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64);
	// 83162A24: ECA2337A  fmadds f5, f2, f13, f6
	ctx.f[5].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162A28: D0AB004C  stfs f5, 0x4c(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 83162A2C: 7D58542E  lfsx f10, r24, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83162A30: EC250032  fmuls f1, f5, f0
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162A34: FC802890  fmr f4, f5
	ctx.f[4].f64 = ctx.f[5].f64;
	// 83162A38: ED2A08FA  fmadds f9, f10, f3, f1
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 83162A3C: EC440372  fmuls f2, f4, f13
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162A40: ED0D4A3A  fmadds f8, f13, f8, f9
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162A44: D10B0050  stfs f8, 0x50(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83162A48: 7CF7542E  lfsx f7, r23, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83162A4C: ECC710FA  fmadds f6, f7, f3, f2
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162A50: EC88303A  fmadds f4, f8, f0, f6
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[0].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162A54: D08B0054  stfs f4, 0x54(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 83162A58: 7C5E542E  lfsx f2, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83162A5C: FCA04090  fmr f5, f8
	ctx.f[5].f64 = ctx.f[8].f64;
	// 83162A60: ED440032  fmuls f10, f4, f0
	ctx.f[10].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162A64: FC202090  fmr f1, f4
	ctx.f[1].f64 = ctx.f[4].f64;
	// 83162A68: ED2250FA  fmadds f9, f2, f3, f10
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[3].f64 + ctx.f[10].f64) as f32) as f64);
	// 83162A6C: ED054B7A  fmadds f8, f5, f13, f9
	ctx.f[8].f64 = (((ctx.f[5].f64 * ctx.f[13].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162A70: D10B0058  stfs f8, 0x58(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 83162A74: 7CE5542E  lfsx f7, r5, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83162A78: ECA80032  fmuls f5, f8, f0
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162A7C: FCC04090  fmr f6, f8
	ctx.f[6].f64 = ctx.f[8].f64;
	// 83162A80: EC8728FA  fmadds f4, f7, f3, f5
	ctx.f[4].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162A84: EC41237A  fmadds f2, f1, f13, f4
	ctx.f[2].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 83162A88: D04B005C  stfs f2, 0x5c(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 83162A8C: 80A8000C  lwz r5, 0xc(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162A90: 54BE36BA  rlwinm r30, r5, 6, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x03FFFFFFu64;
	// 83162A94: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 83162A98: 7D3E542E  lfsx f9, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83162A9C: 54BE56BA  rlwinm r30, r5, 0xa, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x003FFFFFu64;
	// 83162AA0: 54BA76BA  rlwinm r26, r5, 0xe, 0x1a, 0x1d
	ctx.r[26].u64 = ctx.r[5].u32 as u64 & 0x0003FFFFu64;
	// 83162AA4: 54B996BA  rlwinm r25, r5, 0x12, 0x1a, 0x1d
	ctx.r[25].u64 = ctx.r[5].u32 as u64 & 0x00003FFFu64;
	// 83162AA8: 54B8B6BA  rlwinm r24, r5, 0x16, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[5].u32 as u64 & 0x000003FFu64;
	// 83162AAC: 54B7D6BA  rlwinm r23, r5, 0x1a, 0x1a, 0x1d
	ctx.r[23].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 83162AB0: 54B6F6BA  rlwinm r22, r5, 0x1e, 0x1a, 0x1d
	ctx.r[22].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 83162AB4: 54B516BA  rlwinm r21, r5, 2, 0x1a, 0x1d
	ctx.r[21].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 83162AB8: 38A80010  addi r5, r8, 0x10
	ctx.r[5].s64 = ctx.r[8].s64 + 16;
	// 83162ABC: ED410032  fmuls f10, f1, f0
	ctx.f[10].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162AC0: ED0950FA  fmadds f8, f9, f3, f10
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[3].f64 + ctx.f[10].f64) as f32) as f64);
	// 83162AC4: ECED41BA  fmadds f7, f13, f6, f8
	ctx.f[7].f64 = (((ctx.f[13].f64 * ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64);
	// 83162AC8: D0EB0060  stfs f7, 0x60(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 83162ACC: 7C9E542E  lfsx f4, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83162AD0: FCC03890  fmr f6, f7
	ctx.f[6].f64 = ctx.f[7].f64;
	// 83162AD4: ECA60032  fmuls f5, f6, f0
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162AD8: EC4428FA  fmadds f2, f4, f3, f5
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162ADC: EC21137A  fmadds f1, f1, f13, f2
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162AE0: D02B0064  stfs f1, 0x64(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 83162AE4: 7D1A542E  lfsx f8, r26, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83162AE8: ED210032  fmuls f9, f1, f0
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162AEC: FD400890  fmr f10, f1
	ctx.f[10].f64 = ctx.f[1].f64;
	// 83162AF0: ECE848FA  fmadds f7, f8, f3, f9
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162AF4: ECC63B7A  fmadds f6, f6, f13, f7
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[7].f64) as f32) as f64);
	// 83162AF8: D0CB0068  stfs f6, 0x68(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 83162AFC: 7C99542E  lfsx f4, r25, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83162B00: ECA60032  fmuls f5, f6, f0
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162B04: FC203090  fmr f1, f6
	ctx.f[1].f64 = ctx.f[6].f64;
	// 83162B08: EC4428FA  fmadds f2, f4, f3, f5
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64);
	// 83162B0C: ED210372  fmuls f9, f1, f13
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162B10: ED4D12BA  fmadds f10, f13, f10, f2
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[2].f64) as f32) as f64);
	// 83162B14: D14B006C  stfs f10, 0x6c(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 83162B18: 7D18542E  lfsx f8, r24, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83162B1C: ECC848FA  fmadds f6, f8, f3, f9
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 83162B20: ECAA303A  fmadds f5, f10, f0, f6
	ctx.f[5].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162B24: D0AB0070  stfs f5, 0x70(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 83162B28: 7C57542E  lfsx f2, r23, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83162B2C: FCE05090  fmr f7, f10
	ctx.f[7].f64 = ctx.f[10].f64;
	// 83162B30: EC250032  fmuls f1, f5, f0
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162B34: FC802890  fmr f4, f5
	ctx.f[4].f64 = ctx.f[5].f64;
	// 83162B38: ED4208FA  fmadds f10, f2, f3, f1
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 83162B3C: ED27537A  fmadds f9, f7, f13, f10
	ctx.f[9].f64 = (((ctx.f[7].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 83162B40: D12B0074  stfs f9, 0x74(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 83162B44: ED090032  fmuls f8, f9, f0
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162B48: FCA04890  fmr f5, f9
	ctx.f[5].f64 = ctx.f[9].f64;
	// 83162B4C: 7CF6542E  lfsx f7, r22, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83162B50: ECC740FA  fmadds f6, f7, f3, f8
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[8].f64) as f32) as f64);
	// 83162B54: EC84337A  fmadds f4, f4, f13, f6
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[13].f64 + ctx.f[6].f64) as f32) as f64);
	// 83162B58: D08B0078  stfs f4, 0x78(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 83162B5C: EC240032  fmuls f1, f4, f0
	ctx.f[1].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162B60: 7C55542E  lfsx f2, r21, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83162B64: ED4208FA  fmadds f10, f2, f3, f1
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 83162B68: ED2D517A  fmadds f9, f13, f5, f10
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[10].f64) as f32) as f64);
	// 83162B6C: D12B007C  stfs f9, 0x7c(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 83162B70: ED09602A  fadds f8, f9, f12
	ctx.f[8].f64 = ((ctx.f[9].f64 + ctx.f[12].f64) as f32) as f64;
	// 83162B74: D106FFFC  stfs f8, -4(r6)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 83162B78: C0EB0078  lfs f7, 0x78(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83162B7C: ECC7602A  fadds f6, f7, f12
	ctx.f[6].f64 = ((ctx.f[7].f64 + ctx.f[12].f64) as f32) as f64;
	// 83162B80: D0C60000  stfs f6, 0(r6)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83162B84: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 83162B88: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162B8C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83162B90: 4198FBC4  blt cr6, 0x83162754
	if ctx.cr[6].lt {
	pc = 0x83162754; continue 'dispatch;
	}
	// 83162B94: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83162B98: 3BBD0080  addi r29, r29, 0x80
	ctx.r[29].s64 = ctx.r[29].s64 + 128;
	// 83162B9C: 7F1CD840  cmplw cr6, r28, r27
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83162BA0: 4198FB9C  blt cr6, 0x8316273c
	if ctx.cr[6].lt {
	pc = 0x8316273C; continue 'dispatch;
	}
	// 83162BA4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162BA8: 57832834  slwi r3, r28, 5
	ctx.r[3].u32 = ctx.r[28].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83162BAC: 7D6BE1D6  mullw r11, r11, r28
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83162BB0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83162BB4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83162BB8: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83162BBC: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83162BC0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83162BC4: 480455D8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162BC8 size=12
    let mut pc: u32 = 0x83162BC8;
    'dispatch: loop {
        match pc {
            0x83162BC8 => {
    //   block [0x83162BC8..0x83162BD4)
	// 83162BC8: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 83162BCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83162BD0: 48045610  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162BD8 size=8
    let mut pc: u32 = 0x83162BD8;
    'dispatch: loop {
        match pc {
            0x83162BD8 => {
    //   block [0x83162BD8..0x83162BE0)
	// 83162BD8: 98830000  stb r4, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83162BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83162BE0 size=108
    let mut pc: u32 = 0x83162BE0;
    'dispatch: loop {
        match pc {
            0x83162BE0 => {
    //   block [0x83162BE0..0x83162C4C)
	// 83162BE0: 7CC90734  extsh r9, r6
	ctx.r[9].s64 = ctx.r[6].s16 as i64;
	// 83162BE4: 7CA80734  extsh r8, r5
	ctx.r[8].s64 = ctx.r[5].s16 as i64;
	// 83162BE8: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 83162BEC: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162BF0: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 83162BF4: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162BF8: FD60069C  fcfid f11, f0
	ctx.f[11].f64 = (ctx.f[0].s64 as f64);
	// 83162BFC: 38E40003  addi r7, r4, 3
	ctx.r[7].s64 = ctx.r[4].s64 + 3;
	// 83162C00: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 83162C04: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83162C08: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83162C0C: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83162C10: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83162C14: 38840006  addi r4, r4, 6
	ctx.r[4].s64 = ctx.r[4].s64 + 6;
	// 83162C18: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83162C1C: 7CEB1A14  add r7, r11, r3
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83162C20: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83162C24: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83162C28: 7CA81B2E  sthx r5, r8, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[5].u16) };
	// 83162C2C: C0094E28  lfs f0, 0x4e28(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20008 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83162C30: FD406018  frsp f10, f12
	ctx.f[10].f64 = (ctx.f[12].f64 as f32) as f64;
	// 83162C34: B0C7000E  sth r6, 0xe(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(14 as u32), ctx.r[6].u16 ) };
	// 83162C38: ECE90032  fmuls f7, f9, f0
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162C3C: 7CEA1D2E  stfsx f7, r10, r3
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 83162C40: ED0A0032  fmuls f8, f10, f0
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162C44: D10B002C  stfs f8, 0x2c(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 83162C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83162C50 size=224
    let mut pc: u32 = 0x83162C50;
    'dispatch: loop {
        match pc {
            0x83162C50 => {
    //   block [0x83162C50..0x83162D30)
	// 83162C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83162C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162C58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83162C5C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 83162C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83162C64: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 83162C68: 78AA0020  clrldi r10, r5, 0x20
	ctx.r[10].u64 = ctx.r[5].u64 & 0x00000000FFFFFFFFu64;
	// 83162C6C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83162C70: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83162C74: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83162C78: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83162C7C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83162C80: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 83162C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83162C88: C1A92490  lfs f13, 0x2490(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9360 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83162C8C: C808AA10  lfd f0, -0x55f0(r8)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-22000 as u32) ) };
	// 83162C90: EFE0002C  fsqrts f31, f0
	ctx.f[31].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 83162C94: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 83162C98: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 83162C9C: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 83162CA0: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 83162CA4: ECC80372  fmuls f6, f8, f13
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162CA8: EC263824  fdivs f1, f6, f7
	ctx.f[1].f64 = ((ctx.f[6].f64 / ctx.f[7].f64) as f32) as f64;
	// 83162CAC: 480461FD  bl 0x831a8ea8
	ctx.lr = 0x83162CB0;
	sub_831A8EA8(ctx, base);
	// 83162CB0: FCA00818  frsp f5, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83162CB4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 83162CB8: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 83162CBC: 3CA08219  lis r5, -0x7de7
	ctx.r[5].s64 = -2112290816;
	// 83162CC0: C00708A8  lfs f0, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83162CC4: EC9F0028  fsubs f4, f31, f0
	ctx.f[4].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 83162CC8: C0068AD8  lfs f0, -0x7528(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-29992 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83162CCC: C1A51CC0  lfs f13, 0x1cc0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(7360 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83162CD0: EC7F2828  fsubs f3, f31, f5
	ctx.f[3].f64 = (((ctx.f[31].f64 - ctx.f[5].f64) as f32) as f64);
	// 83162CD4: EC432028  fsubs f2, f3, f4
	ctx.f[2].f64 = (((ctx.f[3].f64 - ctx.f[4].f64) as f32) as f64);
	// 83162CD8: EC24182A  fadds f1, f4, f3
	ctx.f[1].f64 = ((ctx.f[4].f64 + ctx.f[3].f64) as f32) as f64;
	// 83162CDC: ED820072  fmuls f12, f2, f1
	ctx.f[12].f64 = (((ctx.f[2].f64 * ctx.f[1].f64) as f32) as f64);
	// 83162CE0: ED60602C  fsqrts f11, f12
	ctx.f[11].f64 = ((ctx.f[12].f64).sqrt() as f32) as f64;
	// 83162CE4: ED435828  fsubs f10, f3, f11
	ctx.f[10].f64 = (((ctx.f[3].f64 - ctx.f[11].f64) as f32) as f64);
	// 83162CE8: ED2A2024  fdivs f9, f10, f4
	ctx.f[9].f64 = ((ctx.f[10].f64 / ctx.f[4].f64) as f32) as f64;
	// 83162CEC: ED090272  fmuls f8, f9, f9
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[9].f64) as f32) as f64);
	// 83162CF0: ECE90032  fmuls f7, f9, f0
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 83162CF4: ECC80372  fmuls f6, f8, f13
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 83162CF8: FCA0381E  fctiwz f5, f7
	ctx.f[5].s64 = if ctx.f[7].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[7].f64.trunc() as i32 as i64 };
	// 83162CFC: D8A10050  stfd f5, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[5].u64 ) };
	// 83162D00: A0810056  lhz r4, 0x56(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83162D04: B09F0008  sth r4, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 83162D08: FC80301E  fctiwz f4, f6
	ctx.f[4].s64 = if ctx.f[6].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[6].f64.trunc() as i32 as i64 };
	// 83162D0C: D8810050  stfd f4, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[4].u64 ) };
	// 83162D10: A0610056  lhz r3, 0x56(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83162D14: B07F000A  sth r3, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[3].u16 ) };
	// 83162D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83162D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83162D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162D24: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83162D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162D30 size=20
    let mut pc: u32 = 0x83162D30;
    'dispatch: loop {
        match pc {
            0x83162D30 => {
    //   block [0x83162D30..0x83162D44)
	// 83162D30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83162D34: 2B040014  cmplwi cr6, r4, 0x14
	ctx.cr[6].compare_u32(ctx.r[4].u32, 20 as u32, &mut ctx.xer);
	// 83162D38: 4098000C  bge cr6, 0x83162d44
	if !ctx.cr[6].lt {
		sub_83162D44(ctx, base);
		return;
	}
	// 83162D3C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83162D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162D44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162D44 size=20
    let mut pc: u32 = 0x83162D44;
    'dispatch: loop {
        match pc {
            0x83162D44 => {
    //   block [0x83162D44..0x83162D58)
	// 83162D44: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162D48: 2B0A8000  cmplwi cr6, r10, 0x8000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32768 as u32, &mut ctx.xer);
	// 83162D4C: 419A000C  beq cr6, 0x83162d58
	if ctx.cr[6].eq {
		sub_83162D58(ctx, base);
		return;
	}
	// 83162D50: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83162D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162D58 size=40
    let mut pc: u32 = 0x83162D58;
    'dispatch: loop {
        match pc {
            0x83162D58 => {
    //   block [0x83162D58..0x83162D80)
	// 83162D58: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83162D5C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 83162D60: 2F090010  cmpwi cr6, r9, 0x10
	ctx.cr[6].compare_i32(ctx.r[9].s32, 16, &mut ctx.xer);
	// 83162D64: 4198FFD8  blt cr6, 0x83162d3c
	if ctx.cr[6].lt {
		sub_83162D30(ctx, base);
		return;
	}
	// 83162D68: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 83162D6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162D70: 99450000  stb r10, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83162D74: 892B0013  lbz r9, 0x13(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 83162D78: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83162D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162D80 size=288
    let mut pc: u32 = 0x83162D80;
    'dispatch: loop {
        match pc {
            0x83162D80 => {
    //   block [0x83162D80..0x83162EA0)
	// 83162D80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83162D84: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 83162D88: 4098000C  bge cr6, 0x83162d94
	if !ctx.cr[6].lt {
	pc = 0x83162D94; continue 'dispatch;
	}
	// 83162D8C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83162D90: 4E800020  blr
	return;
	// 83162D94: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162D98: 88830001  lbz r4, 1(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83162D9C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83162DA0: 7D642378  or r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 83162DA4: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 83162DA8: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 83162DAC: 419A0010  beq cr6, 0x83162dbc
	if ctx.cr[6].eq {
	pc = 0x83162DBC; continue 'dispatch;
	}
	// 83162DB0: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83162DB4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83162DB8: 4E800020  blr
	return;
	// 83162DBC: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83162DC0: 88830003  lbz r4, 3(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 83162DC4: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83162DC8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83162DCC: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 83162DD0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83162DD4: B0850000  sth r4, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83162DD8: 88A30004  lbz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83162DDC: 98A60000  stb r5, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83162DE0: 88830005  lbz r4, 5(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 83162DE4: 98880000  stb r4, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83162DE8: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 83162DEC: 99670000  stb r11, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83162DF0: 88C30007  lbz r6, 7(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 83162DF4: 98C90000  stb r6, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 83162DF8: 88A30009  lbz r5, 9(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 83162DFC: 8883000A  lbz r4, 0xa(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 83162E00: 8963000B  lbz r11, 0xb(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11 as u32) ) } as u64;
	// 83162E04: 89230008  lbz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83162E08: 5526403E  rotlwi r6, r9, 8
	ctx.r[6].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83162E0C: 7CA53378  or r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 83162E10: 54A9402E  slwi r9, r5, 8
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83162E14: 7D262378  or r6, r9, r4
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[4].u64;
	// 83162E18: 54C5402E  slwi r5, r6, 8
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83162E1C: 7CA45B78  or r4, r5, r11
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 83162E20: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83162E24: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 83162E28: 8943000E  lbz r10, 0xe(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 83162E2C: 8923000F  lbz r9, 0xf(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 83162E30: 88C3000C  lbz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83162E34: 54C5403E  rotlwi r5, r6, 8
	ctx.r[5].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 83162E38: 7CA45B78  or r4, r5, r11
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 83162E3C: 5483402E  slwi r3, r4, 8
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83162E40: 7C6B5378  or r11, r3, r10
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[10].u64;
	// 83162E44: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83162E48: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83162E4C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83162E50: 88E70000  lbz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162E54: 7CEB0774  extsb r11, r7
	ctx.r[11].s64 = ctx.r[7].s8 as i64;
	// 83162E58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83162E5C: 409A001C  bne cr6, 0x83162e78
	if !ctx.cr[6].eq {
	pc = 0x83162E78; continue 'dispatch;
	}
	// 83162E60: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83162E64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83162E68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162E6C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83162E70: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83162E74: 4E800020  blr
	return;
	// 83162E78: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162E7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162E80: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83162E84: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83162E88: 390AFFFE  addi r8, r10, -2
	ctx.r[8].s64 = ctx.r[10].s64 + -2;
	// 83162E8C: 55071838  slwi r7, r8, 3
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83162E90: 7CC75BD6  divw r6, r7, r11
	ctx.r[6].s32 = ctx.r[7].s32 / ctx.r[11].s32;
	// 83162E94: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83162E98: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83162E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83162EA0 size=12
    let mut pc: u32 = 0x83162EA0;
    'dispatch: loop {
        match pc {
            0x83162EA0 => {
    //   block [0x83162EA0..0x83162EAC)
	// 83162EA0: 89630007  lbz r11, 7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 83162EA4: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83162EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162EB0 size=252
    let mut pc: u32 = 0x83162EB0;
    'dispatch: loop {
        match pc {
            0x83162EB0 => {
    //   block [0x83162EB0..0x83162FAC)
	// 83162EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83162EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83162EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83162EBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83162EC0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83162EC4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 83162EC8: 38C10051  addi r6, r1, 0x51
	ctx.r[6].s64 = ctx.r[1].s64 + 81;
	// 83162ECC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83162ED0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83162ED4: 4BFFFE5D  bl 0x83162d30
	ctx.lr = 0x83162ED8;
	sub_83162D30(ctx, base);
	// 83162ED8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83162EDC: 419800BC  blt cr6, 0x83162f98
	if ctx.cr[6].lt {
	pc = 0x83162F98; continue 'dispatch;
	}
	// 83162EE0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83162EE4: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 83162EE8: 41980098  blt cr6, 0x83162f80
	if ctx.cr[6].lt {
	pc = 0x83162F80; continue 'dispatch;
	}
	// 83162EEC: A1680002  lhz r11, 2(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 83162EF0: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 83162EF4: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 83162EF8: 4098001C  bge cr6, 0x83162f14
	if !ctx.cr[6].lt {
	pc = 0x83162F14; continue 'dispatch;
	}
	// 83162EFC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83162F00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83162F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83162F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162F10: 4E800020  blr
	return;
	// 83162F14: A1680018  lhz r11, 0x18(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 83162F18: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 83162F1C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83162F20: A148001A  lhz r10, 0x1a(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(26 as u32) ) } as u64;
	// 83162F24: B1470000  sth r10, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83162F28: A128001C  lhz r9, 0x1c(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 83162F2C: B13F0002  sth r9, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83162F30: A0C8001E  lhz r6, 0x1e(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(30 as u32) ) } as u64;
	// 83162F34: B0C70002  sth r6, 2(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 83162F38: 4BFFFF69  bl 0x83162ea0
	ctx.lr = 0x83162F3C;
	sub_83162EA0(ctx, base);
	// 83162F3C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83162F40: 41980054  blt cr6, 0x83162f94
	if ctx.cr[6].lt {
	pc = 0x83162F94; continue 'dispatch;
	}
	// 83162F44: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 83162F48: 4099004C  ble cr6, 0x83162f94
	if !ctx.cr[6].gt {
	pc = 0x83162F94; continue 'dispatch;
	}
	// 83162F4C: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
	// 83162F50: 39470004  addi r10, r7, 4
	ctx.r[10].s64 = ctx.r[7].s64 + 4;
	// 83162F54: 7D07F850  subf r8, r7, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[7].s64;
	// 83162F58: 3923FFFE  addi r9, r3, -2
	ctx.r[9].s64 = ctx.r[3].s64 + -2;
	// 83162F5C: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83162F60: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83162F64: 7CE8532E  sthx r7, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 83162F68: A0CB0002  lhz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83162F6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83162F70: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83162F74: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83162F78: 4082FFE4  bne 0x83162f5c
	if !ctx.cr[0].eq {
	pc = 0x83162F5C; continue 'dispatch;
	}
	// 83162F7C: 48000018  b 0x83162f94
	pc = 0x83162F94; continue 'dispatch;
	// 83162F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83162F84: B1670002  sth r11, 2(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83162F88: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83162F8C: B1670000  sth r11, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83162F90: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83162F94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83162F98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83162F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83162FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83162FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83162FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83162FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83162FB0 size=288
    let mut pc: u32 = 0x83162FB0;
    'dispatch: loop {
        match pc {
            0x83162FB0 => {
    //   block [0x83162FB0..0x831630D0)
	// 83162FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83162FB4: 480451B1  bl 0x831a8164
	ctx.lr = 0x83162FB8;
	sub_831A8130(ctx, base);
	// 83162FB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83162FBC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83162FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83162FC4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83162FC8: 38C10051  addi r6, r1, 0x51
	ctx.r[6].s64 = ctx.r[1].s64 + 81;
	// 83162FCC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83162FD0: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83162FD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83162FD8: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 83162FDC: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 83162FE0: 4BFFFD51  bl 0x83162d30
	ctx.lr = 0x83162FE4;
	sub_83162D30(ctx, base);
	// 83162FE4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83162FE8: 419800E0  blt cr6, 0x831630c8
	if ctx.cr[6].lt {
	pc = 0x831630C8; continue 'dispatch;
	}
	// 83162FEC: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83162FF0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 83162FF4: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 83162FF8: 409A0028  bne cr6, 0x83163020
	if !ctx.cr[6].eq {
	pc = 0x83163020; continue 'dispatch;
	}
	// 83162FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163000: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 83163004: 4BFFFE9D  bl 0x83162ea0
	ctx.lr = 0x83163008;
	sub_83162EA0(ctx, base);
	// 83163008: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 8316300C: 41980014  blt cr6, 0x83163020
	if ctx.cr[6].lt {
	pc = 0x83163020; continue 'dispatch;
	}
	// 83163010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163014: 4BFFFE8D  bl 0x83162ea0
	ctx.lr = 0x83163018;
	sub_83162EA0(ctx, base);
	// 83163018: 3963000D  addi r11, r3, 0xd
	ctx.r[11].s64 = ctx.r[3].s64 + 13;
	// 8316301C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163020: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83163024: 40980010  bge cr6, 0x83163034
	if !ctx.cr[6].lt {
	pc = 0x83163034; continue 'dispatch;
	}
	// 83163028: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316302C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83163030: 48045184  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83163034: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83163038: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8316303C: 7D660734  extsh r6, r11
	ctx.r[6].s64 = ctx.r[11].s16 as i64;
	// 83163040: 7F065040  cmplw cr6, r6, r10
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83163044: 4198FFE4  blt cr6, 0x83163028
	if ctx.cr[6].lt {
	pc = 0x83163028; continue 'dispatch;
	}
	// 83163048: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8316304C: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 83163050: 409A0008  bne cr6, 0x83163058
	if !ctx.cr[6].eq {
	pc = 0x83163058; continue 'dispatch;
	}
	// 83163054: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 83163058: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8316305C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83163060: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 83163064: 913B0000  stw r9, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83163068: 7CCBFA2E  lhzx r6, r11, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8316306C: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 83163070: B0DE0000  sth r6, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83163074: 419A0010  beq cr6, 0x83163084
	if ctx.cr[6].eq {
	pc = 0x83163084; continue 'dispatch;
	}
	// 83163078: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8316307C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83163080: 48045134  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83163084: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83163088: 812100E4  lwz r9, 0xe4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 8316308C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163090: 7CCBFA2E  lhzx r6, r11, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83163094: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83163098: B0C70000  sth r6, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 8316309C: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831630A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831630A4: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 831630A8: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831630AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831630B0: 909D0000  stw r4, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831630B4: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831630B8: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831630BC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831630C0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831630C4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831630C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831630CC: 480450E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831630D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831630D0 size=376
    let mut pc: u32 = 0x831630D0;
    'dispatch: loop {
        match pc {
            0x831630D0 => {
    //   block [0x831630D0..0x83163248)
	// 831630D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831630D4: 4804508D  bl 0x831a8160
	ctx.lr = 0x831630D8;
	sub_831A8130(ctx, base);
	// 831630D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831630DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831630E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831630E4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831630E8: 38C10051  addi r6, r1, 0x51
	ctx.r[6].s64 = ctx.r[1].s64 + 81;
	// 831630EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831630F0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831630F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831630F8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 831630FC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83163100: 4BFFFC31  bl 0x83162d30
	ctx.lr = 0x83163104;
	sub_83162D30(ctx, base);
	// 83163104: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83163108: 41980138  blt cr6, 0x83163240
	if ctx.cr[6].lt {
	pc = 0x83163240; continue 'dispatch;
	}
	// 8316310C: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83163110: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 83163114: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 83163118: 409A0028  bne cr6, 0x83163140
	if !ctx.cr[6].eq {
	pc = 0x83163140; continue 'dispatch;
	}
	// 8316311C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163120: 39400048  li r10, 0x48
	ctx.r[10].s64 = 72;
	// 83163124: 4BFFFD7D  bl 0x83162ea0
	ctx.lr = 0x83163128;
	sub_83162EA0(ctx, base);
	// 83163128: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 8316312C: 41980014  blt cr6, 0x83163140
	if ctx.cr[6].lt {
	pc = 0x83163140; continue 'dispatch;
	}
	// 83163130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163134: 4BFFFD6D  bl 0x83162ea0
	ctx.lr = 0x83163138;
	sub_83162EA0(ctx, base);
	// 83163138: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 8316313C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163140: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83163144: 40980010  bge cr6, 0x83163154
	if !ctx.cr[6].lt {
	pc = 0x83163154; continue 'dispatch;
	}
	// 83163148: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316314C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83163150: 48045060  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83163154: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83163158: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8316315C: 7D680734  extsh r8, r11
	ctx.r[8].s64 = ctx.r[11].s16 as i64;
	// 83163160: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83163164: 4198FFE4  blt cr6, 0x83163148
	if ctx.cr[6].lt {
	pc = 0x83163148; continue 'dispatch;
	}
	// 83163168: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8316316C: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 83163170: 409A0028  bne cr6, 0x83163198
	if !ctx.cr[6].eq {
	pc = 0x83163198; continue 'dispatch;
	}
	// 83163174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163178: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8316317C: 4BFFFD25  bl 0x83162ea0
	ctx.lr = 0x83163180;
	sub_83162EA0(ctx, base);
	// 83163180: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83163184: 41980014  blt cr6, 0x83163198
	if ctx.cr[6].lt {
	pc = 0x83163198; continue 'dispatch;
	}
	// 83163188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316318C: 4BFFFD15  bl 0x83162ea0
	ctx.lr = 0x83163190;
	sub_83162EA0(ctx, base);
	// 83163190: 39630006  addi r11, r3, 6
	ctx.r[11].s64 = ctx.r[3].s64 + 6;
	// 83163194: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163198: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8316319C: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831631A0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831631A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831631A8: 419A0008  beq cr6, 0x831631b0
	if ctx.cr[6].eq {
	pc = 0x831631B0; continue 'dispatch;
	}
	// 831631AC: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 831631B0: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831631B4: 3D204149  lis r9, 0x4149
	ctx.r[9].s64 = 1095303168;
	// 831631B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831631BC: 61284E46  ori r8, r9, 0x4e46
	ctx.r[8].u64 = ctx.r[9].u64 | 20038;
	// 831631C0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831631C4: 88CA0001  lbz r6, 1(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 831631C8: 54E5403E  rotlwi r5, r7, 8
	ctx.r[5].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 831631CC: 888A0002  lbz r4, 2(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 831631D0: 886A0003  lbz r3, 3(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 831631D4: 7CAA3378  or r10, r5, r6
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 831631D8: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831631DC: 7D272378  or r7, r9, r4
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[4].u64;
	// 831631E0: 54E6402E  slwi r6, r7, 8
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831631E4: 7CC51B78  or r5, r6, r3
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[3].u64;
	// 831631E8: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831631EC: 419A0010  beq cr6, 0x831631fc
	if ctx.cr[6].eq {
	pc = 0x831631FC; continue 'dispatch;
	}
	// 831631F0: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 831631F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831631F8: 48044FB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 831631FC: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83163200: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 83163204: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83163208: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8316320C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83163210: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163214: 480452FD  bl 0x831a8510
	ctx.lr = 0x83163218;
	sub_831A8510(ctx, base);
	// 83163218: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 8316321C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163220: 7D2BFA2E  lhzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83163224: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83163228: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8316322C: B13A0000  sth r9, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83163230: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163234: B11D0000  sth r8, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83163238: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8316323C: B0FD0002  sth r7, 2(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 83163240: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83163244: 48044F6C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163248 size=16
    let mut pc: u32 = 0x83163248;
    'dispatch: loop {
        match pc {
            0x83163248 => {
    //   block [0x83163248..0x83163258)
	// 83163248: 2F040012  cmpwi cr6, r4, 0x12
	ctx.cr[6].compare_i32(ctx.r[4].s32, 18, &mut ctx.xer);
	// 8316324C: 4098000C  bge cr6, 0x83163258
	if !ctx.cr[6].lt {
		sub_83163258(ctx, base);
		return;
	}
	// 83163250: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83163254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163258 size=20
    let mut pc: u32 = 0x83163258;
    'dispatch: loop {
        match pc {
            0x83163258 => {
    //   block [0x83163258..0x8316326C)
	// 83163258: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316325C: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 83163260: 419A000C  beq cr6, 0x8316326c
	if ctx.cr[6].eq {
		sub_8316326C(ctx, base);
		return;
	}
	// 83163264: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83163268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316326C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316326C size=32
    let mut pc: u32 = 0x8316326C;
    'dispatch: loop {
        match pc {
            0x8316326C => {
    //   block [0x8316326C..0x8316328C)
	// 8316326C: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83163270: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 83163274: 2F0A000E  cmpwi cr6, r10, 0xe
	ctx.cr[6].compare_i32(ctx.r[10].s32, 14, &mut ctx.xer);
	// 83163278: 4198FFD8  blt cr6, 0x83163250
	if ctx.cr[6].lt {
		sub_83163248(ctx, base);
		return;
	}
	// 8316327C: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83163280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163284: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83163288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163290 size=28
    let mut pc: u32 = 0x83163290;
    'dispatch: loop {
        match pc {
            0x83163290 => {
    //   block [0x83163290..0x831632AC)
	// 83163290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83163294: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83163298: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 8316329C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831632A0: 4098000C  bge cr6, 0x831632ac
	if !ctx.cr[6].lt {
		sub_831632AC(ctx, base);
		return;
	}
	// 831632A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831632A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831632AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831632AC size=36
    let mut pc: u32 = 0x831632AC;
    'dispatch: loop {
        match pc {
            0x831632AC => {
    //   block [0x831632AC..0x831632D0)
	// 831632AC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831632B0: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 831632B4: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831632B8: 7D074B78  or r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 831632BC: 54E6043E  clrlwi r6, r7, 0x10
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 831632C0: 2B068001  cmplwi cr6, r6, 0x8001
	ctx.cr[6].compare_u32(ctx.r[6].u32, 32769 as u32, &mut ctx.xer);
	// 831632C4: 419A000C  beq cr6, 0x831632d0
	if ctx.cr[6].eq {
		sub_831632D0(ctx, base);
		return;
	}
	// 831632C8: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 831632CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831632D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831632D0 size=36
    let mut pc: u32 = 0x831632D0;
    'dispatch: loop {
        match pc {
            0x831632D0 => {
    //   block [0x831632D0..0x831632F4)
	// 831632D0: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 831632D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831632D8: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 831632DC: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831632E0: 7D0B4B78  or r11, r8, r9
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 831632E4: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 831632E8: 7CE40734  extsh r4, r7
	ctx.r[4].s64 = ctx.r[7].s16 as i64;
	// 831632EC: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831632F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831632F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831632F8 size=220
    let mut pc: u32 = 0x831632F8;
    'dispatch: loop {
        match pc {
            0x831632F8 => {
    //   block [0x831632F8..0x831633D4)
	// 831632F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831632FC: 48044E71  bl 0x831a816c
	ctx.lr = 0x83163300;
	sub_831A8130(ctx, base);
	// 83163300: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163304: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163308: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316330C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83163310: 392B67C8  addi r9, r11, 0x67c8
	ctx.r[9].s64 = ctx.r[11].s64 + 26568;
	// 83163314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83163318: 38DF0001  addi r6, r31, 1
	ctx.r[6].s64 = ctx.r[31].s64 + 1;
	// 8316331C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83163320: B1010060  sth r8, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u16 ) };
	// 83163324: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83163328: 912A8314  stw r9, -0x7cec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31980 as u32), ctx.r[9].u32 ) };
	// 8316332C: 4BFFFA05  bl 0x83162d30
	ctx.lr = 0x83163330;
	sub_83162D30(ctx, base);
	// 83163330: 38FF0010  addi r7, r31, 0x10
	ctx.r[7].s64 = ctx.r[31].s64 + 16;
	// 83163334: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 83163338: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8316333C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83163340: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83163344: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 83163348: 393F0005  addi r9, r31, 5
	ctx.r[9].s64 = ctx.r[31].s64 + 5;
	// 8316334C: 391F0004  addi r8, r31, 4
	ctx.r[8].s64 = ctx.r[31].s64 + 4;
	// 83163350: 38FF0003  addi r7, r31, 3
	ctx.r[7].s64 = ctx.r[31].s64 + 3;
	// 83163354: 38DF0002  addi r6, r31, 2
	ctx.r[6].s64 = ctx.r[31].s64 + 2;
	// 83163358: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8316335C: 4BFFFA25  bl 0x83162d80
	ctx.lr = 0x83163360;
	sub_83162D80(ctx, base);
	// 83163360: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83163364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83163368: 38DF0026  addi r6, r31, 0x26
	ctx.r[6].s64 = ctx.r[31].s64 + 38;
	// 8316336C: 38BF0016  addi r5, r31, 0x16
	ctx.r[5].s64 = ctx.r[31].s64 + 22;
	// 83163370: 4BFFFB41  bl 0x83162eb0
	ctx.lr = 0x83163374;
	sub_83162EB0(ctx, base);
	// 83163374: 38BF004C  addi r5, r31, 0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + 76;
	// 83163378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316337C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83163380: 395F0048  addi r10, r31, 0x48
	ctx.r[10].s64 = ctx.r[31].s64 + 72;
	// 83163384: 393F0044  addi r9, r31, 0x44
	ctx.r[9].s64 = ctx.r[31].s64 + 68;
	// 83163388: 391F0040  addi r8, r31, 0x40
	ctx.r[8].s64 = ctx.r[31].s64 + 64;
	// 8316338C: 38FF003E  addi r7, r31, 0x3e
	ctx.r[7].s64 = ctx.r[31].s64 + 62;
	// 83163390: 38DF003C  addi r6, r31, 0x3c
	ctx.r[6].s64 = ctx.r[31].s64 + 60;
	// 83163394: 38BF0038  addi r5, r31, 0x38
	ctx.r[5].s64 = ctx.r[31].s64 + 56;
	// 83163398: 4BFFFC19  bl 0x83162fb0
	ctx.lr = 0x8316339C;
	sub_83162FB0(ctx, base);
	// 8316339C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831633A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831633A4: 391F0062  addi r8, r31, 0x62
	ctx.r[8].s64 = ctx.r[31].s64 + 98;
	// 831633A8: 38FF0060  addi r7, r31, 0x60
	ctx.r[7].s64 = ctx.r[31].s64 + 96;
	// 831633AC: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 831633B0: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 831633B4: 4BFFFD1D  bl 0x831630d0
	ctx.lr = 0x831633B8;
	sub_831630D0(ctx, base);
	// 831633B8: 38BF0014  addi r5, r31, 0x14
	ctx.r[5].s64 = ctx.r[31].s64 + 20;
	// 831633BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831633C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831633C4: 4BFFFE85  bl 0x83163248
	ctx.lr = 0x831633C8;
	sub_83163248(ctx, base);
	// 831633C8: A0610060  lhz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831633CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831633D0: 48044DEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831633D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831633D8 size=160
    let mut pc: u32 = 0x831633D8;
    'dispatch: loop {
        match pc {
            0x831633D8 => {
    //   block [0x831633D8..0x83163478)
	// 831633D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831633DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831633E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831633E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831633E8: 409A0024  bne cr6, 0x8316340c
	if !ctx.cr[6].eq {
	pc = 0x8316340C; continue 'dispatch;
	}
	// 831633EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831633F0: 388B68F4  addi r4, r11, 0x68f4
	ctx.r[4].s64 = ctx.r[11].s64 + 26868;
	// 831633F4: 4BFFC725  bl 0x8315fb18
	ctx.lr = 0x831633F8;
	sub_8315FB18(ctx, base);
	// 831633F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831633FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163408: 4E800020  blr
	return;
	// 8316340C: 2B040014  cmplwi cr6, r4, 0x14
	ctx.cr[6].compare_u32(ctx.r[4].u32, 20 as u32, &mut ctx.xer);
	// 83163410: 40980028  bge cr6, 0x83163438
	if !ctx.cr[6].lt {
	pc = 0x83163438; continue 'dispatch;
	}
	// 83163414: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316341C: 388B68D0  addi r4, r11, 0x68d0
	ctx.r[4].s64 = ctx.r[11].s64 + 26832;
	// 83163420: 4BFFC6F9  bl 0x8315fb18
	ctx.lr = 0x83163424;
	sub_8315FB18(ctx, base);
	// 83163424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163434: 4E800020  blr
	return;
	// 83163438: 39430007  addi r10, r3, 7
	ctx.r[10].s64 = ctx.r[3].s64 + 7;
	// 8316343C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163440: 55430038  rlwinm r3, r10, 0, 0, 0x1c
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83163444: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 83163448: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 8316344C: 38E9BCAC  addi r7, r9, -0x4354
	ctx.r[7].s64 = ctx.r[9].s64 + -17236;
	// 83163450: 38C868B8  addi r6, r8, 0x68b8
	ctx.r[6].s64 = ctx.r[8].s64 + 26808;
	// 83163454: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163458: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316345C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83163460: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83163464: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83163468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316346C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163478 size=20
    let mut pc: u32 = 0x83163478;
    'dispatch: loop {
        match pc {
            0x83163478 => {
    //   block [0x83163478..0x8316348C)
	// 83163478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316347C: 409A0010  bne cr6, 0x8316348c
	if !ctx.cr[6].eq {
		sub_8316348C(ctx, base);
		return;
	}
	// 83163480: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163484: 388B6918  addi r4, r11, 0x6918
	ctx.r[4].s64 = ctx.r[11].s64 + 26904;
	// 83163488: 4BFFC690  b 0x8315fb18
	sub_8315FB18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316348C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316348C size=20
    let mut pc: u32 = 0x8316348C;
    'dispatch: loop {
        match pc {
            0x8316348C => {
    //   block [0x8316348C..0x831634A0)
	// 8316348C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163490: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163494: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83163498: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316349C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831634A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831634A0 size=20
    let mut pc: u32 = 0x831634A0;
    'dispatch: loop {
        match pc {
            0x831634A0 => {
    //   block [0x831634A0..0x831634B4)
	// 831634A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831634A4: 409A0010  bne cr6, 0x831634b4
	if !ctx.cr[6].eq {
		sub_831634B4(ctx, base);
		return;
	}
	// 831634A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831634AC: 388B693C  addi r4, r11, 0x693c
	ctx.r[4].s64 = ctx.r[11].s64 + 26940;
	// 831634B0: 4BFFC668  b 0x8315fb18
	sub_8315FB18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831634B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831634B4 size=8
    let mut pc: u32 = 0x831634B4;
    'dispatch: loop {
        match pc {
            0x831634B4 => {
    //   block [0x831634B4..0x831634BC)
	// 831634B4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831634B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831634C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831634C0 size=72
    let mut pc: u32 = 0x831634C0;
    'dispatch: loop {
        match pc {
            0x831634C0 => {
    //   block [0x831634C0..0x83163508)
	// 831634C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831634C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831634C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831634CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831634D0: 409A0024  bne cr6, 0x831634f4
	if !ctx.cr[6].eq {
	pc = 0x831634F4; continue 'dispatch;
	}
	// 831634D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831634D8: 388B6960  addi r4, r11, 0x6960
	ctx.r[4].s64 = ctx.r[11].s64 + 26976;
	// 831634DC: 4BFFC63D  bl 0x8315fb18
	ctx.lr = 0x831634E0;
	sub_8315FB18(ctx, base);
	// 831634E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831634E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831634E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831634EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831634F0: 4E800020  blr
	return;
	// 831634F4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831634F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831634FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163508 size=20
    let mut pc: u32 = 0x83163508;
    'dispatch: loop {
        match pc {
            0x83163508 => {
    //   block [0x83163508..0x8316351C)
	// 83163508: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316350C: 409A0010  bne cr6, 0x8316351c
	if !ctx.cr[6].eq {
		sub_8316351C(ctx, base);
		return;
	}
	// 83163510: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163514: 388B6984  addi r4, r11, 0x6984
	ctx.r[4].s64 = ctx.r[11].s64 + 27012;
	// 83163518: 4BFFC600  b 0x8315fb18
	sub_8315FB18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316351C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316351C size=12
    let mut pc: u32 = 0x8316351C;
    'dispatch: loop {
        match pc {
            0x8316351C => {
    //   block [0x8316351C..0x83163528)
	// 8316351C: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83163520: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163524: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163528 size=8
    let mut pc: u32 = 0x83163528;
    'dispatch: loop {
        match pc {
            0x83163528 => {
    //   block [0x83163528..0x83163530)
	// 83163528: 4BFFE640  b 0x83161b68
	sub_83161B68(ctx, base);
	return;
	// 8316352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163530 size=116
    let mut pc: u32 = 0x83163530;
    'dispatch: loop {
        match pc {
            0x83163530 => {
    //   block [0x83163530..0x831635A4)
	// 83163530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316353C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163540: 409A0024  bne cr6, 0x83163564
	if !ctx.cr[6].eq {
	pc = 0x83163564; continue 'dispatch;
	}
	// 83163544: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163548: 388B69A8  addi r4, r11, 0x69a8
	ctx.r[4].s64 = ctx.r[11].s64 + 27048;
	// 8316354C: 4BFFC5CD  bl 0x8315fb18
	ctx.lr = 0x83163550;
	sub_8315FB18(ctx, base);
	// 83163550: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83163554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316355C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163560: 4E800020  blr
	return;
	// 83163564: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83163568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316356C: 409A0014  bne cr6, 0x83163580
	if !ctx.cr[6].eq {
	pc = 0x83163580; continue 'dispatch;
	}
	// 83163570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316357C: 4E800020  blr
	return;
	// 83163580: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163584: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163588: 392B68C8  addi r9, r11, 0x68c8
	ctx.r[9].s64 = ctx.r[11].s64 + 26824;
	// 8316358C: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83163590: 4BFFE619  bl 0x83161ba8
	ctx.lr = 0x83163594;
	sub_83161BA8(ctx, base);
	// 83163594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316359C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831635A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831635A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831635A8 size=176
    let mut pc: u32 = 0x831635A8;
    'dispatch: loop {
        match pc {
            0x831635A8 => {
    //   block [0x831635A8..0x83163658)
	// 831635A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831635AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831635B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831635B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831635B8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831635BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831635C0: 419A0074  beq cr6, 0x83163634
	if ctx.cr[6].eq {
	pc = 0x83163634; continue 'dispatch;
	}
	// 831635C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831635C8: 419A006C  beq cr6, 0x83163634
	if ctx.cr[6].eq {
	pc = 0x83163634; continue 'dispatch;
	}
	// 831635CC: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831635D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831635D4: 409A0024  bne cr6, 0x831635f8
	if !ctx.cr[6].eq {
	pc = 0x831635F8; continue 'dispatch;
	}
	// 831635D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831635DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831635E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831635E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831635E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831635EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831635F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831635F4: 4E800020  blr
	return;
	// 831635F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831635FC: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163600: 392B68C8  addi r9, r11, 0x68c8
	ctx.r[9].s64 = ctx.r[11].s64 + 26824;
	// 83163604: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83163608: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8316360C: 4BFFE56D  bl 0x83161b78
	ctx.lr = 0x83163610;
	sub_83161B78(ctx, base);
	// 83163610: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83163614: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83163618: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316361C: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83163620: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83163624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316362C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163630: 4E800020  blr
	return;
	// 83163634: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316363C: 388B69CC  addi r4, r11, 0x69cc
	ctx.r[4].s64 = ctx.r[11].s64 + 27084;
	// 83163640: 4BFFC4D9  bl 0x8315fb18
	ctx.lr = 0x83163644;
	sub_8315FB18(ctx, base);
	// 83163644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83163648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316364C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163658 size=128
    let mut pc: u32 = 0x83163658;
    'dispatch: loop {
        match pc {
            0x83163658 => {
    //   block [0x83163658..0x831636D8)
	// 83163658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163664: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163668: 419A0050  beq cr6, 0x831636b8
	if ctx.cr[6].eq {
	pc = 0x831636B8; continue 'dispatch;
	}
	// 8316366C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83163670: 419A0048  beq cr6, 0x831636b8
	if ctx.cr[6].eq {
	pc = 0x831636B8; continue 'dispatch;
	}
	// 83163674: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83163678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316367C: 419A004C  beq cr6, 0x831636c8
	if ctx.cr[6].eq {
	pc = 0x831636C8; continue 'dispatch;
	}
	// 83163680: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163684: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83163688: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316368C: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83163690: 38EA68C8  addi r7, r10, 0x68c8
	ctx.r[7].s64 = ctx.r[10].s64 + 26824;
	// 83163694: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83163698: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8316369C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 831636A0: 7C88382E  lwzx r4, r8, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 831636A4: 4BFFE4F5  bl 0x83161b98
	ctx.lr = 0x831636A8;
	sub_83161B98(ctx, base);
	// 831636A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831636AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831636B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831636B4: 4E800020  blr
	return;
	// 831636B8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831636BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831636C0: 388B69F0  addi r4, r11, 0x69f0
	ctx.r[4].s64 = ctx.r[11].s64 + 27120;
	// 831636C4: 4BFFC455  bl 0x8315fb18
	ctx.lr = 0x831636C8;
	sub_8315FB18(ctx, base);
	// 831636C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831636CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831636D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831636D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831636D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831636D8 size=128
    let mut pc: u32 = 0x831636D8;
    'dispatch: loop {
        match pc {
            0x831636D8 => {
    //   block [0x831636D8..0x83163758)
	// 831636D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831636DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831636E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831636E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831636E8: 419A0050  beq cr6, 0x83163738
	if ctx.cr[6].eq {
	pc = 0x83163738; continue 'dispatch;
	}
	// 831636EC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831636F0: 419A0048  beq cr6, 0x83163738
	if ctx.cr[6].eq {
	pc = 0x83163738; continue 'dispatch;
	}
	// 831636F4: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831636F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831636FC: 419A004C  beq cr6, 0x83163748
	if ctx.cr[6].eq {
	pc = 0x83163748; continue 'dispatch;
	}
	// 83163700: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163704: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83163708: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316370C: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83163710: 38EA68C8  addi r7, r10, 0x68c8
	ctx.r[7].s64 = ctx.r[10].s64 + 26824;
	// 83163714: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83163718: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8316371C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83163720: 7C88382E  lwzx r4, r8, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 83163724: 4BFFE465  bl 0x83161b88
	ctx.lr = 0x83163728;
	sub_83161B88(ctx, base);
	// 83163728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316372C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163734: 4E800020  blr
	return;
	// 83163738: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316373C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163740: 388B6A14  addi r4, r11, 0x6a14
	ctx.r[4].s64 = ctx.r[11].s64 + 27156;
	// 83163744: 4BFFC3D5  bl 0x8315fb18
	ctx.lr = 0x83163748;
	sub_8315FB18(ctx, base);
	// 83163748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316374C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163758 size=180
    let mut pc: u32 = 0x83163758;
    'dispatch: loop {
        match pc {
            0x83163758 => {
    //   block [0x83163758..0x8316380C)
	// 83163758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316375C: 48044A09  bl 0x831a8164
	ctx.lr = 0x83163760;
	sub_831A8130(ctx, base);
	// 83163760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163764: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83163768: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316376C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83163770: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83163774: 409A001C  bne cr6, 0x83163790
	if !ctx.cr[6].eq {
	pc = 0x83163790; continue 'dispatch;
	}
	// 83163778: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316377C: 388B6A38  addi r4, r11, 0x6a38
	ctx.r[4].s64 = ctx.r[11].s64 + 27192;
	// 83163780: 4BFFC399  bl 0x8315fb18
	ctx.lr = 0x83163784;
	sub_8315FB18(ctx, base);
	// 83163784: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163788: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316378C: 48044A28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83163790: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83163794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163798: 409A0020  bne cr6, 0x831637b8
	if !ctx.cr[6].eq {
	pc = 0x831637B8; continue 'dispatch;
	}
	// 8316379C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831637A0: 419A000C  beq cr6, 0x831637ac
	if ctx.cr[6].eq {
	pc = 0x831637AC; continue 'dispatch;
	}
	// 831637A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831637A8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831637AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831637B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831637B4: 48044A00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831637B8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831637BC: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 831637C0: 3BEB68C8  addi r31, r11, 0x68c8
	ctx.r[31].s64 = ctx.r[11].s64 + 26824;
	// 831637C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831637C8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831637CC: 7C9EF82E  lwzx r4, r30, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831637D0: 4BFFE3A9  bl 0x83161b78
	ctx.lr = 0x831637D4;
	sub_83161B78(ctx, base);
	// 831637D4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831637D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831637DC: 419A0008  beq cr6, 0x831637e4
	if ctx.cr[6].eq {
	pc = 0x831637E4; continue 'dispatch;
	}
	// 831637E0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831637E4: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 831637E8: 7C9EF82E  lwzx r4, r30, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831637EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831637F0: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 831637F4: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831637F8: 555FDFFE  rlwinm r31, r10, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 831637FC: 4BFFE38D  bl 0x83161b88
	ctx.lr = 0x83163800;
	sub_83161B88(ctx, base);
	// 83163800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163804: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83163808: 480449AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163810 size=16
    let mut pc: u32 = 0x83163810;
    'dispatch: loop {
        match pc {
            0x83163810 => {
    //   block [0x83163810..0x83163820)
	// 83163810: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163814: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163818: 388B6A5C  addi r4, r11, 0x6a5c
	ctx.r[4].s64 = ctx.r[11].s64 + 27228;
	// 8316381C: 4BFFC2FC  b 0x8315fb18
	sub_8315FB18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163820 size=20
    let mut pc: u32 = 0x83163820;
    'dispatch: loop {
        match pc {
            0x83163820 => {
    //   block [0x83163820..0x83163834)
	// 83163820: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163824: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83163828: 396B6A88  addi r11, r11, 0x6a88
	ctx.r[11].s64 = ctx.r[11].s64 + 27272;
	// 8316382C: 916A831C  stw r11, -0x7ce4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31972 as u32), ctx.r[11].u32 ) };
	// 83163830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163838 size=96
    let mut pc: u32 = 0x83163838;
    'dispatch: loop {
        match pc {
            0x83163838 => {
    //   block [0x83163838..0x83163898)
	// 83163838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316383C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316384C: 4BFFC4C5  bl 0x8315fd10
	ctx.lr = 0x83163850;
	sub_8315FD10(ctx, base);
	// 83163850: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83163858: 394B6ADC  addi r10, r11, 0x6adc
	ctx.r[10].s64 = ctx.r[11].s64 + 27356;
	// 8316385C: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 83163860: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163864: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 83163868: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8316386C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83163870: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83163874: 4200FFF8  bdnz 0x8316386c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8316386C; continue 'dispatch;
	}
	// 83163878: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8316387C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163880: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 83163884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316388C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163898 size=16
    let mut pc: u32 = 0x83163898;
    'dispatch: loop {
        match pc {
            0x83163898 => {
    //   block [0x83163898..0x831638A8)
	// 83163898: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316389C: 394B6ADC  addi r10, r11, 0x6adc
	ctx.r[10].s64 = ctx.r[11].s64 + 27356;
	// 831638A0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831638A4: 4BFFC484  b 0x8315fd28
	sub_8315FD28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831638A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831638A8 size=8
    let mut pc: u32 = 0x831638A8;
    'dispatch: loop {
        match pc {
            0x831638A8 => {
    //   block [0x831638A8..0x831638B0)
	// 831638A8: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831638AC: 8224BEB0  lwz r17, -0x4150(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831638B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831638B0 size=100
    let mut pc: u32 = 0x831638B0;
    'dispatch: loop {
        match pc {
            0x831638B0 => {
    //   block [0x831638B0..0x83163914)
	// 831638B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831638B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831638B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831638BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831638C0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 831638C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831638C8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831638CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831638D0: 419A002C  beq cr6, 0x831638fc
	if ctx.cr[6].eq {
	pc = 0x831638FC; continue 'dispatch;
	}
	// 831638D4: 60000000  nop
	// 831638D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831638DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831638E0: 480DF0FD  bl 0x832429dc
	ctx.lr = 0x831638E4;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 831638E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831638E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831638EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831638F0: 48000008  b 0x831638f8
	pc = 0x831638F8; continue 'dispatch;
	// 831638F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831638F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831638FC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83163900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316390C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163914 size=12
    let mut pc: u32 = 0x83163914;
    'dispatch: loop {
        match pc {
            0x83163914 => {
    //   block [0x83163914..0x83163920)
	// 83163914: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163918: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8316391C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163920 size=8
    let mut pc: u32 = 0x83163920;
    'dispatch: loop {
        match pc {
            0x83163920 => {
    //   block [0x83163920..0x83163928)
	// 83163920: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 83163924: 8224BEC8  lwz r17, -0x4138(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163928 size=100
    let mut pc: u32 = 0x83163928;
    'dispatch: loop {
        match pc {
            0x83163928 => {
    //   block [0x83163928..0x8316398C)
	// 83163928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83163934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163938: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8316393C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163940: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83163944: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163948: 419A002C  beq cr6, 0x83163974
	if ctx.cr[6].eq {
	pc = 0x83163974; continue 'dispatch;
	}
	// 8316394C: 60000000  nop
	// 83163950: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163954: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163958: 480DF015  bl 0x8324296c
	ctx.lr = 0x8316395C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8316395C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163960: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163964: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163968: 48000008  b 0x83163970
	pc = 0x83163970; continue 'dispatch;
	// 8316396C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83163970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83163974: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83163978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83163984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316398C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316398C size=12
    let mut pc: u32 = 0x8316398C;
    'dispatch: loop {
        match pc {
            0x8316398C => {
    //   block [0x8316398C..0x83163998)
	// 8316398C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163990: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163998 size=8
    let mut pc: u32 = 0x83163998;
    'dispatch: loop {
        match pc {
            0x83163998 => {
    //   block [0x83163998..0x831639A0)
	// 83163998: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 8316399C: 8224BEE0  lwz r17, -0x4120(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831639A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831639A0 size=100
    let mut pc: u32 = 0x831639A0;
    'dispatch: loop {
        match pc {
            0x831639A0 => {
    //   block [0x831639A0..0x83163A04)
	// 831639A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831639A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831639A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831639AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831639B0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 831639B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831639B8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831639BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831639C0: 419A002C  beq cr6, 0x831639ec
	if ctx.cr[6].eq {
	pc = 0x831639EC; continue 'dispatch;
	}
	// 831639C4: 60000000  nop
	// 831639C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831639CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831639D0: 480DEF8D  bl 0x8324295c
	ctx.lr = 0x831639D4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831639D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831639D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831639DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831639E0: 48000008  b 0x831639e8
	pc = 0x831639E8; continue 'dispatch;
	// 831639E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831639E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831639EC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 831639F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831639F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831639F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831639FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163A04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163A04 size=12
    let mut pc: u32 = 0x83163A04;
    'dispatch: loop {
        match pc {
            0x83163A04 => {
    //   block [0x83163A04..0x83163A10)
	// 83163A04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163A08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163A10 size=8
    let mut pc: u32 = 0x83163A10;
    'dispatch: loop {
        match pc {
            0x83163A10 => {
    //   block [0x83163A10..0x83163A18)
	// 83163A10: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 83163A14: 8224BEF8  lwz r17, -0x4108(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16648 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163A18 size=108
    let mut pc: u32 = 0x83163A18;
    'dispatch: loop {
        match pc {
            0x83163A18 => {
    //   block [0x83163A18..0x83163A84)
	// 83163A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163A24: 3BE1FFE0  addi r31, r1, -0x20
	ctx.r[31].s64 = ctx.r[1].s64 + -32;
	// 83163A28: 9421FFE0  stwu r1, -0x20(r1)
	ea = ctx.r[1].u32.wrapping_add(-32 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163A2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83163A30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163A34: 409A0018  bne cr6, 0x83163a4c
	if !ctx.cr[6].eq {
	pc = 0x83163A4C; continue 'dispatch;
	}
	// 83163A38: 383F0020  addi r1, r31, 0x20
	ctx.r[1].s64 = ctx.r[31].s64 + 32;
	// 83163A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163A48: 4E800020  blr
	return;
	// 83163A4C: 60000000  nop
	// 83163A50: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A58: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A64: 48000008  b 0x83163a6c
	pc = 0x83163A6C; continue 'dispatch;
	// 83163A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163A6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83163A70: 383F0020  addi r1, r31, 0x20
	ctx.r[1].s64 = ctx.r[31].s64 + 32;
	// 83163A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163A84 size=12
    let mut pc: u32 = 0x83163A84;
    'dispatch: loop {
        match pc {
            0x83163A84 => {
    //   block [0x83163A84..0x83163A90)
	// 83163A84: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163A88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83163A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163A90 size=84
    let mut pc: u32 = 0x83163A90;
    'dispatch: loop {
        match pc {
            0x83163A90 => {
    //   block [0x83163A90..0x83163AE4)
	// 83163A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83163A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83163AA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83163AAC: 4BFFFDED  bl 0x83163898
	ctx.lr = 0x83163AB0;
	sub_83163898(ctx, base);
	// 83163AB0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83163AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83163ABC: 419A0010  beq cr6, 0x83163acc
	if ctx.cr[6].eq {
	pc = 0x83163ACC; continue 'dispatch;
	}
	// 83163AC0: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 83163AC4: 4BFFC1BD  bl 0x8315fc80
	ctx.lr = 0x83163AC8;
	sub_8315FC80(ctx, base);
	// 83163AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83163AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83163ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163AE8 size=204
    let mut pc: u32 = 0x83163AE8;
    'dispatch: loop {
        match pc {
            0x83163AE8 => {
    //   block [0x83163AE8..0x83163BB4)
	// 83163AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163AEC: 4804467D  bl 0x831a8168
	ctx.lr = 0x83163AF0;
	sub_831A8130(ctx, base);
	// 83163AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163AF4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83163AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163AFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83163B00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83163B04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83163B08: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163B0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83163B10: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83163B14: 4BFFC1F5  bl 0x8315fd08
	ctx.lr = 0x83163B18;
	sub_8315FD08(ctx, base);
	// 83163B18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83163B1C: 419A0014  beq cr6, 0x83163b30
	if ctx.cr[6].eq {
	pc = 0x83163B30; continue 'dispatch;
	}
	// 83163B20: 4BFFFD19  bl 0x83163838
	ctx.lr = 0x83163B24;
	sub_83163838(ctx, base);
	// 83163B24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83163B28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83163B2C: 409A0028  bne cr6, 0x83163b54
	if !ctx.cr[6].eq {
	pc = 0x83163B54; continue 'dispatch;
	}
	// 83163B30: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163B34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163B38: 388B6B28  addi r4, r11, 0x6b28
	ctx.r[4].s64 = ctx.r[11].s64 + 27432;
	// 83163B3C: 4BFFBFDD  bl 0x8315fb18
	ctx.lr = 0x83163B40;
	sub_8315FB18(ctx, base);
	// 83163B40: 3940FFFD  li r10, -3
	ctx.r[10].s64 = -3;
	// 83163B44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163B48: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83163B50: 48044668  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83163B54: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83163B58: 4BFFFD59  bl 0x831638b0
	ctx.lr = 0x83163B5C;
	sub_831638B0(ctx, base);
	// 83163B5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83163B60: 409A0040  bne cr6, 0x83163ba0
	if !ctx.cr[6].eq {
	pc = 0x83163BA0; continue 'dispatch;
	}
	// 83163B64: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163B68: 388B6AEC  addi r4, r11, 0x6aec
	ctx.r[4].s64 = ctx.r[11].s64 + 27372;
	// 83163B6C: 4BFFBFAD  bl 0x8315fb18
	ctx.lr = 0x83163B70;
	sub_8315FB18(ctx, base);
	// 83163B70: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83163B74: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 83163B78: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163B80: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163B84: 388981F0  addi r4, r9, -0x7e10
	ctx.r[4].s64 = ctx.r[9].s64 + -32272;
	// 83163B88: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163B8C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 83163B90: 4E800421  bctrl
	ctx.lr = 0x83163B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83163B94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83163B9C: 4804461C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83163BA0: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83163BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163BA8: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83163BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83163BB0: 48044608  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163BB8 size=140
    let mut pc: u32 = 0x83163BB8;
    'dispatch: loop {
        match pc {
            0x83163BB8 => {
    //   block [0x83163BB8..0x83163C44)
	// 83163BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163BBC: 480445AD  bl 0x831a8168
	ctx.lr = 0x83163BC0;
	sub_831A8130(ctx, base);
	// 83163BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163BC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83163BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163BCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83163BD0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83163BD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163BD8: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83163BDC: 839F0024  lwz r28, 0x24(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83163BE0: 4BFFFE39  bl 0x83163a18
	ctx.lr = 0x83163BE4;
	sub_83163A18(ctx, base);
	// 83163BE4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83163BE8: 409A0018  bne cr6, 0x83163c00
	if !ctx.cr[6].eq {
	pc = 0x83163C00; continue 'dispatch;
	}
	// 83163BEC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163BF0: 388B6B50  addi r4, r11, 0x6b50
	ctx.r[4].s64 = ctx.r[11].s64 + 27472;
	// 83163BF4: 4BFFBF25  bl 0x8315fb18
	ctx.lr = 0x83163BF8;
	sub_8315FB18(ctx, base);
	// 83163BF8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83163BFC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163C00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163C04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83163C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163C0C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83163C10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83163C14: 4E800421  bctrl
	ctx.lr = 0x83163C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83163C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163C1C: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83163C20: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83163C24: 4BF63EBD  bl 0x830c7ae0
	ctx.lr = 0x83163C28;
	sub_830C7AE0(ctx, base);
	// 83163C28: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83163C2C: 419A0010  beq cr6, 0x83163c3c
	if ctx.cr[6].eq {
	pc = 0x83163C3C; continue 'dispatch;
	}
	// 83163C30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83163C34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83163C38: 4BFFBB31  bl 0x8315f768
	ctx.lr = 0x83163C3C;
	sub_8315F768(ctx, base);
	// 83163C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83163C40: 48044578  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163C48 size=84
    let mut pc: u32 = 0x83163C48;
    'dispatch: loop {
        match pc {
            0x83163C48 => {
    //   block [0x83163C48..0x83163C9C)
	// 83163C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163C50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163C58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83163C5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163C60: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 83163C64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163C68: 4BFFFCC1  bl 0x83163928
	ctx.lr = 0x83163C6C;
	sub_83163928(ctx, base);
	// 83163C6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83163C70: 409A0018  bne cr6, 0x83163c88
	if !ctx.cr[6].eq {
	pc = 0x83163C88; continue 'dispatch;
	}
	// 83163C74: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163C78: 388B6B88  addi r4, r11, 0x6b88
	ctx.r[4].s64 = ctx.r[11].s64 + 27528;
	// 83163C7C: 4BFFBE9D  bl 0x8315fb18
	ctx.lr = 0x83163C80;
	sub_8315FB18(ctx, base);
	// 83163C80: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83163C84: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163CA0 size=84
    let mut pc: u32 = 0x83163CA0;
    'dispatch: loop {
        match pc {
            0x83163CA0 => {
    //   block [0x83163CA0..0x83163CF4)
	// 83163CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163CB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83163CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163CB8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 83163CBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163CC0: 4BFFFCE1  bl 0x831639a0
	ctx.lr = 0x83163CC4;
	sub_831639A0(ctx, base);
	// 83163CC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83163CC8: 409A0018  bne cr6, 0x83163ce0
	if !ctx.cr[6].eq {
	pc = 0x83163CE0; continue 'dispatch;
	}
	// 83163CCC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83163CD0: 388B6BC0  addi r4, r11, 0x6bc0
	ctx.r[4].s64 = ctx.r[11].s64 + 27584;
	// 83163CD4: 4BFFBE45  bl 0x8315fb18
	ctx.lr = 0x83163CD8;
	sub_8315FB18(ctx, base);
	// 83163CD8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83163CDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83163CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163CF8 size=32
    let mut pc: u32 = 0x83163CF8;
    'dispatch: loop {
        match pc {
            0x83163CF8 => {
    //   block [0x83163CF8..0x83163D18)
	// 83163CF8: 2B03000B  cmplwi cr6, r3, 0xb
	ctx.cr[6].compare_u32(ctx.r[3].u32, 11 as u32, &mut ctx.xer);
	// 83163CFC: 4199006C  bgt cr6, 0x83163d68
	if ctx.cr[6].gt {
		sub_83163D68(ctx, base);
		return;
	}
	// 83163D00: 3D808316  lis r12, -0x7cea
	ctx.r[12].s64 = -2095710208;
	// 83163D04: 398C3D18  addi r12, r12, 0x3d18
	ctx.r[12].s64 = ctx.r[12].s64 + 15640;
	// 83163D08: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83163D0C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83163D10: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83163D14: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
			// ERROR: 0x83163D48
			return;
		},
		1 => {
			// ERROR: 0x83163D48
			return;
		},
		2 => {
			// ERROR: 0x83163D50
			return;
		},
		3 => {
			// ERROR: 0x83163D50
			return;
		},
		4 => {
			// ERROR: 0x83163D58
			return;
		},
		5 => {
			// ERROR: 0x83163D58
			return;
		},
		6 => {
			// ERROR: 0x83163D60
			return;
		},
		7 => {
			// ERROR: 0x83163D60
			return;
		},
		8 => {
			// ERROR: 0x83163D58
			return;
		},
		9 => {
			// ERROR: 0x83163D60
			return;
		},
		10 => {
			// ERROR: 0x83163D58
			return;
		},
		11 => {
			// ERROR: 0x83163D60
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D18 size=56
    let mut pc: u32 = 0x83163D18;
    'dispatch: loop {
        match pc {
            0x83163D18 => {
    //   block [0x83163D18..0x83163D50)
	// 83163D18: 83163D48  lwz r24, 0x3d48(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15688 as u32) ) } as u64;
	// 83163D1C: 83163D48  lwz r24, 0x3d48(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15688 as u32) ) } as u64;
	// 83163D20: 83163D50  lwz r24, 0x3d50(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15696 as u32) ) } as u64;
	// 83163D24: 83163D50  lwz r24, 0x3d50(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15696 as u32) ) } as u64;
	// 83163D28: 83163D58  lwz r24, 0x3d58(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15704 as u32) ) } as u64;
	// 83163D2C: 83163D58  lwz r24, 0x3d58(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15704 as u32) ) } as u64;
	// 83163D30: 83163D60  lwz r24, 0x3d60(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15712 as u32) ) } as u64;
	// 83163D34: 83163D60  lwz r24, 0x3d60(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15712 as u32) ) } as u64;
	// 83163D38: 83163D58  lwz r24, 0x3d58(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15704 as u32) ) } as u64;
	// 83163D3C: 83163D60  lwz r24, 0x3d60(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15712 as u32) ) } as u64;
	// 83163D40: 83163D58  lwz r24, 0x3d58(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15704 as u32) ) } as u64;
	// 83163D44: 83163D60  lwz r24, 0x3d60(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15712 as u32) ) } as u64;
	// 83163D48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83163D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D50 size=8
    let mut pc: u32 = 0x83163D50;
    'dispatch: loop {
        match pc {
            0x83163D50 => {
    //   block [0x83163D50..0x83163D58)
	// 83163D50: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83163D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D58 size=8
    let mut pc: u32 = 0x83163D58;
    'dispatch: loop {
        match pc {
            0x83163D58 => {
    //   block [0x83163D58..0x83163D60)
	// 83163D58: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83163D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D60 size=8
    let mut pc: u32 = 0x83163D60;
    'dispatch: loop {
        match pc {
            0x83163D60 => {
    //   block [0x83163D60..0x83163D68)
	// 83163D60: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83163D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D68 size=8
    let mut pc: u32 = 0x83163D68;
    'dispatch: loop {
        match pc {
            0x83163D68 => {
    //   block [0x83163D68..0x83163D70)
	// 83163D68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83163D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163D70 size=100
    let mut pc: u32 = 0x83163D70;
    'dispatch: loop {
        match pc {
            0x83163D70 => {
    //   block [0x83163D70..0x83163DD4)
	// 83163D70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83163D74: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83163D78: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163D7C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83163D80: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83163D84: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83163D88: 88CB0003  lbz r6, 3(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83163D8C: 7D054B78  or r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 83163D90: 54AA402E  slwi r10, r5, 8
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163D94: 7D493B78  or r9, r10, r7
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83163D98: 5528402E  slwi r8, r9, 8
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83163D9C: 7D073378  or r7, r8, r6
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 83163DA0: 90E40000  stw r7, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83163DA4: 88CB0005  lbz r6, 5(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 83163DA8: 88AB0006  lbz r5, 6(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 83163DAC: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83163DB0: 892B0004  lbz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83163DB4: 5528403E  rotlwi r8, r9, 8
	ctx.r[8].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83163DB8: 7D073378  or r7, r8, r6
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 83163DBC: 54E6402E  slwi r6, r7, 8
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83163DC0: 7CC52B78  or r5, r6, r5
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 83163DC4: 54AB402E  slwi r11, r5, 8
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83163DC8: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83163DCC: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83163DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83163DD8 size=264
    let mut pc: u32 = 0x83163DD8;
    'dispatch: loop {
        match pc {
            0x83163DD8 => {
    //   block [0x83163DD8..0x83163EE0)
	// 83163DD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83163DDC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83163DE0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83163DE4: 99440000  stb r10, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83163DE8: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83163DEC: 99240001  stb r9, 1(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 83163DF0: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83163DF4: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83163DF8: 54E6403E  rotlwi r6, r7, 8
	ctx.r[6].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 83163DFC: 7CC54378  or r5, r6, r8
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 83163E00: B0A40002  sth r5, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 83163E04: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 83163E08: 890B0006  lbz r8, 6(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 83163E0C: 88EB0007  lbz r7, 7(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83163E10: 88CB0004  lbz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83163E14: 54C5403E  rotlwi r5, r6, 8
	ctx.r[5].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 83163E18: 7CAA4B78  or r10, r5, r9
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 83163E1C: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83163E20: 7D284378  or r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 83163E24: 5506402E  slwi r6, r8, 8
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83163E28: 7CC53B78  or r5, r6, r7
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 83163E2C: 90A40004  stw r5, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83163E30: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 83163E34: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83163E38: 890B000B  lbz r8, 0xb(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 83163E3C: 88EB0008  lbz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83163E40: 54E6403E  rotlwi r6, r7, 8
	ctx.r[6].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 83163E44: 7CC55378  or r5, r6, r10
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	// 83163E48: 54AA402E  slwi r10, r5, 8
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83163E4C: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83163E50: 5527402E  slwi r7, r9, 8
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83163E54: 7CE64378  or r6, r7, r8
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 83163E58: 90C40008  stw r6, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83163E5C: 88AB000D  lbz r5, 0xd(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 83163E60: 894B000E  lbz r10, 0xe(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 83163E64: 892B000F  lbz r9, 0xf(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 83163E68: 890B000C  lbz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83163E6C: 5507403E  rotlwi r7, r8, 8
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 83163E70: 7CE62B78  or r6, r7, r5
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 83163E74: 54C5402E  slwi r5, r6, 8
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83163E78: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 83163E7C: 5548402E  slwi r8, r10, 8
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83163E80: 7D074B78  or r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 83163E84: 90E4000C  stw r7, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83163E88: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 83163E8C: 88AB0010  lbz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83163E90: 54AA403E  rotlwi r10, r5, 8
	ctx.r[10].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 83163E94: 7D493378  or r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 83163E98: B1240010  sth r9, 0x10(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[9].u16 ) };
	// 83163E9C: 88EB0013  lbz r7, 0x13(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 83163EA0: 88CB0012  lbz r6, 0x12(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 83163EA4: 54C5403E  rotlwi r5, r6, 8
	ctx.r[5].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 83163EA8: 7CAA3B78  or r10, r5, r7
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 83163EAC: B1440012  sth r10, 0x12(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(18 as u32), ctx.r[10].u16 ) };
	// 83163EB0: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83163EB4: 88EB0016  lbz r7, 0x16(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 83163EB8: 88CB0017  lbz r6, 0x17(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(23 as u32) ) } as u64;
	// 83163EBC: 88AB0014  lbz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83163EC0: 54AB403E  rotlwi r11, r5, 8
	ctx.r[11].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 83163EC4: 7D6A4378  or r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 83163EC8: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83163ECC: 7D283B78  or r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 83163ED0: 5507402E  slwi r7, r8, 8
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83163ED4: 7CE63378  or r6, r7, r6
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 83163ED8: 90C40014  stw r6, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 83163EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163EE0 size=156
    let mut pc: u32 = 0x83163EE0;
    'dispatch: loop {
        match pc {
            0x83163EE0 => {
    //   block [0x83163EE0..0x83163F7C)
	// 83163EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163EEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83163EF4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83163EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163EFC: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83163F00: 3D00833A  lis r8, -0x7cc6
	ctx.r[8].s64 = -2093350912;
	// 83163F04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163F08: 39296BF8  addi r9, r9, 0x6bf8
	ctx.r[9].s64 = ctx.r[9].s64 + 27640;
	// 83163F0C: 816A8324  lwz r11, -0x7cdc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31964 as u32) ) } as u64;
	// 83163F10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83163F14: 91288320  stw r9, -0x7ce0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-31968 as u32), ctx.r[9].u32 ) };
	// 83163F18: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83163F1C: 916A8324  stw r11, -0x7cdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31964 as u32), ctx.r[11].u32 ) };
	// 83163F20: 409A0048  bne cr6, 0x83163f68
	if !ctx.cr[6].eq {
	pc = 0x83163F68; continue 'dispatch;
	}
	// 83163F24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83163F28: 48000A39  bl 0x83164960
	ctx.lr = 0x83163F2C;
	sub_83164960(ctx, base);
	// 83163F2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83163F30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83163F34: 409A002C  bne cr6, 0x83163f60
	if !ctx.cr[6].eq {
	pc = 0x83163F60; continue 'dispatch;
	}
	// 83163F38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83163F3C: 4800179D  bl 0x831656d8
	ctx.lr = 0x83163F40;
	sub_831656D8(ctx, base);
	// 83163F40: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83163F44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83163F48: 409A0018  bne cr6, 0x83163f60
	if !ctx.cr[6].eq {
	pc = 0x83163F60; continue 'dispatch;
	}
	// 83163F4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83163F50: 48003041  bl 0x83166f90
	ctx.lr = 0x83163F54;
	sub_83166F90(ctx, base);
	// 83163F54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83163F58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83163F5C: 419A000C  beq cr6, 0x83163f68
	if ctx.cr[6].eq {
	pc = 0x83163F68; continue 'dispatch;
	}
	// 83163F60: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83163F64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163F68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83163F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163F74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163F80 size=108
    let mut pc: u32 = 0x83163F80;
    'dispatch: loop {
        match pc {
            0x83163F80 => {
    //   block [0x83163F80..0x83163FEC)
	// 83163F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83163F90: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83163F94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163F98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83163F9C: 816A8324  lwz r11, -0x7cdc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31964 as u32) ) } as u64;
	// 83163FA0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83163FA4: 916A8324  stw r11, -0x7cdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31964 as u32), ctx.r[11].u32 ) };
	// 83163FA8: 40820030  bne 0x83163fd8
	if !ctx.cr[0].eq {
	pc = 0x83163FD8; continue 'dispatch;
	}
	// 83163FAC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83163FB0: 3BEB8328  addi r31, r11, -0x7cd8
	ctx.r[31].s64 = ctx.r[11].s64 + -31960;
	// 83163FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163FB8: 48003039  bl 0x83166ff0
	ctx.lr = 0x83163FBC;
	sub_83166FF0(ctx, base);
	// 83163FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163FC0: 48001779  bl 0x83165738
	ctx.lr = 0x83163FC4;
	sub_83165738(ctx, base);
	// 83163FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83163FC8: 480009F9  bl 0x831649c0
	ctx.lr = 0x83163FCC;
	sub_831649C0(ctx, base);
	// 83163FCC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83163FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83163FD4: 916A832C  stw r11, -0x7cd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31956 as u32), ctx.r[11].u32 ) };
	// 83163FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83163FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83163FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83163FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83163FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83163FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83163FF0 size=108
    let mut pc: u32 = 0x83163FF0;
    'dispatch: loop {
        match pc {
            0x83163FF0 => {
    //   block [0x83163FF0..0x8316405C)
	// 83163FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83163FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83163FF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83163FFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164000: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83164004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164008: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8316400C: 60A58000  ori r5, r5, 0x8000
	ctx.r[5].u64 = ctx.r[5].u64 | 32768;
	// 83164010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164014: 54AA402E  slwi r10, r5, 8
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83164018: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8316401C: 41980014  blt cr6, 0x83164030
	if ctx.cr[6].lt {
	pc = 0x83164030; continue 'dispatch;
	}
	// 83164020: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83164024: 54A5083C  slwi r5, r5, 1
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83164028: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8316402C: 4198FFE8  blt cr6, 0x83164014
	if ctx.cr[6].lt {
	pc = 0x83164014; continue 'dispatch;
	}
	// 83164030: 38840400  addi r4, r4, 0x400
	ctx.r[4].s64 = ctx.r[4].s64 + 1024;
	// 83164034: 480057BD  bl 0x831697f0
	ctx.lr = 0x83164038;
	sub_831697F0(ctx, base);
	// 83164038: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316403C: 4082000C  bne 0x83164048
	if !ctx.cr[0].eq {
	pc = 0x83164048; continue 'dispatch;
	}
	// 83164040: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164044: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316404C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164060 size=12
    let mut pc: u32 = 0x83164060;
    'dispatch: loop {
        match pc {
            0x83164060 => {
    //   block [0x83164060..0x8316406C)
	// 83164060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164064: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164068: 48004BC8  b 0x83168c30
	sub_83168C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164070 size=72
    let mut pc: u32 = 0x83164070;
    'dispatch: loop {
        match pc {
            0x83164070 => {
    //   block [0x83164070..0x831640B8)
	// 83164070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316407C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164084: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164088: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316408C: 396B6C54  addi r11, r11, 0x6c54
	ctx.r[11].s64 = ctx.r[11].s64 + 27732;
	// 83164090: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164094: 4182000C  beq 0x831640a0
	if ctx.cr[0].eq {
	pc = 0x831640A0; continue 'dispatch;
	}
	// 83164098: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8316409C: 4BFFBBE5  bl 0x8315fc80
	ctx.lr = 0x831640A0;
	sub_8315FC80(ctx, base);
	// 831640A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831640A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831640A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831640AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831640B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831640B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831640B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831640B8 size=212
    let mut pc: u32 = 0x831640B8;
    'dispatch: loop {
        match pc {
            0x831640B8 => {
    //   block [0x831640B8..0x8316418C)
	// 831640B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831640BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831640C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831640C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831640C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831640CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831640D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831640D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831640D8: 419A001C  beq cr6, 0x831640f4
	if ctx.cr[6].eq {
	pc = 0x831640F4; continue 'dispatch;
	}
	// 831640DC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831640E0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831640E4: 396B6CA0  addi r11, r11, 0x6ca0
	ctx.r[11].s64 = ctx.r[11].s64 + 27808;
	// 831640E8: 394A6C64  addi r10, r10, 0x6c64
	ctx.r[10].s64 = ctx.r[10].s64 + 27748;
	// 831640EC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831640F0: 915F01D8  stw r10, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[10].u32 ) };
	// 831640F4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 831640F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831640FC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83164100: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83164104: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83164108: 396B6C94  addi r11, r11, 0x6c94
	ctx.r[11].s64 = ctx.r[11].s64 + 27796;
	// 8316410C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164110: 394A6C7C  addi r10, r10, 0x6c7c
	ctx.r[10].s64 = ctx.r[10].s64 + 27772;
	// 83164114: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164118: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8316411C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164120: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83164124: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83164128: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316412C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164130: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83164134: 396BFE38  addi r11, r11, -0x1c8
	ctx.r[11].s64 = ctx.r[11].s64 + -456;
	// 83164138: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8316413C: 480059AD  bl 0x83169ae8
	ctx.lr = 0x83164140;
	sub_83169AE8(ctx, base);
	// 83164140: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83164144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83164148: 93DF01D0  stw r30, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[30].u32 ) };
	// 8316414C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83164150: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83164154: 4804408D  bl 0x831a81e0
	ctx.lr = 0x83164158;
	sub_831A81E0(ctx, base);
	// 83164158: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 8316415C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83164160: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83164164: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83164168: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8316416C: 48044075  bl 0x831a81e0
	ctx.lr = 0x83164170;
	sub_831A81E0(ctx, base);
	// 83164170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164174: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316417C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164180: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83164184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164190 size=68
    let mut pc: u32 = 0x83164190;
    'dispatch: loop {
        match pc {
            0x83164190 => {
    //   block [0x83164190..0x831641D4)
	// 83164190: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83164194: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83164198: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 8316419C: 394A6C94  addi r10, r10, 0x6c94
	ctx.r[10].s64 = ctx.r[10].s64 + 27796;
	// 831641A0: 39086C7C  addi r8, r8, 0x6c7c
	ctx.r[8].s64 = ctx.r[8].s64 + 27772;
	// 831641A4: 812BFE38  lwz r9, -0x1c8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-456 as u32) ) } as u64;
	// 831641A8: 386BFEE8  addi r3, r11, -0x118
	ctx.r[3].s64 = ctx.r[11].s64 + -280;
	// 831641AC: 914BFE28  stw r10, -0x1d8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-472 as u32), ctx.r[10].u32 ) };
	// 831641B0: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 831641B4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831641B8: 910AFE38  stw r8, -0x1c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-456 as u32), ctx.r[8].u32 ) };
	// 831641BC: 814BFE38  lwz r10, -0x1c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-456 as u32) ) } as u64;
	// 831641C0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831641C4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831641C8: 394AFE38  addi r10, r10, -0x1c8
	ctx.r[10].s64 = ctx.r[10].s64 + -456;
	// 831641CC: 914BFE34  stw r10, -0x1cc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-460 as u32), ctx.r[10].u32 ) };
	// 831641D0: 4BF63910  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831641D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831641D8 size=136
    let mut pc: u32 = 0x831641D8;
    'dispatch: loop {
        match pc {
            0x831641D8 => {
    //   block [0x831641D8..0x83164260)
	// 831641D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831641DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831641E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831641E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831641E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831641EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831641F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831641F4: 4BFFB82D  bl 0x8315fa20
	ctx.lr = 0x831641F8;
	sub_8315FA20(ctx, base);
	// 831641F8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831641FC: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 83164200: 48008681  bl 0x8316c880
	ctx.lr = 0x83164204;
	sub_8316C880(ctx, base);
	// 83164204: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164208: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316420C: 386B8338  addi r3, r11, -0x7cc8
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	// 83164210: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83164214: 48005F35  bl 0x8316a148
	ctx.lr = 0x83164218;
	sub_8316A148(ctx, base);
	// 83164218: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316421C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164220: 906B8334  stw r3, -0x7ccc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-31948 as u32), ctx.r[3].u32 ) };
	// 83164224: 41820010  beq 0x83164234
	if ctx.cr[0].eq {
	pc = 0x83164234; continue 'dispatch;
	}
	// 83164228: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316422C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164230: 419A001C  beq cr6, 0x8316424c
	if ctx.cr[6].eq {
	pc = 0x8316424C; continue 'dispatch;
	}
	// 83164234: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164238: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316423C: 388B6CA8  addi r4, r11, 0x6ca8
	ctx.r[4].s64 = ctx.r[11].s64 + 27816;
	// 83164240: 4BFFB8D9  bl 0x8315fb18
	ctx.lr = 0x83164244;
	sub_8315FB18(ctx, base);
	// 83164244: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164248: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316424C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164260 size=196
    let mut pc: u32 = 0x83164260;
    'dispatch: loop {
        match pc {
            0x83164260 => {
    //   block [0x83164260..0x83164324)
	// 83164260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164264: 48043F01  bl 0x831a8164
	ctx.lr = 0x83164268;
	sub_831A8130(ctx, base);
	// 83164268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316426C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164270: 3FC0833A  lis r30, -0x7cc6
	ctx.r[30].s64 = -2093350912;
	// 83164274: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164278: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316427C: 3B8B81F0  addi r28, r11, -0x7e10
	ctx.r[28].s64 = ctx.r[11].s64 + -32272;
	// 83164280: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83164284: 807E8334  lwz r3, -0x7ccc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31948 as u32) ) } as u64;
	// 83164288: 48005E81  bl 0x8316a108
	ctx.lr = 0x8316428C;
	sub_8316A108(ctx, base);
	// 8316428C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164290: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83164294: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 83164298: 4182005C  beq 0x831642f4
	if ctx.cr[0].eq {
	pc = 0x831642F4; continue 'dispatch;
	}
	// 8316429C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831642A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831642A4: 388B6CD0  addi r4, r11, 0x6cd0
	ctx.r[4].s64 = ctx.r[11].s64 + 27856;
	// 831642A8: 4BFFB871  bl 0x8315fb18
	ctx.lr = 0x831642AC;
	sub_8315FB18(ctx, base);
	// 831642AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831642B0: 807E8334  lwz r3, -0x7ccc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31948 as u32) ) } as u64;
	// 831642B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831642B8: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831642BC: 48005FDD  bl 0x8316a298
	ctx.lr = 0x831642C0;
	sub_8316A298(ctx, base);
	// 831642C0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831642C4: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 831642C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831642CC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831642D0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831642D4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 831642D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831642DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831642E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831642E4: 4E800421  bctrl
	ctx.lr = 0x831642E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831642E8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 831642EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831642F0: 409AFFBC  bne cr6, 0x831642ac
	if !ctx.cr[6].eq {
	pc = 0x831642AC; continue 'dispatch;
	}
	// 831642F4: 807E8334  lwz r3, -0x7ccc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31948 as u32) ) } as u64;
	// 831642F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831642FC: 419A0014  beq cr6, 0x83164310
	if ctx.cr[6].eq {
	pc = 0x83164310; continue 'dispatch;
	}
	// 83164300: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83164304: 48005D95  bl 0x8316a098
	ctx.lr = 0x83164308;
	sub_8316A098(ctx, base);
	// 83164308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316430C: 917E8334  stw r11, -0x7ccc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-31948 as u32), ctx.r[11].u32 ) };
	// 83164310: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83164314: 480067F5  bl 0x8316ab08
	ctx.lr = 0x83164318;
	sub_8316AB08(ctx, base);
	// 83164318: 4BFFB301  bl 0x8315f618
	ctx.lr = 0x8316431C;
	sub_8315F618(ctx, base);
	// 8316431C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164320: 48043E94  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164328 size=196
    let mut pc: u32 = 0x83164328;
    'dispatch: loop {
        match pc {
            0x83164328 => {
    //   block [0x83164328..0x831643EC)
	// 83164328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316432C: 48043E41  bl 0x831a816c
	ctx.lr = 0x83164330;
	sub_831A8130(ctx, base);
	// 83164330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164334: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83164338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316433C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83164340: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83164344: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83164348: 409A0024  bne cr6, 0x8316436c
	if !ctx.cr[6].eq {
	pc = 0x8316436C; continue 'dispatch;
	}
	// 8316434C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164350: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164354: 388B6D6C  addi r4, r11, 0x6d6c
	ctx.r[4].s64 = ctx.r[11].s64 + 28012;
	// 83164358: 4BFFB7C1  bl 0x8315fb18
	ctx.lr = 0x8316435C;
	sub_8315FB18(ctx, base);
	// 8316435C: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 83164360: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164364: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164368: 4800007C  b 0x831643e4
	pc = 0x831643E4; continue 'dispatch;
	// 8316436C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164370: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83164374: 38AB6D50  addi r5, r11, 0x6d50
	ctx.r[5].s64 = ctx.r[11].s64 + 27984;
	// 83164378: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316437C: 386001DC  li r3, 0x1dc
	ctx.r[3].s64 = 476;
	// 83164380: 4BFFB979  bl 0x8315fcf8
	ctx.lr = 0x83164384;
	sub_8315FCF8(ctx, base);
	// 83164384: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164388: 41820010  beq 0x83164398
	if ctx.cr[0].eq {
	pc = 0x83164398; continue 'dispatch;
	}
	// 8316438C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83164390: 4BFFFD29  bl 0x831640b8
	ctx.lr = 0x83164394;
	sub_831640B8(ctx, base);
	// 83164394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164398: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316439C: 409A001C  bne cr6, 0x831643b8
	if !ctx.cr[6].eq {
	pc = 0x831643B8; continue 'dispatch;
	}
	// 831643A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831643A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831643A8: 388B6D28  addi r4, r11, 0x6d28
	ctx.r[4].s64 = ctx.r[11].s64 + 27944;
	// 831643AC: 4BFFB76D  bl 0x8315fb18
	ctx.lr = 0x831643B0;
	sub_8315FB18(ctx, base);
	// 831643B0: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 831643B4: 4BFFFFAC  b 0x83164360
	pc = 0x83164360; continue 'dispatch;
	// 831643B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831643BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831643C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831643C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831643C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831643CC: 4E800421  bctrl
	ctx.lr = 0x831643D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831643D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831643D4: 4082000C  bne 0x831643e0
	if !ctx.cr[0].eq {
	pc = 0x831643E0; continue 'dispatch;
	}
	// 831643D8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831643DC: 4BFFFF84  b 0x83164360
	pc = 0x83164360; continue 'dispatch;
	// 831643E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831643E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831643E8: 48043DD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831643F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831643F0 size=220
    let mut pc: u32 = 0x831643F0;
    'dispatch: loop {
        match pc {
            0x831643F0 => {
    //   block [0x831643F0..0x831644CC)
	// 831643F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831643F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831643F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831643FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164408: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316440C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83164410: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83164414: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83164418: 4BFFB921  bl 0x8315fd38
	ctx.lr = 0x8316441C;
	sub_8315FD38(ctx, base);
	// 8316441C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83164420: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164424: 41820068  beq 0x8316448c
	if ctx.cr[0].eq {
	pc = 0x8316448C; continue 'dispatch;
	}
	// 83164428: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316442C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164430: 409A005C  bne cr6, 0x8316448c
	if !ctx.cr[6].eq {
	pc = 0x8316448C; continue 'dispatch;
	}
	// 83164434: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83164438: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 8316443C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83164440: 48005D09  bl 0x8316a148
	ctx.lr = 0x83164444;
	sub_8316A148(ctx, base);
	// 83164444: 907F005C  stw r3, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83164448: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316444C: 41820034  beq 0x83164480
	if ctx.cr[0].eq {
	pc = 0x83164480; continue 'dispatch;
	}
	// 83164450: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164454: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164458: 409A0028  bne cr6, 0x83164480
	if !ctx.cr[6].eq {
	pc = 0x83164480; continue 'dispatch;
	}
	// 8316445C: 93DF01D0  stw r30, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[30].u32 ) };
	// 83164460: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164464: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83164468: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8316446C: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 83164470: 806B8334  lwz r3, -0x7ccc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31948 as u32) ) } as u64;
	// 83164474: 48005DDD  bl 0x8316a250
	ctx.lr = 0x83164478;
	sub_8316A250(ctx, base);
	// 83164478: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316447C: 48000038  b 0x831644b4
	pc = 0x831644B4; continue 'dispatch;
	// 83164480: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164484: 388B6DC4  addi r4, r11, 0x6dc4
	ctx.r[4].s64 = ctx.r[11].s64 + 28100;
	// 83164488: 4800000C  b 0x83164494
	pc = 0x83164494; continue 'dispatch;
	// 8316448C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164490: 388B6D90  addi r4, r11, 0x6d90
	ctx.r[4].s64 = ctx.r[11].s64 + 28048;
	// 83164494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164498: 4BFFB681  bl 0x8315fb18
	ctx.lr = 0x8316449C;
	sub_8315FB18(ctx, base);
	// 8316449C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831644A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831644A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831644A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831644AC: 4E800421  bctrl
	ctx.lr = 0x831644B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831644B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831644B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831644B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831644BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831644C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831644C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831644C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831644D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831644D0 size=112
    let mut pc: u32 = 0x831644D0;
    'dispatch: loop {
        match pc {
            0x831644D0 => {
    //   block [0x831644D0..0x83164540)
	// 831644D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831644D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831644D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831644DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831644E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831644E4: 3BE3FE28  addi r31, r3, -0x1d8
	ctx.r[31].s64 = ctx.r[3].s64 + -472;
	// 831644E8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831644EC: 8163FE28  lwz r11, -0x1d8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-472 as u32) ) } as u64;
	// 831644F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831644F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831644F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831644FC: 4E800421  bctrl
	ctx.lr = 0x83164500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164500: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164504: 41820028  beq 0x8316452c
	if ctx.cr[0].eq {
	pc = 0x8316452C; continue 'dispatch;
	}
	// 83164508: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316450C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83164510: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164514: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83164518: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316451C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164520: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83164524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164528: 4E800421  bctrl
	ctx.lr = 0x8316452C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316452C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83164530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316453C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164540 size=216
    let mut pc: u32 = 0x83164540;
    'dispatch: loop {
        match pc {
            0x83164540 => {
    //   block [0x83164540..0x83164618)
	// 83164540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164544: 48043C1D  bl 0x831a8160
	ctx.lr = 0x83164548;
	sub_831A8130(ctx, base);
	// 83164548: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316454C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83164550: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164554: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83164558: 3BAB81F0  addi r29, r11, -0x7e10
	ctx.r[29].s64 = ctx.r[11].s64 + -32272;
	// 8316455C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83164560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83164564: 419A0074  beq cr6, 0x831645d8
	if ctx.cr[6].eq {
	pc = 0x831645D8; continue 'dispatch;
	}
	// 83164568: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316456C: 48005B9D  bl 0x8316a108
	ctx.lr = 0x83164570;
	sub_8316A108(ctx, base);
	// 83164570: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83164574: 41820054  beq 0x831645c8
	if ctx.cr[0].eq {
	pc = 0x831645C8; continue 'dispatch;
	}
	// 83164578: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316457C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83164580: 388B6DF8  addi r4, r11, 0x6df8
	ctx.r[4].s64 = ctx.r[11].s64 + 28152;
	// 83164584: 4BFFB595  bl 0x8315fb18
	ctx.lr = 0x83164588;
	sub_8315FB18(ctx, base);
	// 83164588: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316458C: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 83164590: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83164594: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316459C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 831645A0: 48005CF9  bl 0x8316a298
	ctx.lr = 0x831645A4;
	sub_8316A298(ctx, base);
	// 831645A4: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831645A8: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 831645AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831645B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831645B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831645B8: 4E800421  bctrl
	ctx.lr = 0x831645BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831645BC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 831645C0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831645C4: 409AFFCC  bne cr6, 0x83164590
	if !ctx.cr[6].eq {
	pc = 0x83164590; continue 'dispatch;
	}
	// 831645C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831645CC: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 831645D0: 48005AC9  bl 0x8316a098
	ctx.lr = 0x831645D4;
	sub_8316A098(ctx, base);
	// 831645D4: 935E005C  stw r26, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 831645D8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831645DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831645E0: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 831645E4: 806B8334  lwz r3, -0x7ccc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31948 as u32) ) } as u64;
	// 831645E8: 48005CB1  bl 0x8316a298
	ctx.lr = 0x831645EC;
	sub_8316A298(ctx, base);
	// 831645EC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 831645F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831645F4: 419A001C  beq cr6, 0x83164610
	if ctx.cr[6].eq {
	pc = 0x83164610; continue 'dispatch;
	}
	// 831645F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831645FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83164600: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164608: 4E800421  bctrl
	ctx.lr = 0x8316460C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316460C: 935E0014  stw r26, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 83164610: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83164614: 48043B9C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164618 size=336
    let mut pc: u32 = 0x83164618;
    'dispatch: loop {
        match pc {
            0x83164618 => {
    //   block [0x83164618..0x83164768)
	// 83164618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316461C: 48043B51  bl 0x831a816c
	ctx.lr = 0x83164620;
	sub_831A8130(ctx, base);
	// 83164620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164624: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83164628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316462C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83164630: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83164634: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164638: 8163FE38  lwz r11, -0x1c8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-456 as u32) ) } as u64;
	// 8316463C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164640: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83164644: 814BFE38  lwz r10, -0x1c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-456 as u32) ) } as u64;
	// 83164648: 386BFE38  addi r3, r11, -0x1c8
	ctx.r[3].s64 = ctx.r[11].s64 + -456;
	// 8316464C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164654: 4E800421  bctrl
	ctx.lr = 0x83164658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164658: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8316465C: 418200E8  beq 0x83164744
	if ctx.cr[0].eq {
	pc = 0x83164744; continue 'dispatch;
	}
	// 83164660: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164664: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164668: 409A00DC  bne cr6, 0x83164744
	if !ctx.cr[6].eq {
	pc = 0x83164744; continue 'dispatch;
	}
	// 8316466C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164670: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83164674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83164678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316467C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83164680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164684: 4E800421  bctrl
	ctx.lr = 0x83164688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164688: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316468C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164690: 409A009C  bne cr6, 0x8316472c
	if !ctx.cr[6].eq {
	pc = 0x8316472C; continue 'dispatch;
	}
	// 83164694: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164698: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316469C: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 831646A0: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 831646A4: 48000014  b 0x831646b8
	pc = 0x831646B8; continue 'dispatch;
	// 831646A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831646AC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831646B0: 419A0044  beq cr6, 0x831646f4
	if ctx.cr[6].eq {
	pc = 0x831646F4; continue 'dispatch;
	}
	// 831646B4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 831646B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831646BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831646C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831646C4: 4E800421  bctrl
	ctx.lr = 0x831646C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831646C8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831646CC: 409AFFDC  bne cr6, 0x831646a8
	if !ctx.cr[6].eq {
	pc = 0x831646A8; continue 'dispatch;
	}
	// 831646D0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831646D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831646D8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831646DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831646E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831646E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831646E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831646EC: 4E800421  bctrl
	ctx.lr = 0x831646F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831646F0: 4800006C  b 0x8316475c
	pc = 0x8316475C; continue 'dispatch;
	// 831646F4: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 831646F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831646FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164704: 4E800421  bctrl
	ctx.lr = 0x83164708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164708: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316470C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83164710: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83164714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164718: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316471C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164720: 4E800421  bctrl
	ctx.lr = 0x83164724;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164724: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83164728: 48000038  b 0x83164760
	pc = 0x83164760; continue 'dispatch;
	// 8316472C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164730: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164734: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164738: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316473C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164740: 4BFFFFA4  b 0x831646e4
	pc = 0x831646E4; continue 'dispatch;
	// 83164744: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316474C: 388B6E4C  addi r4, r11, 0x6e4c
	ctx.r[4].s64 = ctx.r[11].s64 + 28236;
	// 83164750: 4BFFB3C9  bl 0x8315fb18
	ctx.lr = 0x83164754;
	sub_8315FB18(ctx, base);
	// 83164754: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164758: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316475C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83164760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164764: 48043A58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164768 size=112
    let mut pc: u32 = 0x83164768;
    'dispatch: loop {
        match pc {
            0x83164768 => {
    //   block [0x83164768..0x831647D8)
	// 83164768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164774: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316477C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83164780: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83164784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164788: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316478C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164790: 4E800421  bctrl
	ctx.lr = 0x83164794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164794: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164798: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316479C: 419A0028  beq cr6, 0x831647c4
	if ctx.cr[6].eq {
	pc = 0x831647C4; continue 'dispatch;
	}
	// 831647A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831647A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831647A8: 388B6E74  addi r4, r11, 0x6e74
	ctx.r[4].s64 = ctx.r[11].s64 + 28276;
	// 831647AC: 4BFFB36D  bl 0x8315fb18
	ctx.lr = 0x831647B0;
	sub_8315FB18(ctx, base);
	// 831647B0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 831647B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831647B8: 409A000C  bne cr6, 0x831647c4
	if !ctx.cr[6].eq {
	pc = 0x831647C4; continue 'dispatch;
	}
	// 831647BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831647C0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831647C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831647C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831647CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831647D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831647D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831647D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831647D8 size=112
    let mut pc: u32 = 0x831647D8;
    'dispatch: loop {
        match pc {
            0x831647D8 => {
    //   block [0x831647D8..0x83164848)
	// 831647D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831647DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831647E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831647E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831647E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831647EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831647F0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831647F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831647F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831647FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164800: 4E800421  bctrl
	ctx.lr = 0x83164804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164804: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164808: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316480C: 419A0028  beq cr6, 0x83164834
	if ctx.cr[6].eq {
	pc = 0x83164834; continue 'dispatch;
	}
	// 83164810: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164818: 388B6EA4  addi r4, r11, 0x6ea4
	ctx.r[4].s64 = ctx.r[11].s64 + 28324;
	// 8316481C: 4BFFB2FD  bl 0x8315fb18
	ctx.lr = 0x83164820;
	sub_8315FB18(ctx, base);
	// 83164820: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164824: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164828: 409A000C  bne cr6, 0x83164834
	if !ctx.cr[6].eq {
	pc = 0x83164834; continue 'dispatch;
	}
	// 8316482C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164830: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316483C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164848 size=108
    let mut pc: u32 = 0x83164848;
    'dispatch: loop {
        match pc {
            0x83164848 => {
    //   block [0x83164848..0x831648B4)
	// 83164848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164854: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164858: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316485C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164860: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83164864: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164868: 4BFFFAC1  bl 0x83164328
	ctx.lr = 0x8316486C;
	sub_83164328(ctx, base);
	// 8316486C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164870: 41820024  beq 0x83164894
	if ctx.cr[0].eq {
	pc = 0x83164894; continue 'dispatch;
	}
	// 83164874: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164878: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316487C: 409A0018  bne cr6, 0x83164894
	if !ctx.cr[6].eq {
	pc = 0x83164894; continue 'dispatch;
	}
	// 83164880: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164884: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164888: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8316488C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83164890: 48000010  b 0x831648a0
	pc = 0x831648A0; continue 'dispatch;
	// 83164894: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316489C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831648A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831648A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831648A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831648AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831648B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831648B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831648B8 size=116
    let mut pc: u32 = 0x831648B8;
    'dispatch: loop {
        match pc {
            0x831648B8 => {
    //   block [0x831648B8..0x8316492C)
	// 831648B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831648BC: 480438B1  bl 0x831a816c
	ctx.lr = 0x831648C0;
	sub_831A8130(ctx, base);
	// 831648C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831648C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831648C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831648CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831648D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831648D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831648D8: 4BFFFF71  bl 0x83164848
	ctx.lr = 0x831648DC;
	sub_83164848(ctx, base);
	// 831648DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831648E0: 41820038  beq 0x83164918
	if ctx.cr[0].eq {
	pc = 0x83164918; continue 'dispatch;
	}
	// 831648E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831648E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831648EC: 409A002C  bne cr6, 0x83164918
	if !ctx.cr[6].eq {
	pc = 0x83164918; continue 'dispatch;
	}
	// 831648F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831648F4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831648F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831648FC: 38AA8328  addi r5, r10, -0x7cd8
	ctx.r[5].s64 = ctx.r[10].s64 + -31960;
	// 83164900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164904: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316490C: 4E800421  bctrl
	ctx.lr = 0x83164910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164914: 48000010  b 0x83164924
	pc = 0x83164924; continue 'dispatch;
	// 83164918: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316491C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164920: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164928: 48043894  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164930 size=12
    let mut pc: u32 = 0x83164930;
    'dispatch: loop {
        match pc {
            0x83164930 => {
    //   block [0x83164930..0x8316493C)
	// 83164930: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164934: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164938: 4BFFFCE0  b 0x83164618
	sub_83164618(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164940 size=12
    let mut pc: u32 = 0x83164940;
    'dispatch: loop {
        match pc {
            0x83164940 => {
    //   block [0x83164940..0x8316494C)
	// 83164940: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164944: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164948: 4BFFFB88  b 0x831644d0
	sub_831644D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164950 size=12
    let mut pc: u32 = 0x83164950;
    'dispatch: loop {
        match pc {
            0x83164950 => {
    //   block [0x83164950..0x8316495C)
	// 83164950: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164954: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164958: 480002C8  b 0x83164c20
	sub_83164C20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164960 size=96
    let mut pc: u32 = 0x83164960;
    'dispatch: loop {
        match pc {
            0x83164960 => {
    //   block [0x83164960..0x831649C0)
	// 83164960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316496C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164974: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83164978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316497C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164980: 816A8330  lwz r11, -0x7cd0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31952 as u32) ) } as u64;
	// 83164984: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83164988: 916A8330  stw r11, -0x7cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31952 as u32), ctx.r[11].u32 ) };
	// 8316498C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83164990: 409A001C  bne cr6, 0x831649ac
	if !ctx.cr[6].eq {
	pc = 0x831649AC; continue 'dispatch;
	}
	// 83164994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83164998: 4BFFF841  bl 0x831641d8
	ctx.lr = 0x8316499C;
	sub_831641D8(ctx, base);
	// 8316499C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831649A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831649A4: 419A0008  beq cr6, 0x831649ac
	if ctx.cr[6].eq {
	pc = 0x831649AC; continue 'dispatch;
	}
	// 831649A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831649AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831649B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831649B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831649B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831649BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831649C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831649C0 size=28
    let mut pc: u32 = 0x831649C0;
    'dispatch: loop {
        match pc {
            0x831649C0 => {
    //   block [0x831649C0..0x831649DC)
	// 831649C0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831649C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831649C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831649CC: 816A8330  lwz r11, -0x7cd0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31952 as u32) ) } as u64;
	// 831649D0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831649D4: 916A8330  stw r11, -0x7cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31952 as u32), ctx.r[11].u32 ) };
	// 831649D8: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831649DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831649DC size=12
    let mut pc: u32 = 0x831649DC;
    'dispatch: loop {
        match pc {
            0x831649DC => {
    //   block [0x831649DC..0x831649E8)
	// 831649DC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831649E0: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 831649E4: 4BFFF87C  b 0x83164260
	sub_83164260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831649E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831649E8 size=4
    let mut pc: u32 = 0x831649E8;
    'dispatch: loop {
        match pc {
            0x831649E8 => {
    //   block [0x831649E8..0x831649EC)
	// 831649E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831649F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831649F0 size=88
    let mut pc: u32 = 0x831649F0;
    'dispatch: loop {
        match pc {
            0x831649F0 => {
    //   block [0x831649F0..0x83164A48)
	// 831649F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831649F4: 48043775  bl 0x831a8168
	ctx.lr = 0x831649F8;
	sub_831A8130(ctx, base);
	// 831649F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831649FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164A00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83164A04: 3BDFFE28  addi r30, r31, -0x1d8
	ctx.r[30].s64 = ctx.r[31].s64 + -472;
	// 83164A08: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83164A0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164A10: 4BFFFD59  bl 0x83164768
	ctx.lr = 0x83164A14;
	sub_83164768(ctx, base);
	// 83164A14: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164A18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83164A1C: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 83164A20: 387FFEE8  addi r3, r31, -0x118
	ctx.r[3].s64 = ctx.r[31].s64 + -280;
	// 83164A24: 4800511D  bl 0x83169b40
	ctx.lr = 0x83164A28;
	sub_83169B40(ctx, base);
	// 83164A28: 815FFE80  lwz r10, -0x180(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-384 as u32) ) } as u64;
	// 83164A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164A30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164A34: 917FFE80  stw r11, -0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-384 as u32), ctx.r[11].u32 ) };
	// 83164A38: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83164A3C: 4BFFFD9D  bl 0x831647d8
	ctx.lr = 0x83164A40;
	sub_831647D8(ctx, base);
	// 83164A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164A44: 48043774  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164A48 size=88
    let mut pc: u32 = 0x83164A48;
    'dispatch: loop {
        match pc {
            0x83164A48 => {
    //   block [0x83164A48..0x83164AA0)
	// 83164A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164A4C: 4804371D  bl 0x831a8168
	ctx.lr = 0x83164A50;
	sub_831A8130(ctx, base);
	// 83164A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164A58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83164A5C: 3BDFFE28  addi r30, r31, -0x1d8
	ctx.r[30].s64 = ctx.r[31].s64 + -472;
	// 83164A60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164A64: 4BFFFD05  bl 0x83164768
	ctx.lr = 0x83164A68;
	sub_83164768(ctx, base);
	// 83164A68: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164A6C: 387FFEE8  addi r3, r31, -0x118
	ctx.r[3].s64 = ctx.r[31].s64 + -280;
	// 83164A70: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 83164A74: 480051FD  bl 0x83169c70
	ctx.lr = 0x83164A78;
	sub_83169C70(ctx, base);
	// 83164A78: 817FFE80  lwz r11, -0x180(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-384 as u32) ) } as u64;
	// 83164A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83164A80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83164A84: 915FFE80  stw r10, -0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-384 as u32), ctx.r[10].u32 ) };
	// 83164A88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164A8C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164A90: 4BFFFD49  bl 0x831647d8
	ctx.lr = 0x83164A94;
	sub_831647D8(ctx, base);
	// 83164A94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83164A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164A9C: 4804371C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164AA0 size=96
    let mut pc: u32 = 0x83164AA0;
    'dispatch: loop {
        match pc {
            0x83164AA0 => {
    //   block [0x83164AA0..0x83164B00)
	// 83164AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164AA4: 480436C1  bl 0x831a8164
	ctx.lr = 0x83164AA8;
	sub_831A8130(ctx, base);
	// 83164AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164AB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83164AB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83164AB8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83164ABC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83164AC0: 4BFFFCA9  bl 0x83164768
	ctx.lr = 0x83164AC4;
	sub_83164768(ctx, base);
	// 83164AC4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164AC8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83164ACC: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 83164AD0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83164AD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83164AD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83164ADC: 480053FD  bl 0x83169ed8
	ctx.lr = 0x83164AE0;
	sub_83169ED8(ctx, base);
	// 83164AE0: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164AE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164AE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164AEC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164AF0: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83164AF4: 4BFFFCE5  bl 0x831647d8
	ctx.lr = 0x83164AF8;
	sub_831647D8(ctx, base);
	// 83164AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164AFC: 480436B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164B00 size=96
    let mut pc: u32 = 0x83164B00;
    'dispatch: loop {
        match pc {
            0x83164B00 => {
    //   block [0x83164B00..0x83164B60)
	// 83164B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164B0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164B14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83164B18: 807F01D0  lwz r3, 0x1d0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(464 as u32) ) } as u64;
	// 83164B1C: 48007505  bl 0x8316c020
	ctx.lr = 0x83164B20;
	sub_8316C020(ctx, base);
	// 83164B20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164B24: 41820010  beq 0x83164b34
	if ctx.cr[0].eq {
	pc = 0x83164B34; continue 'dispatch;
	}
	// 83164B28: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164B2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164B30: 419A001C  beq cr6, 0x83164b4c
	if ctx.cr[6].eq {
	pc = 0x83164B4C; continue 'dispatch;
	}
	// 83164B34: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164B38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164B3C: 409A000C  bne cr6, 0x83164b48
	if !ctx.cr[6].eq {
	pc = 0x83164B48; continue 'dispatch;
	}
	// 83164B40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164B44: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164B48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164B58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164B60 size=80
    let mut pc: u32 = 0x83164B60;
    'dispatch: loop {
        match pc {
            0x83164B60 => {
    //   block [0x83164B60..0x83164BB0)
	// 83164B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164B64: 48043609  bl 0x831a816c
	ctx.lr = 0x83164B68;
	sub_831A8130(ctx, base);
	// 83164B68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164B70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83164B74: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83164B78: 4BFFFBF1  bl 0x83164768
	ctx.lr = 0x83164B7C;
	sub_83164768(ctx, base);
	// 83164B7C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164B80: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83164B84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83164B88: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83164B8C: 480056C5  bl 0x8316a250
	ctx.lr = 0x83164B90;
	sub_8316A250(ctx, base);
	// 83164B90: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164B94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164B9C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164BA0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83164BA4: 4BFFFC35  bl 0x831647d8
	ctx.lr = 0x83164BA8;
	sub_831647D8(ctx, base);
	// 83164BA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164BAC: 48043610  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164BB0 size=80
    let mut pc: u32 = 0x83164BB0;
    'dispatch: loop {
        match pc {
            0x83164BB0 => {
    //   block [0x83164BB0..0x83164C00)
	// 83164BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164BB4: 480435B9  bl 0x831a816c
	ctx.lr = 0x83164BB8;
	sub_831A8130(ctx, base);
	// 83164BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164BC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83164BC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83164BC8: 4BFFFBA1  bl 0x83164768
	ctx.lr = 0x83164BCC;
	sub_83164768(ctx, base);
	// 83164BCC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164BD0: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83164BD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83164BD8: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83164BDC: 480056BD  bl 0x8316a298
	ctx.lr = 0x83164BE0;
	sub_8316A298(ctx, base);
	// 83164BE0: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164BE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164BEC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164BF0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83164BF4: 4BFFFBE5  bl 0x831647d8
	ctx.lr = 0x83164BF8;
	sub_831647D8(ctx, base);
	// 83164BF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164BFC: 480435C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164C00 size=12
    let mut pc: u32 = 0x83164C00;
    'dispatch: loop {
        match pc {
            0x83164C00 => {
    //   block [0x83164C00..0x83164C0C)
	// 83164C00: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164C04: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164C08: 4BFFFE40  b 0x83164a48
	sub_83164A48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164C10 size=12
    let mut pc: u32 = 0x83164C10;
    'dispatch: loop {
        match pc {
            0x83164C10 => {
    //   block [0x83164C10..0x83164C1C)
	// 83164C10: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164C14: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164C18: 4BFFFDD8  b 0x831649f0
	sub_831649F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164C20 size=96
    let mut pc: u32 = 0x83164C20;
    'dispatch: loop {
        match pc {
            0x83164C20 => {
    //   block [0x83164C20..0x83164C80)
	// 83164C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83164C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164C34: 3BE3FE28  addi r31, r3, -0x1d8
	ctx.r[31].s64 = ctx.r[3].s64 + -472;
	// 83164C38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83164C3C: 387F01D8  addi r3, r31, 0x1d8
	ctx.r[3].s64 = ctx.r[31].s64 + 472;
	// 83164C40: 4BFFF551  bl 0x83164190
	ctx.lr = 0x83164C44;
	sub_83164190(ctx, base);
	// 83164C44: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164C48: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83164C4C: 396B6C54  addi r11, r11, 0x6c54
	ctx.r[11].s64 = ctx.r[11].s64 + 27732;
	// 83164C50: 917F01D8  stw r11, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[11].u32 ) };
	// 83164C54: 41820010  beq 0x83164c64
	if ctx.cr[0].eq {
	pc = 0x83164C64; continue 'dispatch;
	}
	// 83164C58: 388001DC  li r4, 0x1dc
	ctx.r[4].s64 = 476;
	// 83164C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164C60: 4BFFB021  bl 0x8315fc80
	ctx.lr = 0x83164C64;
	sub_8315FC80(ctx, base);
	// 83164C64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164C68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164C74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83164C78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164C80 size=140
    let mut pc: u32 = 0x83164C80;
    'dispatch: loop {
        match pc {
            0x83164C80 => {
    //   block [0x83164C80..0x83164D0C)
	// 83164C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164C84: 480434E9  bl 0x831a816c
	ctx.lr = 0x83164C88;
	sub_831A8130(ctx, base);
	// 83164C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164C90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164C94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83164C98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164C9C: 4E800421  bctrl
	ctx.lr = 0x83164CA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164CA0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83164CA4: 41820048  beq 0x83164cec
	if ctx.cr[0].eq {
	pc = 0x83164CEC; continue 'dispatch;
	}
	// 83164CA8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83164CB0: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 83164CB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164CB8: 409A0034  bne cr6, 0x83164cec
	if !ctx.cr[6].eq {
	pc = 0x83164CEC; continue 'dispatch;
	}
	// 83164CBC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164CC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83164CC4: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 83164CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164CCC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83164CD0: 48005F41  bl 0x8316ac10
	ctx.lr = 0x83164CD4;
	sub_8316AC10(ctx, base);
	// 83164CD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83164CD8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83164CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164CE0: 4BFFFE81  bl 0x83164b60
	ctx.lr = 0x83164CE4;
	sub_83164B60(ctx, base);
	// 83164CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83164CE8: 4800001C  b 0x83164d04
	pc = 0x83164D04; continue 'dispatch;
	// 83164CEC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83164CF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164CF4: 409A000C  bne cr6, 0x83164d00
	if !ctx.cr[6].eq {
	pc = 0x83164D00; continue 'dispatch;
	}
	// 83164CF8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164CFC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83164D00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164D08: 480434B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164D10 size=80
    let mut pc: u32 = 0x83164D10;
    'dispatch: loop {
        match pc {
            0x83164D10 => {
    //   block [0x83164D10..0x83164D60)
	// 83164D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164D14: 48043455  bl 0x831a8168
	ctx.lr = 0x83164D18;
	sub_831A8130(ctx, base);
	// 83164D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164D1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83164D20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83164D24: 3BFEFE28  addi r31, r30, -0x1d8
	ctx.r[31].s64 = ctx.r[30].s64 + -472;
	// 83164D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164D2C: 4BFFFA3D  bl 0x83164768
	ctx.lr = 0x83164D30;
	sub_83164768(ctx, base);
	// 83164D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164D34: 4BFFFF4D  bl 0x83164c80
	ctx.lr = 0x83164D38;
	sub_83164C80(ctx, base);
	// 83164D38: 817EFE80  lwz r11, -0x180(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-384 as u32) ) } as u64;
	// 83164D3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83164D40: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83164D44: 915EFE80  stw r10, -0x180(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-384 as u32), ctx.r[10].u32 ) };
	// 83164D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164D4C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164D50: 4BFFFA89  bl 0x831647d8
	ctx.lr = 0x83164D54;
	sub_831647D8(ctx, base);
	// 83164D54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83164D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83164D5C: 4804345C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164D60 size=12
    let mut pc: u32 = 0x83164D60;
    'dispatch: loop {
        match pc {
            0x83164D60 => {
    //   block [0x83164D60..0x83164D6C)
	// 83164D60: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83164D64: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83164D68: 4BFFFFA8  b 0x83164d10
	sub_83164D10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164D70 size=252
    let mut pc: u32 = 0x83164D70;
    'dispatch: loop {
        match pc {
            0x83164D70 => {
    //   block [0x83164D70..0x83164E6C)
	// 83164D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164D78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83164D7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164D88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83164D8C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83164D90: 419A001C  beq cr6, 0x83164dac
	if ctx.cr[6].eq {
	pc = 0x83164DAC; continue 'dispatch;
	}
	// 83164D94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164D98: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83164D9C: 396B6F98  addi r11, r11, 0x6f98
	ctx.r[11].s64 = ctx.r[11].s64 + 28568;
	// 83164DA0: 394A6ED8  addi r10, r10, 0x6ed8
	ctx.r[10].s64 = ctx.r[10].s64 + 28376;
	// 83164DA4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83164DA8: 915F0354  stw r10, 0x354(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(852 as u32), ctx.r[10].u32 ) };
	// 83164DAC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83164DB0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164DB4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83164DB8: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83164DBC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83164DC0: 396B6F94  addi r11, r11, 0x6f94
	ctx.r[11].s64 = ctx.r[11].s64 + 28564;
	// 83164DC4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164DC8: 394A6F38  addi r10, r10, 0x6f38
	ctx.r[10].s64 = ctx.r[10].s64 + 28472;
	// 83164DCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164DD0: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 83164DD4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164DD8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83164DDC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83164DE0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164DE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164DE8: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83164DEC: 396BFCBC  addi r11, r11, -0x344
	ctx.r[11].s64 = ctx.r[11].s64 + -836;
	// 83164DF0: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83164DF4: 48004CF5  bl 0x83169ae8
	ctx.lr = 0x83164DF8;
	sub_83169AE8(ctx, base);
	// 83164DF8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83164DFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83164E00: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83164E04: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83164E08: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83164E0C: 480433D5  bl 0x831a81e0
	ctx.lr = 0x83164E10;
	sub_831A81E0(ctx, base);
	// 83164E10: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 83164E14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83164E18: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83164E1C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83164E20: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83164E24: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 83164E28: 480433B9  bl 0x831a81e0
	ctx.lr = 0x83164E2C;
	sub_831A81E0(ctx, base);
	// 83164E2C: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 83164E30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83164E34: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 83164E38: 480433A9  bl 0x831a81e0
	ctx.lr = 0x83164E3C;
	sub_831A81E0(ctx, base);
	// 83164E3C: 93DF033C  stw r30, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[30].u32 ) };
	// 83164E40: 93DF0340  stw r30, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[30].u32 ) };
	// 83164E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83164E48: 93DF0344  stw r30, 0x344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(836 as u32), ctx.r[30].u32 ) };
	// 83164E4C: 93DF0348  stw r30, 0x348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[30].u32 ) };
	// 83164E50: 93DF034C  stw r30, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[30].u32 ) };
	// 83164E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83164E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83164E70 size=68
    let mut pc: u32 = 0x83164E70;
    'dispatch: loop {
        match pc {
            0x83164E70 => {
    //   block [0x83164E70..0x83164EB4)
	// 83164E70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83164E74: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83164E78: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 83164E7C: 394A6F94  addi r10, r10, 0x6f94
	ctx.r[10].s64 = ctx.r[10].s64 + 28564;
	// 83164E80: 39086F38  addi r8, r8, 0x6f38
	ctx.r[8].s64 = ctx.r[8].s64 + 28472;
	// 83164E84: 812BFCBC  lwz r9, -0x344(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83164E88: 386BFED8  addi r3, r11, -0x128
	ctx.r[3].s64 = ctx.r[11].s64 + -296;
	// 83164E8C: 914BFCAC  stw r10, -0x354(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-852 as u32), ctx.r[10].u32 ) };
	// 83164E90: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164E94: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83164E98: 910AFCBC  stw r8, -0x344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-836 as u32), ctx.r[8].u32 ) };
	// 83164E9C: 814BFCBC  lwz r10, -0x344(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83164EA0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164EA4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83164EA8: 394AFCBC  addi r10, r10, -0x344
	ctx.r[10].s64 = ctx.r[10].s64 + -836;
	// 83164EAC: 914BFCB8  stw r10, -0x348(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-840 as u32), ctx.r[10].u32 ) };
	// 83164EB0: 4BF62C30  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164EB8 size=136
    let mut pc: u32 = 0x83164EB8;
    'dispatch: loop {
        match pc {
            0x83164EB8 => {
    //   block [0x83164EB8..0x83164F40)
	// 83164EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83164EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83164EC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83164ECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164ED0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164ED4: 4BFFAB4D  bl 0x8315fa20
	ctx.lr = 0x83164ED8;
	sub_8315FA20(ctx, base);
	// 83164ED8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164EDC: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 83164EE0: 48007D21  bl 0x8316cc00
	ctx.lr = 0x83164EE4;
	sub_8316CC00(ctx, base);
	// 83164EE4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164EE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83164EEC: 386B83A0  addi r3, r11, -0x7c60
	ctx.r[3].s64 = ctx.r[11].s64 + -31840;
	// 83164EF0: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83164EF4: 48005255  bl 0x8316a148
	ctx.lr = 0x83164EF8;
	sub_8316A148(ctx, base);
	// 83164EF8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164EFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83164F00: 906B839C  stw r3, -0x7c64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-31844 as u32), ctx.r[3].u32 ) };
	// 83164F04: 41820010  beq 0x83164f14
	if ctx.cr[0].eq {
	pc = 0x83164F14; continue 'dispatch;
	}
	// 83164F08: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83164F0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83164F10: 419A001C  beq cr6, 0x83164f2c
	if ctx.cr[6].eq {
	pc = 0x83164F2C; continue 'dispatch;
	}
	// 83164F14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83164F1C: 388B6FA0  addi r4, r11, 0x6fa0
	ctx.r[4].s64 = ctx.r[11].s64 + 28576;
	// 83164F20: 4BFFABF9  bl 0x8315fb18
	ctx.lr = 0x83164F24;
	sub_8315FB18(ctx, base);
	// 83164F24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83164F28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164F2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83164F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83164F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83164F38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83164F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83164F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83164F40 size=196
    let mut pc: u32 = 0x83164F40;
    'dispatch: loop {
        match pc {
            0x83164F40 => {
    //   block [0x83164F40..0x83165004)
	// 83164F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83164F44: 48043221  bl 0x831a8164
	ctx.lr = 0x83164F48;
	sub_831A8130(ctx, base);
	// 83164F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83164F4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164F50: 3FC0833A  lis r30, -0x7cc6
	ctx.r[30].s64 = -2093350912;
	// 83164F54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83164F58: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164F5C: 3B8B81F0  addi r28, r11, -0x7e10
	ctx.r[28].s64 = ctx.r[11].s64 + -32272;
	// 83164F60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83164F64: 807E839C  lwz r3, -0x7c64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31844 as u32) ) } as u64;
	// 83164F68: 480051A1  bl 0x8316a108
	ctx.lr = 0x83164F6C;
	sub_8316A108(ctx, base);
	// 83164F6C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83164F70: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83164F74: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 83164F78: 4182005C  beq 0x83164fd4
	if ctx.cr[0].eq {
	pc = 0x83164FD4; continue 'dispatch;
	}
	// 83164F7C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83164F80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83164F84: 388B6FC0  addi r4, r11, 0x6fc0
	ctx.r[4].s64 = ctx.r[11].s64 + 28608;
	// 83164F88: 4BFFAB91  bl 0x8315fb18
	ctx.lr = 0x83164F8C;
	sub_8315FB18(ctx, base);
	// 83164F8C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83164F90: 807E839C  lwz r3, -0x7c64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31844 as u32) ) } as u64;
	// 83164F94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83164F98: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164F9C: 480052FD  bl 0x8316a298
	ctx.lr = 0x83164FA0;
	sub_8316A298(ctx, base);
	// 83164FA0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83164FA4: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 83164FA8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83164FAC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164FB0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83164FB4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83164FB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83164FBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83164FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83164FC4: 4E800421  bctrl
	ctx.lr = 0x83164FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83164FC8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83164FCC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83164FD0: 409AFFBC  bne cr6, 0x83164f8c
	if !ctx.cr[6].eq {
	pc = 0x83164F8C; continue 'dispatch;
	}
	// 83164FD4: 807E839C  lwz r3, -0x7c64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31844 as u32) ) } as u64;
	// 83164FD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83164FDC: 419A0014  beq cr6, 0x83164ff0
	if ctx.cr[6].eq {
	pc = 0x83164FF0; continue 'dispatch;
	}
	// 83164FE0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83164FE4: 480050B5  bl 0x8316a098
	ctx.lr = 0x83164FE8;
	sub_8316A098(ctx, base);
	// 83164FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83164FEC: 917E839C  stw r11, -0x7c64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-31844 as u32), ctx.r[11].u32 ) };
	// 83164FF0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83164FF4: 48007C85  bl 0x8316cc78
	ctx.lr = 0x83164FF8;
	sub_8316CC78(ctx, base);
	// 83164FF8: 4BFFA621  bl 0x8315f618
	ctx.lr = 0x83164FFC;
	sub_8315F618(ctx, base);
	// 83164FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165000: 480431B4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165008 size=432
    let mut pc: u32 = 0x83165008;
    'dispatch: loop {
        match pc {
            0x83165008 => {
    //   block [0x83165008..0x831651B8)
	// 83165008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316500C: 48043159  bl 0x831a8164
	ctx.lr = 0x83165010;
	sub_831A8130(ctx, base);
	// 83165010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165018: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316501C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83165020: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 83165024: 3BAA81F0  addi r29, r10, -0x7e10
	ctx.r[29].s64 = ctx.r[10].s64 + -32272;
	// 83165028: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316502C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165030: 419A005C  beq cr6, 0x8316508c
	if ctx.cr[6].eq {
	pc = 0x8316508C; continue 'dispatch;
	}
	// 83165034: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83165038: 480050D1  bl 0x8316a108
	ctx.lr = 0x8316503C;
	sub_8316A108(ctx, base);
	// 8316503C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83165040: 4182004C  beq 0x8316508c
	if ctx.cr[0].eq {
	pc = 0x8316508C; continue 'dispatch;
	}
	// 83165044: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165048: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316504C: 388B7060  addi r4, r11, 0x7060
	ctx.r[4].s64 = ctx.r[11].s64 + 28768;
	// 83165050: 4BFFAAC9  bl 0x8315fb18
	ctx.lr = 0x83165054;
	sub_8315FB18(ctx, base);
	// 83165054: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165058: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316505C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165060: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83165064: 48005235  bl 0x8316a298
	ctx.lr = 0x83165068;
	sub_8316A298(ctx, base);
	// 83165068: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316506C: 387EFFFC  addi r3, r30, -4
	ctx.r[3].s64 = ctx.r[30].s64 + -4;
	// 83165070: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83165074: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316507C: 4E800421  bctrl
	ctx.lr = 0x83165080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165080: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83165084: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83165088: 409AFFCC  bne cr6, 0x83165054
	if !ctx.cr[6].eq {
	pc = 0x83165054; continue 'dispatch;
	}
	// 8316508C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83165090: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165094: 419A0050  beq cr6, 0x831650e4
	if ctx.cr[6].eq {
	pc = 0x831650E4; continue 'dispatch;
	}
	// 83165098: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316509C: 4800506D  bl 0x8316a108
	ctx.lr = 0x831650A0;
	sub_8316A108(ctx, base);
	// 831650A0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831650A4: 41820040  beq 0x831650e4
	if ctx.cr[0].eq {
	pc = 0x831650E4; continue 'dispatch;
	}
	// 831650A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831650AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831650B0: 388B7010  addi r4, r11, 0x7010
	ctx.r[4].s64 = ctx.r[11].s64 + 28688;
	// 831650B4: 4BFFAA65  bl 0x8315fb18
	ctx.lr = 0x831650B8;
	sub_8315FB18(ctx, base);
	// 831650B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831650BC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831650C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831650C4: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 831650C8: 480051D1  bl 0x8316a298
	ctx.lr = 0x831650CC;
	sub_8316A298(ctx, base);
	// 831650CC: 387EFFFC  addi r3, r30, -4
	ctx.r[3].s64 = ctx.r[30].s64 + -4;
	// 831650D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831650D4: 48007F75  bl 0x8316d048
	ctx.lr = 0x831650D8;
	sub_8316D048(ctx, base);
	// 831650D8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 831650DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831650E0: 409AFFD8  bne cr6, 0x831650b8
	if !ctx.cr[6].eq {
	pc = 0x831650B8; continue 'dispatch;
	}
	// 831650E4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831650E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831650EC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 831650F0: 806B839C  lwz r3, -0x7c64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31844 as u32) ) } as u64;
	// 831650F4: 480051A5  bl 0x8316a298
	ctx.lr = 0x831650F8;
	sub_8316A298(ctx, base);
	// 831650F8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 831650FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83165100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165104: 419A0010  beq cr6, 0x83165114
	if ctx.cr[6].eq {
	pc = 0x83165114; continue 'dispatch;
	}
	// 83165108: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316510C: 48004F8D  bl 0x8316a098
	ctx.lr = 0x83165110;
	sub_8316A098(ctx, base);
	// 83165110: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83165114: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83165118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316511C: 419A0010  beq cr6, 0x8316512c
	if ctx.cr[6].eq {
	pc = 0x8316512C; continue 'dispatch;
	}
	// 83165120: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83165124: 48004F75  bl 0x8316a098
	ctx.lr = 0x83165128;
	sub_8316A098(ctx, base);
	// 83165128: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8316512C: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165130: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165134: 419A001C  beq cr6, 0x83165150
	if ctx.cr[6].eq {
	pc = 0x83165150; continue 'dispatch;
	}
	// 83165138: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316513C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83165140: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165148: 4E800421  bctrl
	ctx.lr = 0x8316514C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316514C: 93DF034C  stw r30, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[30].u32 ) };
	// 83165150: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83165154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165158: 419A001C  beq cr6, 0x83165174
	if ctx.cr[6].eq {
	pc = 0x83165174; continue 'dispatch;
	}
	// 8316515C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165160: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83165164: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316516C: 4E800421  bctrl
	ctx.lr = 0x83165170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165170: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83165174: 807F0348  lwz r3, 0x348(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(840 as u32) ) } as u64;
	// 83165178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316517C: 419A0010  beq cr6, 0x8316518c
	if ctx.cr[6].eq {
	pc = 0x8316518C; continue 'dispatch;
	}
	// 83165180: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83165184: 48007B3D  bl 0x8316ccc0
	ctx.lr = 0x83165188;
	sub_8316CCC0(ctx, base);
	// 83165188: 93DF0348  stw r30, 0x348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[30].u32 ) };
	// 8316518C: 807F0340  lwz r3, 0x340(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 83165190: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165194: 419A001C  beq cr6, 0x831651b0
	if ctx.cr[6].eq {
	pc = 0x831651B0; continue 'dispatch;
	}
	// 83165198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316519C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831651A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831651A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831651A8: 4E800421  bctrl
	ctx.lr = 0x831651AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831651AC: 93DF0340  stw r30, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[30].u32 ) };
	// 831651B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831651B4: 48043000  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831651B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831651B8 size=52
    let mut pc: u32 = 0x831651B8;
    'dispatch: loop {
        match pc {
            0x831651B8 => {
    //   block [0x831651B8..0x831651EC)
	// 831651B8: 8163FCBC  lwz r11, -0x344(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-836 as u32) ) } as u64;
	// 831651BC: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 831651C0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831651C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831651C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831651CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831651D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831651D4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831651D8: 386BFCBC  addi r3, r11, -0x344
	ctx.r[3].s64 = ctx.r[11].s64 + -836;
	// 831651DC: 816BFCBC  lwz r11, -0x344(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 831651E0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 831651E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831651E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831651F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831651F0 size=52
    let mut pc: u32 = 0x831651F0;
    'dispatch: loop {
        match pc {
            0x831651F0 => {
    //   block [0x831651F0..0x83165224)
	// 831651F0: 8163FCBC  lwz r11, -0x344(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-836 as u32) ) } as u64;
	// 831651F4: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 831651F8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831651FC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83165200: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83165204: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83165208: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316520C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83165210: 386BFCBC  addi r3, r11, -0x344
	ctx.r[3].s64 = ctx.r[11].s64 + -836;
	// 83165214: 816BFCBC  lwz r11, -0x344(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83165218: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316521C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165220: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165228 size=44
    let mut pc: u32 = 0x83165228;
    'dispatch: loop {
        match pc {
            0x83165228 => {
    //   block [0x83165228..0x83165254)
	// 83165228: 8163FCBC  lwz r11, -0x344(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-836 as u32) ) } as u64;
	// 8316522C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 83165230: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83165234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83165238: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316523C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83165240: 386BFCBC  addi r3, r11, -0x344
	ctx.r[3].s64 = ctx.r[11].s64 + -836;
	// 83165244: 816BFCBC  lwz r11, -0x344(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83165248: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316524C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165250: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165258 size=112
    let mut pc: u32 = 0x83165258;
    'dispatch: loop {
        match pc {
            0x83165258 => {
    //   block [0x83165258..0x831652C8)
	// 83165258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316525C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83165260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83165264: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316526C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165270: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165274: 480089F5  bl 0x8316dc68
	ctx.lr = 0x83165278;
	sub_8316DC68(ctx, base);
	// 83165278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316527C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165280: 409A0030  bne cr6, 0x831652b0
	if !ctx.cr[6].eq {
	pc = 0x831652B0; continue 'dispatch;
	}
	// 83165284: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165288: 48008F99  bl 0x8316e220
	ctx.lr = 0x8316528C;
	sub_8316E220(ctx, base);
	// 8316528C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83165290: 419A0020  beq cr6, 0x831652b0
	if ctx.cr[6].eq {
	pc = 0x831652B0; continue 'dispatch;
	}
	// 83165294: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165298: 48008F71  bl 0x8316e208
	ctx.lr = 0x8316529C;
	sub_8316E208(ctx, base);
	// 8316529C: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 831652A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831652A4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831652A8: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 831652AC: 48000008  b 0x831652b4
	pc = 0x831652B4; continue 'dispatch;
	// 831652B0: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 831652B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831652B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831652BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831652C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831652C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831652C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831652C8 size=104
    let mut pc: u32 = 0x831652C8;
    'dispatch: loop {
        match pc {
            0x831652C8 => {
    //   block [0x831652C8..0x83165330)
	// 831652C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831652CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831652D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831652D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831652D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831652DC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831652E0: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 831652E4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831652E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831652EC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831652F0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 831652F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831652F8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 831652FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165300: 4E800421  bctrl
	ctx.lr = 0x83165304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165304: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165308: 419A000C  beq cr6, 0x83165314
	if ctx.cr[6].eq {
	pc = 0x83165314; continue 'dispatch;
	}
	// 8316530C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83165310: 4800000C  b 0x8316531c
	pc = 0x8316531C; continue 'dispatch;
	// 83165314: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165318: 48009041  bl 0x8316e358
	ctx.lr = 0x8316531C;
	sub_8316E358(ctx, base);
	// 8316531C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83165320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83165324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83165328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316532C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165330 size=8
    let mut pc: u32 = 0x83165330;
    'dispatch: loop {
        match pc {
            0x83165330 => {
    //   block [0x83165330..0x83165338)
	// 83165330: 80630348  lwz r3, 0x348(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(840 as u32) ) } as u64;
	// 83165334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165338 size=16
    let mut pc: u32 = 0x83165338;
    'dispatch: loop {
        match pc {
            0x83165338 => {
    //   block [0x83165338..0x83165348)
	// 83165338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316533C: 3863FDD4  addi r3, r3, -0x22c
	ctx.r[3].s64 = ctx.r[3].s64 + -556;
	// 83165340: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165348 size=112
    let mut pc: u32 = 0x83165348;
    'dispatch: loop {
        match pc {
            0x83165348 => {
    //   block [0x83165348..0x831653B8)
	// 83165348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83165350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83165354: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316535C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165360: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83165364: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165368: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316536C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165370: 4E800421  bctrl
	ctx.lr = 0x83165374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165374: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165378: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316537C: 419A0028  beq cr6, 0x831653a4
	if ctx.cr[6].eq {
	pc = 0x831653A4; continue 'dispatch;
	}
	// 83165380: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165384: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165388: 388B70B0  addi r4, r11, 0x70b0
	ctx.r[4].s64 = ctx.r[11].s64 + 28848;
	// 8316538C: 4BFFA78D  bl 0x8315fb18
	ctx.lr = 0x83165390;
	sub_8315FB18(ctx, base);
	// 83165390: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165394: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165398: 409A000C  bne cr6, 0x831653a4
	if !ctx.cr[6].eq {
	pc = 0x831653A4; continue 'dispatch;
	}
	// 8316539C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831653A0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831653A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831653A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831653AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831653B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831653B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831653B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831653B8 size=112
    let mut pc: u32 = 0x831653B8;
    'dispatch: loop {
        match pc {
            0x831653B8 => {
    //   block [0x831653B8..0x83165428)
	// 831653B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831653BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831653C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831653C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831653C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831653CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831653D0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831653D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831653D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831653DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831653E0: 4E800421  bctrl
	ctx.lr = 0x831653E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831653E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831653E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831653EC: 419A0028  beq cr6, 0x83165414
	if ctx.cr[6].eq {
	pc = 0x83165414; continue 'dispatch;
	}
	// 831653F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831653F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831653F8: 388B70E0  addi r4, r11, 0x70e0
	ctx.r[4].s64 = ctx.r[11].s64 + 28896;
	// 831653FC: 4BFFA71D  bl 0x8315fb18
	ctx.lr = 0x83165400;
	sub_8315FB18(ctx, base);
	// 83165400: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165404: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165408: 409A000C  bne cr6, 0x83165414
	if !ctx.cr[6].eq {
	pc = 0x83165414; continue 'dispatch;
	}
	// 8316540C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165410: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316541C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83165420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83165424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165428 size=60
    let mut pc: u32 = 0x83165428;
    'dispatch: loop {
        match pc {
            0x83165428 => {
    //   block [0x83165428..0x83165464)
	// 83165428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316542C: 48042D41  bl 0x831a816c
	ctx.lr = 0x83165430;
	sub_831A8130(ctx, base);
	// 83165430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165434: 3BE3FCAC  addi r31, r3, -0x354
	ctx.r[31].s64 = ctx.r[3].s64 + -852;
	// 83165438: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316543C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165440: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165444: 4BFFFF05  bl 0x83165348
	ctx.lr = 0x83165448;
	sub_83165348(ctx, base);
	// 83165448: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316544C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83165450: 48007BF9  bl 0x8316d048
	ctx.lr = 0x83165454;
	sub_8316D048(ctx, base);
	// 83165454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165458: 4BFFFF61  bl 0x831653b8
	ctx.lr = 0x8316545C;
	sub_831653B8(ctx, base);
	// 8316545C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165460: 48042D5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165468 size=132
    let mut pc: u32 = 0x83165468;
    'dispatch: loop {
        match pc {
            0x83165468 => {
    //   block [0x83165468..0x831654EC)
	// 83165468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316546C: 48042CFD  bl 0x831a8168
	ctx.lr = 0x83165470;
	sub_831A8130(ctx, base);
	// 83165470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165478: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316547C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165480: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165484: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83165488: 817FFCBC  lwz r11, -0x344(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-836 as u32) ) } as u64;
	// 8316548C: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83165490: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83165494: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83165498: 386BFCBC  addi r3, r11, -0x344
	ctx.r[3].s64 = ctx.r[11].s64 + -836;
	// 8316549C: 816BFCBC  lwz r11, -0x344(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 831654A0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 831654A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831654A8: 4E800421  bctrl
	ctx.lr = 0x831654AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831654AC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831654B0: 419A0014  beq cr6, 0x831654c4
	if ctx.cr[6].eq {
	pc = 0x831654C4; continue 'dispatch;
	}
	// 831654B4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831654B8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831654BC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831654C0: 48000024  b 0x831654e4
	pc = 0x831654E4; continue 'dispatch;
	// 831654C4: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831654C8: 48008E19  bl 0x8316e2e0
	ctx.lr = 0x831654CC;
	sub_8316E2E0(ctx, base);
	// 831654CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831654D0: 4182FFE4  beq 0x831654b4
	if ctx.cr[0].eq {
	pc = 0x831654B4; continue 'dispatch;
	}
	// 831654D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831654D8: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831654DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831654E0: 48008E81  bl 0x8316e360
	ctx.lr = 0x831654E4;
	sub_8316E360(ctx, base);
	// 831654E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831654E8: 48042CD0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831654F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831654F0 size=132
    let mut pc: u32 = 0x831654F0;
    'dispatch: loop {
        match pc {
            0x831654F0 => {
    //   block [0x831654F0..0x83165574)
	// 831654F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831654F4: 48042C75  bl 0x831a8168
	ctx.lr = 0x831654F8;
	sub_831A8130(ctx, base);
	// 831654F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831654FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165500: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83165504: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165508: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316550C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83165510: 817FFCBC  lwz r11, -0x344(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83165514: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83165518: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316551C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83165520: 386BFCBC  addi r3, r11, -0x344
	ctx.r[3].s64 = ctx.r[11].s64 + -836;
	// 83165524: 816BFCBC  lwz r11, -0x344(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-836 as u32) ) } as u64;
	// 83165528: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8316552C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165530: 4E800421  bctrl
	ctx.lr = 0x83165534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165534: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165538: 419A0014  beq cr6, 0x8316554c
	if ctx.cr[6].eq {
	pc = 0x8316554C; continue 'dispatch;
	}
	// 8316553C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83165540: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83165544: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165548: 48000024  b 0x8316556c
	pc = 0x8316556C; continue 'dispatch;
	// 8316554C: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83165550: 48008D91  bl 0x8316e2e0
	ctx.lr = 0x83165554;
	sub_8316E2E0(ctx, base);
	// 83165554: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83165558: 4182FFE4  beq 0x8316553c
	if ctx.cr[0].eq {
	pc = 0x8316553C; continue 'dispatch;
	}
	// 8316555C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165560: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83165564: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165568: 48008E11  bl 0x8316e378
	ctx.lr = 0x8316556C;
	sub_8316E378(ctx, base);
	// 8316556C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165570: 48042C48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165578 size=104
    let mut pc: u32 = 0x83165578;
    'dispatch: loop {
        match pc {
            0x83165578 => {
    //   block [0x83165578..0x831655E0)
	// 83165578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316557C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83165580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83165584: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316558C: 8063034C  lwz r3, 0x34c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165594: 480090C5  bl 0x8316e658
	ctx.lr = 0x83165598;
	sub_8316E658(ctx, base);
	// 83165598: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316559C: 409A001C  bne cr6, 0x831655b8
	if !ctx.cr[6].eq {
	pc = 0x831655B8; continue 'dispatch;
	}
	// 831655A0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831655A4: E9210058  ld r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831655A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831655AC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831655B0: F93F0008  std r9, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 831655B4: 48000014  b 0x831655c8
	pc = 0x831655C8; continue 'dispatch;
	// 831655B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831655BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831655C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831655C4: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 831655C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831655CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831655D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831655D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831655D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831655DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831655E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831655E0 size=8
    let mut pc: u32 = 0x831655E0;
    'dispatch: loop {
        match pc {
            0x831655E0 => {
    //   block [0x831655E0..0x831655E8)
	// 831655E0: 8063034C  lwz r3, 0x34c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 831655E4: 48008D3C  b 0x8316e320
	sub_8316E320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831655E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831655E8 size=112
    let mut pc: u32 = 0x831655E8;
    'dispatch: loop {
        match pc {
            0x831655E8 => {
    //   block [0x831655E8..0x83165658)
	// 831655E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831655EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831655F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831655F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831655F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831655FC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83165600: 38AB7120  addi r5, r11, 0x7120
	ctx.r[5].s64 = ctx.r[11].s64 + 28960;
	// 83165604: 38600138  li r3, 0x138
	ctx.r[3].s64 = 312;
	// 83165608: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316560C: 4BFFA6ED  bl 0x8315fcf8
	ctx.lr = 0x83165610;
	sub_8315FCF8(ctx, base);
	// 83165610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165614: 41820010  beq 0x83165624
	if ctx.cr[0].eq {
	pc = 0x83165624; continue 'dispatch;
	}
	// 83165618: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316561C: 48008465  bl 0x8316da80
	ctx.lr = 0x83165620;
	sub_8316DA80(ctx, base);
	// 83165620: 48000008  b 0x83165628
	pc = 0x83165628; continue 'dispatch;
	// 83165624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316562C: 409A0018  bne cr6, 0x83165644
	if !ctx.cr[6].eq {
	pc = 0x83165644; continue 'dispatch;
	}
	// 83165630: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165634: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83165638: 388B7110  addi r4, r11, 0x7110
	ctx.r[4].s64 = ctx.r[11].s64 + 28944;
	// 8316563C: 4BFFA505  bl 0x8315fb40
	ctx.lr = 0x83165640;
	sub_8315FB40(ctx, base);
	// 83165640: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83165648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316564C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83165650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83165654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165658 size=12
    let mut pc: u32 = 0x83165658;
    'dispatch: loop {
        match pc {
            0x83165658 => {
    //   block [0x83165658..0x83165664)
	// 83165658: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316565C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83165660: 4BFFFE08  b 0x83165468
	sub_83165468(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165668 size=12
    let mut pc: u32 = 0x83165668;
    'dispatch: loop {
        match pc {
            0x83165668 => {
    //   block [0x83165668..0x83165674)
	// 83165668: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316566C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83165670: 4BFFFBB8  b 0x83165228
	sub_83165228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165678 size=12
    let mut pc: u32 = 0x83165678;
    'dispatch: loop {
        match pc {
            0x83165678 => {
    //   block [0x83165678..0x83165684)
	// 83165678: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316567C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83165680: 4BFFFDA8  b 0x83165428
	sub_83165428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165688 size=12
    let mut pc: u32 = 0x83165688;
    'dispatch: loop {
        match pc {
            0x83165688 => {
    //   block [0x83165688..0x83165694)
	// 83165688: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316568C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83165690: 4BFFFB60  b 0x831651f0
	sub_831651F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165698 size=12
    let mut pc: u32 = 0x83165698;
    'dispatch: loop {
        match pc {
            0x83165698 => {
    //   block [0x83165698..0x831656A4)
	// 83165698: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316569C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831656A0: 4BFFFC98  b 0x83165338
	sub_83165338(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831656A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831656A8 size=12
    let mut pc: u32 = 0x831656A8;
    'dispatch: loop {
        match pc {
            0x831656A8 => {
    //   block [0x831656A8..0x831656B4)
	// 831656A8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831656AC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831656B0: 4BFFFE40  b 0x831654f0
	sub_831654F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831656B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831656B8 size=12
    let mut pc: u32 = 0x831656B8;
    'dispatch: loop {
        match pc {
            0x831656B8 => {
    //   block [0x831656B8..0x831656C4)
	// 831656B8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831656BC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831656C0: 48000A60  b 0x83166120
	sub_83166120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831656C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831656C8 size=12
    let mut pc: u32 = 0x831656C8;
    'dispatch: loop {
        match pc {
            0x831656C8 => {
    //   block [0x831656C8..0x831656D4)
	// 831656C8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831656CC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831656D0: 4BFFFAE8  b 0x831651b8
	sub_831651B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831656D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831656D8 size=96
    let mut pc: u32 = 0x831656D8;
    'dispatch: loop {
        match pc {
            0x831656D8 => {
    //   block [0x831656D8..0x83165738)
	// 831656D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831656DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831656E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831656E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831656E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831656EC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831656F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831656F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831656F8: 816A8398  lwz r11, -0x7c68(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31848 as u32) ) } as u64;
	// 831656FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83165700: 916A8398  stw r11, -0x7c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31848 as u32), ctx.r[11].u32 ) };
	// 83165704: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83165708: 409A001C  bne cr6, 0x83165724
	if !ctx.cr[6].eq {
	pc = 0x83165724; continue 'dispatch;
	}
	// 8316570C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83165710: 4BFFF7A9  bl 0x83164eb8
	ctx.lr = 0x83165714;
	sub_83164EB8(ctx, base);
	// 83165714: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165718: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316571C: 419A0008  beq cr6, 0x83165724
	if ctx.cr[6].eq {
	pc = 0x83165724; continue 'dispatch;
	}
	// 83165720: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165724: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316572C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83165730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83165734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165738 size=28
    let mut pc: u32 = 0x83165738;
    'dispatch: loop {
        match pc {
            0x83165738 => {
    //   block [0x83165738..0x83165754)
	// 83165738: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316573C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165740: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165744: 816A8398  lwz r11, -0x7c68(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31848 as u32) ) } as u64;
	// 83165748: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316574C: 916A8398  stw r11, -0x7c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31848 as u32), ctx.r[11].u32 ) };
	// 83165750: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165754 size=12
    let mut pc: u32 = 0x83165754;
    'dispatch: loop {
        match pc {
            0x83165754 => {
    //   block [0x83165754..0x83165760)
	// 83165754: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165758: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 8316575C: 4BFFF7E4  b 0x83164f40
	sub_83164F40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83165760 size=4
    let mut pc: u32 = 0x83165760;
    'dispatch: loop {
        match pc {
            0x83165760 => {
    //   block [0x83165760..0x83165764)
	// 83165760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165768 size=100
    let mut pc: u32 = 0x83165768;
    'dispatch: loop {
        match pc {
            0x83165768 => {
    //   block [0x83165768..0x831657CC)
	// 83165768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83165770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83165774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316577C: 3BE3FCAC  addi r31, r3, -0x354
	ctx.r[31].s64 = ctx.r[3].s64 + -852;
	// 83165780: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165788: 4BFFF881  bl 0x83165008
	ctx.lr = 0x8316578C;
	sub_83165008(ctx, base);
	// 8316578C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165790: 41820028  beq 0x831657b8
	if ctx.cr[0].eq {
	pc = 0x831657B8; continue 'dispatch;
	}
	// 83165794: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165798: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8316579C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831657A0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831657A4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 831657A8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831657AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831657B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831657B4: 4E800421  bctrl
	ctx.lr = 0x831657B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831657B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831657BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831657C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831657C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831657C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831657D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831657D0 size=336
    let mut pc: u32 = 0x831657D0;
    'dispatch: loop {
        match pc {
            0x831657D0 => {
    //   block [0x831657D0..0x83165920)
	// 831657D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831657D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831657D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831657DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831657E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831657E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831657E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831657EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831657F0: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 831657F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831657F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831657FC: 4E800421  bctrl
	ctx.lr = 0x83165800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165800: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165804: 418200F8  beq 0x831658fc
	if ctx.cr[0].eq {
	pc = 0x831658FC; continue 'dispatch;
	}
	// 83165808: 907F034C  stw r3, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[3].u32 ) };
	// 8316580C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83165814: 48007245  bl 0x8316ca58
	ctx.lr = 0x83165818;
	sub_8316CA58(ctx, base);
	// 83165818: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316581C: 907F0348  stw r3, 0x348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[3].u32 ) };
	// 83165820: 40820010  bne 0x83165830
	if !ctx.cr[0].eq {
	pc = 0x83165830; continue 'dispatch;
	}
	// 83165824: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165828: 388B71C0  addi r4, r11, 0x71c0
	ctx.r[4].s64 = ctx.r[11].s64 + 29120;
	// 8316582C: 480000C8  b 0x831658f4
	pc = 0x831658F4; continue 'dispatch;
	// 83165830: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83165834: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83165838: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8316583C: 4BFFA4FD  bl 0x8315fd38
	ctx.lr = 0x83165840;
	sub_8315FD38(ctx, base);
	// 83165840: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165844: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83165848: 418200A4  beq 0x831658ec
	if ctx.cr[0].eq {
	pc = 0x831658EC; continue 'dispatch;
	}
	// 8316584C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165854: 409A0098  bne cr6, 0x831658ec
	if !ctx.cr[6].eq {
	pc = 0x831658EC; continue 'dispatch;
	}
	// 83165858: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316585C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83165860: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83165864: 480048E5  bl 0x8316a148
	ctx.lr = 0x83165868;
	sub_8316A148(ctx, base);
	// 83165868: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8316586C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165870: 41820070  beq 0x831658e0
	if ctx.cr[0].eq {
	pc = 0x831658E0; continue 'dispatch;
	}
	// 83165874: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165878: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316587C: 409A0064  bne cr6, 0x831658e0
	if !ctx.cr[6].eq {
	pc = 0x831658E0; continue 'dispatch;
	}
	// 83165880: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83165884: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83165888: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 8316588C: 480048BD  bl 0x8316a148
	ctx.lr = 0x83165890;
	sub_8316A148(ctx, base);
	// 83165890: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 83165894: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165898: 4182003C  beq 0x831658d4
	if ctx.cr[0].eq {
	pc = 0x831658D4; continue 'dispatch;
	}
	// 8316589C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831658A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831658A4: 409A0030  bne cr6, 0x831658d4
	if !ctx.cr[6].eq {
	pc = 0x831658D4; continue 'dispatch;
	}
	// 831658A8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831658AC: 807F0348  lwz r3, 0x348(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(840 as u32) ) } as u64;
	// 831658B0: 48007349  bl 0x8316cbf8
	ctx.lr = 0x831658B4;
	sub_8316CBF8(ctx, base);
	// 831658B4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831658B8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831658BC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 831658C0: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 831658C4: 806B839C  lwz r3, -0x7c64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31844 as u32) ) } as u64;
	// 831658C8: 48004989  bl 0x8316a250
	ctx.lr = 0x831658CC;
	sub_8316A250(ctx, base);
	// 831658CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831658D0: 48000038  b 0x83165908
	pc = 0x83165908; continue 'dispatch;
	// 831658D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831658D8: 388B718C  addi r4, r11, 0x718c
	ctx.r[4].s64 = ctx.r[11].s64 + 29068;
	// 831658DC: 48000018  b 0x831658f4
	pc = 0x831658F4; continue 'dispatch;
	// 831658E0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831658E4: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 831658E8: 4800000C  b 0x831658f4
	pc = 0x831658F4; continue 'dispatch;
	// 831658EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831658F0: 388B712C  addi r4, r11, 0x712c
	ctx.r[4].s64 = ctx.r[11].s64 + 28972;
	// 831658F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831658F8: 4BFFA221  bl 0x8315fb18
	ctx.lr = 0x831658FC;
	sub_8315FB18(ctx, base);
	// 831658FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165900: 4BFFF709  bl 0x83165008
	ctx.lr = 0x83165904;
	sub_83165008(ctx, base);
	// 83165904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316590C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83165910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83165914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83165918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165920 size=88
    let mut pc: u32 = 0x83165920;
    'dispatch: loop {
        match pc {
            0x83165920 => {
    //   block [0x83165920..0x83165978)
	// 83165920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165924: 48042845  bl 0x831a8168
	ctx.lr = 0x83165928;
	sub_831A8130(ctx, base);
	// 83165928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316592C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83165934: 3BDFFCAC  addi r30, r31, -0x354
	ctx.r[30].s64 = ctx.r[31].s64 + -852;
	// 83165938: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316593C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83165940: 4BFFFA09  bl 0x83165348
	ctx.lr = 0x83165944;
	sub_83165348(ctx, base);
	// 83165944: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165948: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316594C: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 83165950: 387FFED8  addi r3, r31, -0x128
	ctx.r[3].s64 = ctx.r[31].s64 + -296;
	// 83165954: 480041ED  bl 0x83169b40
	ctx.lr = 0x83165958;
	sub_83169B40(ctx, base);
	// 83165958: 815FFD08  lwz r10, -0x2f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-760 as u32) ) } as u64;
	// 8316595C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83165964: 917FFD08  stw r11, -0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-760 as u32), ctx.r[11].u32 ) };
	// 83165968: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316596C: 4BFFFA4D  bl 0x831653b8
	ctx.lr = 0x83165970;
	sub_831653B8(ctx, base);
	// 83165970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165974: 48042844  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165978 size=88
    let mut pc: u32 = 0x83165978;
    'dispatch: loop {
        match pc {
            0x83165978 => {
    //   block [0x83165978..0x831659D0)
	// 83165978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316597C: 480427ED  bl 0x831a8168
	ctx.lr = 0x83165980;
	sub_831A8130(ctx, base);
	// 83165980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165988: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316598C: 3BDFFCAC  addi r30, r31, -0x354
	ctx.r[30].s64 = ctx.r[31].s64 + -852;
	// 83165990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83165994: 4BFFF9B5  bl 0x83165348
	ctx.lr = 0x83165998;
	sub_83165348(ctx, base);
	// 83165998: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316599C: 387FFED8  addi r3, r31, -0x128
	ctx.r[3].s64 = ctx.r[31].s64 + -296;
	// 831659A0: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 831659A4: 480042CD  bl 0x83169c70
	ctx.lr = 0x831659A8;
	sub_83169C70(ctx, base);
	// 831659A8: 817FFD08  lwz r11, -0x2f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-760 as u32) ) } as u64;
	// 831659AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831659B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831659B4: 915FFD08  stw r10, -0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 831659B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831659BC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831659C0: 4BFFF9F9  bl 0x831653b8
	ctx.lr = 0x831659C4;
	sub_831653B8(ctx, base);
	// 831659C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831659C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831659CC: 480427EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831659D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831659D0 size=96
    let mut pc: u32 = 0x831659D0;
    'dispatch: loop {
        match pc {
            0x831659D0 => {
    //   block [0x831659D0..0x83165A30)
	// 831659D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831659D4: 48042791  bl 0x831a8164
	ctx.lr = 0x831659D8;
	sub_831A8130(ctx, base);
	// 831659D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831659DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831659E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831659E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831659E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831659EC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831659F0: 4BFFF959  bl 0x83165348
	ctx.lr = 0x831659F4;
	sub_83165348(ctx, base);
	// 831659F4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831659F8: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 831659FC: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 83165A00: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83165A04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165A08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165A0C: 480044CD  bl 0x83169ed8
	ctx.lr = 0x83165A10;
	sub_83169ED8(ctx, base);
	// 83165A10: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165A14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165A1C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165A20: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83165A24: 4BFFF995  bl 0x831653b8
	ctx.lr = 0x83165A28;
	sub_831653B8(ctx, base);
	// 83165A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165A2C: 48042788  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165A30 size=80
    let mut pc: u32 = 0x83165A30;
    'dispatch: loop {
        match pc {
            0x83165A30 => {
    //   block [0x83165A30..0x83165A80)
	// 83165A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165A34: 48042739  bl 0x831a816c
	ctx.lr = 0x83165A38;
	sub_831A8130(ctx, base);
	// 83165A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165A44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165A48: 4BFFF901  bl 0x83165348
	ctx.lr = 0x83165A4C;
	sub_83165348(ctx, base);
	// 83165A4C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165A50: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83165A54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165A58: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83165A5C: 480047F5  bl 0x8316a250
	ctx.lr = 0x83165A60;
	sub_8316A250(ctx, base);
	// 83165A60: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165A64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165A6C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165A70: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83165A74: 4BFFF945  bl 0x831653b8
	ctx.lr = 0x83165A78;
	sub_831653B8(ctx, base);
	// 83165A78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165A7C: 48042740  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165A80 size=80
    let mut pc: u32 = 0x83165A80;
    'dispatch: loop {
        match pc {
            0x83165A80 => {
    //   block [0x83165A80..0x83165AD0)
	// 83165A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165A84: 480426E9  bl 0x831a816c
	ctx.lr = 0x83165A88;
	sub_831A8130(ctx, base);
	// 83165A88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165A90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165A94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165A98: 4BFFF8B1  bl 0x83165348
	ctx.lr = 0x83165A9C;
	sub_83165348(ctx, base);
	// 83165A9C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165AA0: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83165AA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165AA8: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83165AAC: 480047A5  bl 0x8316a250
	ctx.lr = 0x83165AB0;
	sub_8316A250(ctx, base);
	// 83165AB0: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165AB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165ABC: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165AC0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83165AC4: 4BFFF8F5  bl 0x831653b8
	ctx.lr = 0x83165AC8;
	sub_831653B8(ctx, base);
	// 83165AC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165ACC: 480426F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165AD0 size=80
    let mut pc: u32 = 0x83165AD0;
    'dispatch: loop {
        match pc {
            0x83165AD0 => {
    //   block [0x83165AD0..0x83165B20)
	// 83165AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165AD4: 48042699  bl 0x831a816c
	ctx.lr = 0x83165AD8;
	sub_831A8130(ctx, base);
	// 83165AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165AE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165AE4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165AE8: 4BFFF861  bl 0x83165348
	ctx.lr = 0x83165AEC;
	sub_83165348(ctx, base);
	// 83165AEC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165AF0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83165AF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165AF8: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83165AFC: 4800479D  bl 0x8316a298
	ctx.lr = 0x83165B00;
	sub_8316A298(ctx, base);
	// 83165B00: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165B0C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165B10: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83165B14: 4BFFF8A5  bl 0x831653b8
	ctx.lr = 0x83165B18;
	sub_831653B8(ctx, base);
	// 83165B18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165B1C: 480426A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165B20 size=80
    let mut pc: u32 = 0x83165B20;
    'dispatch: loop {
        match pc {
            0x83165B20 => {
    //   block [0x83165B20..0x83165B70)
	// 83165B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165B24: 48042649  bl 0x831a816c
	ctx.lr = 0x83165B28;
	sub_831A8130(ctx, base);
	// 83165B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165B34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165B38: 4BFFF811  bl 0x83165348
	ctx.lr = 0x83165B3C;
	sub_83165348(ctx, base);
	// 83165B3C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165B40: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83165B44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165B48: 38AB81F0  addi r5, r11, -0x7e10
	ctx.r[5].s64 = ctx.r[11].s64 + -32272;
	// 83165B4C: 4800474D  bl 0x8316a298
	ctx.lr = 0x83165B50;
	sub_8316A298(ctx, base);
	// 83165B50: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165B54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165B5C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165B60: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83165B64: 4BFFF855  bl 0x831653b8
	ctx.lr = 0x83165B68;
	sub_831653B8(ctx, base);
	// 83165B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83165B6C: 48042650  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165B70 size=224
    let mut pc: u32 = 0x83165B70;
    'dispatch: loop {
        match pc {
            0x83165B70 => {
    //   block [0x83165B70..0x83165C50)
	// 83165B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165B74: 480425F1  bl 0x831a8164
	ctx.lr = 0x83165B78;
	sub_831A8130(ctx, base);
	// 83165B78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165B80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165B84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83165B88: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83165B8C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83165B90: 419A000C  beq cr6, 0x83165b9c
	if ctx.cr[6].eq {
	pc = 0x83165B9C; continue 'dispatch;
	}
	// 83165B94: 909F033C  stw r4, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[4].u32 ) };
	// 83165B98: 48000040  b 0x83165bd8
	pc = 0x83165BD8; continue 'dispatch;
	// 83165B9C: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 83165BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83165BA4: 419A000C  beq cr6, 0x83165bb0
	if ctx.cr[6].eq {
	pc = 0x83165BB0; continue 'dispatch;
	}
	// 83165BA8: 917F033C  stw r11, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[11].u32 ) };
	// 83165BAC: 4800002C  b 0x83165bd8
	pc = 0x83165BD8; continue 'dispatch;
	// 83165BB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165BB4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83165BB8: 4BFFEC91  bl 0x83164848
	ctx.lr = 0x83165BBC;
	sub_83164848(ctx, base);
	// 83165BBC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165BC0: 907F0340  stw r3, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[3].u32 ) };
	// 83165BC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165BC8: 409A005C  bne cr6, 0x83165c24
	if !ctx.cr[6].eq {
	pc = 0x83165C24; continue 'dispatch;
	}
	// 83165BCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83165BD0: 419A0054  beq cr6, 0x83165c24
	if ctx.cr[6].eq {
	pc = 0x83165C24; continue 'dispatch;
	}
	// 83165BD4: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 83165BD8: 3BDF0128  addi r30, r31, 0x128
	ctx.r[30].s64 = ctx.r[31].s64 + 296;
	// 83165BDC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83165BE0: 7D5DF050  subf r10, r29, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 83165BE4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83165BE8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83165BEC: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83165BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83165BF4: 4082FFF0  bne 0x83165be4
	if !ctx.cr[0].eq {
	pc = 0x83165BE4; continue 'dispatch;
	}
	// 83165BF8: 5767003E  slwi r7, r27, 0
	ctx.r[7].u32 = ctx.r[27].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83165BFC: 809F033C  lwz r4, 0x33c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 83165C00: 5786003E  slwi r6, r28, 0
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83165C04: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165C08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83165C0C: 48008905  bl 0x8316e510
	ctx.lr = 0x83165C10;
	sub_8316E510(ctx, base);
	// 83165C10: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83165C14: 40820034  bne 0x83165c48
	if !ctx.cr[0].eq {
	pc = 0x83165C48; continue 'dispatch;
	}
	// 83165C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83165C1C: 995E0000  stb r10, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83165C20: 48000014  b 0x83165c34
	pc = 0x83165C34; continue 'dispatch;
	// 83165C24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165C28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165C2C: 388B71F0  addi r4, r11, 0x71f0
	ctx.r[4].s64 = ctx.r[11].s64 + 29168;
	// 83165C30: 4BFF9EE9  bl 0x8315fb18
	ctx.lr = 0x83165C34;
	sub_8315FB18(ctx, base);
	// 83165C34: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165C38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165C3C: 409A000C  bne cr6, 0x83165c48
	if !ctx.cr[6].eq {
	pc = 0x83165C48; continue 'dispatch;
	}
	// 83165C40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83165C44: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165C48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83165C4C: 48042568  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165C50 size=80
    let mut pc: u32 = 0x83165C50;
    'dispatch: loop {
        match pc {
            0x83165C50 => {
    //   block [0x83165C50..0x83165CA0)
	// 83165C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165C54: 48042515  bl 0x831a8168
	ctx.lr = 0x83165C58;
	sub_831A8130(ctx, base);
	// 83165C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165C5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83165C60: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83165C64: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83165C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165C6C: 4BFFF6DD  bl 0x83165348
	ctx.lr = 0x83165C70;
	sub_83165348(ctx, base);
	// 83165C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165C74: 4BFFF5E5  bl 0x83165258
	ctx.lr = 0x83165C78;
	sub_83165258(ctx, base);
	// 83165C78: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 83165C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83165C80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83165C84: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 83165C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165C8C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165C90: 4BFFF729  bl 0x831653b8
	ctx.lr = 0x83165C94;
	sub_831653B8(ctx, base);
	// 83165C94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83165C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165C9C: 4804251C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165CA0 size=140
    let mut pc: u32 = 0x83165CA0;
    'dispatch: loop {
        match pc {
            0x83165CA0 => {
    //   block [0x83165CA0..0x83165D2C)
	// 83165CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165CA4: 480424C9  bl 0x831a816c
	ctx.lr = 0x83165CA8;
	sub_831A8130(ctx, base);
	// 83165CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165CAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165CB8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83165CBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165CC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165CC4: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83165CC8: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83165CCC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165CD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83165CD4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83165CD8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165CDC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83165CE0: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 83165CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165CE8: 4E800421  bctrl
	ctx.lr = 0x83165CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165CEC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165CF0: 409A0030  bne cr6, 0x83165d20
	if !ctx.cr[6].eq {
	pc = 0x83165D20; continue 'dispatch;
	}
	// 83165CF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165CF8: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165CFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165D00: 48008539  bl 0x8316e238
	ctx.lr = 0x83165D04;
	sub_8316E238(ctx, base);
	// 83165D04: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83165D08: 409A0018  bne cr6, 0x83165d20
	if !ctx.cr[6].eq {
	pc = 0x83165D20; continue 'dispatch;
	}
	// 83165D0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165D10: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83165D14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165D18: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83165D1C: 48000008  b 0x83165d24
	pc = 0x83165D24; continue 'dispatch;
	// 83165D20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165D28: 48042494  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165D30 size=184
    let mut pc: u32 = 0x83165D30;
    'dispatch: loop {
        match pc {
            0x83165D30 => {
    //   block [0x83165D30..0x83165DE8)
	// 83165D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165D34: 48042439  bl 0x831a816c
	ctx.lr = 0x83165D38;
	sub_831A8130(ctx, base);
	// 83165D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165D3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83165D40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165D44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165D48: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83165D4C: 409A0034  bne cr6, 0x83165d80
	if !ctx.cr[6].eq {
	pc = 0x83165D80; continue 'dispatch;
	}
	// 83165D50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165D54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165D58: 388B7220  addi r4, r11, 0x7220
	ctx.r[4].s64 = ctx.r[11].s64 + 29216;
	// 83165D5C: 4BFF9DBD  bl 0x8315fb18
	ctx.lr = 0x83165D60;
	sub_8315FB18(ctx, base);
	// 83165D60: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165D64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165D68: 409A000C  bne cr6, 0x83165d74
	if !ctx.cr[6].eq {
	pc = 0x83165D74; continue 'dispatch;
	}
	// 83165D6C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83165D70: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165D74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165D7C: 48042440  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83165D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165D84: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83165D88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165D8C: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83165D90: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83165D94: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165D98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83165D9C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83165DA0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165DA4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83165DA8: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 83165DAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165DB0: 4E800421  bctrl
	ctx.lr = 0x83165DB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165DB4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165DB8: 409AFFBC  bne cr6, 0x83165d74
	if !ctx.cr[6].eq {
	pc = 0x83165D74; continue 'dispatch;
	}
	// 83165DBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165DC0: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165DC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165DC8: 48008891  bl 0x8316e658
	ctx.lr = 0x83165DCC;
	sub_8316E658(ctx, base);
	// 83165DCC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83165DD0: 409AFFA4  bne cr6, 0x83165d74
	if !ctx.cr[6].eq {
	pc = 0x83165D74; continue 'dispatch;
	}
	// 83165DD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165DD8: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83165DDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165DE0: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83165DE4: 4BFFFF94  b 0x83165d78
	pc = 0x83165D78; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165DE8 size=208
    let mut pc: u32 = 0x83165DE8;
    'dispatch: loop {
        match pc {
            0x83165DE8 => {
    //   block [0x83165DE8..0x83165EB8)
	// 83165DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165DEC: 48042381  bl 0x831a816c
	ctx.lr = 0x83165DF0;
	sub_831A8130(ctx, base);
	// 83165DF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165DF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83165DF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83165DFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83165E00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83165E04: 409A0034  bne cr6, 0x83165e38
	if !ctx.cr[6].eq {
	pc = 0x83165E38; continue 'dispatch;
	}
	// 83165E08: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83165E0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165E10: 388B7240  addi r4, r11, 0x7240
	ctx.r[4].s64 = ctx.r[11].s64 + 29248;
	// 83165E14: 4BFF9D05  bl 0x8315fb18
	ctx.lr = 0x83165E18;
	sub_8315FB18(ctx, base);
	// 83165E18: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83165E1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83165E20: 409A000C  bne cr6, 0x83165e2c
	if !ctx.cr[6].eq {
	pc = 0x83165E2C; continue 'dispatch;
	}
	// 83165E24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83165E28: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83165E2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165E30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83165E34: 48042388  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83165E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83165E3C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83165E40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165E44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83165E48: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83165E4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83165E50: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83165E54: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165E58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83165E5C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83165E60: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165E64: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83165E68: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 83165E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165E70: 4E800421  bctrl
	ctx.lr = 0x83165E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165E74: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165E78: 409AFFB4  bne cr6, 0x83165e2c
	if !ctx.cr[6].eq {
	pc = 0x83165E2C; continue 'dispatch;
	}
	// 83165E7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83165E80: 807E034C  lwz r3, 0x34c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165E84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83165E88: 48008859  bl 0x8316e6e0
	ctx.lr = 0x83165E8C;
	sub_8316E6E0(ctx, base);
	// 83165E8C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83165E90: 409AFF9C  bne cr6, 0x83165e2c
	if !ctx.cr[6].eq {
	pc = 0x83165E2C; continue 'dispatch;
	}
	// 83165E94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165E98: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83165E9C: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83165EA0: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83165EA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165EA8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83165EAC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83165EB0: F91F0010  std r8, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u64 ) };
	// 83165EB4: 4BFFFF7C  b 0x83165e30
	pc = 0x83165E30; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165EB8 size=80
    let mut pc: u32 = 0x83165EB8;
    'dispatch: loop {
        match pc {
            0x83165EB8 => {
    //   block [0x83165EB8..0x83165F08)
	// 83165EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165EBC: 480422AD  bl 0x831a8168
	ctx.lr = 0x83165EC0;
	sub_831A8130(ctx, base);
	// 83165EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165EC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83165EC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83165ECC: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83165ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165ED4: 4BFFF475  bl 0x83165348
	ctx.lr = 0x83165ED8;
	sub_83165348(ctx, base);
	// 83165ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165EDC: 4BFFF3ED  bl 0x831652c8
	ctx.lr = 0x83165EE0;
	sub_831652C8(ctx, base);
	// 83165EE0: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 83165EE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83165EE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83165EEC: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 83165EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165EF4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83165EF8: 4BFFF4C1  bl 0x831653b8
	ctx.lr = 0x83165EFC;
	sub_831653B8(ctx, base);
	// 83165EFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83165F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83165F04: 480422B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165F08 size=228
    let mut pc: u32 = 0x83165F08;
    'dispatch: loop {
        match pc {
            0x83165F08 => {
    //   block [0x83165F08..0x83165FEC)
	// 83165F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165F0C: 48042259  bl 0x831a8164
	ctx.lr = 0x83165F10;
	sub_831A8130(ctx, base);
	// 83165F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83165F18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83165F1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83165F20: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83165F24: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83165F28: 80BF034C  lwz r5, 0x34c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165F2C: 48006F3D  bl 0x8316ce68
	ctx.lr = 0x83165F30;
	sub_8316CE68(ctx, base);
	// 83165F30: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83165F34: 4082000C  bne 0x83165f40
	if !ctx.cr[0].eq {
	pc = 0x83165F40; continue 'dispatch;
	}
	// 83165F38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83165F3C: 480000A8  b 0x83165fe4
	pc = 0x83165FE4; continue 'dispatch;
	// 83165F40: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83165F44: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83165F48: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 83165F4C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83165F50: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83165F54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83165F58: 480071B9  bl 0x8316d110
	ctx.lr = 0x83165F5C;
	sub_8316D110(ctx, base);
	// 83165F5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165F60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83165F64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83165F68: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83165F6C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83165F70: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83165F74: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83165F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83165F7C: 4E800421  bctrl
	ctx.lr = 0x83165F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83165F80: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83165F84: 409A004C  bne cr6, 0x83165fd0
	if !ctx.cr[6].eq {
	pc = 0x83165FD0; continue 'dispatch;
	}
	// 83165F88: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165F8C: 48008355  bl 0x8316e2e0
	ctx.lr = 0x83165F90;
	sub_8316E2E0(ctx, base);
	// 83165F90: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83165F94: 409A003C  bne cr6, 0x83165fd0
	if !ctx.cr[6].eq {
	pc = 0x83165FD0; continue 'dispatch;
	}
	// 83165F98: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 83165F9C: 807F034C  lwz r3, 0x34c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83165FA0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83165FA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83165FA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83165FAC: 48008485  bl 0x8316e430
	ctx.lr = 0x83165FB0;
	sub_8316E430(ctx, base);
	// 83165FB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83165FB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83165FB8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83165FBC: 48007195  bl 0x8316d150
	ctx.lr = 0x83165FC0;
	sub_8316D150(ctx, base);
	// 83165FC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83165FC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83165FC8: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83165FCC: 480071C5  bl 0x8316d190
	ctx.lr = 0x83165FD0;
	sub_8316D190(ctx, base);
	// 83165FD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83165FD4: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83165FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83165FDC: 4BFFFAA5  bl 0x83165a80
	ctx.lr = 0x83165FE0;
	sub_83165A80(ctx, base);
	// 83165FE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83165FE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83165FE8: 480421CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83165FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83165FF0 size=152
    let mut pc: u32 = 0x83165FF0;
    'dispatch: loop {
        match pc {
            0x83165FF0 => {
    //   block [0x83165FF0..0x83166088)
	// 83165FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83165FF4: 48042175  bl 0x831a8168
	ctx.lr = 0x83165FF8;
	sub_831A8130(ctx, base);
	// 83165FF8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83165FFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83166000: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83166004: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83166008: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316600C: 38A00103  li r5, 0x103
	ctx.r[5].s64 = 259;
	// 83166010: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83166014: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 83166018: 38610061  addi r3, r1, 0x61
	ctx.r[3].s64 = ctx.r[1].s64 + 97;
	// 8316601C: 480421C5  bl 0x831a81e0
	ctx.lr = 0x83166020;
	sub_831A81E0(ctx, base);
	// 83166020: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166024: 38C00104  li r6, 0x104
	ctx.r[6].s64 = 260;
	// 83166028: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 8316602C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83166030: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83166034: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83166038: 4BFFF999  bl 0x831659d0
	ctx.lr = 0x8316603C;
	sub_831659D0(ctx, base);
	// 8316603C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83166040: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166044: 807D034C  lwz r3, 0x34c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(844 as u32) ) } as u64;
	// 83166048: 480081F1  bl 0x8316e238
	ctx.lr = 0x8316604C;
	sub_8316E238(ctx, base);
	// 8316604C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83166050: 409A0020  bne cr6, 0x83166070
	if !ctx.cr[6].eq {
	pc = 0x83166070; continue 'dispatch;
	}
	// 83166054: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166058: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316605C: E9210058  ld r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83166060: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166064: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83166068: F93F0008  std r9, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 8316606C: 48000014  b 0x83166080
	pc = 0x83166080; continue 'dispatch;
	// 83166070: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83166074: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166078: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8316607C: FBDF0008  std r30, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 83166080: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 83166084: 48042134  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166088 size=68
    let mut pc: u32 = 0x83166088;
    'dispatch: loop {
        match pc {
            0x83166088 => {
    //   block [0x83166088..0x831660CC)
	// 83166088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316608C: 480420E1  bl 0x831a816c
	ctx.lr = 0x83166090;
	sub_831A8130(ctx, base);
	// 83166090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316609C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831660A0: 4BFFF2A9  bl 0x83165348
	ctx.lr = 0x831660A4;
	sub_83165348(ctx, base);
	// 831660A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831660A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831660AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831660B0: 4BFFF4C9  bl 0x83165578
	ctx.lr = 0x831660B4;
	sub_83165578(ctx, base);
	// 831660B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831660B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831660BC: 4BFFF2FD  bl 0x831653b8
	ctx.lr = 0x831660C0;
	sub_831653B8(ctx, base);
	// 831660C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831660C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831660C8: 480420F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831660D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831660D0 size=12
    let mut pc: u32 = 0x831660D0;
    'dispatch: loop {
        match pc {
            0x831660D0 => {
    //   block [0x831660D0..0x831660DC)
	// 831660D0: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831660D4: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831660D8: 4BFFFDE0  b 0x83165eb8
	sub_83165EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831660E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831660E0 size=12
    let mut pc: u32 = 0x831660E0;
    'dispatch: loop {
        match pc {
            0x831660E0 => {
    //   block [0x831660E0..0x831660EC)
	// 831660E0: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831660E4: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831660E8: 4BFFF890  b 0x83165978
	sub_83165978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


