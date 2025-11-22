pub fn sub_82D9C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C168 size=4
    let mut pc: u32 = 0x82D9C168;
    'dispatch: loop {
        match pc {
            0x82D9C168 => {
    //   block [0x82D9C168..0x82D9C16C)
	// 82D9C168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C170 size=4
    let mut pc: u32 = 0x82D9C170;
    'dispatch: loop {
        match pc {
            0x82D9C170 => {
    //   block [0x82D9C170..0x82D9C174)
	// 82D9C170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C178 size=4
    let mut pc: u32 = 0x82D9C178;
    'dispatch: loop {
        match pc {
            0x82D9C178 => {
    //   block [0x82D9C178..0x82D9C17C)
	// 82D9C178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C180 size=4
    let mut pc: u32 = 0x82D9C180;
    'dispatch: loop {
        match pc {
            0x82D9C180 => {
    //   block [0x82D9C180..0x82D9C184)
	// 82D9C180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C188 size=4
    let mut pc: u32 = 0x82D9C188;
    'dispatch: loop {
        match pc {
            0x82D9C188 => {
    //   block [0x82D9C188..0x82D9C18C)
	// 82D9C188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C190 size=4
    let mut pc: u32 = 0x82D9C190;
    'dispatch: loop {
        match pc {
            0x82D9C190 => {
    //   block [0x82D9C190..0x82D9C194)
	// 82D9C190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C198 size=4
    let mut pc: u32 = 0x82D9C198;
    'dispatch: loop {
        match pc {
            0x82D9C198 => {
    //   block [0x82D9C198..0x82D9C19C)
	// 82D9C198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1A0 size=4
    let mut pc: u32 = 0x82D9C1A0;
    'dispatch: loop {
        match pc {
            0x82D9C1A0 => {
    //   block [0x82D9C1A0..0x82D9C1A4)
	// 82D9C1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1A8 size=4
    let mut pc: u32 = 0x82D9C1A8;
    'dispatch: loop {
        match pc {
            0x82D9C1A8 => {
    //   block [0x82D9C1A8..0x82D9C1AC)
	// 82D9C1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1B0 size=12
    let mut pc: u32 = 0x82D9C1B0;
    'dispatch: loop {
        match pc {
            0x82D9C1B0 => {
    //   block [0x82D9C1B0..0x82D9C1BC)
	// 82D9C1B0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D9C1B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9C1B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1BC size=12
    let mut pc: u32 = 0x82D9C1BC;
    'dispatch: loop {
        match pc {
            0x82D9C1BC => {
    //   block [0x82D9C1BC..0x82D9C1C8)
	// 82D9C1BC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82D9C1C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D9C1C4: 4BFBCB7C  b 0x82d58d40
	sub_82D58D40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1C8 size=4
    let mut pc: u32 = 0x82D9C1C8;
    'dispatch: loop {
        match pc {
            0x82D9C1C8 => {
    //   block [0x82D9C1C8..0x82D9C1CC)
	// 82D9C1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1D0 size=8
    let mut pc: u32 = 0x82D9C1D0;
    'dispatch: loop {
        match pc {
            0x82D9C1D0 => {
    //   block [0x82D9C1D0..0x82D9C1D8)
	// 82D9C1D0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D9C1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C1D8 size=120
    let mut pc: u32 = 0x82D9C1D8;
    'dispatch: loop {
        match pc {
            0x82D9C1D8 => {
    //   block [0x82D9C1D8..0x82D9C250)
	// 82D9C1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D9C1DC: 90850014  stw r4, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82D9C1E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D9C1E4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82D9C1E8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9C1EC: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D9C1F0: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D9C1F4: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C1F8: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D9C1FC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9C200: 2B0A0016  cmplwi cr6, r10, 0x16
	ctx.cr[6].compare_u32(ctx.r[10].u32, 22 as u32, &mut ctx.xer);
	// 82D9C204: 409A0064  bne cr6, 0x82d9c268
	if !ctx.cr[6].eq {
		sub_82D9C268(ctx, base);
		return;
	}
	// 82D9C208: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C20C: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D9C210: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82D9C214: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82D9C218: 1CE80070  mulli r7, r8, 0x70
	ctx.r[7].s64 = ctx.r[8].s64 * 112;
	// 82D9C21C: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82D9C220: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82D9C224: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82D9C228: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9C22C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D9C230: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C234: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D9C238: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9C23C: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82D9C240: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82D9C244: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9C248: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9C24C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C250 size=24
    let mut pc: u32 = 0x82D9C250;
    'dispatch: loop {
        match pc {
            0x82D9C250 => {
    //   block [0x82D9C250..0x82D9C268)
	// 82D9C250: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82D9C254: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D9C258: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 82D9C25C: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9C260: 9125000C  stw r9, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82D9C264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C268 size=12
    let mut pc: u32 = 0x82D9C268;
    'dispatch: loop {
        match pc {
            0x82D9C268 => {
    //   block [0x82D9C268..0x82D9C274)
	// 82D9C268: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D9C26C: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82D9C270: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C274 size=852
    let mut pc: u32 = 0x82D9C274;
    'dispatch: loop {
        match pc {
            0x82D9C274 => {
    //   block [0x82D9C274..0x82D9C304)
	// 82D9C274: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9C278: 2B0A001A  cmplwi cr6, r10, 0x1a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 26 as u32, &mut ctx.xer);
	// 82D9C27C: 41990340  bgt cr6, 0x82d9c5bc
	if ctx.cr[6].gt {
	pc = 0x82D9C5BC; continue 'dispatch;
	}
	// 82D9C280: 3D8082DA  lis r12, -0x7d26
	ctx.r[12].s64 = -2099642368;
	// 82D9C284: 398CC298  addi r12, r12, -0x3d68
	ctx.r[12].s64 = ctx.r[12].s64 + -15720;
	// 82D9C288: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D9C28C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D9C290: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D9C294: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x82D9C304; continue 'dispatch;
		},
		1 => {
	pc = 0x82D9C5C4; continue 'dispatch;
		},
		2 => {
	pc = 0x82D9C5A8; continue 'dispatch;
		},
		3 => {
	pc = 0x82D9C5B8; continue 'dispatch;
		},
		4 => {
	pc = 0x82D9C5B0; continue 'dispatch;
		},
		5 => {
	pc = 0x82D9C31C; continue 'dispatch;
		},
		6 => {
	pc = 0x82D9C348; continue 'dispatch;
		},
		7 => {
	pc = 0x82D9C374; continue 'dispatch;
		},
		8 => {
	pc = 0x82D9C3A0; continue 'dispatch;
		},
		9 => {
	pc = 0x82D9C3A0; continue 'dispatch;
		},
		10 => {
	pc = 0x82D9C3CC; continue 'dispatch;
		},
		11 => {
	pc = 0x82D9C3F8; continue 'dispatch;
		},
		12 => {
	pc = 0x82D9C424; continue 'dispatch;
		},
		13 => {
	pc = 0x82D9C450; continue 'dispatch;
		},
		14 => {
	pc = 0x82D9C48C; continue 'dispatch;
		},
		15 => {
	pc = 0x82D9C4B8; continue 'dispatch;
		},
		16 => {
	pc = 0x82D9C4B8; continue 'dispatch;
		},
		17 => {
	pc = 0x82D9C4E4; continue 'dispatch;
		},
		18 => {
	pc = 0x82D9C524; continue 'dispatch;
		},
		19 => {
	pc = 0x82D9C550; continue 'dispatch;
		},
		20 => {
	pc = 0x82D9C57C; continue 'dispatch;
		},
		21 => {
	pc = 0x82D9C5BC; continue 'dispatch;
		},
		22 => {
	pc = 0x82D9C5BC; continue 'dispatch;
		},
		23 => {
	pc = 0x82D9C5C4; continue 'dispatch;
		},
		24 => {
	pc = 0x82D9C5C4; continue 'dispatch;
		},
		25 => {
	pc = 0x82D9C5C4; continue 'dispatch;
		},
		26 => {
	pc = 0x82D9C5C4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D9C298: 82D9C304  lwz r22, -0x3cfc(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15612 as u32) ) } as u64;
	// 82D9C29C: 82D9C5C4  lwz r22, -0x3a3c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14908 as u32) ) } as u64;
	// 82D9C2A0: 82D9C5A8  lwz r22, -0x3a58(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14936 as u32) ) } as u64;
	// 82D9C2A4: 82D9C5B8  lwz r22, -0x3a48(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14920 as u32) ) } as u64;
	// 82D9C2A8: 82D9C5B0  lwz r22, -0x3a50(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14928 as u32) ) } as u64;
	// 82D9C2AC: 82D9C31C  lwz r22, -0x3ce4(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15588 as u32) ) } as u64;
	// 82D9C2B0: 82D9C348  lwz r22, -0x3cb8(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15544 as u32) ) } as u64;
	// 82D9C2B4: 82D9C374  lwz r22, -0x3c8c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15500 as u32) ) } as u64;
	// 82D9C2B8: 82D9C3A0  lwz r22, -0x3c60(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15456 as u32) ) } as u64;
	// 82D9C2BC: 82D9C3A0  lwz r22, -0x3c60(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15456 as u32) ) } as u64;
	// 82D9C2C0: 82D9C3CC  lwz r22, -0x3c34(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15412 as u32) ) } as u64;
	// 82D9C2C4: 82D9C3F8  lwz r22, -0x3c08(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15368 as u32) ) } as u64;
	// 82D9C2C8: 82D9C424  lwz r22, -0x3bdc(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15324 as u32) ) } as u64;
	// 82D9C2CC: 82D9C450  lwz r22, -0x3bb0(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15280 as u32) ) } as u64;
	// 82D9C2D0: 82D9C48C  lwz r22, -0x3b74(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15220 as u32) ) } as u64;
	// 82D9C2D4: 82D9C4B8  lwz r22, -0x3b48(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15176 as u32) ) } as u64;
	// 82D9C2D8: 82D9C4B8  lwz r22, -0x3b48(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15176 as u32) ) } as u64;
	// 82D9C2DC: 82D9C4E4  lwz r22, -0x3b1c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15132 as u32) ) } as u64;
	// 82D9C2E0: 82D9C524  lwz r22, -0x3adc(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15068 as u32) ) } as u64;
	// 82D9C2E4: 82D9C550  lwz r22, -0x3ab0(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-15024 as u32) ) } as u64;
	// 82D9C2E8: 82D9C57C  lwz r22, -0x3a84(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14980 as u32) ) } as u64;
	// 82D9C2EC: 82D9C5BC  lwz r22, -0x3a44(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14916 as u32) ) } as u64;
	// 82D9C2F0: 82D9C5BC  lwz r22, -0x3a44(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14916 as u32) ) } as u64;
	// 82D9C2F4: 82D9C5C4  lwz r22, -0x3a3c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14908 as u32) ) } as u64;
	// 82D9C2F8: 82D9C5C4  lwz r22, -0x3a3c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14908 as u32) ) } as u64;
	// 82D9C2FC: 82D9C5C4  lwz r22, -0x3a3c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14908 as u32) ) } as u64;
	// 82D9C300: 82D9C5C4  lwz r22, -0x3a3c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14908 as u32) ) } as u64;
            }
            0x82D9C304 => {
    //   block [0x82D9C304..0x82D9C31C)
	// 82D9C304: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D9C308: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9C30C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9C310: 2B0A001A  cmplwi cr6, r10, 0x1a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 26 as u32, &mut ctx.xer);
	// 82D9C314: 4099FF6C  ble cr6, 0x82d9c280
	if !ctx.cr[6].gt {
	pc = 0x82D9C280; continue 'dispatch;
	}
	// 82D9C318: 480002A4  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C31C => {
    //   block [0x82D9C31C..0x82D9C348)
	// 82D9C31C: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C320: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82D9C324: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C328: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C32C: 39080090  addi r8, r8, 0x90
	ctx.r[8].s64 = ctx.r[8].s64 + 144;
	// 82D9C330: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 82D9C334: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82D9C338: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C33C: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C340: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C344: 48000278  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C348 => {
    //   block [0x82D9C348..0x82D9C374)
	// 82D9C348: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C34C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D9C350: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C354: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C358: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 82D9C35C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C360: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C364: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C368: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C36C: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C370: 4800024C  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C374 => {
    //   block [0x82D9C374..0x82D9C3A0)
	// 82D9C374: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C378: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D9C37C: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C380: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C384: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 82D9C388: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C38C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C390: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C394: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C398: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C39C: 48000220  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C3A0 => {
    //   block [0x82D9C3A0..0x82D9C3CC)
	// 82D9C3A0: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C3A4: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82D9C3A8: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C3AC: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C3B0: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82D9C3B4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C3B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C3BC: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C3C0: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C3C4: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C3C8: 480001F4  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C3CC => {
    //   block [0x82D9C3CC..0x82D9C3F8)
	// 82D9C3CC: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C3D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D9C3D4: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C3D8: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C3DC: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82D9C3E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C3E4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D9C3E8: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C3EC: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C3F0: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C3F4: 480001C8  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C3F8 => {
    //   block [0x82D9C3F8..0x82D9C424)
	// 82D9C3F8: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C3FC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D9C400: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C404: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C408: 39080050  addi r8, r8, 0x50
	ctx.r[8].s64 = ctx.r[8].s64 + 80;
	// 82D9C40C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C410: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82D9C414: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C418: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C41C: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C420: 4800019C  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C424 => {
    //   block [0x82D9C424..0x82D9C450)
	// 82D9C424: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C428: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D9C42C: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C430: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C434: 39080060  addi r8, r8, 0x60
	ctx.r[8].s64 = ctx.r[8].s64 + 96;
	// 82D9C438: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82D9C43C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D9C440: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C444: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C448: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C44C: 48000170  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C450 => {
    //   block [0x82D9C450..0x82D9C48C)
	// 82D9C450: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82D9C454: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D9C458: 8105000C  lwz r8, 0xc(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C45C: 5547083E  rotlwi r7, r10, 1
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82D9C460: 80C50004  lwz r6, 4(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C464: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C468: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82D9C46C: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82D9C470: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D9C474: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D9C478: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82D9C47C: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D9C480: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D9C484: 90E50004  stw r7, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82D9C488: 48000134  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C48C => {
    //   block [0x82D9C48C..0x82D9C4B8)
	// 82D9C48C: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C490: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D9C494: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C498: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C49C: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 82D9C4A0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C4A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C4A8: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C4AC: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C4B0: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C4B4: 48000108  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C4B8 => {
    //   block [0x82D9C4B8..0x82D9C4E4)
	// 82D9C4B8: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C4BC: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82D9C4C0: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C4C4: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C4C8: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 82D9C4CC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C4D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C4D4: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C4D8: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C4DC: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C4E0: 480000DC  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C4E4 => {
    //   block [0x82D9C4E4..0x82D9C524)
	// 82D9C4E4: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C4E8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82D9C4EC: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C4F0: 5546083E  rotlwi r6, r10, 1
	ctx.r[6].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82D9C4F4: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C4F8: 80E5000C  lwz r7, 0xc(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C4FC: 5549083E  rotlwi r9, r10, 1
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82D9C500: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D9C504: 7D274A14  add r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D9C508: 54C62036  slwi r6, r6, 4
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D9C50C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82D9C510: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D9C514: 9125000C  stw r9, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82D9C518: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D9C51C: 90C50004  stw r6, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82D9C520: 4800009C  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C524 => {
    //   block [0x82D9C524..0x82D9C550)
	// 82D9C524: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C528: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82D9C52C: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C530: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C534: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82D9C538: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C53C: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82D9C540: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C544: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C548: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C54C: 48000070  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C550 => {
    //   block [0x82D9C550..0x82D9C57C)
	// 82D9C550: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C554: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 82D9C558: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C55C: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C560: 390800C0  addi r8, r8, 0xc0
	ctx.r[8].s64 = ctx.r[8].s64 + 192;
	// 82D9C564: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 82D9C568: 394A0009  addi r10, r10, 9
	ctx.r[10].s64 = ctx.r[10].s64 + 9;
	// 82D9C56C: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C570: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C574: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C578: 48000044  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C57C => {
    //   block [0x82D9C57C..0x82D9C5A8)
	// 82D9C57C: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9C580: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82D9C584: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9C588: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9C58C: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82D9C590: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D9C594: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9C598: 91050004  stw r8, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9C59C: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9C5A0: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D9C5A4: 48000018  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C5A8 => {
    //   block [0x82D9C5A8..0x82D9C5B0)
	// 82D9C5A8: 396B0090  addi r11, r11, 0x90
	ctx.r[11].s64 = ctx.r[11].s64 + 144;
	// 82D9C5AC: 48000010  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C5B0 => {
    //   block [0x82D9C5B0..0x82D9C5B8)
	// 82D9C5B0: 396B0070  addi r11, r11, 0x70
	ctx.r[11].s64 = ctx.r[11].s64 + 112;
	// 82D9C5B4: 48000008  b 0x82d9c5bc
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C5B8 => {
    //   block [0x82D9C5B8..0x82D9C5BC)
	// 82D9C5B8: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	pc = 0x82D9C5BC; continue 'dispatch;
            }
            0x82D9C5BC => {
    //   block [0x82D9C5BC..0x82D9C5C4)
	// 82D9C5BC: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82D9C5C0: 4198FCB4  blt cr6, 0x82d9c274
	if ctx.cr[6].lt {
	pc = 0x82D9C274; continue 'dispatch;
	}
	pc = 0x82D9C5C4; continue 'dispatch;
            }
            0x82D9C5C4 => {
    //   block [0x82D9C5C4..0x82D9C5C8)
	// 82D9C5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C5C8 size=32
    let mut pc: u32 = 0x82D9C5C8;
    'dispatch: loop {
        match pc {
            0x82D9C5C8 => {
    //   block [0x82D9C5C8..0x82D9C5E8)
	// 82D9C5C8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82D9C5CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9C5D0: 419A0018  beq cr6, 0x82d9c5e8
	if ctx.cr[6].eq {
		sub_82D9C5E8(ctx, base);
		return;
	}
	// 82D9C5D4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82D9C5D8: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 82D9C5DC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9C5E0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9C5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C5E8 size=16
    let mut pc: u32 = 0x82D9C5E8;
    'dispatch: loop {
        match pc {
            0x82D9C5E8 => {
    //   block [0x82D9C5E8..0x82D9C5F8)
	// 82D9C5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9C5EC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9C5F0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9C5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C5F8 size=16
    let mut pc: u32 = 0x82D9C5F8;
    'dispatch: loop {
        match pc {
            0x82D9C5F8 => {
    //   block [0x82D9C5F8..0x82D9C608)
	// 82D9C5F8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D9C5FC: 38800096  li r4, 0x96
	ctx.r[4].s64 = 150;
	// 82D9C600: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82D9C604: 4BFFFBD4  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9C608 size=140
    let mut pc: u32 = 0x82D9C608;
    'dispatch: loop {
        match pc {
            0x82D9C608 => {
    //   block [0x82D9C608..0x82D9C694)
	// 82D9C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9C60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9C610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9C614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9C618: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D9C61C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9C620: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82D9C624: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9C628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9C62C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82D9C630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D9C634: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D9C638: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9C63C: 4BFBB26D  bl 0x82d578a8
	ctx.lr = 0x82D9C640;
	sub_82D578A8(ctx, base);
	// 82D9C640: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9C644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9C648: 419A0024  beq cr6, 0x82d9c66c
	if ctx.cr[6].eq {
	pc = 0x82D9C66C; continue 'dispatch;
	}
	// 82D9C64C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82D9C650: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9C654: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82D9C658: 4BFBB251  bl 0x82d578a8
	ctx.lr = 0x82D9C65C;
	sub_82D578A8(ctx, base);
	// 82D9C65C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9C660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9C664: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9C668: 409A0008  bne cr6, 0x82d9c670
	if !ctx.cr[6].eq {
	pc = 0x82D9C670; continue 'dispatch;
	}
	// 82D9C66C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9C670: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9C674: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9C678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D9C67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9C680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9C684: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82D9C688: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9C68C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9C690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C698 size=8
    let mut pc: u32 = 0x82D9C698;
    'dispatch: loop {
        match pc {
            0x82D9C698 => {
    //   block [0x82D9C698..0x82D9C6A0)
	// 82D9C698: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D9C69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9C6A0 size=400
    let mut pc: u32 = 0x82D9C6A0;
    'dispatch: loop {
        match pc {
            0x82D9C6A0 => {
    //   block [0x82D9C6A0..0x82D9C830)
	// 82D9C6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9C6A4: 4BF0CD61  bl 0x82ca9404
	ctx.lr = 0x82D9C6A8;
	sub_82CA93D0(ctx, base);
	// 82D9C6A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9C6AC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82D9C6B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9C6B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D9C6B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D9C6BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82D9C6C0: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9C6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D9C6C8: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9C6CC: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82D9C6D0: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82D9C6D4: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D9C6D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9C6DC: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82D9C6E0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D9C6E4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D9C6E8: 40980010  bge cr6, 0x82d9c6f8
	if !ctx.cr[6].lt {
	pc = 0x82D9C6F8; continue 'dispatch;
	}
	// 82D9C6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9C6F0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82D9C6F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D9C6F8: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82D9C6FC: 40980008  bge cr6, 0x82d9c704
	if !ctx.cr[6].lt {
	pc = 0x82D9C704; continue 'dispatch;
	}
	// 82D9C700: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D9C704: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9C830 size=568
    let mut pc: u32 = 0x82D9C830;
    'dispatch: loop {
        match pc {
            0x82D9C830 => {
    //   block [0x82D9C830..0x82D9CA68)
	// 82D9C830: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D9C834: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D9C838: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CA68 size=196
    let mut pc: u32 = 0x82D9CA68;
    'dispatch: loop {
        match pc {
            0x82D9CA68 => {
    //   block [0x82D9CA68..0x82D9CB2C)
	// 82D9CA68: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D9CA6C: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 82D9CA70: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82D9CA74: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82D9CA78: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82D9CA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9CB30 size=364
    let mut pc: u32 = 0x82D9CB30;
    'dispatch: loop {
        match pc {
            0x82D9CB30 => {
    //   block [0x82D9CB30..0x82D9CC9C)
	// 82D9CB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9CB34: 4BF0C8D5  bl 0x82ca9408
	ctx.lr = 0x82D9CB38;
	sub_82CA93D0(ctx, base);
	// 82D9CB38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9CB3C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82D9CB40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9CB44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D9CB48: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D9CB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D9CB50: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9CB54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9CB58: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9CB5C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82D9CB60: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82D9CB64: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D9CB68: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82D9CB6C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D9CB70: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D9CB74: 40980010  bge cr6, 0x82d9cb84
	if !ctx.cr[6].lt {
	pc = 0x82D9CB84; continue 'dispatch;
	}
	// 82D9CB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9CB7C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82D9CB80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D9CB84: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82D9CB88: 40980008  bge cr6, 0x82d9cb90
	if !ctx.cr[6].lt {
	pc = 0x82D9CB90; continue 'dispatch;
	}
	// 82D9CB8C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D9CB90: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CCA0 size=356
    let mut pc: u32 = 0x82D9CCA0;
    'dispatch: loop {
        match pc {
            0x82D9CCA0 => {
    //   block [0x82D9CCA0..0x82D9CE04)
	// 82D9CCA0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D9CCA4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CE08 size=16
    let mut pc: u32 = 0x82D9CE08;
    'dispatch: loop {
        match pc {
            0x82D9CE08 => {
    //   block [0x82D9CE08..0x82D9CE18)
	// 82D9CE08: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D9CE0C: 38800084  li r4, 0x84
	ctx.r[4].s64 = 132;
	// 82D9CE10: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82D9CE14: 4BFFF3C4  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CE18 size=20
    let mut pc: u32 = 0x82D9CE18;
    'dispatch: loop {
        match pc {
            0x82D9CE18 => {
    //   block [0x82D9CE18..0x82D9CE2C)
	// 82D9CE18: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82D9CE1C: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82D9CE20: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9CE24: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9CE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9CE30 size=140
    let mut pc: u32 = 0x82D9CE30;
    'dispatch: loop {
        match pc {
            0x82D9CE30 => {
    //   block [0x82D9CE30..0x82D9CEBC)
	// 82D9CE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9CE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9CE38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9CE3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9CE40: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D9CE44: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9CE48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82D9CE4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9CE50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9CE54: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82D9CE58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D9CE5C: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D9CE60: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9CE64: 4BFBAA45  bl 0x82d578a8
	ctx.lr = 0x82D9CE68;
	sub_82D578A8(ctx, base);
	// 82D9CE68: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9CE6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9CE70: 419A0024  beq cr6, 0x82d9ce94
	if ctx.cr[6].eq {
	pc = 0x82D9CE94; continue 'dispatch;
	}
	// 82D9CE74: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82D9CE78: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9CE7C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82D9CE80: 4BFBAA29  bl 0x82d578a8
	ctx.lr = 0x82D9CE84;
	sub_82D578A8(ctx, base);
	// 82D9CE84: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9CE88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9CE8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9CE90: 409A0008  bne cr6, 0x82d9ce98
	if !ctx.cr[6].eq {
	pc = 0x82D9CE98; continue 'dispatch;
	}
	// 82D9CE94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9CE98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9CE9C: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9CEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D9CEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9CEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9CEAC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82D9CEB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9CEB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9CEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CEC0 size=8
    let mut pc: u32 = 0x82D9CEC0;
    'dispatch: loop {
        match pc {
            0x82D9CEC0 => {
    //   block [0x82D9CEC0..0x82D9CEC8)
	// 82D9CEC0: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82D9CEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CEC8 size=204
    let mut pc: u32 = 0x82D9CEC8;
    'dispatch: loop {
        match pc {
            0x82D9CEC8 => {
    //   block [0x82D9CEC8..0x82D9CF94)
	// 82D9CEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9CECC: 4BF0C541  bl 0x82ca940c
	ctx.lr = 0x82D9CED0;
	sub_82CA93D0(ctx, base);
	// 82D9CED0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82D9CED4: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 82D9CED8: 38E8901C  addi r7, r8, -0x6fe4
	ctx.r[7].s64 = ctx.r[8].s64 + -28644;
	// 82D9CEDC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82D9CEE0: 3941FFD0  addi r10, r1, -0x30
	ctx.r[10].s64 = ctx.r[1].s64 + -48;
	// 82D9CEE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9CEE8: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9CF98 size=60
    let mut pc: u32 = 0x82D9CF98;
    'dispatch: loop {
        match pc {
            0x82D9CF98 => {
    //   block [0x82D9CF98..0x82D9CFD4)
	// 82D9CF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9CF9C: 4BF0C471  bl 0x82ca940c
	ctx.lr = 0x82D9CFA0;
	sub_82CA93D0(ctx, base);
	// 82D9CFA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9CFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9CFA8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D9CFAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D9CFB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D9CFB4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82D9CFB8: 4BFB9439  bl 0x82d563f0
	ctx.lr = 0x82D9CFBC;
	sub_82D563F0(ctx, base);
	// 82D9CFBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D9CFC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D9CFC4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82D9CFC8: 4BFB9429  bl 0x82d563f0
	ctx.lr = 0x82D9CFCC;
	sub_82D563F0(ctx, base);
	// 82D9CFCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9CFD0: 4BF0C48C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CFD8 size=28
    let mut pc: u32 = 0x82D9CFD8;
    'dispatch: loop {
        match pc {
            0x82D9CFD8 => {
    //   block [0x82D9CFD8..0x82D9CFF4)
	// 82D9CFD8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9CFF8 size=16
    let mut pc: u32 = 0x82D9CFF8;
    'dispatch: loop {
        match pc {
            0x82D9CFF8 => {
    //   block [0x82D9CFF8..0x82D9D008)
	// 82D9CFF8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D9CFFC: 38800032  li r4, 0x32
	ctx.r[4].s64 = 50;
	// 82D9D000: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82D9D004: 4BFFF1D4  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D008 size=32
    let mut pc: u32 = 0x82D9D008;
    'dispatch: loop {
        match pc {
            0x82D9D008 => {
    //   block [0x82D9D008..0x82D9D028)
	// 82D9D008: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82D9D00C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9D010: 419A0018  beq cr6, 0x82d9d028
	if ctx.cr[6].eq {
		sub_82D9D028(ctx, base);
		return;
	}
	// 82D9D014: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82D9D018: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82D9D01C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9D020: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9D024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D028 size=16
    let mut pc: u32 = 0x82D9D028;
    'dispatch: loop {
        match pc {
            0x82D9D028 => {
    //   block [0x82D9D028..0x82D9D038)
	// 82D9D028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9D02C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D030: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9D034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D038 size=12
    let mut pc: u32 = 0x82D9D038;
    'dispatch: loop {
        match pc {
            0x82D9D038 => {
    //   block [0x82D9D038..0x82D9D044)
	// 82D9D038: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9D03C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9D040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D048 size=8
    let mut pc: u32 = 0x82D9D048;
    'dispatch: loop {
        match pc {
            0x82D9D048 => {
    //   block [0x82D9D048..0x82D9D050)
	// 82D9D048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9D04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D050 size=80
    let mut pc: u32 = 0x82D9D050;
    'dispatch: loop {
        match pc {
            0x82D9D050 => {
    //   block [0x82D9D050..0x82D9D0A0)
	// 82D9D050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82D9D054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D9D058: 392A90CC  addi r9, r10, -0x6f34
	ctx.r[9].s64 = ctx.r[10].s64 + -28468;
	// 82D9D05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D9D060: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82D9D064: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82D9D068: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82D9D06C: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82D9D070: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D9D074: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D9D078: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82D9D07C: B0E30010  sth r7, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u16 ) };
	// 82D9D080: B0C30040  sth r6, 0x40(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[6].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D0A0 size=8
    let mut pc: u32 = 0x82D9D0A0;
    'dispatch: loop {
        match pc {
            0x82D9D0A0 => {
    //   block [0x82D9D0A0..0x82D9D0A8)
	// 82D9D0A0: 38600065  li r3, 0x65
	ctx.r[3].s64 = 101;
	// 82D9D0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D0A8 size=84
    let mut pc: u32 = 0x82D9D0A8;
    'dispatch: loop {
        match pc {
            0x82D9D0A8 => {
    //   block [0x82D9D0A8..0x82D9D0FC)
	// 82D9D0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9D0AC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82D9D0B0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82D9D0B4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82D9D0B8: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9D0BC: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D9D0C0: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D9D0C4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D0C8: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9D0CC: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82D9D0D0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D9D0D4: 1D4B0134  mulli r10, r11, 0x134
	ctx.r[10].s64 = ctx.r[11].s64 * 308;
	// 82D9D0D8: 394A0033  addi r10, r10, 0x33
	ctx.r[10].s64 = ctx.r[10].s64 + 51;
	// 82D9D0DC: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9D0E0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D9D0E4: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D9D0E8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9D0EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D9D0F0: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9D0F4: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D9D0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D100 size=40
    let mut pc: u32 = 0x82D9D100;
    'dispatch: loop {
        match pc {
            0x82D9D100 => {
    //   block [0x82D9D100..0x82D9D128)
	// 82D9D100: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82D9D104: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9D108: 419A0020  beq cr6, 0x82d9d128
	if ctx.cr[6].eq {
		sub_82D9D128(ctx, base);
		return;
	}
	// 82D9D10C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D9D110: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9D114: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D9D118: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9D11C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9D120: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D128 size=16
    let mut pc: u32 = 0x82D9D128;
    'dispatch: loop {
        match pc {
            0x82D9D128 => {
    //   block [0x82D9D128..0x82D9D138)
	// 82D9D128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9D12C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D130: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9D134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9D138 size=152
    let mut pc: u32 = 0x82D9D138;
    'dispatch: loop {
        match pc {
            0x82D9D138 => {
    //   block [0x82D9D138..0x82D9D1D0)
	// 82D9D138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9D140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9D144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D148: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D14C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D150: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D9D154: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82D9D158: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82D9D15C: C00B0B0C  lfs f0, 0xb0c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2828 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9D160: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D164: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9D168: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D9D16C: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9D170: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9D174: 394BFB84  addi r10, r11, -0x47c
	ctx.r[10].s64 = ctx.r[11].s64 + -1148;
	// 82D9D178: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9D17C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9D180: C18BE240  lfs f12, -0x1dc0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7616 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D9D184: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D188: C16B0BF8  lfs f11, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D9D18C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9D190: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9D194: B13F000C  sth r9, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82D9D198: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D9D19C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82D9D1A0: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82D9D1A4: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D9D1A8: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D9D1AC: D19F002C  stfs f12, 0x2c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D9D1B0: D17F0030  stfs f11, 0x30(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D9D1B4: 480013ED  bl 0x82d9e5a0
	ctx.lr = 0x82D9D1B8;
	sub_82D9E5A0(ctx, base);
	// 82D9D1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9D1BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9D1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9D1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9D1C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9D1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9D1D0 size=104
    let mut pc: u32 = 0x82D9D1D0;
    'dispatch: loop {
        match pc {
            0x82D9D1D0 => {
    //   block [0x82D9D1D0..0x82D9D238)
	// 82D9D1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9D1D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9D1DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9D1E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D1E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9D1E8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82D9D1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D1F0: 396BFB84  addi r11, r11, -0x47c
	ctx.r[11].s64 = ctx.r[11].s64 + -1148;
	// 82D9D1F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D9D1F8: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82D9D1FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9D200: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D204: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D9D208: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9D20C: 48001395  bl 0x82d9e5a0
	ctx.lr = 0x82D9D210;
	sub_82D9E5A0(ctx, base);
	// 82D9D210: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9D214: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9D218: 48001389  bl 0x82d9e5a0
	ctx.lr = 0x82D9D21C;
	sub_82D9E5A0(ctx, base);
	// 82D9D21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9D220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9D224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9D228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9D22C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9D230: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9D234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9D238 size=108
    let mut pc: u32 = 0x82D9D238;
    'dispatch: loop {
        match pc {
            0x82D9D238 => {
    //   block [0x82D9D238..0x82D9D2A4)
	// 82D9D238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9D240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9D244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D248: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9D24C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D250: 396BFB84  addi r11, r11, -0x47c
	ctx.r[11].s64 = ctx.r[11].s64 + -1148;
	// 82D9D254: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D258: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9D25C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9D260: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D9D264: 409A0020  bne cr6, 0x82d9d284
	if !ctx.cr[6].eq {
	pc = 0x82D9D284; continue 'dispatch;
	}
	// 82D9D268: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D26C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D9D270: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D9D274: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9D278: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D9D27C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D9D280: 4BFB8049  bl 0x82d552c8
	ctx.lr = 0x82D9D284;
	sub_82D552C8(ctx, base);
	// 82D9D284: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9D288: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D9D28C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9D294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9D298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9D29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9D2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9D2A8 size=108
    let mut pc: u32 = 0x82D9D2A8;
    'dispatch: loop {
        match pc {
            0x82D9D2A8 => {
    //   block [0x82D9D2A8..0x82D9D314)
	// 82D9D2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D2AC: 4BF0C161  bl 0x82ca940c
	ctx.lr = 0x82D9D2B0;
	sub_82CA93D0(ctx, base);
	// 82D9D2B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D2B4: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 82D9D2B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D9D2BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D9D2C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9D2C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9D2C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D9D2CC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D9D2D0: 409A0010  bne cr6, 0x82d9d2e0
	if !ctx.cr[6].eq {
	pc = 0x82D9D2E0; continue 'dispatch;
	}
	// 82D9D2D4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82D9D2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9D2DC: 4BFB9CBD  bl 0x82d56f98
	ctx.lr = 0x82D9D2E0;
	sub_82D56F98(ctx, base);
	// 82D9D2E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9D2E4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82D9D2E8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D2EC: 556A2834  slwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9D2F0: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82D9D2F4: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D9D2F8: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9D318 size=640
    let mut pc: u32 = 0x82D9D318;
    'dispatch: loop {
        match pc {
            0x82D9D318 => {
    //   block [0x82D9D318..0x82D9D598)
	// 82D9D318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D31C: 4BF0C0C1  bl 0x82ca93dc
	ctx.lr = 0x82D9D320;
	sub_82CA93D0(ctx, base);
	// 82D9D320: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D324: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82D9D328: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82D9D32C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D9D330: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82D9D334: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82D9D338: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82D9D33C: 80B3004C  lwz r5, 0x4c(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D9D340: 4804BB39  bl 0x82de8e78
	ctx.lr = 0x82D9D344;
	sub_82DE8E78(ctx, base);
	// 82D9D344: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82D9D348: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82D9D34C: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82D9D350: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82D9D354: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82D9D358: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D9D35C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82D9D360: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D9D364: 4200FFF0  bdnz 0x82d9d354
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82D9D354; continue 'dispatch;
	}
	// 82D9D368: 394100EC  addi r10, r1, 0xec
	ctx.r[10].s64 = ctx.r[1].s64 + 236;
	// 82D9D36C: 82930048  lwz r20, 0x48(r19)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D9D370: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D9D374: 81730030  lwz r11, 0x30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D9D378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D9D37C: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D380: 914100E0  stw r10, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82D9D384: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D9D388: 93C100E4  stw r30, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 82D9D38C: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 82D9D390: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82D9D394: 93C100AC  stw r30, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82D9D398: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82D9D39C: 81540014  lwz r10, 0x14(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D9D3A0: 814A0090  lwz r10, 0x90(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D9D3A4: 7EAA5850  subf r21, r10, r11
	ctx.r[21].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D9D3A8: 8154002C  lwz r10, 0x2c(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9D3AC: 81740030  lwz r11, 0x30(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D9D3B0: 3A4BFFFF  addi r18, r11, -1
	ctx.r[18].s64 = ctx.r[11].s64 + -1;
	// 82D9D3B4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D3B8: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D9D3BC: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 82D9D3C0: 912100E4  stw r9, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82D9D3C4: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82D9D3C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D3CC: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9D3D0: 7D6AAA14  add r11, r10, r21
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 82D9D3D4: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82D9D3D8: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82D9D3DC: 7D755850  subf r11, r21, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 82D9D3E0: 916100EC  stw r11, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 82D9D3E4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D9D3E8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82D9D3EC: 4804EE85  bl 0x82dec270
	ctx.lr = 0x82D9D3F0;
	sub_82DEC270(ctx, base);
	// 82D9D3F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D3F4: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82D9D3F8: 40990144  ble cr6, 0x82d9d53c
	if !ctx.cr[6].gt {
	pc = 0x82D9D53C; continue 'dispatch;
	}
	// 82D9D3FC: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82D9D400: 7E5D9378  mr r29, r18
	ctx.r[29].u64 = ctx.r[18].u64;
	// 82D9D404: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82D9D408: 3AE00020  li r23, 0x20
	ctx.r[23].s64 = 32;
	// 82D9D40C: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82D9D410: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82D9D414: 814100E8  lwz r10, 0xe8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 82D9D418: 812100E4  lwz r9, 0xe4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82D9D41C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D9D420: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82D9D424: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D9D428: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82D9D42C: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82D9D430: 8174002C  lwz r11, 0x2c(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9D434: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D9D438: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9D43C: 3B6B00D0  addi r27, r11, 0xd0
	ctx.r[27].s64 = ctx.r[11].s64 + 208;
	// 82D9D440: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D9D444: 7D6AAA14  add r11, r10, r21
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 82D9D448: 7F555850  subf r26, r21, r11
	ctx.r[26].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 82D9D44C: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82D9D450: 409A0010  bne cr6, 0x82d9d460
	if !ctx.cr[6].eq {
	pc = 0x82D9D460; continue 'dispatch;
	}
	// 82D9D454: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D9D458: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82D9D45C: 4BFB9B3D  bl 0x82d56f98
	ctx.lr = 0x82D9D460;
	sub_82D56F98(ctx, base);
	// 82D9D460: 816100E4  lwz r11, 0xe4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82D9D464: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D9D468: 814100E0  lwz r10, 0xe0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 82D9D46C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9D470: 7F4B512E  stwx r26, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 82D9D474: 816100E4  lwz r11, 0xe4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82D9D478: 81390018  lwz r9, 0x18(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9D47C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9D480: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82D9D484: 7D3C4A14  add r9, r28, r9
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 82D9D488: 916100E4  stw r11, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82D9D48C: 397B0010  addi r11, r27, 0x10
	ctx.r[11].s64 = ctx.r[27].s64 + 16;
	// 82D9D490: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9D598 size=164
    let mut pc: u32 = 0x82D9D598;
    'dispatch: loop {
        match pc {
            0x82D9D598 => {
    //   block [0x82D9D598..0x82D9D63C)
	// 82D9D598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9D5A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9D5A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9D5A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D5AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9D5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D5B4: 396BFB84  addi r11, r11, -0x47c
	ctx.r[11].s64 = ctx.r[11].s64 + -1148;
	// 82D9D5B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D9D5BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D5C0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9D5C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9D5C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D9D5CC: 409A0020  bne cr6, 0x82d9d5ec
	if !ctx.cr[6].eq {
	pc = 0x82D9D5EC; continue 'dispatch;
	}
	// 82D9D5D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D5D4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D9D5D8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D9D5DC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9D5E0: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D9D5E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D9D5E8: 4BFB7CE1  bl 0x82d552c8
	ctx.lr = 0x82D9D5EC;
	sub_82D552C8(ctx, base);
	// 82D9D5EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9D5F0: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D9D5F4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D9D5F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D9D5FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9D600: 419A0020  beq cr6, 0x82d9d620
	if ctx.cr[6].eq {
	pc = 0x82D9D620; continue 'dispatch;
	}
	// 82D9D604: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9D608: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D9D60C: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D9D610: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9D614: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9D618: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D9D61C: 4BFB7CAD  bl 0x82d552c8
	ctx.lr = 0x82D9D620;
	sub_82D552C8(ctx, base);
	// 82D9D620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9D624: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9D628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9D62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9D630: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9D634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9D638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D640 size=8
    let mut pc: u32 = 0x82D9D640;
    'dispatch: loop {
        match pc {
            0x82D9D640 => {
    //   block [0x82D9D640..0x82D9D648)
	// 82D9D640: 9883000C  stb r4, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 82D9D644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9D648 size=12
    let mut pc: u32 = 0x82D9D648;
    'dispatch: loop {
        match pc {
            0x82D9D648 => {
    //   block [0x82D9D648..0x82D9D654)
	// 82D9D648: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9D64C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9D650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9D658 size=12
    let mut pc: u32 = 0x82D9D658;
    'dispatch: loop {
        match pc {
            0x82D9D658 => {
    //   block [0x82D9D658..0x82D9D664)
	// 82D9D658: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D65C: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D9D660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9D668 size=8
    let mut pc: u32 = 0x82D9D668;
    'dispatch: loop {
        match pc {
            0x82D9D668 => {
    //   block [0x82D9D668..0x82D9D670)
	// 82D9D668: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D9D66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9D670 size=8
    let mut pc: u32 = 0x82D9D670;
    'dispatch: loop {
        match pc {
            0x82D9D670 => {
    //   block [0x82D9D670..0x82D9D678)
	// 82D9D670: D0230008  stfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D9D674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9D678 size=216
    let mut pc: u32 = 0x82D9D678;
    'dispatch: loop {
        match pc {
            0x82D9D678 => {
    //   block [0x82D9D678..0x82D9D750)
	// 82D9D678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9D680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9D684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9D688: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D9D68C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D690: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D9D694: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D69C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D9D6A0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9D6A4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82D9D6A8: 4098000C  bge cr6, 0x82d9d6b4
	if !ctx.cr[6].lt {
	pc = 0x82D9D6B4; continue 'dispatch;
	}
	// 82D9D6AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9D6B0: 4800000C  b 0x82d9d6bc
	pc = 0x82D9D6BC; continue 'dispatch;
	// 82D9D6B4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9D6B8: 4BFB9F91  bl 0x82d57648
	ctx.lr = 0x82D9D6BC;
	sub_82D57648(ctx, base);
	// 82D9D6BC: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82D9D6C0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9D6C4: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 82D9D6C8: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D9D6CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82D9D6D0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D9D6D4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82D9D6D8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9D6DC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82D9D6E0: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D9D6E4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82D9D6E8: 40980040  bge cr6, 0x82d9d728
	if !ctx.cr[6].lt {
	pc = 0x82D9D728; continue 'dispatch;
	}
	// 82D9D6EC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82D9D6F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D9D6F4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D9D6F8: 394A4C40  addi r10, r10, 0x4c40
	ctx.r[10].s64 = ctx.r[10].s64 + 19520;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9D750 size=724
    let mut pc: u32 = 0x82D9D750;
    'dispatch: loop {
        match pc {
            0x82D9D750 => {
    //   block [0x82D9D750..0x82D9DA24)
	// 82D9D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9D754: 4BF0BCA1  bl 0x82ca93f4
	ctx.lr = 0x82D9D758;
	sub_82CA93D0(ctx, base);
	// 82D9D758: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82D9D75C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9D760: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9D764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9D768: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D9D76C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82D9D770: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D9D774: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82D9D778: 40980008  bge cr6, 0x82d9d780
	if !ctx.cr[6].lt {
	pc = 0x82D9D780; continue 'dispatch;
	}
	// 82D9D77C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9D780: 4BFB9EC9  bl 0x82d57648
	ctx.lr = 0x82D9D784;
	sub_82D57648(ctx, base);
	// 82D9D784: 839F0024  lwz r28, 0x24(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9D788: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82D9D78C: 3B3F0024  addi r25, r31, 0x24
	ctx.r[25].s64 = ctx.r[31].s64 + 36;
	// 82D9D790: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82D9D794: 4198000C  blt cr6, 0x82d9d7a0
	if ctx.cr[6].lt {
	pc = 0x82D9D7A0; continue 'dispatch;
	}
	// 82D9D798: 395CFFFF  addi r10, r28, -1
	ctx.r[10].s64 = ctx.r[28].s64 + -1;
	// 82D9D79C: 386AFFFF  addi r3, r10, -1
	ctx.r[3].s64 = ctx.r[10].s64 + -1;
	// 82D9D7A0: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9D7A4: 54682036  slwi r8, r3, 4
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9DA28 size=660
    let mut pc: u32 = 0x82D9DA28;
    'dispatch: loop {
        match pc {
            0x82D9DA28 => {
    //   block [0x82D9DA28..0x82D9DCBC)
	// 82D9DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9DA30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9DA34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9DA38: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 82D9DA3C: 4BF1029D  bl 0x82cadcd8
	ctx.lr = 0x82D9DA40;
	sub_82CADCA0(ctx, base);
	// 82D9DA40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9DA44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9DA48: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82D9DA4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9DA50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D9DA54: C3DF0008  lfs f30, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82D9DA58: C3EB0C14  lfs f31, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D9DA5C: EF9FF024  fdivs f28, f31, f30
	ctx.f[28].f64 = ((ctx.f[31].f64 / ctx.f[30].f64) as f32) as f64;
	// 82D9DA60: 4BFB9BE9  bl 0x82d57648
	ctx.lr = 0x82D9DA64;
	sub_82D57648(ctx, base);
	// 82D9DA64: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9DA68: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82D9DA6C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D9DA70: 41980010  blt cr6, 0x82d9da80
	if ctx.cr[6].lt {
	pc = 0x82D9DA80; continue 'dispatch;
	}
	// 82D9DA74: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82D9DA78: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82D9DA7C: 48000014  b 0x82d9da90
	pc = 0x82D9DA90; continue 'dispatch;
	// 82D9DA80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D9DA84: 4098000C  bge cr6, 0x82d9da90
	if !ctx.cr[6].lt {
	pc = 0x82D9DA90; continue 'dispatch;
	}
	// 82D9DA88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9DA8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9DA90: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9DA94: 546A2036  slwi r10, r3, 4
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9DA98: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D9DA9C: 7C6707B4  extsw r7, r3
	ctx.r[7].s64 = ctx.r[3].s32 as i64;
	// 82D9DAA0: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9DCC0 size=32
    let mut pc: u32 = 0x82D9DCC0;
    'dispatch: loop {
        match pc {
            0x82D9DCC0 => {
    //   block [0x82D9DCC0..0x82D9DCE0)
	// 82D9DCC0: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9DCC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9DCC8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82D9DCCC: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82D9DCD0: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9DCD4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82D9DCD8: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82D9DCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9DCE0 size=172
    let mut pc: u32 = 0x82D9DCE0;
    'dispatch: loop {
        match pc {
            0x82D9DCE0 => {
    //   block [0x82D9DCE0..0x82D9DD8C)
	// 82D9DCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9DCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9DCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9DCEC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82D9DCF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9DCF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9DCF8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D9DCFC: 4BFB994D  bl 0x82d57648
	ctx.lr = 0x82D9DD00;
	sub_82D57648(ctx, base);
	// 82D9DD00: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9DD04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9DD08: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D9DD0C: 41980020  blt cr6, 0x82d9dd2c
	if ctx.cr[6].lt {
	pc = 0x82D9DD2C; continue 'dispatch;
	}
	// 82D9DD10: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D9DD14: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9DD18: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9DD1C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D9DD20: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9DD24: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9DD28: 48000024  b 0x82d9dd4c
	pc = 0x82D9DD4C; continue 'dispatch;
	// 82D9DD2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D9DD30: 40980008  bge cr6, 0x82d9dd38
	if !ctx.cr[6].lt {
	pc = 0x82D9DD38; continue 'dispatch;
	}
	// 82D9DD34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9DD38: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9DD3C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9DD40: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D9DD44: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9DD48: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9DD4C: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82D9DD50: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D9DD54: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9DD58: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82D9DD5C: 7DA95C2E  lfsx f13, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9DD60: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D9DD64: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82D9DD68: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82D9DD6C: ED9F6028  fsubs f12, f31, f12
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[12].f64) as f32) as f64);
	// 82D9DD70: EC2C683A  fmadds f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D9DD74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9DD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9DD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9DD80: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9DD84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9DD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9DD90 size=348
    let mut pc: u32 = 0x82D9DD90;
    'dispatch: loop {
        match pc {
            0x82D9DD90 => {
    //   block [0x82D9DD90..0x82D9DEEC)
	// 82D9DD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9DD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9DD98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9DD9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9DDA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9DDA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9DDA8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D9DDAC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D9DDB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9DDB4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D9DDB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9DDBC: 4E800421  bctrl
	ctx.lr = 0x82D9DDC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9DDC0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D9DDC4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D9DDC8: 396B6E60  addi r11, r11, 0x6e60
	ctx.r[11].s64 = ctx.r[11].s64 + 28256;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9DEF0 size=120
    let mut pc: u32 = 0x82D9DEF0;
    'dispatch: loop {
        match pc {
            0x82D9DEF0 => {
    //   block [0x82D9DEF0..0x82D9DF68)
	// 82D9DEF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9DEF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D9DEF8: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82D9DEFC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9DF00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D9DF04: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D9DF08: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82D9DF0C: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D9DF10: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D9DF14: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9DF18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9DF1C: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82D9DF20: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82D9DF24: 394B9F2C  addi r10, r11, -0x60d4
	ctx.r[10].s64 = ctx.r[11].s64 + -24788;
	// 82D9DF28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9DF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9DF68 size=388
    let mut pc: u32 = 0x82D9DF68;
    'dispatch: loop {
        match pc {
            0x82D9DF68 => {
    //   block [0x82D9DF68..0x82D9E0EC)
	// 82D9DF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9DF6C: 4BF0B4A1  bl 0x82ca940c
	ctx.lr = 0x82D9DF70;
	sub_82CA93D0(ctx, base);
	// 82D9DF70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9DF74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9DF78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D9DF7C: 3BFE0020  addi r31, r30, 0x20
	ctx.r[31].s64 = ctx.r[30].s64 + 32;
	// 82D9DF80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9DF84: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9DF88: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D9DF8C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D9DF90: 409A0010  bne cr6, 0x82d9dfa0
	if !ctx.cr[6].eq {
	pc = 0x82D9DFA0; continue 'dispatch;
	}
	// 82D9DF94: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D9DF98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9DF9C: 4BFB8FFD  bl 0x82d56f98
	ctx.lr = 0x82D9DFA0;
	sub_82D56F98(ctx, base);
	// 82D9DFA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9DFA4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9DFA8: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D9DFAC: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82D9DFB0: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D9DFB4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E0F0 size=216
    let mut pc: u32 = 0x82D9E0F0;
    'dispatch: loop {
        match pc {
            0x82D9E0F0 => {
    //   block [0x82D9E0F0..0x82D9E1C8)
	// 82D9E0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E0F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9E0FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E104: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9E108: 3BC30020  addi r30, r3, 0x20
	ctx.r[30].s64 = ctx.r[3].s64 + 32;
	// 82D9E10C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9E110: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E114: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D9E118: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D9E11C: 40980060  bge cr6, 0x82d9e17c
	if !ctx.cr[6].lt {
	pc = 0x82D9E17C; continue 'dispatch;
	}
	// 82D9E120: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9E124: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9E128: 409A0020  bne cr6, 0x82d9e148
	if !ctx.cr[6].eq {
	pc = 0x82D9E148; continue 'dispatch;
	}
	// 82D9E12C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E130: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D9E134: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D9E138: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E13C: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D9E140: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D9E144: 4BFB7185  bl 0x82d552c8
	ctx.lr = 0x82D9E148;
	sub_82D552C8(ctx, base);
	// 82D9E148: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E14C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D9E150: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E154: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D9E158: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D9E15C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D9E160: 4BFB70E9  bl 0x82d55248
	ctx.lr = 0x82D9E164;
	sub_82D55248(ctx, base);
	// 82D9E164: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9E168: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D9E16C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E170: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9E174: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D9E178: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9E17C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E180: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E184: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D9E188: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D9E18C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E190: 40990020  ble cr6, 0x82d9e1b0
	if !ctx.cr[6].gt {
	pc = 0x82D9E1B0; continue 'dispatch;
	}
	// 82D9E194: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82D9E198: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E1C8 size=88
    let mut pc: u32 = 0x82D9E1C8;
    'dispatch: loop {
        match pc {
            0x82D9E1C8 => {
    //   block [0x82D9E1C8..0x82D9E220)
	// 82D9E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E1D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9E1D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E1D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E1DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9E1E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9E1E4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E1E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E1EC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9E1F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E1F4: 4E800421  bctrl
	ctx.lr = 0x82D9E1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9E1F8: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 82D9E1FC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82D9E200: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D9E204: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82D9E208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9E20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E214: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9E218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E220 size=20
    let mut pc: u32 = 0x82D9E220;
    'dispatch: loop {
        match pc {
            0x82D9E220 => {
    //   block [0x82D9E220..0x82D9E234)
	// 82D9E220: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E228: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D9E22C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E230: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9E238 size=8
    let mut pc: u32 = 0x82D9E238;
    'dispatch: loop {
        match pc {
            0x82D9E238 => {
    //   block [0x82D9E238..0x82D9E240)
	// 82D9E238: D023001C  stfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D9E23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9E240 size=8
    let mut pc: u32 = 0x82D9E240;
    'dispatch: loop {
        match pc {
            0x82D9E240 => {
    //   block [0x82D9E240..0x82D9E248)
	// 82D9E240: C023001C  lfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D9E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E248 size=64
    let mut pc: u32 = 0x82D9E248;
    'dispatch: loop {
        match pc {
            0x82D9E248 => {
    //   block [0x82D9E248..0x82D9E288)
	// 82D9E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E258: 8084000C  lwz r4, 0xc(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E25C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E260: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E264: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E26C: 4E800421  bctrl
	ctx.lr = 0x82D9E270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9E270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9E274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9E278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E288 size=8
    let mut pc: u32 = 0x82D9E288;
    'dispatch: loop {
        match pc {
            0x82D9E288 => {
    //   block [0x82D9E288..0x82D9E290)
	// 82D9E288: 3860000D  li r3, 0xd
	ctx.r[3].s64 = 13;
	// 82D9E28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E290 size=8
    let mut pc: u32 = 0x82D9E290;
    'dispatch: loop {
        match pc {
            0x82D9E290 => {
    //   block [0x82D9E290..0x82D9E298)
	// 82D9E290: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E298 size=8
    let mut pc: u32 = 0x82D9E298;
    'dispatch: loop {
        match pc {
            0x82D9E298 => {
    //   block [0x82D9E298..0x82D9E2A0)
	// 82D9E298: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9E2A0 size=132
    let mut pc: u32 = 0x82D9E2A0;
    'dispatch: loop {
        match pc {
            0x82D9E2A0 => {
    //   block [0x82D9E2A0..0x82D9E324)
	// 82D9E2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E2A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E2AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E2B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D9E2B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9E2B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E2BC: 396BFBD4  addi r11, r11, -0x42c
	ctx.r[11].s64 = ctx.r[11].s64 + -1068;
	// 82D9E2C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D9E2C4: C00A0BE8  lfs f0, 0xbe8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9E2C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D9E2CC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82D9E2D0: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82D9E2D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E2D8: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D9E2DC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D9E2E0: B15F0010  sth r10, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 82D9E2E4: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D9E2E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E2EC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E2F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D9E2F4: 419A0010  beq cr6, 0x82d9e304
	if ctx.cr[6].eq {
	pc = 0x82D9E304; continue 'dispatch;
	}
	// 82D9E2F8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9E2FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D9E300: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D9E304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9E308: 48000299  bl 0x82d9e5a0
	ctx.lr = 0x82D9E30C;
	sub_82D9E5A0(ctx, base);
	// 82D9E30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9E310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9E314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E31C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E328 size=104
    let mut pc: u32 = 0x82D9E328;
    'dispatch: loop {
        match pc {
            0x82D9E328 => {
    //   block [0x82D9E328..0x82D9E390)
	// 82D9E328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9E334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E33C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9E340: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82D9E344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E348: 396BFBD4  addi r11, r11, -0x42c
	ctx.r[11].s64 = ctx.r[11].s64 + -1068;
	// 82D9E34C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D9E350: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82D9E354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9E358: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E35C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D9E360: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9E364: 4800023D  bl 0x82d9e5a0
	ctx.lr = 0x82D9E368;
	sub_82D9E5A0(ctx, base);
	// 82D9E368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9E36C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9E370: 48000231  bl 0x82d9e5a0
	ctx.lr = 0x82D9E374;
	sub_82D9E5A0(ctx, base);
	// 82D9E374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9E378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9E37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9E388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E390 size=124
    let mut pc: u32 = 0x82D9E390;
    'dispatch: loop {
        match pc {
            0x82D9E390 => {
    //   block [0x82D9E390..0x82D9E40C)
	// 82D9E390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E39C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E3A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E3A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9E3A8: 396BFBD4  addi r11, r11, -0x42c
	ctx.r[11].s64 = ctx.r[11].s64 + -1068;
	// 82D9E3AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E3B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E3B4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9E3BC: 419A0030  beq cr6, 0x82d9e3ec
	if ctx.cr[6].eq {
	pc = 0x82D9E3EC; continue 'dispatch;
	}
	// 82D9E3C0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9E3C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9E3C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D9E3CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9E3D0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9E3D4: 409A0018  bne cr6, 0x82d9e3ec
	if !ctx.cr[6].eq {
	pc = 0x82D9E3EC; continue 'dispatch;
	}
	// 82D9E3D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E3DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9E3E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E3E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E3E8: 4E800421  bctrl
	ctx.lr = 0x82D9E3EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9E3EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9E3F0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D9E3F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9E3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E404: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9E410 size=184
    let mut pc: u32 = 0x82D9E410;
    'dispatch: loop {
        match pc {
            0x82D9E410 => {
    //   block [0x82D9E410..0x82D9E4C8)
	// 82D9E410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E41C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E420: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D9E424: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D9E428: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82D9E42C: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82D9E430: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82D9E434: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82D9E438: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D9E43C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82D9E440: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D9E444: 4200FFF0  bdnz 0x82d9e434
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82D9E434; continue 'dispatch;
	}
	// 82D9E448: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9E44C: C003001C  lfs f0, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9E450: C1A10090  lfs f13, 0x90(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9E454: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D9E458: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D9E45C: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82D9E460: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D9E464: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E468: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D9E46C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E470: 4E800421  bctrl
	ctx.lr = 0x82D9E474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9E474: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D9E478: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82D9E47C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E480: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82D9E484: 409A0024  bne cr6, 0x82d9e4a8
	if !ctx.cr[6].eq {
	pc = 0x82D9E4A8; continue 'dispatch;
	}
	// 82D9E488: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D9E48C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82D9E490: 48055379  bl 0x82df3808
	ctx.lr = 0x82D9E494;
	sub_82DF3808(ctx, base);
	// 82D9E494: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82D9E498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E4A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E4A4: 4E800020  blr
	return;
	// 82D9E4A8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82D9E4AC: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D9E4B0: 4804AB71  bl 0x82de9020
	ctx.lr = 0x82D9E4B4;
	sub_82DE9020(ctx, base);
	// 82D9E4B4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82D9E4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E4C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E4C8 size=100
    let mut pc: u32 = 0x82D9E4C8;
    'dispatch: loop {
        match pc {
            0x82D9E4C8 => {
    //   block [0x82D9E4C8..0x82D9E52C)
	// 82D9E4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E4D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9E4D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E4D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E4DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E4E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D9E4E4: 4BFFFEAD  bl 0x82d9e390
	ctx.lr = 0x82D9E4E8;
	sub_82D9E390(ctx, base);
	// 82D9E4E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D9E4EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9E4F0: 419A0020  beq cr6, 0x82d9e510
	if ctx.cr[6].eq {
	pc = 0x82D9E510; continue 'dispatch;
	}
	// 82D9E4F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E4F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D9E4FC: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D9E500: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E504: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9E508: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D9E50C: 4BFB6DBD  bl 0x82d552c8
	ctx.lr = 0x82D9E510;
	sub_82D552C8(ctx, base);
	// 82D9E510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9E514: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9E518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E520: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9E524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E530 size=96
    let mut pc: u32 = 0x82D9E530;
    'dispatch: loop {
        match pc {
            0x82D9E530 => {
    //   block [0x82D9E530..0x82D9E590)
	// 82D9E530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E53C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E544: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D9E548: 419A0030  beq cr6, 0x82d9e578
	if ctx.cr[6].eq {
	pc = 0x82D9E578; continue 'dispatch;
	}
	// 82D9E54C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9E550: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D9E554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D9E558: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82D9E55C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E560: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D9E564: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D9E568: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D9E56C: 4BFEACD5  bl 0x82d89240
	ctx.lr = 0x82D9E570;
	sub_82D89240(ctx, base);
	// 82D9E570: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E574: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9E578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9E57C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9E580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E590 size=16
    let mut pc: u32 = 0x82D9E590;
    'dispatch: loop {
        match pc {
            0x82D9E590 => {
    //   block [0x82D9E590..0x82D9E5A0)
	// 82D9E590: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E594: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9E598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E59C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E5A0 size=20
    let mut pc: u32 = 0x82D9E5A0;
    'dispatch: loop {
        match pc {
            0x82D9E5A0 => {
    //   block [0x82D9E5A0..0x82D9E5B4)
	// 82D9E5A0: 3D6082DA  lis r11, -0x7d26
	ctx.r[11].s64 = -2099642368;
	// 82D9E5A4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82D9E5A8: 396BE590  addi r11, r11, -0x1a70
	ctx.r[11].s64 = ctx.r[11].s64 + -6768;
	// 82D9E5AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9E5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9E5B8 size=156
    let mut pc: u32 = 0x82D9E5B8;
    'dispatch: loop {
        match pc {
            0x82D9E5B8 => {
    //   block [0x82D9E5B8..0x82D9E654)
	// 82D9E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E5C4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82D9E5C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E5CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D9E5D0: C1A50040  lfs f13, 0x40(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D9E5D4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82D9E5D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D9E5DC: 38650010  addi r3, r5, 0x10
	ctx.r[3].s64 = ctx.r[5].s64 + 16;
	// 82D9E5E0: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82D9E5E4: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9E5E8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82D9E5EC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E658 size=4
    let mut pc: u32 = 0x82D9E658;
    'dispatch: loop {
        match pc {
            0x82D9E658 => {
    //   block [0x82D9E658..0x82D9E65C)
	// 82D9E658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D9E660 size=224
    let mut pc: u32 = 0x82D9E660;
    'dispatch: loop {
        match pc {
            0x82D9E660 => {
    //   block [0x82D9E660..0x82D9E740)
	// 82D9E660: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82D9E664: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D9E668: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D9E66C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9E670: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E740 size=32
    let mut pc: u32 = 0x82D9E740;
    'dispatch: loop {
        match pc {
            0x82D9E740 => {
    //   block [0x82D9E740..0x82D9E760)
	// 82D9E740: 89630009  lbz r11, 9(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 82D9E744: 89430008  lbz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9E748: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82D9E74C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D9E750: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82D9E754: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9E758: 9963000A  stb r11, 0xa(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82D9E75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E760 size=60
    let mut pc: u32 = 0x82D9E760;
    'dispatch: loop {
        match pc {
            0x82D9E760 => {
    //   block [0x82D9E760..0x82D9E79C)
	// 82D9E760: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E7A0 size=68
    let mut pc: u32 = 0x82D9E7A0;
    'dispatch: loop {
        match pc {
            0x82D9E7A0 => {
    //   block [0x82D9E7A0..0x82D9E7E4)
	// 82D9E7A0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E7E8 size=68
    let mut pc: u32 = 0x82D9E7E8;
    'dispatch: loop {
        match pc {
            0x82D9E7E8 => {
    //   block [0x82D9E7E8..0x82D9E82C)
	// 82D9E7E8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E830 size=20
    let mut pc: u32 = 0x82D9E830;
    'dispatch: loop {
        match pc {
            0x82D9E830 => {
    //   block [0x82D9E830..0x82D9E844)
	// 82D9E830: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9E838: 396BFFE9  addi r11, r11, -0x17
	ctx.r[11].s64 = ctx.r[11].s64 + -23;
	// 82D9E83C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82D9E840: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E844 size=64
    let mut pc: u32 = 0x82D9E844;
    'dispatch: loop {
        match pc {
            0x82D9E844 => {
    //   block [0x82D9E844..0x82D9E86C)
	// 82D9E844: 3D8082DA  lis r12, -0x7d26
	ctx.r[12].s64 = -2099642368;
	// 82D9E848: 398CE85C  addi r12, r12, -0x17a4
	ctx.r[12].s64 = ctx.r[12].s64 + -6052;
	// 82D9E84C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D9E850: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D9E854: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D9E858: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82D9E86C; continue 'dispatch;
		},
		1 => {
	pc = 0x82D9E874; continue 'dispatch;
		},
		2 => {
	pc = 0x82D9E86C; continue 'dispatch;
		},
		3 => {
	pc = 0x82D9E874; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D9E85C: 82D9E86C  lwz r22, -0x1794(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-6036 as u32) ) } as u64;
	// 82D9E860: 82D9E874  lwz r22, -0x178c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-6028 as u32) ) } as u64;
	// 82D9E864: 82D9E86C  lwz r22, -0x1794(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-6036 as u32) ) } as u64;
	// 82D9E868: 82D9E874  lwz r22, -0x178c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-6028 as u32) ) } as u64;
            }
            0x82D9E86C => {
    //   block [0x82D9E86C..0x82D9E874)
	// 82D9E86C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D9E870: 4E800020  blr
	return;
            }
            0x82D9E874 => {
    //   block [0x82D9E874..0x82D9E884)
	// 82D9E874: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E878: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 82D9E87C: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9E880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E888 size=20
    let mut pc: u32 = 0x82D9E888;
    'dispatch: loop {
        match pc {
            0x82D9E888 => {
    //   block [0x82D9E888..0x82D9E89C)
	// 82D9E888: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82D9E88C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9E890: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E894: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82D9E898: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E89C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E89C size=96
    let mut pc: u32 = 0x82D9E89C;
    'dispatch: loop {
        match pc {
            0x82D9E89C => {
    //   block [0x82D9E89C..0x82D9E8D4)
	// 82D9E89C: 396BFFE9  addi r11, r11, -0x17
	ctx.r[11].s64 = ctx.r[11].s64 + -23;
	// 82D9E8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D9E8A4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82D9E8A8: 41990048  bgt cr6, 0x82d9e8f0
	if ctx.cr[6].gt {
	pc = 0x82D9E8F0; continue 'dispatch;
	}
	// 82D9E8AC: 3D8082DA  lis r12, -0x7d26
	ctx.r[12].s64 = -2099642368;
	// 82D9E8B0: 398CE8C4  addi r12, r12, -0x173c
	ctx.r[12].s64 = ctx.r[12].s64 + -5948;
	// 82D9E8B4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D9E8B8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D9E8BC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D9E8C0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82D9E8D4; continue 'dispatch;
		},
		1 => {
	pc = 0x82D9E8E4; continue 'dispatch;
		},
		2 => {
	pc = 0x82D9E8D4; continue 'dispatch;
		},
		3 => {
	pc = 0x82D9E8E4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D9E8C4: 82D9E8D4  lwz r22, -0x172c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-5932 as u32) ) } as u64;
	// 82D9E8C8: 82D9E8E4  lwz r22, -0x171c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-5916 as u32) ) } as u64;
	// 82D9E8CC: 82D9E8D4  lwz r22, -0x172c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-5932 as u32) ) } as u64;
	// 82D9E8D0: 82D9E8E4  lwz r22, -0x171c(r25)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-5916 as u32) ) } as u64;
            }
            0x82D9E8D4 => {
    //   block [0x82D9E8D4..0x82D9E8E4)
	// 82D9E8D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D9E8D8: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D9E8DC: 7D431B78  or r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 82D9E8E0: 4BFFFFB0  b 0x82d9e890
	sub_82D9E888(ctx, base);
	return;
            }
            0x82D9E8E4 => {
    //   block [0x82D9E8E4..0x82D9E8FC)
	// 82D9E8E4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E8E8: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 82D9E8EC: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9E8F0: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D9E8F4: 7D431B78  or r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 82D9E8F8: 4BFFFF98  b 0x82d9e890
	sub_82D9E888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E8FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E8FC size=4
    let mut pc: u32 = 0x82D9E8FC;
    'dispatch: loop {
        match pc {
            0x82D9E8FC => {
    //   block [0x82D9E8FC..0x82D9E900)
	// 82D9E8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E900 size=156
    let mut pc: u32 = 0x82D9E900;
    'dispatch: loop {
        match pc {
            0x82D9E900 => {
    //   block [0x82D9E900..0x82D9E99C)
	// 82D9E900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9E908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9E90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9E910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E914: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9E918: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9E91C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D9E920: 419A001C  beq cr6, 0x82d9e93c
	if ctx.cr[6].eq {
	pc = 0x82D9E93C; continue 'dispatch;
	}
	// 82D9E924: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9E92C: 419A0010  beq cr6, 0x82d9e93c
	if ctx.cr[6].eq {
	pc = 0x82D9E93C; continue 'dispatch;
	}
	// 82D9E930: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9E934: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9E938: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9E93C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9E940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9E944: 419A003C  beq cr6, 0x82d9e980
	if ctx.cr[6].eq {
	pc = 0x82D9E980; continue 'dispatch;
	}
	// 82D9E948: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9E950: 419A0030  beq cr6, 0x82d9e980
	if ctx.cr[6].eq {
	pc = 0x82D9E980; continue 'dispatch;
	}
	// 82D9E954: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9E958: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9E95C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D9E960: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9E964: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9E968: 409A0018  bne cr6, 0x82d9e980
	if !ctx.cr[6].eq {
	pc = 0x82D9E980; continue 'dispatch;
	}
	// 82D9E96C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E970: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9E974: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9E978: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9E97C: 4E800421  bctrl
	ctx.lr = 0x82D9E980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9E980: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82D9E984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9E988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9E98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9E990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9E994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9E998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E9A0 size=20
    let mut pc: u32 = 0x82D9E9A0;
    'dispatch: loop {
        match pc {
            0x82D9E9A0 => {
    //   block [0x82D9E9A0..0x82D9E9B4)
	// 82D9E9A0: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82D9E9A4: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 82D9E9A8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9E9AC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9E9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E9B8 size=16
    let mut pc: u32 = 0x82D9E9B8;
    'dispatch: loop {
        match pc {
            0x82D9E9B8 => {
    //   block [0x82D9E9B8..0x82D9E9C8)
	// 82D9E9B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D9E9BC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D9E9C0: 38AB0060  addi r5, r11, 0x60
	ctx.r[5].s64 = ctx.r[11].s64 + 96;
	// 82D9E9C4: 4BFB79E4  b 0x82d563a8
	sub_82D563A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9E9C8 size=8
    let mut pc: u32 = 0x82D9E9C8;
    'dispatch: loop {
        match pc {
            0x82D9E9C8 => {
    //   block [0x82D9E9C8..0x82D9E9D0)
	// 82D9E9C8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82D9E9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9E9D0 size=292
    let mut pc: u32 = 0x82D9E9D0;
    'dispatch: loop {
        match pc {
            0x82D9E9D0 => {
    //   block [0x82D9E9D0..0x82D9EAF4)
	// 82D9E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9E9D4: 4BF0AA35  bl 0x82ca9408
	ctx.lr = 0x82D9E9D8;
	sub_82CA93D0(ctx, base);
	// 82D9E9D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9E9DC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D9E9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9E9E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D9E9E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D9E9EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D9E9F0: 419A001C  beq cr6, 0x82d9ea0c
	if ctx.cr[6].eq {
	pc = 0x82D9EA0C; continue 'dispatch;
	}
	// 82D9E9F4: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9E9F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9E9FC: 419A0010  beq cr6, 0x82d9ea0c
	if ctx.cr[6].eq {
	pc = 0x82D9EA0C; continue 'dispatch;
	}
	// 82D9EA00: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9EA04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9EA08: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9EA0C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9EA10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9EA14: 419A003C  beq cr6, 0x82d9ea50
	if ctx.cr[6].eq {
	pc = 0x82D9EA50; continue 'dispatch;
	}
	// 82D9EA18: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9EA1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9EA20: 419A0030  beq cr6, 0x82d9ea50
	if ctx.cr[6].eq {
	pc = 0x82D9EA50; continue 'dispatch;
	}
	// 82D9EA24: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9EA28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9EA2C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D9EA30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9EA34: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9EA38: 409A0018  bne cr6, 0x82d9ea50
	if !ctx.cr[6].eq {
	pc = 0x82D9EA50; continue 'dispatch;
	}
	// 82D9EA3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9EA40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9EA44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9EA48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9EA4C: 4E800421  bctrl
	ctx.lr = 0x82D9EA50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9EA50: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82D9EA54: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82D9EA58: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82D9EA5C: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82D9EA60: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82D9EA64: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9EAF8 size=236
    let mut pc: u32 = 0x82D9EAF8;
    'dispatch: loop {
        match pc {
            0x82D9EAF8 => {
    //   block [0x82D9EAF8..0x82D9EBE4)
	// 82D9EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9EB00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9EB04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9EB08: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D9EB0C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82D9EB10: 3909A6A8  addi r8, r9, -0x5958
	ctx.r[8].s64 = ctx.r[9].s64 + -22872;
	// 82D9EB14: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D9EB18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9EB1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9EBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9EBE8 size=132
    let mut pc: u32 = 0x82D9EBE8;
    'dispatch: loop {
        match pc {
            0x82D9EBE8 => {
    //   block [0x82D9EBE8..0x82D9EC6C)
	// 82D9EBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9EBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9EBF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9EBF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9EBF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9EBFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9EC00: 396BA6A8  addi r11, r11, -0x5958
	ctx.r[11].s64 = ctx.r[11].s64 + -22872;
	// 82D9EC04: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9EC08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9EC0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9EC10: 419A003C  beq cr6, 0x82d9ec4c
	if ctx.cr[6].eq {
	pc = 0x82D9EC4C; continue 'dispatch;
	}
	// 82D9EC14: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9EC18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9EC1C: 419A0030  beq cr6, 0x82d9ec4c
	if ctx.cr[6].eq {
	pc = 0x82D9EC4C; continue 'dispatch;
	}
	// 82D9EC20: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9EC24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9EC28: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D9EC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9EC30: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9EC34: 409A0018  bne cr6, 0x82d9ec4c
	if !ctx.cr[6].eq {
	pc = 0x82D9EC4C; continue 'dispatch;
	}
	// 82D9EC38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9EC3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9EC40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9EC44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9EC48: 4E800421  bctrl
	ctx.lr = 0x82D9EC4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9EC4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9EC50: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D9EC54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9EC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9EC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9EC70 size=52
    let mut pc: u32 = 0x82D9EC70;
    'dispatch: loop {
        match pc {
            0x82D9EC70 => {
    //   block [0x82D9EC70..0x82D9ECA4)
	// 82D9EC70: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82D9EC74: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82D9EC78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D9EC7C: 39000180  li r8, 0x180
	ctx.r[8].s64 = 384;
	// 82D9EC80: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82D9EC84: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82D9EC88: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D9EC8C: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82D9EC90: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D9EC94: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9EC98: 90E40008  stw r7, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82D9EC9C: 90C4000C  stw r6, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82D9ECA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9ECA8 size=40
    let mut pc: u32 = 0x82D9ECA8;
    'dispatch: loop {
        match pc {
            0x82D9ECA8 => {
    //   block [0x82D9ECA8..0x82D9ECD0)
	// 82D9ECA8: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9ECAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9ECB0: 419A0014  beq cr6, 0x82d9ecc4
	if ctx.cr[6].eq {
	pc = 0x82D9ECC4; continue 'dispatch;
	}
	// 82D9ECB4: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9ECB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9ECBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9ECC0: 409A0008  bne cr6, 0x82d9ecc8
	if !ctx.cr[6].eq {
	pc = 0x82D9ECC8; continue 'dispatch;
	}
	// 82D9ECC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9ECC8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9ECCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9ECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9ECD0 size=512
    let mut pc: u32 = 0x82D9ECD0;
    'dispatch: loop {
        match pc {
            0x82D9ECD0 => {
    //   block [0x82D9ECD0..0x82D9EED0)
	// 82D9ECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9ECD4: 4BF0A725  bl 0x82ca93f8
	ctx.lr = 0x82D9ECD8;
	sub_82CA93D0(ctx, base);
	// 82D9ECD8: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82D9ECDC: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9ECE0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82D9ECE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9ECE8: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82D9ECEC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82D9ECF0: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82D9ECF4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82D9ECF8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D9ECFC: 419A001C  beq cr6, 0x82d9ed18
	if ctx.cr[6].eq {
	pc = 0x82D9ED18; continue 'dispatch;
	}
	// 82D9ED00: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9ED04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9ED08: 419A0010  beq cr6, 0x82d9ed18
	if ctx.cr[6].eq {
	pc = 0x82D9ED18; continue 'dispatch;
	}
	// 82D9ED0C: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9ED10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9ED14: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9ED18: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D9ED1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9ED20: 419A003C  beq cr6, 0x82d9ed5c
	if ctx.cr[6].eq {
	pc = 0x82D9ED5C; continue 'dispatch;
	}
	// 82D9ED24: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9ED28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9ED2C: 419A0030  beq cr6, 0x82d9ed5c
	if ctx.cr[6].eq {
	pc = 0x82D9ED5C; continue 'dispatch;
	}
	// 82D9ED30: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D9ED34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D9ED38: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D9ED3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9ED40: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D9ED44: 409A0018  bne cr6, 0x82d9ed5c
	if !ctx.cr[6].eq {
	pc = 0x82D9ED5C; continue 'dispatch;
	}
	// 82D9ED48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9ED4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D9ED50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9ED54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D9ED58: 4E800421  bctrl
	ctx.lr = 0x82D9ED5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D9ED5C: 397F0070  addi r11, r31, 0x70
	ctx.r[11].s64 = ctx.r[31].s64 + 112;
	// 82D9ED60: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82D9ED64: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9EED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9EED0 size=1140
    let mut pc: u32 = 0x82D9EED0;
    'dispatch: loop {
        match pc {
            0x82D9EED0 => {
    //   block [0x82D9EED0..0x82D9F344)
	// 82D9EED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9EED4: 4BF0A531  bl 0x82ca9404
	ctx.lr = 0x82D9EED8;
	sub_82CA93D0(ctx, base);
	// 82D9EED8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9F348 size=432
    let mut pc: u32 = 0x82D9F348;
    'dispatch: loop {
        match pc {
            0x82D9F348 => {
    //   block [0x82D9F348..0x82D9F4F8)
	// 82D9F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9F34C: 4BF0A0BD  bl 0x82ca9408
	ctx.lr = 0x82D9F350;
	sub_82CA93D0(ctx, base);
	// 82D9F350: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82D9F354: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9F358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9F35C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D9F360: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D9F364: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D9F368: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82D9F36C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82D9F370: 4BFB7081  bl 0x82d563f0
	ctx.lr = 0x82D9F374;
	sub_82D563F0(ctx, base);
	// 82D9F374: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D9F378: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D9F37C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82D9F380: 4BFB7071  bl 0x82d563f0
	ctx.lr = 0x82D9F384;
	sub_82D563F0(ctx, base);
	// 82D9F384: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D9F388: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D9F38C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82D9F390: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D9F394: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F4F8 size=396
    let mut pc: u32 = 0x82D9F4F8;
    'dispatch: loop {
        match pc {
            0x82D9F4F8 => {
    //   block [0x82D9F4F8..0x82D9F684)
	// 82D9F4F8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D9F4FC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D9F500: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F688 size=16
    let mut pc: u32 = 0x82D9F688;
    'dispatch: loop {
        match pc {
            0x82D9F688 => {
    //   block [0x82D9F688..0x82D9F698)
	// 82D9F688: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D9F68C: 38800094  li r4, 0x94
	ctx.r[4].s64 = 148;
	// 82D9F690: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82D9F694: 4BFFCB44  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F698 size=32
    let mut pc: u32 = 0x82D9F698;
    'dispatch: loop {
        match pc {
            0x82D9F698 => {
    //   block [0x82D9F698..0x82D9F6B8)
	// 82D9F698: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82D9F69C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D9F6A0: 419A0018  beq cr6, 0x82d9f6b8
	if ctx.cr[6].eq {
		sub_82D9F6B8(ctx, base);
		return;
	}
	// 82D9F6A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9F6A8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D9F6AC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9F6B0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D9F6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F6B8 size=16
    let mut pc: u32 = 0x82D9F6B8;
    'dispatch: loop {
        match pc {
            0x82D9F6B8 => {
    //   block [0x82D9F6B8..0x82D9F6C8)
	// 82D9F6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9F6BC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9F6C0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9F6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D9F6C8 size=140
    let mut pc: u32 = 0x82D9F6C8;
    'dispatch: loop {
        match pc {
            0x82D9F6C8 => {
    //   block [0x82D9F6C8..0x82D9F754)
	// 82D9F6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9F6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9F6D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9F6D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9F6D8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D9F6DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9F6E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82D9F6E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9F6E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9F6EC: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82D9F6F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D9F6F4: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D9F6F8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9F6FC: 4BFB81AD  bl 0x82d578a8
	ctx.lr = 0x82D9F700;
	sub_82D578A8(ctx, base);
	// 82D9F700: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9F704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9F708: 419A0024  beq cr6, 0x82d9f72c
	if ctx.cr[6].eq {
	pc = 0x82D9F72C; continue 'dispatch;
	}
	// 82D9F70C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82D9F710: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D9F714: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82D9F718: 4BFB8191  bl 0x82d578a8
	ctx.lr = 0x82D9F71C;
	sub_82D578A8(ctx, base);
	// 82D9F71C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9F720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9F724: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9F728: 409A0008  bne cr6, 0x82d9f730
	if !ctx.cr[6].eq {
	pc = 0x82D9F730; continue 'dispatch;
	}
	// 82D9F72C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9F730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9F734: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9F738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D9F73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9F740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9F744: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82D9F748: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9F74C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9F750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F758 size=8
    let mut pc: u32 = 0x82D9F758;
    'dispatch: loop {
        match pc {
            0x82D9F758 => {
    //   block [0x82D9F758..0x82D9F760)
	// 82D9F758: 3860000E  li r3, 0xe
	ctx.r[3].s64 = 14;
	// 82D9F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F760 size=188
    let mut pc: u32 = 0x82D9F760;
    'dispatch: loop {
        match pc {
            0x82D9F760 => {
    //   block [0x82D9F760..0x82D9F81C)
	// 82D9F760: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D9F764: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 82D9F768: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82D9F76C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82D9F770: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D9F774: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F820 size=8
    let mut pc: u32 = 0x82D9F820;
    'dispatch: loop {
        match pc {
            0x82D9F820 => {
    //   block [0x82D9F820..0x82D9F828)
	// 82D9F820: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D9F824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F828 size=28
    let mut pc: u32 = 0x82D9F828;
    'dispatch: loop {
        match pc {
            0x82D9F828 => {
    //   block [0x82D9F828..0x82D9F844)
	// 82D9F828: 396300A0  addi r11, r3, 0xa0
	ctx.r[11].s64 = ctx.r[3].s64 + 160;
	// 82D9F82C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9F848 size=32
    let mut pc: u32 = 0x82D9F848;
    'dispatch: loop {
        match pc {
            0x82D9F848 => {
    //   block [0x82D9F848..0x82D9F868)
	// 82D9F848: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D9F84C: 388300A0  addi r4, r3, 0xa0
	ctx.r[4].s64 = ctx.r[3].s64 + 160;
	// 82D9F850: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9F868 size=416
    let mut pc: u32 = 0x82D9F868;
    'dispatch: loop {
        match pc {
            0x82D9F868 => {
    //   block [0x82D9F868..0x82D9FA08)
	// 82D9F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9F86C: 4BF09B8D  bl 0x82ca93f8
	ctx.lr = 0x82D9F870;
	sub_82CA93D0(ctx, base);
	// 82D9F870: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9F874: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9F878: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82D9F87C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D9F880: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D9F884: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D9F888: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9F88C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9F890: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D9F894: 40980020  bge cr6, 0x82d9f8b4
	if !ctx.cr[6].lt {
	pc = 0x82D9F8B4; continue 'dispatch;
	}
	// 82D9F898: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D9F89C: 3929FC30  addi r9, r9, -0x3d0
	ctx.r[9].s64 = ctx.r[9].s64 + -976;
	// 82D9F8A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D9F8A4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D9F8A8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D9F8AC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D9F8B0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9F8B4: 813E0024  lwz r9, 0x24(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9F8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D9F8BC: 897E0020  lbz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9F8C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9F8C4: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82D9F8C8: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82D9F8CC: 419A0020  beq cr6, 0x82d9f8ec
	if ctx.cr[6].eq {
	pc = 0x82D9F8EC; continue 'dispatch;
	}
	// 82D9F8D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9F8D4: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D9F8D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9F8DC: 419A0010  beq cr6, 0x82d9f8ec
	if ctx.cr[6].eq {
	pc = 0x82D9F8EC; continue 'dispatch;
	}
	// 82D9F8E0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D9F8E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82D9F8E8: 48000008  b 0x82d9f8f0
	pc = 0x82D9F8F0; continue 'dispatch;
	// 82D9F8EC: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82D9F8F0: 816300C4  lwz r11, 0xc4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82D9F8F4: 83E300C0  lwz r31, 0xc0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9F8F8: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 82D9F8FC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82D9F900: 419800D0  blt cr6, 0x82d9f9d0
	if ctx.cr[6].lt {
	pc = 0x82D9F9D0; continue 'dispatch;
	}
	// 82D9F904: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9F908: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82D9F90C: 3BAB4C30  addi r29, r11, 0x4c30
	ctx.r[29].s64 = ctx.r[11].s64 + 19504;
	// 82D9F910: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9F914: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9F918: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D9F91C: 419A00A4  beq cr6, 0x82d9f9c0
	if ctx.cr[6].eq {
	pc = 0x82D9F9C0; continue 'dispatch;
	}
	// 82D9F920: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FA08 size=464
    let mut pc: u32 = 0x82D9FA08;
    'dispatch: loop {
        match pc {
            0x82D9FA08 => {
    //   block [0x82D9FA08..0x82D9FBD8)
	// 82D9FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FA0C: 4BF099ED  bl 0x82ca93f8
	ctx.lr = 0x82D9FA10;
	sub_82CA93D0(ctx, base);
	// 82D9FA10: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9FA14: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FA18: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82D9FA1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9FA20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D9FA24: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D9FA28: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9FA2C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9FA30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D9FA34: 40980020  bge cr6, 0x82d9fa54
	if !ctx.cr[6].lt {
	pc = 0x82D9FA54; continue 'dispatch;
	}
	// 82D9FA38: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D9FA3C: 3929FC30  addi r9, r9, -0x3d0
	ctx.r[9].s64 = ctx.r[9].s64 + -976;
	// 82D9FA40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D9FA44: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D9FA48: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D9FA4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D9FA50: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9FA54: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D9FA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D9FA5C: 897F0020  lbz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D9FA60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9FA64: 91410084  stw r10, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82D9FA68: 91210080  stw r9, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82D9FA6C: 419A0020  beq cr6, 0x82d9fa8c
	if ctx.cr[6].eq {
	pc = 0x82D9FA8C; continue 'dispatch;
	}
	// 82D9FA70: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9FA74: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D9FA78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D9FA7C: 419A0010  beq cr6, 0x82d9fa8c
	if ctx.cr[6].eq {
	pc = 0x82D9FA8C; continue 'dispatch;
	}
	// 82D9FA80: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D9FA84: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D9FA88: 48000008  b 0x82d9fa90
	pc = 0x82D9FA90; continue 'dispatch;
	// 82D9FA8C: 91410084  stw r10, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82D9FA90: 816300C4  lwz r11, 0xc4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82D9FA94: 836300C0  lwz r27, 0xc0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9FA98: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 82D9FA9C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82D9FAA0: 419800E4  blt cr6, 0x82d9fb84
	if ctx.cr[6].lt {
	pc = 0x82D9FB84; continue 'dispatch;
	}
	// 82D9FAA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D9FAA8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82D9FAAC: 3BCB4C30  addi r30, r11, 0x4c30
	ctx.r[30].s64 = ctx.r[11].s64 + 19504;
	// 82D9FAB0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FAB4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FAB8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D9FABC: 419A00B8  beq cr6, 0x82d9fb74
	if ctx.cr[6].eq {
	pc = 0x82D9FB74; continue 'dispatch;
	}
	// 82D9FAC0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9FBD8 size=60
    let mut pc: u32 = 0x82D9FBD8;
    'dispatch: loop {
        match pc {
            0x82D9FBD8 => {
    //   block [0x82D9FBD8..0x82D9FC14)
	// 82D9FBD8: 812400C4  lwz r9, 0xc4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(196 as u32) ) } as u64;
	// 82D9FBDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9FBE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D9FBE4: 40990024  ble cr6, 0x82d9fc08
	if !ctx.cr[6].gt {
	pc = 0x82D9FC08; continue 'dispatch;
	}
	// 82D9FBE8: 814400C0  lwz r10, 0xc0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9FBEC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FBF0: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82D9FBF4: 419A0020  beq cr6, 0x82d9fc14
	if ctx.cr[6].eq {
		sub_82D9FC14(ctx, base);
		return;
	}
	// 82D9FBF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9FBFC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D9FC00: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D9FC04: 4198FFE8  blt cr6, 0x82d9fbec
	if ctx.cr[6].lt {
	pc = 0x82D9FBEC; continue 'dispatch;
	}
	// 82D9FC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9FC0C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9FC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D9FC14 size=12
    let mut pc: u32 = 0x82D9FC14;
    'dispatch: loop {
        match pc {
            0x82D9FC14 => {
    //   block [0x82D9FC14..0x82D9FC20)
	// 82D9FC14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D9FC18: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D9FC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FC20 size=184
    let mut pc: u32 = 0x82D9FC20;
    'dispatch: loop {
        match pc {
            0x82D9FC20 => {
    //   block [0x82D9FC20..0x82D9FCD8)
	// 82D9FC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9FC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9FC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9FC30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9FC34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D9FC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D9FC3C: 813E00C4  lwz r9, 0xc4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82D9FC40: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D9FC44: 40990024  ble cr6, 0x82d9fc68
	if !ctx.cr[6].gt {
	pc = 0x82D9FC68; continue 'dispatch;
	}
	// 82D9FC48: 815E00C0  lwz r10, 0xc0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9FC4C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FC50: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D9FC54: 419A007C  beq cr6, 0x82d9fcd0
	if ctx.cr[6].eq {
	pc = 0x82D9FCD0; continue 'dispatch;
	}
	// 82D9FC58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9FC5C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D9FC60: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D9FC64: 4198FFE8  blt cr6, 0x82d9fc4c
	if ctx.cr[6].lt {
	pc = 0x82D9FC4C; continue 'dispatch;
	}
	// 82D9FC68: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82D9FC6C: 7FEA0034  cntlzw r10, r31
	ctx.r[10].u64 = if ctx.r[31].u32 == 0 { 32 } else { ctx.r[31].u32.leading_zeros() as u64 };
	// 82D9FC70: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D9FC74: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82D9FC78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D9FC7C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82D9FC80: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82D9FC84: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D9FC88: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FC8C: 4B4EF505  bl 0x8228f190
	ctx.lr = 0x82D9FC90;
	sub_8228F190(ctx, base);
	// 82D9FC90: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D9FC94: 41980024  blt cr6, 0x82d9fcb8
	if ctx.cr[6].lt {
	pc = 0x82D9FCB8; continue 'dispatch;
	}
	// 82D9FC98: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82D9FC9C: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D9FCA0: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9FCA4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D9FCA8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D9FCAC: 915E00C4  stw r10, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 82D9FCB0: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D9FCB4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82D9FCB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9FCC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9FCC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9FCCC: 4E800020  blr
	return;
	// 82D9FCD0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D9FCD4: 4BFFFF98  b 0x82d9fc6c
	pc = 0x82D9FC6C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FCD8 size=104
    let mut pc: u32 = 0x82D9FCD8;
    'dispatch: loop {
        match pc {
            0x82D9FCD8 => {
    //   block [0x82D9FCD8..0x82D9FD40)
	// 82D9FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9FCE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9FCE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9FCE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D9FCEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9FCF0: 396BA94C  addi r11, r11, -0x56b4
	ctx.r[11].s64 = ctx.r[11].s64 + -22196;
	// 82D9FCF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D9FCF8: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82D9FCFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D9FD00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D9FD04: 409A0020  bne cr6, 0x82d9fd24
	if !ctx.cr[6].eq {
	pc = 0x82D9FD24; continue 'dispatch;
	}
	// 82D9FD08: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FD0C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D9FD10: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D9FD14: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D9FD18: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D9FD1C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D9FD20: 4BFB55A9  bl 0x82d552c8
	ctx.lr = 0x82D9FD24;
	sub_82D552C8(ctx, base);
	// 82D9FD24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9FD28: 4BFF9ED1  bl 0x82d99bf8
	ctx.lr = 0x82D9FD2C;
	sub_82D99BF8(ctx, base);
	// 82D9FD2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D9FD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9FD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9FD38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9FD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FD40 size=560
    let mut pc: u32 = 0x82D9FD40;
    'dispatch: loop {
        match pc {
            0x82D9FD40 => {
    //   block [0x82D9FD40..0x82D9FF70)
	// 82D9FD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FD44: 4BF096AD  bl 0x82ca93f0
	ctx.lr = 0x82D9FD48;
	sub_82CA93D0(ctx, base);
	// 82D9FD48: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9FD4C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FD50: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82D9FD54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D9FD58: 7ECB5214  add r22, r11, r10
	ctx.r[22].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D9FD5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D9FD60: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82D9FD64: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82D9FD68: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FD6C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9FD70: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D9FD74: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D9FD78: 40980020  bge cr6, 0x82d9fd98
	if !ctx.cr[6].lt {
	pc = 0x82D9FD98; continue 'dispatch;
	}
	// 82D9FD7C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D9FD80: 3929FC3C  addi r9, r9, -0x3c4
	ctx.r[9].s64 = ctx.r[9].s64 + -964;
	// 82D9FD84: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D9FD88: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D9FD8C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D9FD90: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D9FD94: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D9FD98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9FD9C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82D9FDA0: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FF70 size=128
    let mut pc: u32 = 0x82D9FF70;
    'dispatch: loop {
        match pc {
            0x82D9FF70 => {
    //   block [0x82D9FF70..0x82D9FFF0)
	// 82D9FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9FF78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9FF7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D9FF80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D9FF84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D9FF88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D9FF8C: 4B4E9B5D  bl 0x82289ae8
	ctx.lr = 0x82D9FF90;
	sub_82289AE8(ctx, base);
	// 82D9FF90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D9FF94: 409A0044  bne cr6, 0x82d9ffd8
	if !ctx.cr[6].eq {
	pc = 0x82D9FFD8; continue 'dispatch;
	}
	// 82D9FF98: 3BFF00C0  addi r31, r31, 0xc0
	ctx.r[31].s64 = ctx.r[31].s64 + 192;
	// 82D9FF9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D9FFA0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9FFA4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D9FFA8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D9FFAC: 409A0010  bne cr6, 0x82d9ffbc
	if !ctx.cr[6].eq {
	pc = 0x82D9FFBC; continue 'dispatch;
	}
	// 82D9FFB0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D9FFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D9FFB8: 4BFB6FE1  bl 0x82d56f98
	ctx.lr = 0x82D9FFBC;
	sub_82D56F98(ctx, base);
	// 82D9FFBC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9FFC0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D9FFC4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D9FFC8: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D9FFCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D9FFD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D9FFD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D9FFD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D9FFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D9FFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D9FFE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D9FFE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D9FFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D9FFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D9FFF0 size=184
    let mut pc: u32 = 0x82D9FFF0;
    'dispatch: loop {
        match pc {
            0x82D9FFF0 => {
    //   block [0x82D9FFF0..0x82DA00A8)
	// 82D9FFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D9FFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D9FFF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D9FFFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA0000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA0004: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA0008: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA000C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA0010: 388BFC6C  addi r4, r11, -0x394
	ctx.r[4].s64 = ctx.r[11].s64 + -916;
	// 82DA0014: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DA0018: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA001C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DA0020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA0024: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA002C: 4E800421  bctrl
	ctx.lr = 0x82DA0030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA0030: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA0034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA0038: 4BFF9A49  bl 0x82d99a80
	ctx.lr = 0x82DA003C;
	sub_82D99A80(ctx, base);
	// 82DA003C: 817E00C8  lwz r11, 0xc8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 82DA0040: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0044: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA0048: 409A0034  bne cr6, 0x82da007c
	if !ctx.cr[6].eq {
	pc = 0x82DA007C; continue 'dispatch;
	}
	// 82DA004C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA0054: 80FE00C4  lwz r7, 0xc4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82DA0058: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DA005C: 80DE00C0  lwz r6, 0xc0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 82DA0060: 388AFC5C  addi r4, r10, -0x3a4
	ctx.r[4].s64 = ctx.r[10].s64 + -932;
	// 82DA0064: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DA0068: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DA006C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA0070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA0074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA0078: 4E800421  bctrl
	ctx.lr = 0x82DA007C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA007C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA0084: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA0088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA008C: 4E800421  bctrl
	ctx.lr = 0x82DA0090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA0090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA0094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA0098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA009C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA00A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA00A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA00A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA00A8 size=144
    let mut pc: u32 = 0x82DA00A8;
    'dispatch: loop {
        match pc {
            0x82DA00A8 => {
    //   block [0x82DA00A8..0x82DA0138)
	// 82DA00A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA00AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA00B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA00B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA00B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA00BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA00C0: 817E00C4  lwz r11, 0xc4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82DA00C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA00C8: 409A0050  bne cr6, 0x82da0118
	if !ctx.cr[6].eq {
	pc = 0x82DA0118; continue 'dispatch;
	}
	// 82DA00CC: 3BFE00C0  addi r31, r30, 0xc0
	ctx.r[31].s64 = ctx.r[30].s64 + 192;
	// 82DA00D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA00D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA00D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA00DC: 409A0020  bne cr6, 0x82da00fc
	if !ctx.cr[6].eq {
	pc = 0x82DA00FC; continue 'dispatch;
	}
	// 82DA00E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA00E4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA00E8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA00EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA00F0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA00F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DA00F8: 4BFB51D1  bl 0x82d552c8
	ctx.lr = 0x82DA00FC;
	sub_82D552C8(ctx, base);
	// 82DA00FC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA0100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA0104: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA0108: 512AF880  rlwimi r10, r9, 0x1f, 2, 0
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[10].u64 & 0x0000000040000000);
	// 82DA010C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0110: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA0114: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DA0118: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA011C: 4BFF9A1D  bl 0x82d99b38
	ctx.lr = 0x82DA0120;
	sub_82D99B38(ctx, base);
	// 82DA0120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA0124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA0128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA012C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA0130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA0134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA0138 size=156
    let mut pc: u32 = 0x82DA0138;
    'dispatch: loop {
        match pc {
            0x82DA0138 => {
    //   block [0x82DA0138..0x82DA01D4)
	// 82DA0138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA013C: 4BF092CD  bl 0x82ca9408
	ctx.lr = 0x82DA0140;
	sub_82CA93D0(ctx, base);
	// 82DA0140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA0144: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DA0148: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DA014C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DA0150: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DA0154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA0158: 4BFE0731  bl 0x82d80888
	ctx.lr = 0x82DA015C;
	sub_82D80888(ctx, base);
	// 82DA015C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DA0160: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82DA0164: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA0168: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82DA016C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DA0170: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82DA0174: 7F8AF850  subf r28, r10, r31
	ctx.r[28].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 82DA0178: 38E7A94C  addi r7, r7, -0x56b4
	ctx.r[7].s64 = ctx.r[7].s64 + -22196;
	// 82DA017C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DA0180: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82DA0184: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DA0188: 90DF0088  stw r6, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[6].u32 ) };
	// 82DA018C: 393F00A0  addi r9, r31, 0xa0
	ctx.r[9].s64 = ctx.r[31].s64 + 160;
	// 82DA0190: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82DA0194: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DA0198: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82DA019C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA01A0: 90BF0094  stw r5, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82DA01A4: 9B8A0010  stb r28, 0x10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[28].u8 ) };
	// 82DA01A8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DA01AC: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82DA01B0: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82DA01B4: 909F00C8  stw r4, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[4].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA01D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA01D8 size=400
    let mut pc: u32 = 0x82DA01D8;
    'dispatch: loop {
        match pc {
            0x82DA01D8 => {
    //   block [0x82DA01D8..0x82DA0368)
	// 82DA01D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA01DC: 4BF09225  bl 0x82ca9400
	ctx.lr = 0x82DA01E0;
	sub_82CA93D0(ctx, base);
	// 82DA01E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA01E4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA01E8: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82DA01EC: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82DA01F0: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 82DA01F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DA01F8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DA01FC: 4BFB504D  bl 0x82d55248
	ctx.lr = 0x82DA0200;
	sub_82D55248(ctx, base);
	// 82DA0200: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
	// 82DA0204: 389B00A0  addi r4, r27, 0xa0
	ctx.r[4].s64 = ctx.r[27].s64 + 160;
	// 82DA0208: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DA020C: 80BB002C  lwz r5, 0x2c(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DA0210: 4BFFFF29  bl 0x82da0138
	ctx.lr = 0x82DA0214;
	sub_82DA0138(ctx, base);
	// 82DA0214: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DA0218: 3BDB0080  addi r30, r27, 0x80
	ctx.r[30].s64 = ctx.r[27].s64 + 128;
	// 82DA021C: 3BFA0080  addi r31, r26, 0x80
	ctx.r[31].s64 = ctx.r[26].s64 + 128;
	// 82DA0220: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0224: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA0228: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA022C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA0230: 40980050  bge cr6, 0x82da0280
	if !ctx.cr[6].lt {
	pc = 0x82DA0280; continue 'dispatch;
	}
	// 82DA0234: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0238: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA023C: 409A0018  bne cr6, 0x82da0254
	if !ctx.cr[6].eq {
	pc = 0x82DA0254; continue 'dispatch;
	}
	// 82DA0240: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA0244: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0248: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA024C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DA0250: 4BFB5079  bl 0x82d552c8
	ctx.lr = 0x82DA0254;
	sub_82D552C8(ctx, base);
	// 82DA0254: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0258: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DA025C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DA0260: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DA0264: 4BFB4FE5  bl 0x82d55248
	ctx.lr = 0x82DA0268;
	sub_82D55248(ctx, base);
	// 82DA0268: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA026C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA0270: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0274: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0278: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DA027C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA0280: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0284: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0288: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA028C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA0290: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0294: 40990020  ble cr6, 0x82da02b4
	if !ctx.cr[6].gt {
	pc = 0x82DA02B4; continue 'dispatch;
	}
	// 82DA0298: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DA029C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA02A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA02A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA02A8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA02AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA02B0: 409AFFEC  bne cr6, 0x82da029c
	if !ctx.cr[6].eq {
	pc = 0x82DA029C; continue 'dispatch;
	}
	// 82DA02B4: 3BFA008C  addi r31, r26, 0x8c
	ctx.r[31].s64 = ctx.r[26].s64 + 140;
	// 82DA02B8: 3BDB008C  addi r30, r27, 0x8c
	ctx.r[30].s64 = ctx.r[27].s64 + 140;
	// 82DA02BC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA02C0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA02C4: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA02C8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA02CC: 40980050  bge cr6, 0x82da031c
	if !ctx.cr[6].lt {
	pc = 0x82DA031C; continue 'dispatch;
	}
	// 82DA02D0: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA02D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA02D8: 409A0018  bne cr6, 0x82da02f0
	if !ctx.cr[6].eq {
	pc = 0x82DA02F0; continue 'dispatch;
	}
	// 82DA02DC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA02E0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA02E4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA02E8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DA02EC: 4BFB4FDD  bl 0x82d552c8
	ctx.lr = 0x82DA02F0;
	sub_82D552C8(ctx, base);
	// 82DA02F0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA02F4: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DA02F8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DA02FC: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DA0300: 4BFB4F49  bl 0x82d55248
	ctx.lr = 0x82DA0304;
	sub_82D55248(ctx, base);
	// 82DA0304: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA0308: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA030C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0310: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0314: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DA0318: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA031C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0324: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA0328: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA032C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0330: 40990020  ble cr6, 0x82da0350
	if !ctx.cr[6].gt {
	pc = 0x82DA0350; continue 'dispatch;
	}
	// 82DA0334: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DA0338: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA033C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA0340: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA0344: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA0348: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA034C: 409AFFEC  bne cr6, 0x82da0338
	if !ctx.cr[6].eq {
	pc = 0x82DA0338; continue 'dispatch;
	}
	// 82DA0350: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DA0354: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DA0358: 4BFDFEB1  bl 0x82d80208
	ctx.lr = 0x82DA035C;
	sub_82D80208(ctx, base);
	// 82DA035C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DA0360: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA0364: 4BF090EC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA0368 size=100
    let mut pc: u32 = 0x82DA0368;
    'dispatch: loop {
        match pc {
            0x82DA0368 => {
    //   block [0x82DA0368..0x82DA03CC)
	// 82DA0368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA036C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA0370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA0374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DA0378: 396BAA5C  addi r11, r11, -0x55a4
	ctx.r[11].s64 = ctx.r[11].s64 + -21924;
	// 82DA037C: C1AAFC78  lfs f13, -0x388(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-904 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA0380: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA0384: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DA0388: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82DA038C: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA0390: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0394: C18A0A48  lfs f12, 0xa48(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2632 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA0398: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA039C: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA03A0: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82DA03A4: C16A0B30  lfs f11, 0xb30(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2864 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DA03A8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA03AC: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DA03B0: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA03B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA03B8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DA03BC: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DA03C0: C14A0C4C  lfs f10, 0xc4c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3148 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DA03C4: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DA03C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA03D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA03D0 size=172
    let mut pc: u32 = 0x82DA03D0;
    'dispatch: loop {
        match pc {
            0x82DA03D0 => {
    //   block [0x82DA03D0..0x82DA047C)
	// 82DA03D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA03D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA03D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA03DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA03E0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA03E4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DA03E8: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82DA03EC: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82DA03F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA03F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA03F8: 4BFB4E51  bl 0x82d55248
	ctx.lr = 0x82DA03FC;
	sub_82D55248(ctx, base);
	// 82DA03FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA0400: 39000024  li r8, 0x24
	ctx.r[8].s64 = 36;
	// 82DA0404: 396BAA34  addi r11, r11, -0x55cc
	ctx.r[11].s64 = ctx.r[11].s64 + -21964;
	// 82DA0408: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DA040C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA0410: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DA0414: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82DA0418: 394AAA48  addi r10, r10, -0x55b8
	ctx.r[10].s64 = ctx.r[10].s64 + -21944;
	// 82DA041C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0420: 3929AA5C  addi r9, r9, -0x55a4
	ctx.r[9].s64 = ctx.r[9].s64 + -21924;
	// 82DA0424: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DA0428: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA042C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA0430: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82DA0434: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0438: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA043C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0440: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DA0444: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA0448: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA044C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DA0450: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0454: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DA0458: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA045C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DA0460: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0464: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DA0468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA046C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA0470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA0474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA0478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA0480 size=8
    let mut pc: u32 = 0x82DA0480;
    'dispatch: loop {
        match pc {
            0x82DA0480 => {
    //   block [0x82DA0480..0x82DA0488)
	// 82DA0480: 38600066  li r3, 0x66
	ctx.r[3].s64 = 102;
	// 82DA0484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA0488 size=132
    let mut pc: u32 = 0x82DA0488;
    'dispatch: loop {
        match pc {
            0x82DA0488 => {
    //   block [0x82DA0488..0x82DA050C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA050C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA050C size=28
    let mut pc: u32 = 0x82DA050C;
    'dispatch: loop {
        match pc {
            0x82DA050C => {
    //   block [0x82DA050C..0x82DA0528)
	// 82DA050C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA0528 size=88
    let mut pc: u32 = 0x82DA0528;
    'dispatch: loop {
        match pc {
            0x82DA0528 => {
    //   block [0x82DA0528..0x82DA0580)
	// 82DA0528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA052C: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82DA0530: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DA0534: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DA0538: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA053C: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DA0540: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DA0544: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0548: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DA054C: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DA0550: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA0554: 1D4B0534  mulli r10, r11, 0x534
	ctx.r[10].s64 = ctx.r[11].s64 * 1332;
	// 82DA0558: 394A0053  addi r10, r10, 0x53
	ctx.r[10].s64 = ctx.r[10].s64 + 83;
	// 82DA055C: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0560: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DA0564: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA0568: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA056C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DA0570: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA0574: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA0578: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DA057C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA0580 size=52
    let mut pc: u32 = 0x82DA0580;
    'dispatch: loop {
        match pc {
            0x82DA0580 => {
    //   block [0x82DA0580..0x82DA05B4)
	// 82DA0580: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA0584: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DA0588: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA058C: 392B0003  addi r9, r11, 3
	ctx.r[9].s64 = ctx.r[11].s64 + 3;
	// 82DA0590: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DA0594: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0598: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA059C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DA05A0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA05A4: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA05A8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DA05AC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA05B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA05B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA05B8 size=72
    let mut pc: u32 = 0x82DA05B8;
    'dispatch: loop {
        match pc {
            0x82DA05B8 => {
    //   block [0x82DA05B8..0x82DA0600)
	// 82DA05B8: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA05BC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA05C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82DA05C4: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DA05C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DA05CC: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA05D0: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA05D4: 7D085830  slw r8, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DA05D8: 7CE75830  slw r7, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DA05DC: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA05E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DA05E4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA05E8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DA05EC: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DA05F0: 7D4A4078  andc r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[8].u64;
	// 82DA05F4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82DA05F8: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82DA05FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA0600 size=72
    let mut pc: u32 = 0x82DA0600;
    'dispatch: loop {
        match pc {
            0x82DA0600 => {
    //   block [0x82DA0600..0x82DA0648)
	// 82DA0600: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA0604: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA0608: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82DA060C: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DA0610: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82DA0614: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA0618: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA061C: 7D085830  slw r8, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DA0620: 7CE75830  slw r7, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DA0624: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA0628: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DA062C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA0630: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DA0634: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DA0638: 7D4A4078  andc r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[8].u64;
	// 82DA063C: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82DA0640: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82DA0644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA0648 size=164
    let mut pc: u32 = 0x82DA0648;
    'dispatch: loop {
        match pc {
            0x82DA0648 => {
    //   block [0x82DA0648..0x82DA06EC)
	// 82DA0648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA0650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA0654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA0658: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA065C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DA0660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA0664: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DA0668: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DA066C: C18B0B0C  lfs f12, 0xb0c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2828 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA0670: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA0674: C1690BF8  lfs f11, 0xbf8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3064 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DA0678: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DA067C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA0680: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DA0684: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0688: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA068C: 394BFC94  addi r10, r11, -0x36c
	ctx.r[10].s64 = ctx.r[11].s64 + -876;
	// 82DA0690: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA0694: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA0698: C1ABE240  lfs f13, -0x1dc0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7616 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA069C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA06A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA06A4: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DA06A8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DA06AC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DA06B0: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DA06B4: D19F0024  stfs f12, 0x24(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DA06B8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DA06BC: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DA06C0: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DA06C4: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DA06C8: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82DA06CC: D17F003C  stfs f11, 0x3c(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DA06D0: 4BFFDED1  bl 0x82d9e5a0
	ctx.lr = 0x82DA06D4;
	sub_82D9E5A0(ctx, base);
	// 82DA06D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA06D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA06DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA06E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA06E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA06E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA06F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA06F0 size=104
    let mut pc: u32 = 0x82DA06F0;
    'dispatch: loop {
        match pc {
            0x82DA06F0 => {
    //   block [0x82DA06F0..0x82DA0758)
	// 82DA06F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA06F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA06F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA06FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA0700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA0704: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA0708: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82DA070C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA0710: 396BFC94  addi r11, r11, -0x36c
	ctx.r[11].s64 = ctx.r[11].s64 + -876;
	// 82DA0714: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DA0718: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DA071C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA0720: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0724: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DA0728: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA072C: 4BFFDE75  bl 0x82d9e5a0
	ctx.lr = 0x82DA0730;
	sub_82D9E5A0(ctx, base);
	// 82DA0730: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA0734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA0738: 4BFFDE69  bl 0x82d9e5a0
	ctx.lr = 0x82DA073C;
	sub_82D9E5A0(ctx, base);
	// 82DA073C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA0740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA0744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA0748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA074C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA0750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA0754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA0758 size=244
    let mut pc: u32 = 0x82DA0758;
    'dispatch: loop {
        match pc {
            0x82DA0758 => {
    //   block [0x82DA0758..0x82DA084C)
	// 82DA0758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA075C: 4BF08CA9  bl 0x82ca9404
	ctx.lr = 0x82DA0760;
	sub_82CA93D0(ctx, base);
	// 82DA0760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA0764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA0768: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA076C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DA0770: 396BFC94  addi r11, r11, -0x36c
	ctx.r[11].s64 = ctx.r[11].s64 + -876;
	// 82DA0774: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA0778: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA077C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0780: 40990080  ble cr6, 0x82da0800
	if !ctx.cr[6].gt {
	pc = 0x82DA0800; continue 'dispatch;
	}
	// 82DA0784: 3B800040  li r28, 0x40
	ctx.r[28].s64 = 64;
	// 82DA0788: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DA078C: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82DA0790: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA0794: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DA0798: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA079C: 419A0040  beq cr6, 0x82da07dc
	if ctx.cr[6].eq {
	pc = 0x82DA07DC; continue 'dispatch;
	}
	// 82DA07A0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DA07A4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA07A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA07AC: 419A0030  beq cr6, 0x82da07dc
	if ctx.cr[6].eq {
	pc = 0x82DA07DC; continue 'dispatch;
	}
	// 82DA07B0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA07B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DA07B8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DA07BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA07C0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DA07C4: 409A0018  bne cr6, 0x82da07dc
	if !ctx.cr[6].eq {
	pc = 0x82DA07DC; continue 'dispatch;
	}
	// 82DA07C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA07CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DA07D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA07D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA07D8: 4E800421  bctrl
	ctx.lr = 0x82DA07DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA07DC: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DA07E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DA07E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DA07E8: 409AFFA8  bne cr6, 0x82da0790
	if !ctx.cr[6].eq {
	pc = 0x82DA0790; continue 'dispatch;
	}
	// 82DA07EC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA07F0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DA07F4: 3B9C0050  addi r28, r28, 0x50
	ctx.r[28].s64 = ctx.r[28].s64 + 80;
	// 82DA07F8: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DA07FC: 4198FF8C  blt cr6, 0x82da0788
	if ctx.cr[6].lt {
	pc = 0x82DA0788; continue 'dispatch;
	}
	// 82DA0800: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DA0804: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA0808: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA080C: 409A002C  bne cr6, 0x82da0838
	if !ctx.cr[6].eq {
	pc = 0x82DA0838; continue 'dispatch;
	}
	// 82DA0810: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0814: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA0818: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA081C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA0820: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA0824: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DA0828: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA082C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DA0830: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA0834: 4BFB4A95  bl 0x82d552c8
	ctx.lr = 0x82DA0838;
	sub_82D552C8(ctx, base);
	// 82DA0838: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DA083C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DA0840: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA0844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA0848: 4BF08C0C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA0850 size=256
    let mut pc: u32 = 0x82DA0850;
    'dispatch: loop {
        match pc {
            0x82DA0850 => {
    //   block [0x82DA0850..0x82DA0950)
	// 82DA0850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA0854: 4BF08BA9  bl 0x82ca93fc
	ctx.lr = 0x82DA0858;
	sub_82CA93D0(ctx, base);
	// 82DA0858: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA085C: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 82DA0860: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DA0864: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DA0868: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DA086C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DA0870: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA0874: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DA0878: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA087C: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DA0880: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA0884: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DA0888: 409A0010  bne cr6, 0x82da0898
	if !ctx.cr[6].eq {
	pc = 0x82DA0898; continue 'dispatch;
	}
	// 82DA088C: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82DA0890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA0894: 4BFB6705  bl 0x82d56f98
	ctx.lr = 0x82DA0898;
	sub_82D56F98(ctx, base);
	// 82DA0898: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA089C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DA08A0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA08A4: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DA08A8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DA08AC: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82DA08B0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DA08B4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA08B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DA08BC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA0950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA0950 size=76
    let mut pc: u32 = 0x82DA0950;
    'dispatch: loop {
        match pc {
            0x82DA0950 => {
    //   block [0x82DA0950..0x82DA099C)
	// 82DA0950: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA0954: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0958: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DA095C: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DA0960: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA0964: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0968: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DA096C: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DA0970: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0974: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DA0978: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA097C: D006000C  stfs f0, 0xc(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA0980: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0984: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA0988: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA098C: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA0990: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DA0994: D005001C  stfs f0, 0x1c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DA0998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA09A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA09A0 size=2004
    let mut pc: u32 = 0x82DA09A0;
    'dispatch: loop {
        match pc {
            0x82DA09A0 => {
    //   block [0x82DA09A0..0x82DA1174)
	// 82DA09A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA09A4: 4BF08A2D  bl 0x82ca93d0
	ctx.lr = 0x82DA09A8;
	sub_82CA93D0(ctx, base);
	// 82DA09A8: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DA09AC: 9421F460  stwu r1, -0xba0(r1)
	ea = ctx.r[1].u32.wrapping_add(-2976 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA09B0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DA09B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DA09B8: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DA09BC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DA09C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DA09C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DA09C8: 80BB004C  lwz r5, 0x4c(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DA09CC: 93610BBC  stw r27, 0xbbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(3004 as u32), ctx.r[27].u32 ) };
	// 82DA09D0: 93810BC4  stw r28, 0xbc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(3012 as u32), ctx.r[28].u32 ) };
	// 82DA09D4: 480484A5  bl 0x82de8e78
	ctx.lr = 0x82DA09D8;
	sub_82DE8E78(ctx, base);
	// 82DA09D8: 39410150  addi r10, r1, 0x150
	ctx.r[10].s64 = ctx.r[1].s64 + 336;
	// 82DA09DC: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82DA09E0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DA09E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DA09E8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DA09EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DA09F0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DA09F4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DA09F8: 4200FFF0  bdnz 0x82da09e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DA09E8; continue 'dispatch;
	}
	// 82DA09FC: 396101FC  addi r11, r1, 0x1fc
	ctx.r[11].s64 = ctx.r[1].s64 + 508;
	// 82DA0A00: 83FB0048  lwz r31, 0x48(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DA0A04: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DA0A08: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DA0A0C: 3BBF002C  addi r29, r31, 0x2c
	ctx.r[29].s64 = ctx.r[31].s64 + 44;
	// 82DA0A10: 916101F0  stw r11, 0x1f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82DA0A14: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DA0A18: 93210198  stw r25, 0x198(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), ctx.r[25].u32 ) };
	// 82DA0A1C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82DA0A20: 9321019C  stw r25, 0x19c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), ctx.r[25].u32 ) };
	// 82DA0A24: 932101F4  stw r25, 0x1f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), ctx.r[25].u32 ) };
	// 82DA0A28: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82DA0A2C: 916101F8  stw r11, 0x1f8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(504 as u32), ctx.r[11].u32 ) };
	// 82DA0A30: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DA0A34: 81290090  lwz r9, 0x90(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DA0A38: 7FC95050  subf r30, r9, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82DA0A3C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0A40: 392102FC  addi r9, r1, 0x2fc
	ctx.r[9].s64 = ctx.r[1].s64 + 764;
	// 82DA0A44: 932102F4  stw r25, 0x2f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(756 as u32), ctx.r[25].u32 ) };
	// 82DA0A48: 39EAFFFF  addi r15, r10, -1
	ctx.r[15].s64 = ctx.r[10].s64 + -1;
	// 82DA0A4C: 916102F8  stw r11, 0x2f8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(760 as u32), ctx.r[11].u32 ) };
	// 82DA0A50: 2F0F0020  cmpwi cr6, r15, 0x20
	ctx.cr[6].compare_i32(ctx.r[15].s32, 32, &mut ctx.xer);
	// 82DA0A54: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82DA0A58: 912102F0  stw r9, 0x2f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(752 as u32), ctx.r[9].u32 ) };
	// 82DA0A5C: 40990020  ble cr6, 0x82da0a7c
	if !ctx.cr[6].gt {
	pc = 0x82DA0A7C; continue 'dispatch;
	}
	// 82DA0A60: 2F0F0040  cmpwi cr6, r15, 0x40
	ctx.cr[6].compare_i32(ctx.r[15].s32, 64, &mut ctx.xer);
	// 82DA0A64: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DA0A68: 41980008  blt cr6, 0x82da0a70
	if ctx.cr[6].lt {
	pc = 0x82DA0A70; continue 'dispatch;
	}
	// 82DA0A6C: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82DA0A70: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82DA0A74: 386102F0  addi r3, r1, 0x2f0
	ctx.r[3].s64 = ctx.r[1].s64 + 752;
	// 82DA0A78: 4BFB6499  bl 0x82d56f10
	ctx.lr = 0x82DA0A7C;
	sub_82D56F10(ctx, base);
	// 82DA0A7C: 91E102F4  stw r15, 0x2f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(756 as u32), ctx.r[15].u32 ) };
	// 82DA0A80: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0A84: 814101F4  lwz r10, 0x1f4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 82DA0A88: 810101F0  lwz r8, 0x1f0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(496 as u32) ) } as u64;
	// 82DA0A8C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DA0A90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0A94: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DA0A98: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 82DA0A9C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DA0AA0: 91410184  stw r10, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[10].u32 ) };
	// 82DA0AA4: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82DA0AA8: 7D49412E  stwx r10, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82DA0AAC: 9161018C  stw r11, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
	// 82DA0AB0: 81610158  lwz r11, 0x158(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(344 as u32) ) } as u64;
	// 82DA0AB4: 814101F4  lwz r10, 0x1f4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 82DA0AB8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0ABC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DA0AC0: 9161016C  stw r11, 0x16c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 82DA0AC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA0AC8: 914101F4  stw r10, 0x1f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), ctx.r[10].u32 ) };
	// 82DA0ACC: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA0AD0: D0010170  stfs f0, 0x170(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 82DA0AD4: 4804B7A5  bl 0x82dec278
	ctx.lr = 0x82DA0AD8;
	sub_82DEC278(ctx, base);
	// 82DA0AD8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0ADC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82DA0AE0: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82DA0AE4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DA0AE8: 4804B799  bl 0x82dec280
	ctx.lr = 0x82DA0AEC;
	sub_82DEC280(ctx, base);
	// 82DA0AEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA0AF0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DA0AF4: 7F35CB78  mr r21, r25
	ctx.r[21].u64 = ctx.r[25].u64;
	// 82DA0AF8: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82DA0AFC: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DA0B00: 40990564  ble cr6, 0x82da1064
	if !ctx.cr[6].gt {
	pc = 0x82DA1064; continue 'dispatch;
	}
	// 82DA0B04: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82DA0B08: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 82DA0B0C: 39D6001C  addi r14, r22, 0x1c
	ctx.r[14].s64 = ctx.r[22].s64 + 28;
	// 82DA0B10: 3A7F0028  addi r19, r31, 0x28
	ctx.r[19].s64 = ctx.r[31].s64 + 40;
	// 82DA0B14: 3B560018  addi r26, r22, 0x18
	ctx.r[26].s64 = ctx.r[22].s64 + 24;
	// 82DA0B18: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 82DA0B1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DA0B20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA0B24: 3A000010  li r16, 0x10
	ctx.r[16].s64 = 16;
	// 82DA0B28: 396BFC80  addi r11, r11, -0x380
	ctx.r[11].s64 = ctx.r[11].s64 + -896;
	// 82DA0B2C: 3A200020  li r17, 0x20
	ctx.r[17].s64 = 32;
	// 82DA0B30: 3A400030  li r18, 0x30
	ctx.r[18].s64 = 48;
	// 82DA0B34: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DA0B38: 81610184  lwz r11, 0x184(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(388 as u32) ) } as u64;
	// 82DA0B3C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DA0B40: 814101F8  lwz r10, 0x1f8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(504 as u32) ) } as u64;
	// 82DA0B44: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA0B48: 91610180  stw r11, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 82DA0B4C: 8161018C  lwz r11, 0x18c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(396 as u32) ) } as u64;
	// 82DA0B50: 91610188  stw r11, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 82DA0B54: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 82DA0B58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0B5C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DA0B60: 812101F4  lwz r9, 0x1f4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 82DA0B64: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DA0B68: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA0B6C: 3BEB00D0  addi r31, r11, 0xd0
	ctx.r[31].s64 = ctx.r[11].s64 + 208;
	// 82DA0B70: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DA0B74: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DA0B78: 7FDE5850  subf r30, r30, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82DA0B7C: 91610184  stw r11, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 82DA0B80: 409A0010  bne cr6, 0x82da0b90
	if !ctx.cr[6].eq {
	pc = 0x82DA0B90; continue 'dispatch;
	}
	// 82DA0B84: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DA0B88: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 82DA0B8C: 4BFB640D  bl 0x82d56f98
	ctx.lr = 0x82DA0B90;
	sub_82D56F98(ctx, base);
	// 82DA0B90: 816101F4  lwz r11, 0x1f4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 82DA0B94: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 82DA0B98: 814101F0  lwz r10, 0x1f0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(496 as u32) ) } as u64;
	// 82DA0B9C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA0BA0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82DA0BA4: 816101F4  lwz r11, 0x1f4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 82DA0BA8: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA0BAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DA0BB0: 81410188  lwz r10, 0x188(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(392 as u32) ) } as u64;
	// 82DA0BB4: 7D384A14  add r9, r24, r9
	ctx.r[9].u64 = ctx.r[24].u64 + ctx.r[9].u64;
	// 82DA0BB8: 916101F4  stw r11, 0x1f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), ctx.r[11].u32 ) };
	// 82DA0BBC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82DA0BC0: 9161018C  stw r11, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA1178 size=100
    let mut pc: u32 = 0x82DA1178;
    'dispatch: loop {
        match pc {
            0x82DA1178 => {
    //   block [0x82DA1178..0x82DA11DC)
	// 82DA1178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA117C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA1184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA118C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA1190: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DA1194: 4BFFF5C5  bl 0x82da0758
	ctx.lr = 0x82DA1198;
	sub_82DA0758(ctx, base);
	// 82DA1198: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DA119C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA11A0: 419A0020  beq cr6, 0x82da11c0
	if ctx.cr[6].eq {
	pc = 0x82DA11C0; continue 'dispatch;
	}
	// 82DA11A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA11A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DA11AC: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82DA11B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA11B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA11B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA11BC: 4BFB410D  bl 0x82d552c8
	ctx.lr = 0x82DA11C0;
	sub_82D552C8(ctx, base);
	// 82DA11C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA11C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA11C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA11CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA11D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA11D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA11D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA11E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA11E0 size=364
    let mut pc: u32 = 0x82DA11E0;
    'dispatch: loop {
        match pc {
            0x82DA11E0 => {
    //   block [0x82DA11E0..0x82DA134C)
	// 82DA11E0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DA11E4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DA11E8: 3BC00050  li r30, 0x50
	ctx.r[30].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA1350 size=156
    let mut pc: u32 = 0x82DA1350;
    'dispatch: loop {
        match pc {
            0x82DA1350 => {
    //   block [0x82DA1350..0x82DA13EC)
	// 82DA1350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA135C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1364: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA1368: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA136C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DA1370: 419A001C  beq cr6, 0x82da138c
	if ctx.cr[6].eq {
	pc = 0x82DA138C; continue 'dispatch;
	}
	// 82DA1374: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA1378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA137C: 419A0010  beq cr6, 0x82da138c
	if ctx.cr[6].eq {
	pc = 0x82DA138C; continue 'dispatch;
	}
	// 82DA1380: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA1384: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DA1388: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DA138C: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DA1390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DA1394: 419A003C  beq cr6, 0x82da13d0
	if ctx.cr[6].eq {
	pc = 0x82DA13D0; continue 'dispatch;
	}
	// 82DA1398: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA139C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA13A0: 419A0030  beq cr6, 0x82da13d0
	if ctx.cr[6].eq {
	pc = 0x82DA13D0; continue 'dispatch;
	}
	// 82DA13A4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA13A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DA13AC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DA13B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA13B4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DA13B8: 409A0018  bne cr6, 0x82da13d0
	if !ctx.cr[6].eq {
	pc = 0x82DA13D0; continue 'dispatch;
	}
	// 82DA13BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA13C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DA13C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA13C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA13CC: 4E800421  bctrl
	ctx.lr = 0x82DA13D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA13D0: 93FE00AC  stw r31, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82DA13D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA13D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA13DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA13E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA13E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA13E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA13F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA13F0 size=16
    let mut pc: u32 = 0x82DA13F0;
    'dispatch: loop {
        match pc {
            0x82DA13F0 => {
    //   block [0x82DA13F0..0x82DA1400)
	// 82DA13F0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DA13F4: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82DA13F8: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DA13FC: 4BFFADDC  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1400 size=20
    let mut pc: u32 = 0x82DA1400;
    'dispatch: loop {
        match pc {
            0x82DA1400 => {
    //   block [0x82DA1400..0x82DA1414)
	// 82DA1400: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DA1404: 39400048  li r10, 0x48
	ctx.r[10].s64 = 72;
	// 82DA1408: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA140C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA1410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1418 size=156
    let mut pc: u32 = 0x82DA1418;
    'dispatch: loop {
        match pc {
            0x82DA1418 => {
    //   block [0x82DA1418..0x82DA14B4)
	// 82DA1418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA141C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA1424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1428: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DA142C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1430: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DA1434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA1438: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA143C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DA1440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DA1444: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DA1448: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA144C: 4BFB645D  bl 0x82d578a8
	ctx.lr = 0x82DA1450;
	sub_82D578A8(ctx, base);
	// 82DA1450: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA1458: 419A0034  beq cr6, 0x82da148c
	if ctx.cr[6].eq {
	pc = 0x82DA148C; continue 'dispatch;
	}
	// 82DA145C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82DA1460: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA1464: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DA1468: 4BFB6441  bl 0x82d578a8
	ctx.lr = 0x82DA146C;
	sub_82D578A8(ctx, base);
	// 82DA146C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1470: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA1474: 419A0018  beq cr6, 0x82da148c
	if ctx.cr[6].eq {
	pc = 0x82DA148C; continue 'dispatch;
	}
	// 82DA1478: C01F00C8  lfs f0, 0xc8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA147C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA1480: C1BF00CC  lfs f13, 0xcc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1484: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DA1488: 40990008  ble cr6, 0x82da1490
	if !ctx.cr[6].gt {
	pc = 0x82DA1490; continue 'dispatch;
	}
	// 82DA148C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA1490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA1494: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA1498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA149C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA14A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA14A4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DA14A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA14AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA14B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA14B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA14B8 size=8
    let mut pc: u32 = 0x82DA14B8;
    'dispatch: loop {
        match pc {
            0x82DA14B8 => {
    //   block [0x82DA14B8..0x82DA14C0)
	// 82DA14B8: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82DA14BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA14C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA14C0 size=132
    let mut pc: u32 = 0x82DA14C0;
    'dispatch: loop {
        match pc {
            0x82DA14C0 => {
    //   block [0x82DA14C0..0x82DA1544)
	// 82DA14C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA14C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA14C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA14CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA14D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA14D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA14D8: 396BAF0C  addi r11, r11, -0x50f4
	ctx.r[11].s64 = ctx.r[11].s64 + -20724;
	// 82DA14DC: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DA14E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DA14E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA14E8: 419A003C  beq cr6, 0x82da1524
	if ctx.cr[6].eq {
	pc = 0x82DA1524; continue 'dispatch;
	}
	// 82DA14EC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA14F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA14F4: 419A0030  beq cr6, 0x82da1524
	if ctx.cr[6].eq {
	pc = 0x82DA1524; continue 'dispatch;
	}
	// 82DA14F8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA14FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DA1500: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DA1504: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA1508: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DA150C: 409A0018  bne cr6, 0x82da1524
	if !ctx.cr[6].eq {
	pc = 0x82DA1524; continue 'dispatch;
	}
	// 82DA1510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1514: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DA1518: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA151C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA1520: 4E800421  bctrl
	ctx.lr = 0x82DA1524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA1524: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DA1528: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DA152C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA1530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA1534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA1538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA153C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA1540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1548 size=28
    let mut pc: u32 = 0x82DA1548;
    'dispatch: loop {
        match pc {
            0x82DA1548 => {
    //   block [0x82DA1548..0x82DA1564)
	// 82DA1548: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 82DA154C: 98A300A2  stb r5, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[5].u8 ) };
	// 82DA1550: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DA1554: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DA1558: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DA155C: 996300B2  stb r11, 0xb2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(178 as u32), ctx.r[11].u8 ) };
	// 82DA1560: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1564 size=12
    let mut pc: u32 = 0x82DA1564;
    'dispatch: loop {
        match pc {
            0x82DA1564 => {
    //   block [0x82DA1564..0x82DA1570)
	// 82DA1564: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DA1568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA156C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1570 size=12
    let mut pc: u32 = 0x82DA1570;
    'dispatch: loop {
        match pc {
            0x82DA1570 => {
    //   block [0x82DA1570..0x82DA157C)
	// 82DA1570: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA1574: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA1578: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA157C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA157C size=28
    let mut pc: u32 = 0x82DA157C;
    'dispatch: loop {
        match pc {
            0x82DA157C => {
    //   block [0x82DA157C..0x82DA1598)
	// 82DA157C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA1580: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1584: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DA1588: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DA158C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DA1590: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA1594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1598 size=336
    let mut pc: u32 = 0x82DA1598;
    'dispatch: loop {
        match pc {
            0x82DA1598 => {
    //   block [0x82DA1598..0x82DA16E8)
	// 82DA1598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA159C: 4BF07E61  bl 0x82ca93fc
	ctx.lr = 0x82DA15A0;
	sub_82CA93D0(ctx, base);
	// 82DA15A0: 3941FFB0  addi r10, r1, -0x50
	ctx.r[10].s64 = ctx.r[1].s64 + -80;
	// 82DA15A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DA15A8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82DA15AC: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82DA15B0: 3B80000B  li r28, 0xb
	ctx.r[28].s64 = 11;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA16E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA16E8 size=544
    let mut pc: u32 = 0x82DA16E8;
    'dispatch: loop {
        match pc {
            0x82DA16E8 => {
    //   block [0x82DA16E8..0x82DA1908)
	// 82DA16E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA16EC: 4BF07D15  bl 0x82ca9400
	ctx.lr = 0x82DA16F0;
	sub_82CA93D0(ctx, base);
	// 82DA16F0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1908 size=28
    let mut pc: u32 = 0x82DA1908;
    'dispatch: loop {
        match pc {
            0x82DA1908 => {
    //   block [0x82DA1908..0x82DA1924)
	// 82DA1908: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA190C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1910: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1914: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DA1918: EC200824  fdivs f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82DA191C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA1920: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1928 size=112
    let mut pc: u32 = 0x82DA1928;
    'dispatch: loop {
        match pc {
            0x82DA1928 => {
    //   block [0x82DA1928..0x82DA1998)
	// 82DA1928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA192C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1934: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1938: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DA193C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA1940: 4BFE9161  bl 0x82d8aaa0
	ctx.lr = 0x82DA1944;
	sub_82D8AAA0(ctx, base);
	// 82DA1944: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA1948: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 82DA194C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA1950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA1954: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1958: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA195C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DA1960: 396BB13C  addi r11, r11, -0x4ec4
	ctx.r[11].s64 = ctx.r[11].s64 + -20164;
	// 82DA1964: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DA1968: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DA196C: 993F0008  stb r9, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82DA1970: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DA1974: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA1978: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1998 size=80
    let mut pc: u32 = 0x82DA1998;
    'dispatch: loop {
        match pc {
            0x82DA1998 => {
    //   block [0x82DA1998..0x82DA19E8)
	// 82DA1998: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA199C: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA19A0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DA19A4: C18300C4  lfs f12, 0xc4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA19A8: C16300C8  lfs f11, 0xc8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA19E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA19E8 size=72
    let mut pc: u32 = 0x82DA19E8;
    'dispatch: loop {
        match pc {
            0x82DA19E8 => {
    //   block [0x82DA19E8..0x82DA1A30)
	// 82DA19E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA19EC: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA19F0: C1840014  lfs f12, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA19F4: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 82DA19F8: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA19FC: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DA1A00: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DA1A04: C1A40028  lfs f13, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1A08: ED806024  fdivs f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 82DA1A0C: D181FFF4  stfs f12, -0xc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82DA1A10: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DA1A14: C18300CC  lfs f12, 0xcc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA1A18: D181FFFC  stfs f12, -4(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DA1A1C: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DA1A20: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1A30 size=60
    let mut pc: u32 = 0x82DA1A30;
    'dispatch: loop {
        match pc {
            0x82DA1A30 => {
    //   block [0x82DA1A30..0x82DA1A6C)
	// 82DA1A30: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA1A34: C00300C0  lfs f0, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1A38: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DA1A3C: C1A300C4  lfs f13, 0xc4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1A40: C18300C8  lfs f12, 0xc8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1A70 size=52
    let mut pc: u32 = 0x82DA1A70;
    'dispatch: loop {
        match pc {
            0x82DA1A70 => {
    //   block [0x82DA1A70..0x82DA1AA4)
	// 82DA1A70: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1A74: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82DA1A78: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DA1A7C: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1A80: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82DA1A84: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1A88: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DA1A8C: C00300CC  lfs f0, 0xcc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1A90: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DA1A94: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA1AA8 size=124
    let mut pc: u32 = 0x82DA1AA8;
    'dispatch: loop {
        match pc {
            0x82DA1AA8 => {
    //   block [0x82DA1AA8..0x82DA1B24)
	// 82DA1AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1AB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1AB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DA1AB8: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 82DA1ABC: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DA1AC0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DA1AC4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82DA1AC8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1B28 size=120
    let mut pc: u32 = 0x82DA1B28;
    'dispatch: loop {
        match pc {
            0x82DA1B28 => {
    //   block [0x82DA1B28..0x82DA1BA0)
	// 82DA1B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1B34: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DA1B38: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1B3C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DA1B40: C18300C4  lfs f12, 0xc4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA1B44: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DA1B48: C16300C8  lfs f11, 0xc8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DA1B4C: 38830010  addi r4, r3, 0x10
	ctx.r[4].s64 = ctx.r[3].s64 + 16;
	// 82DA1B50: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1BA0 size=56
    let mut pc: u32 = 0x82DA1BA0;
    'dispatch: loop {
        match pc {
            0x82DA1BA0 => {
    //   block [0x82DA1BA0..0x82DA1BD8)
	// 82DA1BA0: 3941001C  addi r10, r1, 0x1c
	ctx.r[10].s64 = ctx.r[1].s64 + 28;
	// 82DA1BA4: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1BD8 size=88
    let mut pc: u32 = 0x82DA1BD8;
    'dispatch: loop {
        match pc {
            0x82DA1BD8 => {
    //   block [0x82DA1BD8..0x82DA1C30)
	// 82DA1BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1BE4: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 82DA1BE8: D021008C  stfs f1, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82DA1BEC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DA1BF0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DA1BF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1C30 size=80
    let mut pc: u32 = 0x82DA1C30;
    'dispatch: loop {
        match pc {
            0x82DA1C30 => {
    //   block [0x82DA1C30..0x82DA1C80)
	// 82DA1C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1C3C: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 82DA1C40: D021008C  stfs f1, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1C80 size=212
    let mut pc: u32 = 0x82DA1C80;
    'dispatch: loop {
        match pc {
            0x82DA1C80 => {
    //   block [0x82DA1C80..0x82DA1D54)
	// 82DA1C80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DA1C84: 394300C0  addi r10, r3, 0xc0
	ctx.r[10].s64 = ctx.r[3].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1D58 size=128
    let mut pc: u32 = 0x82DA1D58;
    'dispatch: loop {
        match pc {
            0x82DA1D58 => {
    //   block [0x82DA1D58..0x82DA1DD8)
	// 82DA1D58: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1DD8 size=16
    let mut pc: u32 = 0x82DA1DD8;
    'dispatch: loop {
        match pc {
            0x82DA1DD8 => {
    //   block [0x82DA1DD8..0x82DA1DE8)
	// 82DA1DD8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DA1DDC: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82DA1DE0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DA1DE4: 4BFFA3F4  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1DE8 size=32
    let mut pc: u32 = 0x82DA1DE8;
    'dispatch: loop {
        match pc {
            0x82DA1DE8 => {
    //   block [0x82DA1DE8..0x82DA1E08)
	// 82DA1DE8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82DA1DEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA1DF0: 419A0018  beq cr6, 0x82da1e08
	if ctx.cr[6].eq {
		sub_82DA1E08(ctx, base);
		return;
	}
	// 82DA1DF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA1DF8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DA1DFC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA1E00: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA1E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1E08 size=16
    let mut pc: u32 = 0x82DA1E08;
    'dispatch: loop {
        match pc {
            0x82DA1E08 => {
    //   block [0x82DA1E08..0x82DA1E18)
	// 82DA1E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA1E0C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA1E10: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA1E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1E18 size=48
    let mut pc: u32 = 0x82DA1E18;
    'dispatch: loop {
        match pc {
            0x82DA1E18 => {
    //   block [0x82DA1E18..0x82DA1E48)
	// 82DA1E18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA1E1C: C1A40070  lfs f13, 0x70(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1E20: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1E24: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DA1E28: 40990014  ble cr6, 0x82da1e3c
	if !ctx.cr[6].gt {
	pc = 0x82DA1E3C; continue 'dispatch;
	}
	// 82DA1E2C: C1A40074  lfs f13, 0x74(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1E30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA1E34: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DA1E38: 41990008  bgt cr6, 0x82da1e40
	if ctx.cr[6].gt {
	pc = 0x82DA1E40; continue 'dispatch;
	}
	// 82DA1E3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA1E40: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA1E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1E48 size=8
    let mut pc: u32 = 0x82DA1E48;
    'dispatch: loop {
        match pc {
            0x82DA1E48 => {
    //   block [0x82DA1E48..0x82DA1E50)
	// 82DA1E48: 3860000F  li r3, 0xf
	ctx.r[3].s64 = 15;
	// 82DA1E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA1E50 size=136
    let mut pc: u32 = 0x82DA1E50;
    'dispatch: loop {
        match pc {
            0x82DA1E50 => {
    //   block [0x82DA1E50..0x82DA1ED8)
	// 82DA1E50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA1E54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA1E58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DA1E5C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82DA1E60: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82DA1E64: C1AA0C14  lfs f13, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA1E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA1E6C: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1E70: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DA1E74: 390AB214  addi r8, r10, -0x4dec
	ctx.r[8].s64 = ctx.r[10].s64 + -19948;
	// 82DA1E78: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DA1E7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DA1E80: 39230050  addi r9, r3, 0x50
	ctx.r[9].s64 = ctx.r[3].s64 + 80;
	// 82DA1E84: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA1E88: 39030060  addi r8, r3, 0x60
	ctx.r[8].s64 = ctx.r[3].s64 + 96;
	// 82DA1E8C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DA1E90: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82DA1E94: B0E30010  sth r7, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u16 ) };
	// 82DA1E98: B0C30040  sth r6, 0x40(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[6].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1ED8 size=20
    let mut pc: u32 = 0x82DA1ED8;
    'dispatch: loop {
        match pc {
            0x82DA1ED8 => {
    //   block [0x82DA1ED8..0x82DA1EEC)
	// 82DA1ED8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DA1EDC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DA1EE0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DA1EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DA1EE8: 48046F90  b 0x82de8e78
	sub_82DE8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA1EF0 size=88
    let mut pc: u32 = 0x82DA1EF0;
    'dispatch: loop {
        match pc {
            0x82DA1EF0 => {
    //   block [0x82DA1EF0..0x82DA1F48)
	// 82DA1EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1EF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA1EFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1F04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA1F08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA1F0C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA1F10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1F14: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DA1F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA1F1C: 4E800421  bctrl
	ctx.lr = 0x82DA1F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA1F20: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 82DA1F24: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82DA1F28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DA1F2C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DA1F30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA1F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA1F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA1F3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA1F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA1F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1F48 size=24
    let mut pc: u32 = 0x82DA1F48;
    'dispatch: loop {
        match pc {
            0x82DA1F48 => {
    //   block [0x82DA1F48..0x82DA1F60)
	// 82DA1F48: A163001E  lhz r11, 0x1e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 82DA1F4C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA1F50: A163001C  lhz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA1F54: 396B0034  addi r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 + 52;
	// 82DA1F58: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA1F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA1F60 size=64
    let mut pc: u32 = 0x82DA1F60;
    'dispatch: loop {
        match pc {
            0x82DA1F60 => {
    //   block [0x82DA1F60..0x82DA1FA0)
	// 82DA1F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1F70: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA1F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA1F78: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA1F7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA1F80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA1F84: 4E800421  bctrl
	ctx.lr = 0x82DA1F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA1F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA1F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA1F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA1F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA1F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA1F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA1FA0 size=8
    let mut pc: u32 = 0x82DA1FA0;
    'dispatch: loop {
        match pc {
            0x82DA1FA0 => {
    //   block [0x82DA1FA0..0x82DA1FA8)
	// 82DA1FA0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82DA1FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA1FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA1FA8 size=208
    let mut pc: u32 = 0x82DA1FA8;
    'dispatch: loop {
        match pc {
            0x82DA1FA8 => {
    //   block [0x82DA1FA8..0x82DA2078)
	// 82DA1FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA1FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA1FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA1FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA1FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA1FBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA1FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA1FC4: 392BFCE4  addi r9, r11, -0x31c
	ctx.r[9].s64 = ctx.r[11].s64 + -796;
	// 82DA1FC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA1FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DA1FD0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DA1FD4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DA1FD8: C00B0A54  lfs f0, 0xa54(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA1FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA1FE0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DA1FE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA1FE8: B15F000C  sth r10, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 82DA1FEC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DA1FF0: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DA1FF4: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82DA1FF8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DA1FFC: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82DA2000: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA2004: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2008: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DA200C: 419A0010  beq cr6, 0x82da201c
	if ctx.cr[6].eq {
	pc = 0x82DA201C; continue 'dispatch;
	}
	// 82DA2010: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA2014: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DA2018: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DA201C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA2020: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DA2024: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82DA2028: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DA202C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2030: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2034: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DA2038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA203C: 4E800421  bctrl
	ctx.lr = 0x82DA2040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2040: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DA2044: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DA2048: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA204C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2050: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 82DA2054: B15F001E  sth r10, 0x1e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 82DA2058: 4BFFC549  bl 0x82d9e5a0
	ctx.lr = 0x82DA205C;
	sub_82D9E5A0(ctx, base);
	// 82DA205C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA2064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA2068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA206C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA2070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA2074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2078 size=104
    let mut pc: u32 = 0x82DA2078;
    'dispatch: loop {
        match pc {
            0x82DA2078 => {
    //   block [0x82DA2078..0x82DA20E0)
	// 82DA2078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA207C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA2080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA2084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA2088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA208C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA2090: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82DA2094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA2098: 396BFCE4  addi r11, r11, -0x31c
	ctx.r[11].s64 = ctx.r[11].s64 + -796;
	// 82DA209C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DA20A0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DA20A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA20A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA20AC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DA20B0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA20B4: 4BFFC4ED  bl 0x82d9e5a0
	ctx.lr = 0x82DA20B8;
	sub_82D9E5A0(ctx, base);
	// 82DA20B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA20BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA20C0: 4BFFC4E1  bl 0x82d9e5a0
	ctx.lr = 0x82DA20C4;
	sub_82D9E5A0(ctx, base);
	// 82DA20C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA20C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA20CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA20D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA20D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA20D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA20DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA20E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA20E0 size=124
    let mut pc: u32 = 0x82DA20E0;
    'dispatch: loop {
        match pc {
            0x82DA20E0 => {
    //   block [0x82DA20E0..0x82DA215C)
	// 82DA20E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA20E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA20E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA20EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA20F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA20F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA20F8: 396BFCE4  addi r11, r11, -0x31c
	ctx.r[11].s64 = ctx.r[11].s64 + -796;
	// 82DA20FC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA2100: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA2104: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA210C: 419A0030  beq cr6, 0x82da213c
	if ctx.cr[6].eq {
	pc = 0x82DA213C; continue 'dispatch;
	}
	// 82DA2110: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DA2114: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DA2118: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DA211C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA2120: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DA2124: 409A0018  bne cr6, 0x82da213c
	if !ctx.cr[6].eq {
	pc = 0x82DA213C; continue 'dispatch;
	}
	// 82DA2128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA212C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DA2130: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2138: 4E800421  bctrl
	ctx.lr = 0x82DA213C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA213C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DA2140: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DA2144: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA2148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA214C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA2150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA2154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA2158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA2160 size=680
    let mut pc: u32 = 0x82DA2160;
    'dispatch: loop {
        match pc {
            0x82DA2160 => {
    //   block [0x82DA2160..0x82DA2408)
	// 82DA2160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2164: 4BF072A1  bl 0x82ca9404
	ctx.lr = 0x82DA2168;
	sub_82CA93D0(ctx, base);
	// 82DA2168: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82DA216C: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DA2170: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA2178: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DA217C: A17E001C  lhz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA2180: 837D004C  lwz r27, 0x4c(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DA2184: A39E001E  lhz r28, 0x1e(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(30 as u32) ) } as u64;
	// 82DA2188: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DA218C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2190: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA2194: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA2198: C3CB0C18  lfs f30, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82DA219C: 409A0238  bne cr6, 0x82da23d4
	if !ctx.cr[6].eq {
	pc = 0x82DA23D4; continue 'dispatch;
	}
	// 82DA21A0: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 82DA21A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DA21A8: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82DA21AC: 41980048  blt cr6, 0x82da21f4
	if ctx.cr[6].lt {
	pc = 0x82DA21F4; continue 'dispatch;
	}
	// 82DA21B0: 395CFFFC  addi r10, r28, -4
	ctx.r[10].s64 = ctx.r[28].s64 + -4;
	// 82DA21B4: 397B0010  addi r11, r27, 0x10
	ctx.r[11].s64 = ctx.r[27].s64 + 16;
	// 82DA21B8: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA21BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DA21C0: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DA21C4: C00BFFF0  lfs f0, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA21C8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA21CC: EDA0F83A  fmadds f13, f0, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82DA21D0: C00BFFF8  lfs f0, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA21D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA21D8: EDA0683A  fmadds f13, f0, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DA21DC: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA21E0: EDA0683A  fmadds f13, f0, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DA21E4: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA21E8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82DA21EC: EFE0683A  fmadds f31, f0, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DA21F0: 409AFFD4  bne cr6, 0x82da21c4
	if !ctx.cr[6].eq {
	pc = 0x82DA21C4; continue 'dispatch;
	}
	// 82DA21F4: 7F09E000  cmpw cr6, r9, r28
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DA21F8: 40980028  bge cr6, 0x82da2220
	if !ctx.cr[6].lt {
	pc = 0x82DA2220; continue 'dispatch;
	}
	// 82DA21FC: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA2200: 7D69E050  subf r11, r9, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[9].s64;
	// 82DA2204: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82DA2208: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DA220C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2210: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DA2214: EFE0F83A  fmadds f31, f0, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82DA2218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA221C: 409AFFEC  bne cr6, 0x82da2208
	if !ctx.cr[6].eq {
	pc = 0x82DA2208; continue 'dispatch;
	}
	// 82DA2220: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2224: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DA2228: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DA222C: 409901A8  ble cr6, 0x82da23d4
	if !ctx.cr[6].gt {
	pc = 0x82DA23D4; continue 'dispatch;
	}
	// 82DA2230: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA2234: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA2238: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DA223C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DA2240: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DA2244: 714A00FB  andi. r10, r10, 0xfb
	ctx.r[10].u64 = ctx.r[10].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2248: 994B0012  stb r10, 0x12(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[10].u8 ) };
	// 82DA224C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DA2250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA2254: 419A0038  beq cr6, 0x82da228c
	if ctx.cr[6].eq {
	pc = 0x82DA228C; continue 'dispatch;
	}
	// 82DA2258: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DA225C: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DA2260: EC00F82C  fsqrts f0, f31
	ctx.f[0].f64 = ((ctx.f[31].f64).sqrt() as f32) as f64;
	// 82DA2264: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82DA2268: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DA226C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DA2270: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DA2274: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DA2278: 9961006C  stb r11, 0x6c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 82DA227C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2280: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2288: 4E800421  bctrl
	ctx.lr = 0x82DA228C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA228C: 897E0025  lbz r11, 0x25(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DA2290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA2294: 419A0140  beq cr6, 0x82da23d4
	if ctx.cr[6].eq {
	pc = 0x82DA23D4; continue 'dispatch;
	}
	// 82DA2298: EDA0F82C  fsqrts f13, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[31].f64).sqrt() as f32) as f64;
	// 82DA229C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA22A0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DA22A4: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA22A8: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DA22AC: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DA22B0: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA22B4: 813D0034  lwz r9, 0x34(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DA22B8: D1810064  stfs f12, 0x64(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DA22BC: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82DA22C0: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA22C4: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DA22C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DA22CC: C19F000C  lfs f12, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA22D0: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DA22D4: 38E90010  addi r7, r9, 0x10
	ctx.r[7].s64 = ctx.r[9].s64 + 16;
	// 82DA22D8: 396B4C40  addi r11, r11, 0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + 19520;
	// 82DA22DC: D3C1006C  stfs f30, 0x6c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DA22E0: C19F0010  lfs f12, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA22E4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DA22E8: D1810070  stfs f12, 0x70(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DA22EC: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 82DA22F0: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA2408 size=296
    let mut pc: u32 = 0x82DA2408;
    'dispatch: loop {
        match pc {
            0x82DA2408 => {
    //   block [0x82DA2408..0x82DA2530)
	// 82DA2408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA240C: 4BF07001  bl 0x82ca940c
	ctx.lr = 0x82DA2410;
	sub_82CA93D0(ctx, base);
	// 82DA2410: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA2418: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA241C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DA2420: A17E001C  lhz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA2424: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DA2428: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DA242C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2430: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA2434: 409A00C0  bne cr6, 0x82da24f4
	if !ctx.cr[6].eq {
	pc = 0x82DA24F4; continue 'dispatch;
	}
	// 82DA2438: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DA243C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DA2440: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DA2444: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2448: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DA244C: C00A0014  lfs f0, 0x14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2450: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DA2454: C00A0018  lfs f0, 0x18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2458: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA245C: C0090010  lfs f0, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2460: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA2464: C0090014  lfs f0, 0x14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2468: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DA246C: C0090018  lfs f0, 0x18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2470: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DA2474: C00A0020  lfs f0, 0x20(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2478: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DA247C: C00A0024  lfs f0, 0x24(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2480: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DA2484: C00A0028  lfs f0, 0x28(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2488: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DA248C: C0090020  lfs f0, 0x20(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2490: D00B0028  stfs f0, 0x28(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DA2494: C0090024  lfs f0, 0x24(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA2498: D00B002C  stfs f0, 0x2c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DA249C: C0090028  lfs f0, 0x28(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA24A0: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DA24A4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA24A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA24AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DA24B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA24B4: 4E800421  bctrl
	ctx.lr = 0x82DA24B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA24B8: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82DA24BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DA24C0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA24C4: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82DA24C8: 409A0018  bne cr6, 0x82da24e0
	if !ctx.cr[6].eq {
	pc = 0x82DA24E0; continue 'dispatch;
	}
	// 82DA24CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DA24D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA24D4: 48051335  bl 0x82df3808
	ctx.lr = 0x82DA24D8;
	sub_82DF3808(ctx, base);
	// 82DA24D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DA24DC: 4BF06F80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82DA24E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DA24E4: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DA24E8: 48046B39  bl 0x82de9020
	ctx.lr = 0x82DA24EC;
	sub_82DE9020(ctx, base);
	// 82DA24EC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DA24F0: 4BF06F6C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82DA24F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DA24F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DA24FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DA2500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2504: 48046975  bl 0x82de8e78
	ctx.lr = 0x82DA2508;
	sub_82DE8E78(ctx, base);
	// 82DA2508: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DA250C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA2510: 419A0018  beq cr6, 0x82da2528
	if ctx.cr[6].eq {
	pc = 0x82DA2528; continue 'dispatch;
	}
	// 82DA2514: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DA2518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DA251C: 81650014  lwz r11, 0x14(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DA2520: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2524: 4BFDC0C5  bl 0x82d7e5e8
	ctx.lr = 0x82DA2528;
	sub_82D7E5E8(ctx, base);
	// 82DA2528: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DA252C: 4BF06F30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2530 size=100
    let mut pc: u32 = 0x82DA2530;
    'dispatch: loop {
        match pc {
            0x82DA2530 => {
    //   block [0x82DA2530..0x82DA2594)
	// 82DA2530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA2538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA253C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA2540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA2548: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DA254C: 4BFFFB95  bl 0x82da20e0
	ctx.lr = 0x82DA2550;
	sub_82DA20E0(ctx, base);
	// 82DA2550: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DA2554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA2558: 419A0020  beq cr6, 0x82da2578
	if ctx.cr[6].eq {
	pc = 0x82DA2578; continue 'dispatch;
	}
	// 82DA255C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2560: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DA2564: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82DA2568: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA256C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DA2570: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA2574: 4BFB2D55  bl 0x82d552c8
	ctx.lr = 0x82DA2578;
	sub_82D552C8(ctx, base);
	// 82DA2578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA257C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA2580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA2584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA2588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA258C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA2590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA2598 size=8
    let mut pc: u32 = 0x82DA2598;
    'dispatch: loop {
        match pc {
            0x82DA2598 => {
    //   block [0x82DA2598..0x82DA25A0)
	// 82DA2598: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DA259C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA25A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA25A0 size=132
    let mut pc: u32 = 0x82DA25A0;
    'dispatch: loop {
        match pc {
            0x82DA25A0 => {
    //   block [0x82DA25A0..0x82DA2624)
	// 82DA25A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA25A4: 4BF06E5D  bl 0x82ca9400
	ctx.lr = 0x82DA25A8;
	sub_82CA93D0(ctx, base);
	// 82DA25A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA25AC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DA25B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DA25B4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DA25B8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DA25BC: 409A000C  bne cr6, 0x82da25c8
	if !ctx.cr[6].eq {
	pc = 0x82DA25C8; continue 'dispatch;
	}
	// 82DA25C0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA25C4: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DA25C8: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA25CC: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DA25D0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DA25D4: 41980048  blt cr6, 0x82da261c
	if ctx.cr[6].lt {
	pc = 0x82DA261C; continue 'dispatch;
	}
	// 82DA25D8: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82DA25DC: 57FE1838  slwi r30, r31, 3
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DA25E0: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA25E4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DA25E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DA25EC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DA25F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DA25F4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA25F8: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA25FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2600: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DA2604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2608: 4E800421  bctrl
	ctx.lr = 0x82DA260C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA260C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DA2610: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82DA2614: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DA2618: 4098FFC8  bge cr6, 0x82da25e0
	if !ctx.cr[6].lt {
	pc = 0x82DA25E0; continue 'dispatch;
	}
	// 82DA261C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA2620: 4BF06E30  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2628 size=144
    let mut pc: u32 = 0x82DA2628;
    'dispatch: loop {
        match pc {
            0x82DA2628 => {
    //   block [0x82DA2628..0x82DA26B8)
	// 82DA2628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA262C: 4BF06DD5  bl 0x82ca9400
	ctx.lr = 0x82DA2630;
	sub_82CA93D0(ctx, base);
	// 82DA2630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2634: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DA2638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA263C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DA2640: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DA2644: 409A000C  bne cr6, 0x82da2650
	if !ctx.cr[6].eq {
	pc = 0x82DA2650; continue 'dispatch;
	}
	// 82DA2648: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA264C: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DA2650: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2654: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 82DA2658: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82DA265C: 41980054  blt cr6, 0x82da26b0
	if ctx.cr[6].lt {
	pc = 0x82DA26B0; continue 'dispatch;
	}
	// 82DA2660: 3B9E0010  addi r28, r30, 0x10
	ctx.r[28].s64 = ctx.r[30].s64 + 16;
	// 82DA2664: 575F1838  slwi r31, r26, 3
	ctx.r[31].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DA2668: 817E0150  lwz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA266C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DA2670: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DA2674: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82DA2678: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DA267C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2680: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2688: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA268C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2690: 4E800421  bctrl
	ctx.lr = 0x82DA2694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2694: 897D0004  lbz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA269C: 409A0014  bne cr6, 0x82da26b0
	if !ctx.cr[6].eq {
	pc = 0x82DA26B0; continue 'dispatch;
	}
	// 82DA26A0: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 82DA26A4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82DA26A8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82DA26AC: 4098FFBC  bge cr6, 0x82da2668
	if !ctx.cr[6].lt {
	pc = 0x82DA2668; continue 'dispatch;
	}
	// 82DA26B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA26B4: 4BF06D9C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA26B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA26B8 size=64
    let mut pc: u32 = 0x82DA26B8;
    'dispatch: loop {
        match pc {
            0x82DA26B8 => {
    //   block [0x82DA26B8..0x82DA26F8)
	// 82DA26B8: 81240154  lwz r9, 0x154(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA26BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA26C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DA26C4: 40990028  ble cr6, 0x82da26ec
	if !ctx.cr[6].gt {
	pc = 0x82DA26EC; continue 'dispatch;
	}
	// 82DA26C8: 81640150  lwz r11, 0x150(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA26CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA26D0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA26D4: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DA26D8: 419A0020  beq cr6, 0x82da26f8
	if ctx.cr[6].eq {
		sub_82DA26F8(ctx, base);
		return;
	}
	// 82DA26DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DA26E0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DA26E4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA26E8: 4198FFE8  blt cr6, 0x82da26d0
	if ctx.cr[6].lt {
	pc = 0x82DA26D0; continue 'dispatch;
	}
	// 82DA26EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA26F0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA26F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA26F8 size=12
    let mut pc: u32 = 0x82DA26F8;
    'dispatch: loop {
        match pc {
            0x82DA26F8 => {
    //   block [0x82DA26F8..0x82DA2704)
	// 82DA26F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA26FC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA2700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2708 size=272
    let mut pc: u32 = 0x82DA2708;
    'dispatch: loop {
        match pc {
            0x82DA2708 => {
    //   block [0x82DA2708..0x82DA2818)
	// 82DA2708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA2710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA2714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA2718: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA271C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA2720: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2724: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA2728: 396BE704  addi r11, r11, -0x18fc
	ctx.r[11].s64 = ctx.r[11].s64 + -6396;
	// 82DA272C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA2730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2734: B1210066  sth r9, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[9].u16 ) };
	// 82DA2738: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DA273C: 419A0058  beq cr6, 0x82da2794
	if ctx.cr[6].eq {
	pc = 0x82DA2794; continue 'dispatch;
	}
	// 82DA2740: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2744: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DA2748: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DA274C: 41980030  blt cr6, 0x82da277c
	if ctx.cr[6].lt {
	pc = 0x82DA277C; continue 'dispatch;
	}
	// 82DA2750: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA2754: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA2758: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DA275C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA2760: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2764: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DA2768: 419A0044  beq cr6, 0x82da27ac
	if ctx.cr[6].eq {
	pc = 0x82DA27AC; continue 'dispatch;
	}
	// 82DA276C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DA2770: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82DA2774: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DA2778: 4098FFE8  bge cr6, 0x82da2760
	if !ctx.cr[6].lt {
	pc = 0x82DA2760; continue 'dispatch;
	}
	// 82DA277C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA2780: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DA2784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2788: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82DA278C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2790: 4B4ECA01  bl 0x8228f190
	ctx.lr = 0x82DA2794;
	sub_8228F190(ctx, base);
	// 82DA2794: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA2798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA279C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA27A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA27A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA27A8: 4E800020  blr
	return;
	// 82DA27AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DA27B0: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82DA27B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA27B8: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA27BC: 4B4EC9D5  bl 0x8228f190
	ctx.lr = 0x82DA27C0;
	sub_8228F190(ctx, base);
	// 82DA27C0: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA27C4: 57DE1838  slwi r30, r30, 3
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DA27C8: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA27CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DA27D0: 419A0018  beq cr6, 0x82da27e8
	if ctx.cr[6].eq {
	pc = 0x82DA27E8; continue 'dispatch;
	}
	// 82DA27D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA27D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DA27DC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA27E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA27E4: 4E800421  bctrl
	ctx.lr = 0x82DA27E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA27E8: 815F0154  lwz r10, 0x154(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA27EC: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA27F0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA27F4: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DA27F8: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DA27FC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DA2800: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 82DA2804: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2808: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA280C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2810: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA2814: 4BFFFF80  b 0x82da2794
	pc = 0x82DA2794; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2818 size=172
    let mut pc: u32 = 0x82DA2818;
    'dispatch: loop {
        match pc {
            0x82DA2818 => {
    //   block [0x82DA2818..0x82DA28C4)
	// 82DA2818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA281C: 4BF06BED  bl 0x82ca9408
	ctx.lr = 0x82DA2820;
	sub_82CA93D0(ctx, base);
	// 82DA2820: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2824: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DA2828: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DA282C: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DA2830: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2834: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DA2838: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DA283C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DA2840: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DA2844: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DA2848: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DA284C: 4200FFF0  bdnz 0x82da283c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DA283C; continue 'dispatch;
	}
	// 82DA2850: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA2854: 815D0154  lwz r10, 0x154(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA285C: 396BE704  addi r11, r11, -0x18fc
	ctx.r[11].s64 = ctx.r[11].s64 + -6396;
	// 82DA2860: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DA2864: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2868: B1210056  sth r9, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[9].u16 ) };
	// 82DA286C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DA2870: 4099004C  ble cr6, 0x82da28bc
	if !ctx.cr[6].gt {
	pc = 0x82DA28BC; continue 'dispatch;
	}
	// 82DA2874: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82DA2878: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DA287C: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA2880: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DA2884: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DA2888: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DA288C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DA2890: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2894: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2898: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA289C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DA28A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA28A4: 4E800421  bctrl
	ctx.lr = 0x82DA28A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA28A8: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA28AC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DA28B0: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DA28B4: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DA28B8: 4198FFC4  blt cr6, 0x82da287c
	if ctx.cr[6].lt {
	pc = 0x82DA287C; continue 'dispatch;
	}
	// 82DA28BC: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DA28C0: 4BF06B98  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA28C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA28C8 size=96
    let mut pc: u32 = 0x82DA28C8;
    'dispatch: loop {
        match pc {
            0x82DA28C8 => {
    //   block [0x82DA28C8..0x82DA2928)
	// 82DA28C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA28CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA28D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA28D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA28D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA28DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA28E0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DA28E4: 4BFDE485  bl 0x82d80d68
	ctx.lr = 0x82DA28E8;
	sub_82D80D68(ctx, base);
	// 82DA28E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA28EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA28F0: 396B42BC  addi r11, r11, 0x42bc
	ctx.r[11].s64 = ctx.r[11].s64 + 17084;
	// 82DA28F4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DA28F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA28FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA2900: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 82DA2904: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 82DA2908: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 82DA290C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82DA2910: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA2914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA2918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA291C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA2920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA2924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2928 size=192
    let mut pc: u32 = 0x82DA2928;
    'dispatch: loop {
        match pc {
            0x82DA2928 => {
    //   block [0x82DA2928..0x82DA29E8)
	// 82DA2928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA292C: 4BF06AE1  bl 0x82ca940c
	ctx.lr = 0x82DA2930;
	sub_82CA93D0(ctx, base);
	// 82DA2930: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA2938: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA293C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DA2940: 394A42BC  addi r10, r10, 0x42bc
	ctx.r[10].s64 = ctx.r[10].s64 + 17084;
	// 82DA2944: 3929E704  addi r9, r9, -0x18fc
	ctx.r[9].s64 = ctx.r[9].s64 + -6396;
	// 82DA2948: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA294C: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DA2950: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA2954: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA2958: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DA295C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DA2960: B1610056  sth r11, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 82DA2964: 41980034  blt cr6, 0x82da2998
	if ctx.cr[6].lt {
	pc = 0x82DA2998; continue 'dispatch;
	}
	// 82DA2968: 57FD1838  slwi r29, r31, 3
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DA296C: 817E0150  lwz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA2970: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DA2974: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DA2978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA297C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA2980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2984: 4E800421  bctrl
	ctx.lr = 0x82DA2988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2988: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DA298C: 3BBDFFF8  addi r29, r29, -8
	ctx.r[29].s64 = ctx.r[29].s64 + -8;
	// 82DA2990: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DA2994: 4098FFD8  bge cr6, 0x82da296c
	if !ctx.cr[6].lt {
	pc = 0x82DA296C; continue 'dispatch;
	}
	// 82DA2998: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DA299C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA29A0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DA29A4: 915E0154  stw r10, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 82DA29A8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DA29AC: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 82DA29B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA29B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA29B8: 409A0020  bne cr6, 0x82da29d8
	if !ctx.cr[6].eq {
	pc = 0x82DA29D8; continue 'dispatch;
	}
	// 82DA29BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA29C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA29C4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA29C8: 809E0150  lwz r4, 0x150(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA29CC: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA29D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DA29D4: 4BFB28F5  bl 0x82d552c8
	ctx.lr = 0x82DA29D8;
	sub_82D552C8(ctx, base);
	// 82DA29D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA29DC: 4BFF721D  bl 0x82d99bf8
	ctx.lr = 0x82DA29E0;
	sub_82D99BF8(ctx, base);
	// 82DA29E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA29E4: 4BF06A78  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA29E8 size=440
    let mut pc: u32 = 0x82DA29E8;
    'dispatch: loop {
        match pc {
            0x82DA29E8 => {
    //   block [0x82DA29E8..0x82DA2BA0)
	// 82DA29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA29EC: 4BF06A11  bl 0x82ca93fc
	ctx.lr = 0x82DA29F0;
	sub_82CA93D0(ctx, base);
	// 82DA29F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA29F4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA29F8: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 82DA29FC: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82DA2A00: 38800160  li r4, 0x160
	ctx.r[4].s64 = 352;
	// 82DA2A04: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DA2A08: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DA2A0C: 4BFB283D  bl 0x82d55248
	ctx.lr = 0x82DA2A10;
	sub_82D55248(ctx, base);
	// 82DA2A10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA2A14: 39600160  li r11, 0x160
	ctx.r[11].s64 = 352;
	// 82DA2A18: 38BB00A0  addi r5, r27, 0xa0
	ctx.r[5].s64 = ctx.r[27].s64 + 160;
	// 82DA2A1C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DA2A20: 839B002C  lwz r28, 0x2c(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DA2A24: 809B0010  lwz r4, 0x10(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DA2A28: 4BFDE341  bl 0x82d80d68
	ctx.lr = 0x82DA2A2C;
	sub_82D80D68(ctx, base);
	// 82DA2A2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA2A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA2A34: 396B42BC  addi r11, r11, 0x42bc
	ctx.r[11].s64 = ctx.r[11].s64 + 17084;
	// 82DA2A38: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DA2A3C: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 82DA2A40: 3BBB0080  addi r29, r27, 0x80
	ctx.r[29].s64 = ctx.r[27].s64 + 128;
	// 82DA2A44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA2A48: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 82DA2A4C: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 82DA2A50: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 82DA2A54: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82DA2A58: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2A5C: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2A60: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA2A64: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA2A68: 40980050  bge cr6, 0x82da2ab8
	if !ctx.cr[6].lt {
	pc = 0x82DA2AB8; continue 'dispatch;
	}
	// 82DA2A6C: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2A70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2A74: 409A0018  bne cr6, 0x82da2a8c
	if !ctx.cr[6].eq {
	pc = 0x82DA2A8C; continue 'dispatch;
	}
	// 82DA2A78: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA2A7C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2A80: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA2A84: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DA2A88: 4BFB2841  bl 0x82d552c8
	ctx.lr = 0x82DA2A8C;
	sub_82D552C8(ctx, base);
	// 82DA2A8C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2A90: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DA2A94: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DA2A98: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DA2A9C: 4BFB27AD  bl 0x82d55248
	ctx.lr = 0x82DA2AA0;
	sub_82D55248(ctx, base);
	// 82DA2AA0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2AA4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA2AA8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2AAC: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2AB0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DA2AB4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA2AB8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2ABC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2AC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2AC4: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA2AC8: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2ACC: 40990020  ble cr6, 0x82da2aec
	if !ctx.cr[6].gt {
	pc = 0x82DA2AEC; continue 'dispatch;
	}
	// 82DA2AD0: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DA2AD4: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA2AD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA2ADC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA2AE0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA2AE4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA2AE8: 409AFFEC  bne cr6, 0x82da2ad4
	if !ctx.cr[6].eq {
	pc = 0x82DA2AD4; continue 'dispatch;
	}
	// 82DA2AEC: 3BDF008C  addi r30, r31, 0x8c
	ctx.r[30].s64 = ctx.r[31].s64 + 140;
	// 82DA2AF0: 3BBB008C  addi r29, r27, 0x8c
	ctx.r[29].s64 = ctx.r[27].s64 + 140;
	// 82DA2AF4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2AF8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2AFC: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA2B00: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA2B04: 40980050  bge cr6, 0x82da2b54
	if !ctx.cr[6].lt {
	pc = 0x82DA2B54; continue 'dispatch;
	}
	// 82DA2B08: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2B0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2B10: 409A0018  bne cr6, 0x82da2b28
	if !ctx.cr[6].eq {
	pc = 0x82DA2B28; continue 'dispatch;
	}
	// 82DA2B14: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA2B18: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2B1C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA2B20: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DA2B24: 4BFB27A5  bl 0x82d552c8
	ctx.lr = 0x82DA2B28;
	sub_82D552C8(ctx, base);
	// 82DA2B28: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2B2C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DA2B30: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DA2B34: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DA2B38: 4BFB2711  bl 0x82d55248
	ctx.lr = 0x82DA2B3C;
	sub_82D55248(ctx, base);
	// 82DA2B3C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2B40: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA2B44: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2B48: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2B4C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DA2B50: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA2B54: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2B58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2B5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2B60: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA2B64: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2B68: 40990020  ble cr6, 0x82da2b88
	if !ctx.cr[6].gt {
	pc = 0x82DA2B88; continue 'dispatch;
	}
	// 82DA2B6C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DA2B70: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA2B74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA2B78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA2B7C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA2B80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA2B84: 409AFFEC  bne cr6, 0x82da2b70
	if !ctx.cr[6].eq {
	pc = 0x82DA2B70; continue 'dispatch;
	}
	// 82DA2B88: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DA2B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2B90: 4BFDD679  bl 0x82d80208
	ctx.lr = 0x82DA2B94;
	sub_82D80208(ctx, base);
	// 82DA2B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2B98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DA2B9C: 4BF068B0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2BA0 size=256
    let mut pc: u32 = 0x82DA2BA0;
    'dispatch: loop {
        match pc {
            0x82DA2BA0 => {
    //   block [0x82DA2BA0..0x82DA2CA0)
	// 82DA2BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2BA4: 4BF06865  bl 0x82ca9408
	ctx.lr = 0x82DA2BA8;
	sub_82CA93D0(ctx, base);
	// 82DA2BA8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2BAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DA2BB0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DA2BB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2BB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA2BBC: 419A00DC  beq cr6, 0x82da2c98
	if ctx.cr[6].eq {
	pc = 0x82DA2C98; continue 'dispatch;
	}
	// 82DA2BC0: 4B4E6F29  bl 0x82289ae8
	ctx.lr = 0x82DA2BC4;
	sub_82289AE8(ctx, base);
	// 82DA2BC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DA2BC8: 409A00D0  bne cr6, 0x82da2c98
	if !ctx.cr[6].eq {
	pc = 0x82DA2C98; continue 'dispatch;
	}
	// 82DA2BCC: 3BFC0150  addi r31, r28, 0x150
	ctx.r[31].s64 = ctx.r[28].s64 + 336;
	// 82DA2BD0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2BD4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2BD8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA2BDC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DA2BE0: 409A0010  bne cr6, 0x82da2bf0
	if !ctx.cr[6].eq {
	pc = 0x82DA2BF0; continue 'dispatch;
	}
	// 82DA2BE4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82DA2BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2BEC: 4BFB43AD  bl 0x82d56f98
	ctx.lr = 0x82DA2BF0;
	sub_82D56F98(ctx, base);
	// 82DA2BF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2BF4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DA2BF8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2BFC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DA2C00: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82DA2C04: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DA2C08: 7FCB4214  add r30, r11, r8
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DA2C0C: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DA2C10: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2C14: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DA2C18: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DA2C1C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DA2C20: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DA2C24: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DA2C28: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DA2C2C: 4200FFF0  bdnz 0x82da2c1c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DA2C1C; continue 'dispatch;
	}
	// 82DA2C30: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DA2C34: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 82DA2C38: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DA2C40: 394B1C60  addi r10, r11, 0x1c60
	ctx.r[10].s64 = ctx.r[11].s64 + 7264;
	// 82DA2C44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DA2C48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DA2C4C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2C50: 914100B0  stw r10, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82DA2C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA2C58: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DA2C5C: 394B01A0  addi r10, r11, 0x1a0
	ctx.r[10].s64 = ctx.r[11].s64 + 416;
	// 82DA2C60: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA2C64: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA2C68: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DA2C6C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DA2C70: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DA2C74: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DA2C78: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DA2C7C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DA2C80: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DA2C84: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DA2C88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2C8C: 4E800421  bctrl
	ctx.lr = 0x82DA2C90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2C90: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA2C94: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DA2C98: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DA2C9C: 4BF067BC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2CA0 size=244
    let mut pc: u32 = 0x82DA2CA0;
    'dispatch: loop {
        match pc {
            0x82DA2CA0 => {
    //   block [0x82DA2CA0..0x82DA2D94)
	// 82DA2CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2CA4: 4BF06761  bl 0x82ca9404
	ctx.lr = 0x82DA2CA8;
	sub_82CA93D0(ctx, base);
	// 82DA2CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2CAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DA2CB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA2CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA2CB8: 388BFD20  addi r4, r11, -0x2e0
	ctx.r[4].s64 = ctx.r[11].s64 + -736;
	// 82DA2CBC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DA2CC0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2CC4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DA2CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2CCC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA2CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2CD4: 4E800421  bctrl
	ctx.lr = 0x82DA2CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2CD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DA2CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA2CE0: 4BFF6DA1  bl 0x82d99a80
	ctx.lr = 0x82DA2CE4;
	sub_82D99A80(ctx, base);
	// 82DA2CE4: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82DA2CE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2CEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2CF0: 409A0034  bne cr6, 0x82da2d24
	if !ctx.cr[6].eq {
	pc = 0x82DA2D24; continue 'dispatch;
	}
	// 82DA2CF4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2CF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA2CFC: 80FF0154  lwz r7, 0x154(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2D00: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DA2D04: 80DF0150  lwz r6, 0x150(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA2D08: 388AFD14  addi r4, r10, -0x2ec
	ctx.r[4].s64 = ctx.r[10].s64 + -748;
	// 82DA2D0C: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DA2D10: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DA2D14: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2D20: 4E800421  bctrl
	ctx.lr = 0x82DA2D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2D24: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2D28: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DA2D2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA2D30: 40990048  ble cr6, 0x82da2d78
	if !ctx.cr[6].gt {
	pc = 0x82DA2D78; continue 'dispatch;
	}
	// 82DA2D34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82DA2D38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DA2D3C: 3B6B4D6C  addi r27, r11, 0x4d6c
	ctx.r[27].s64 = ctx.r[11].s64 + 19820;
	// 82DA2D40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2D44: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DA2D48: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82DA2D4C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DA2D50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2D54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DA2D58: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DA2D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2D60: 4E800421  bctrl
	ctx.lr = 0x82DA2D64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2D64: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2D68: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DA2D6C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82DA2D70: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DA2D74: 4198FFCC  blt cr6, 0x82da2d40
	if ctx.cr[6].lt {
	pc = 0x82DA2D40; continue 'dispatch;
	}
	// 82DA2D78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2D80: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA2D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA2D88: 4E800421  bctrl
	ctx.lr = 0x82DA2D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DA2D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA2D90: 4BF066C4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA2D98 size=144
    let mut pc: u32 = 0x82DA2D98;
    'dispatch: loop {
        match pc {
            0x82DA2D98 => {
    //   block [0x82DA2D98..0x82DA2E28)
	// 82DA2D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA2DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA2DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA2DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA2DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA2DB0: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82DA2DB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA2DB8: 409A0050  bne cr6, 0x82da2e08
	if !ctx.cr[6].eq {
	pc = 0x82DA2E08; continue 'dispatch;
	}
	// 82DA2DBC: 3BFE0150  addi r31, r30, 0x150
	ctx.r[31].s64 = ctx.r[30].s64 + 336;
	// 82DA2DC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2DC4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA2DC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA2DCC: 409A0020  bne cr6, 0x82da2dec
	if !ctx.cr[6].eq {
	pc = 0x82DA2DEC; continue 'dispatch;
	}
	// 82DA2DD0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2DD4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA2DD8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA2DDC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA2DE0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA2DE4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DA2DE8: 4BFB24E1  bl 0x82d552c8
	ctx.lr = 0x82DA2DEC;
	sub_82D552C8(ctx, base);
	// 82DA2DEC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA2DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA2DF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA2DF8: 512AF880  rlwimi r10, r9, 0x1f, 2, 0
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[10].u64 & 0x0000000040000000);
	// 82DA2DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA2E00: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA2E04: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DA2E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA2E0C: 4BFDDCAD  bl 0x82d80ab8
	ctx.lr = 0x82DA2E10;
	sub_82D80AB8(ctx, base);
	// 82DA2E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA2E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA2E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA2E1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA2E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA2E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA2E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA2E28 size=492
    let mut pc: u32 = 0x82DA2E28;
    'dispatch: loop {
        match pc {
            0x82DA2E28 => {
    //   block [0x82DA2E28..0x82DA3014)
	// 82DA2E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA2E2C: 4BF065D1  bl 0x82ca93fc
	ctx.lr = 0x82DA2E30;
	sub_82CA93D0(ctx, base);
	// 82DA2E30: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3018 size=328
    let mut pc: u32 = 0x82DA3018;
    'dispatch: loop {
        match pc {
            0x82DA3018 => {
    //   block [0x82DA3018..0x82DA3160)
	// 82DA3018: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3160 size=16
    let mut pc: u32 = 0x82DA3160;
    'dispatch: loop {
        match pc {
            0x82DA3160 => {
    //   block [0x82DA3160..0x82DA3170)
	// 82DA3160: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DA3164: 388000AC  li r4, 0xac
	ctx.r[4].s64 = 172;
	// 82DA3168: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DA316C: 4BFF906C  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3170 size=20
    let mut pc: u32 = 0x82DA3170;
    'dispatch: loop {
        match pc {
            0x82DA3170 => {
    //   block [0x82DA3170..0x82DA3184)
	// 82DA3170: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DA3174: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82DA3178: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA317C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA3180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA3188 size=384
    let mut pc: u32 = 0x82DA3188;
    'dispatch: loop {
        match pc {
            0x82DA3188 => {
    //   block [0x82DA3188..0x82DA3308)
	// 82DA3188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA318C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA3194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA3198: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DA319C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA31A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DA31A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA31A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DA31AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DA31B0: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82DA31B4: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DA31B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA31BC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA31C0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA31C4: 4BFB46E5  bl 0x82d578a8
	ctx.lr = 0x82DA31C8;
	sub_82D578A8(ctx, base);
	// 82DA31C8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA31CC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DA31D0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DA31D4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DA31D8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DA31DC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA31E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA31E4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA31E8: 419A0024  beq cr6, 0x82da320c
	if ctx.cr[6].eq {
	pc = 0x82DA320C; continue 'dispatch;
	}
	// 82DA31EC: 389E0050  addi r4, r30, 0x50
	ctx.r[4].s64 = ctx.r[30].s64 + 80;
	// 82DA31F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA31F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DA31F8: 4BFB46B1  bl 0x82d578a8
	ctx.lr = 0x82DA31FC;
	sub_82D578A8(ctx, base);
	// 82DA31FC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA3204: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA3208: 409A0008  bne cr6, 0x82da3210
	if !ctx.cr[6].eq {
	pc = 0x82DA3210; continue 'dispatch;
	}
	// 82DA320C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA3210: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA3214: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA3218: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA321C: 419A001C  beq cr6, 0x82da3238
	if ctx.cr[6].eq {
	pc = 0x82DA3238; continue 'dispatch;
	}
	// 82DA3220: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DA3224: C1BE009C  lfs f13, 0x9c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3228: C00B144C  lfs f0, 0x144c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA322C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA3230: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DA3234: 419A0008  beq cr6, 0x82da323c
	if ctx.cr[6].eq {
	pc = 0x82DA323C; continue 'dispatch;
	}
	// 82DA3238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA323C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA3240: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA3244: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA3248: 419A001C  beq cr6, 0x82da3264
	if ctx.cr[6].eq {
	pc = 0x82DA3264; continue 'dispatch;
	}
	// 82DA324C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA3250: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3254: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3258: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA325C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DA3260: 40980008  bge cr6, 0x82da3268
	if !ctx.cr[6].lt {
	pc = 0x82DA3268; continue 'dispatch;
	}
	// 82DA3264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA3268: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA326C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA3270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA3274: 419A001C  beq cr6, 0x82da3290
	if ctx.cr[6].eq {
	pc = 0x82DA3290; continue 'dispatch;
	}
	// 82DA3278: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA327C: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3280: C00B0B40  lfs f0, 0xb40(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3284: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA3288: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DA328C: 41980008  blt cr6, 0x82da3294
	if ctx.cr[6].lt {
	pc = 0x82DA3294; continue 'dispatch;
	}
	// 82DA3290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA3294: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA3298: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA329C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA32A0: 419A0018  beq cr6, 0x82da32b8
	if ctx.cr[6].eq {
	pc = 0x82DA32B8; continue 'dispatch;
	}
	// 82DA32A4: C01E00B0  lfs f0, 0xb0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA32A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA32AC: C1BE00B4  lfs f13, 0xb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA32B0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DA32B4: 40990008  ble cr6, 0x82da32bc
	if !ctx.cr[6].gt {
	pc = 0x82DA32BC; continue 'dispatch;
	}
	// 82DA32B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA32BC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82DA32C0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA32C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA32C8: 419A0018  beq cr6, 0x82da32e0
	if ctx.cr[6].eq {
	pc = 0x82DA32E0; continue 'dispatch;
	}
	// 82DA32CC: C01E0088  lfs f0, 0x88(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA32D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA32D4: C1BE008C  lfs f13, 0x8c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA32D8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DA32DC: 40990008  ble cr6, 0x82da32e4
	if !ctx.cr[6].gt {
	pc = 0x82DA32E4; continue 'dispatch;
	}
	// 82DA32E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA32E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DA32E8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA32EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA32F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA32F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA32F8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DA32FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA3300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA3304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3308 size=40
    let mut pc: u32 = 0x82DA3308;
    'dispatch: loop {
        match pc {
            0x82DA3308 => {
    //   block [0x82DA3308..0x82DA3330)
	// 82DA3308: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DA330C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DA3310: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3330 size=40
    let mut pc: u32 = 0x82DA3330;
    'dispatch: loop {
        match pc {
            0x82DA3330 => {
    //   block [0x82DA3330..0x82DA3358)
	// 82DA3330: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82DA3334: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DA3338: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3358 size=8
    let mut pc: u32 = 0x82DA3358;
    'dispatch: loop {
        match pc {
            0x82DA3358 => {
    //   block [0x82DA3358..0x82DA3360)
	// 82DA3358: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 82DA335C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3360 size=312
    let mut pc: u32 = 0x82DA3360;
    'dispatch: loop {
        match pc {
            0x82DA3360 => {
    //   block [0x82DA3360..0x82DA3498)
	// 82DA3360: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DA3364: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DA3368: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82DA336C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DA3370: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82DA3374: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82DA3378: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3498 size=336
    let mut pc: u32 = 0x82DA3498;
    'dispatch: loop {
        match pc {
            0x82DA3498 => {
    //   block [0x82DA3498..0x82DA35E8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA35E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA35E8 size=72
    let mut pc: u32 = 0x82DA35E8;
    'dispatch: loop {
        match pc {
            0x82DA35E8 => {
    //   block [0x82DA35E8..0x82DA3630)
	// 82DA35E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA35EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA35F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DA35F4: 396BB8F0  addi r11, r11, -0x4710
	ctx.r[11].s64 = ctx.r[11].s64 + -18192;
	// 82DA35F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82DA35FC: C00AFC78  lfs f0, -0x388(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-904 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3600: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA3604: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DA3608: C1AA0A48  lfs f13, 0xa48(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2632 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA360C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DA3610: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82DA3614: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA3618: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DA361C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA3620: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA3624: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82DA3628: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82DA362C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA3630 size=180
    let mut pc: u32 = 0x82DA3630;
    'dispatch: loop {
        match pc {
            0x82DA3630 => {
    //   block [0x82DA3630..0x82DA36E4)
	// 82DA3630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA3634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA363C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA3640: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3644: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DA3648: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82DA364C: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DA3650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA3654: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA3658: 4BFB1BF1  bl 0x82d55248
	ctx.lr = 0x82DA365C;
	sub_82D55248(ctx, base);
	// 82DA365C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DA3660: 39000028  li r8, 0x28
	ctx.r[8].s64 = 40;
	// 82DA3664: 396BAA34  addi r11, r11, -0x55cc
	ctx.r[11].s64 = ctx.r[11].s64 + -21964;
	// 82DA3668: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DA366C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DA3670: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DA3674: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82DA3678: 394AAA48  addi r10, r10, -0x55b8
	ctx.r[10].s64 = ctx.r[10].s64 + -21944;
	// 82DA367C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA3680: 3929B8F0  addi r9, r9, -0x4710
	ctx.r[9].s64 = ctx.r[9].s64 + -18192;
	// 82DA3684: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DA3688: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA368C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA3690: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82DA3694: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3698: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DA369C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA36A0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DA36A4: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DA36A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DA36AC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DA36B0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA36B4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DA36B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DA36BC: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DA36C0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DA36C4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DA36C8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DA36CC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DA36D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DA36D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA36D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA36DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA36E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA36E8 size=64
    let mut pc: u32 = 0x82DA36E8;
    'dispatch: loop {
        match pc {
            0x82DA36E8 => {
    //   block [0x82DA36E8..0x82DA3728)
	// 82DA36E8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA36EC: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA36F0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3728 size=64
    let mut pc: u32 = 0x82DA3728;
    'dispatch: loop {
        match pc {
            0x82DA3728 => {
    //   block [0x82DA3728..0x82DA3768)
	// 82DA3728: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA372C: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3730: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3768 size=72
    let mut pc: u32 = 0x82DA3768;
    'dispatch: loop {
        match pc {
            0x82DA3768 => {
    //   block [0x82DA3768..0x82DA37B0)
	// 82DA3768: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA376C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA3770: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3774: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DA3778: C1840028  lfs f12, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA377C: FC0B036E  fsel f0, f11, f13, f0
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82DA3780: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DA3784: FC0D602E  fsel f0, f13, f0, f12
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[12].f64 };
	// 82DA3788: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA378C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DA3790: 40990010  ble cr6, 0x82da37a0
	if !ctx.cr[6].gt {
	pc = 0x82DA37A0; continue 'dispatch;
	}
	// 82DA3794: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA3798: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA379C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DA37A0: D1A300C0  stfs f13, 0xc0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82DA37A4: D1A300C4  stfs f13, 0xc4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82DA37A8: D1A300C8  stfs f13, 0xc8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82DA37AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA37B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA37B0 size=44
    let mut pc: u32 = 0x82DA37B0;
    'dispatch: loop {
        match pc {
            0x82DA37B0 => {
    //   block [0x82DA37B0..0x82DA37DC)
	// 82DA37B0: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA37B4: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA37B8: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DA37BC: C1840028  lfs f12, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA37C0: FC0B682E  fsel f0, f11, f0, f13
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82DA37C4: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DA37C8: FC0D032E  fsel f0, f13, f12, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82DA37CC: D00300C0  stfs f0, 0xc0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82DA37D0: D00300C4  stfs f0, 0xc4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82DA37D4: D00300C8  stfs f0, 0xc8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82DA37D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA37E0 size=52
    let mut pc: u32 = 0x82DA37E0;
    'dispatch: loop {
        match pc {
            0x82DA37E0 => {
    //   block [0x82DA37E0..0x82DA3814)
	// 82DA37E0: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA37E4: C00300C0  lfs f0, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA37E8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3818 size=52
    let mut pc: u32 = 0x82DA3818;
    'dispatch: loop {
        match pc {
            0x82DA3818 => {
    //   block [0x82DA3818..0x82DA384C)
	// 82DA3818: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA381C: C00300C0  lfs f0, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3820: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3850 size=104
    let mut pc: u32 = 0x82DA3850;
    'dispatch: loop {
        match pc {
            0x82DA3850 => {
    //   block [0x82DA3850..0x82DA38B8)
	// 82DA3850: 396300C0  addi r11, r3, 0xc0
	ctx.r[11].s64 = ctx.r[3].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA38B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA38B8 size=32
    let mut pc: u32 = 0x82DA38B8;
    'dispatch: loop {
        match pc {
            0x82DA38B8 => {
    //   block [0x82DA38B8..0x82DA38D8)
	// 82DA38B8: 396300E0  addi r11, r3, 0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA38D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA38D8 size=56
    let mut pc: u32 = 0x82DA38D8;
    'dispatch: loop {
        match pc {
            0x82DA38D8 => {
    //   block [0x82DA38D8..0x82DA3910)
	// 82DA38D8: 3941001C  addi r10, r1, 0x1c
	ctx.r[10].s64 = ctx.r[1].s64 + 28;
	// 82DA38DC: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA3910 size=88
    let mut pc: u32 = 0x82DA3910;
    'dispatch: loop {
        match pc {
            0x82DA3910 => {
    //   block [0x82DA3910..0x82DA3968)
	// 82DA3910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA3914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA391C: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 82DA3920: D021008C  stfs f1, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82DA3924: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DA3928: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DA392C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA3968 size=80
    let mut pc: u32 = 0x82DA3968;
    'dispatch: loop {
        match pc {
            0x82DA3968 => {
    //   block [0x82DA3968..0x82DA39B8)
	// 82DA3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA3974: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 82DA3978: D021008C  stfs f1, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA39B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA39B8 size=72
    let mut pc: u32 = 0x82DA39B8;
    'dispatch: loop {
        match pc {
            0x82DA39B8 => {
    //   block [0x82DA39B8..0x82DA3A00)
	// 82DA39B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA39BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA39C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA39C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA39C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA39CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DA39D0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DA39D4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82DA39D8: 4BFB3DA9  bl 0x82d57780
	ctx.lr = 0x82DA39DC;
	sub_82D57780(ctx, base);
	// 82DA39DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DA39E0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DA39E4: 4BFB3D9D  bl 0x82d57780
	ctx.lr = 0x82DA39E8;
	sub_82D57780(ctx, base);
	// 82DA39E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA39EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA39F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA39F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA39F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA39FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3A00 size=16
    let mut pc: u32 = 0x82DA3A00;
    'dispatch: loop {
        match pc {
            0x82DA3A00 => {
    //   block [0x82DA3A00..0x82DA3A10)
	// 82DA3A00: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DA3A04: 38800074  li r4, 0x74
	ctx.r[4].s64 = 116;
	// 82DA3A08: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DA3A0C: 4BFF87CC  b 0x82d9c1d8
	sub_82D9C1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3A10 size=32
    let mut pc: u32 = 0x82DA3A10;
    'dispatch: loop {
        match pc {
            0x82DA3A10 => {
    //   block [0x82DA3A10..0x82DA3A30)
	// 82DA3A10: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82DA3A14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA3A18: 419A0018  beq cr6, 0x82da3a30
	if ctx.cr[6].eq {
		sub_82DA3A30(ctx, base);
		return;
	}
	// 82DA3A1C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DA3A20: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82DA3A24: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA3A28: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DA3A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3A30 size=16
    let mut pc: u32 = 0x82DA3A30;
    'dispatch: loop {
        match pc {
            0x82DA3A30 => {
    //   block [0x82DA3A30..0x82DA3A40)
	// 82DA3A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA3A34: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DA3A38: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DA3A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA3A40 size=140
    let mut pc: u32 = 0x82DA3A40;
    'dispatch: loop {
        match pc {
            0x82DA3A40 => {
    //   block [0x82DA3A40..0x82DA3ACC)
	// 82DA3A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA3A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA3A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA3A50: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DA3A54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA3A58: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DA3A5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA3A60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DA3A64: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DA3A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DA3A6C: C3EBBE14  lfs f31, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DA3A70: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA3A74: 4BFB3E35  bl 0x82d578a8
	ctx.lr = 0x82DA3A78;
	sub_82D578A8(ctx, base);
	// 82DA3A78: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA3A80: 419A0024  beq cr6, 0x82da3aa4
	if ctx.cr[6].eq {
	pc = 0x82DA3AA4; continue 'dispatch;
	}
	// 82DA3A84: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82DA3A88: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DA3A8C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DA3A90: 4BFB3E19  bl 0x82d578a8
	ctx.lr = 0x82DA3A94;
	sub_82D578A8(ctx, base);
	// 82DA3A94: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3A98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DA3A9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DA3AA0: 409A0008  bne cr6, 0x82da3aa8
	if !ctx.cr[6].eq {
	pc = 0x82DA3AA8; continue 'dispatch;
	}
	// 82DA3AA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DA3AA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DA3AAC: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DA3AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DA3AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA3AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA3ABC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DA3AC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA3AC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA3AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3AD0 size=8
    let mut pc: u32 = 0x82DA3AD0;
    'dispatch: loop {
        match pc {
            0x82DA3AD0 => {
    //   block [0x82DA3AD0..0x82DA3AD8)
	// 82DA3AD0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DA3AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3AD8 size=164
    let mut pc: u32 = 0x82DA3AD8;
    'dispatch: loop {
        match pc {
            0x82DA3AD8 => {
    //   block [0x82DA3AD8..0x82DA3B7C)
	// 82DA3AD8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DA3ADC: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 82DA3AE0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DA3AE4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DA3AE8: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82DA3AEC: 3929BA38  addi r9, r9, -0x45c8
	ctx.r[9].s64 = ctx.r[9].s64 + -17864;
	// 82DA3AF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA3B80 size=92
    let mut pc: u32 = 0x82DA3B80;
    'dispatch: loop {
        match pc {
            0x82DA3B80 => {
    //   block [0x82DA3B80..0x82DA3BDC)
	// 82DA3B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA3B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA3B8C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DA3B90: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DA3B94: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DA3B98: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3BE0 size=4
    let mut pc: u32 = 0x82DA3BE0;
    'dispatch: loop {
        match pc {
            0x82DA3BE0 => {
    //   block [0x82DA3BE0..0x82DA3BE4)
	// 82DA3BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3BE8 size=8
    let mut pc: u32 = 0x82DA3BE8;
    'dispatch: loop {
        match pc {
            0x82DA3BE8 => {
    //   block [0x82DA3BE8..0x82DA3BF0)
	// 82DA3BE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DA3BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3BF0 size=20
    let mut pc: u32 = 0x82DA3BF0;
    'dispatch: loop {
        match pc {
            0x82DA3BF0 => {
    //   block [0x82DA3BF0..0x82DA3C04)
	// 82DA3BF0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA3BF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3BF8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DA3BFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DA3C00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DA3C08 size=220
    let mut pc: u32 = 0x82DA3C08;
    'dispatch: loop {
        match pc {
            0x82DA3C08 => {
    //   block [0x82DA3C08..0x82DA3CE4)
	// 82DA3C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA3C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA3C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DA3C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DA3C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA3C1C: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DA3C20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DA3C24: 3BCA002C  addi r30, r10, 0x2c
	ctx.r[30].s64 = ctx.r[10].s64 + 44;
	// 82DA3C28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA3C2C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA3C30: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DA3C34: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DA3C38: 40980060  bge cr6, 0x82da3c98
	if !ctx.cr[6].lt {
	pc = 0x82DA3C98; continue 'dispatch;
	}
	// 82DA3C3C: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA3C40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DA3C44: 409A0020  bne cr6, 0x82da3c64
	if !ctx.cr[6].eq {
	pc = 0x82DA3C64; continue 'dispatch;
	}
	// 82DA3C48: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3C4C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DA3C50: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DA3C54: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3C58: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DA3C5C: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA3C60: 4BFB1669  bl 0x82d552c8
	ctx.lr = 0x82DA3C64;
	sub_82D552C8(ctx, base);
	// 82DA3C64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3C68: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DA3C6C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA3C70: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DA3C74: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DA3C78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA3C7C: 4BFB15CD  bl 0x82d55248
	ctx.lr = 0x82DA3C80;
	sub_82D55248(ctx, base);
	// 82DA3C80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DA3C84: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DA3C88: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA3C8C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DA3C90: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DA3C94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DA3C98: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DA3C9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3CA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DA3CA4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DA3CA8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DA3CAC: 40990020  ble cr6, 0x82da3ccc
	if !ctx.cr[6].gt {
	pc = 0x82DA3CCC; continue 'dispatch;
	}
	// 82DA3CB0: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DA3CB4: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DA3CB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DA3CBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DA3CC0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DA3CC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DA3CC8: 409AFFEC  bne cr6, 0x82da3cb4
	if !ctx.cr[6].eq {
	pc = 0x82DA3CB4; continue 'dispatch;
	}
	// 82DA3CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DA3CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DA3CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DA3CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DA3CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DA3CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3CE8 size=8
    let mut pc: u32 = 0x82DA3CE8;
    'dispatch: loop {
        match pc {
            0x82DA3CE8 => {
    //   block [0x82DA3CE8..0x82DA3CF0)
	// 82DA3CE8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DA3CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3CF0 size=164
    let mut pc: u32 = 0x82DA3CF0;
    'dispatch: loop {
        match pc {
            0x82DA3CF0 => {
    //   block [0x82DA3CF0..0x82DA3D94)
	// 82DA3CF0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DA3CF4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DA3CF8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DA3CFC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DA3D00: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82DA3D04: C188FD40  lfs f12, -0x2c0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-704 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA3D08: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DA3D0C: C1A90BE8  lfs f13, 0xbe8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3048 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3D10: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DA3D14: C00A0EE0  lfs f0, 0xee0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3D18: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82DA3D1C: 3929C234  addi r9, r9, -0x3dcc
	ctx.r[9].s64 = ctx.r[9].s64 + -15820;
	// 82DA3D20: D0030050  stfs f0, 0x50(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DA3D24: D1A30054  stfs f13, 0x54(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DA3D28: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DA3D2C: C1680BF8  lfs f11, 0xbf8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3064 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DA3D30: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DA3D34: D1830058  stfs f12, 0x58(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DA3D38: D163005C  stfs f11, 0x5c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DA3D3C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DA3D40: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82DA3D44: C1480A94  lfs f10, 0xa94(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2708 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DA3D48: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DA3D4C: D1430060  stfs f10, 0x60(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DA3D50: C0080C64  lfs f0, 0xc64(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3D54: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DA3D58: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3D98 size=32
    let mut pc: u32 = 0x82DA3D98;
    'dispatch: loop {
        match pc {
            0x82DA3D98 => {
    //   block [0x82DA3D98..0x82DA3DB8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3DB8 size=136
    let mut pc: u32 = 0x82DA3DB8;
    'dispatch: loop {
        match pc {
            0x82DA3DB8 => {
    //   block [0x82DA3DB8..0x82DA3E40)
	// 82DA3DB8: 396500E0  addi r11, r5, 0xe0
	ctx.r[11].s64 = ctx.r[5].s64 + 224;
	// 82DA3DBC: 39240010  addi r9, r4, 0x10
	ctx.r[9].s64 = ctx.r[4].s64 + 16;
	// 82DA3DC0: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DA3DC4: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82DA3DC8: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 82DA3DCC: C1AB00A0  lfs f13, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3DD0: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3E40 size=24
    let mut pc: u32 = 0x82DA3E40;
    'dispatch: loop {
        match pc {
            0x82DA3E40 => {
    //   block [0x82DA3E40..0x82DA3E58)
	// 82DA3E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3E58 size=136
    let mut pc: u32 = 0x82DA3E58;
    'dispatch: loop {
        match pc {
            0x82DA3E58 => {
    //   block [0x82DA3E58..0x82DA3EE0)
	// 82DA3E58: 396500E0  addi r11, r5, 0xe0
	ctx.r[11].s64 = ctx.r[5].s64 + 224;
	// 82DA3E5C: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82DA3E60: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DA3E64: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82DA3E68: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 82DA3E6C: C1AB00A0  lfs f13, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3E70: 39040040  addi r8, r4, 0x40
	ctx.r[8].s64 = ctx.r[4].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA3EE0 size=24
    let mut pc: u32 = 0x82DA3EE0;
    'dispatch: loop {
        match pc {
            0x82DA3EE0 => {
    //   block [0x82DA3EE0..0x82DA3EF8)
	// 82DA3EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3EF8 size=64
    let mut pc: u32 = 0x82DA3EF8;
    'dispatch: loop {
        match pc {
            0x82DA3EF8 => {
    //   block [0x82DA3EF8..0x82DA3F38)
	// 82DA3EF8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA3EFC: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3F00: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3F38 size=64
    let mut pc: u32 = 0x82DA3F38;
    'dispatch: loop {
        match pc {
            0x82DA3F38 => {
    //   block [0x82DA3F38..0x82DA3F78)
	// 82DA3F38: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA3F3C: C1A300C0  lfs f13, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3F40: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3F78 size=72
    let mut pc: u32 = 0x82DA3F78;
    'dispatch: loop {
        match pc {
            0x82DA3F78 => {
    //   block [0x82DA3F78..0x82DA3FC0)
	// 82DA3F78: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3F7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA3F80: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3F84: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DA3F88: C1840028  lfs f12, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA3F8C: FC0B036E  fsel f0, f11, f13, f0
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82DA3F90: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DA3F94: FC0D602E  fsel f0, f13, f0, f12
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[12].f64 };
	// 82DA3F98: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3F9C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DA3FA0: 40990010  ble cr6, 0x82da3fb0
	if !ctx.cr[6].gt {
	pc = 0x82DA3FB0; continue 'dispatch;
	}
	// 82DA3FA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DA3FA8: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3FAC: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DA3FB0: D1A300C0  stfs f13, 0xc0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82DA3FB4: D1A300C4  stfs f13, 0xc4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82DA3FB8: D1A300C8  stfs f13, 0xc8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82DA3FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3FC0 size=44
    let mut pc: u32 = 0x82DA3FC0;
    'dispatch: loop {
        match pc {
            0x82DA3FC0 => {
    //   block [0x82DA3FC0..0x82DA3FEC)
	// 82DA3FC0: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DA3FC4: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3FC8: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DA3FCC: C1840028  lfs f12, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DA3FD0: FC0B682E  fsel f0, f11, f0, f13
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82DA3FD4: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DA3FD8: FC0D032E  fsel f0, f13, f12, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82DA3FDC: D00300C0  stfs f0, 0xc0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82DA3FE0: D00300C4  stfs f0, 0xc4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82DA3FE4: D00300C8  stfs f0, 0xc8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82DA3FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA3FF0 size=52
    let mut pc: u32 = 0x82DA3FF0;
    'dispatch: loop {
        match pc {
            0x82DA3FF0 => {
    //   block [0x82DA3FF0..0x82DA4024)
	// 82DA3FF0: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA3FF4: C00300C0  lfs f0, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA3FF8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA4028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA4028 size=52
    let mut pc: u32 = 0x82DA4028;
    'dispatch: loop {
        match pc {
            0x82DA4028 => {
    //   block [0x82DA4028..0x82DA405C)
	// 82DA4028: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DA402C: C00300C0  lfs f0, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DA4030: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA4060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA4060 size=104
    let mut pc: u32 = 0x82DA4060;
    'dispatch: loop {
        match pc {
            0x82DA4060 => {
    //   block [0x82DA4060..0x82DA40C8)
	// 82DA4060: 396300C0  addi r11, r3, 0xc0
	ctx.r[11].s64 = ctx.r[3].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DA40C8 size=32
    let mut pc: u32 = 0x82DA40C8;
    'dispatch: loop {
        match pc {
            0x82DA40C8 => {
    //   block [0x82DA40C8..0x82DA40E8)
	// 82DA40C8: 396300E0  addi r11, r3, 0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DA40E8 size=56
    let mut pc: u32 = 0x82DA40E8;
    'dispatch: loop {
        match pc {
            0x82DA40E8 => {
    //   block [0x82DA40E8..0x82DA4120)
	// 82DA40E8: 3941001C  addi r10, r1, 0x1c
	ctx.r[10].s64 = ctx.r[1].s64 + 28;
	// 82DA40EC: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DA4120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DA4120 size=88
    let mut pc: u32 = 0x82DA4120;
    'dispatch: loop {
        match pc {
            0x82DA4120 => {
    //   block [0x82DA4120..0x82DA4178)
	// 82DA4120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DA4124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DA4128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DA412C: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 82DA4130: D021008C  stfs f1, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82DA4134: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DA4138: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DA413C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


