pub fn sub_82427E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427E78 size=100
    let mut pc: u32 = 0x82427E78;
    'dispatch: loop {
        match pc {
            0x82427E78 => {
    //   block [0x82427E78..0x82427EDC)
	// 82427E78: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 82427E7C: 41980060  blt cr6, 0x82427edc
	if ctx.cr[6].lt {
		sub_82427EDC(ctx, base);
		return;
	}
	// 82427E80: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427E84: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 82427E88: 409A0054  bne cr6, 0x82427edc
	if !ctx.cr[6].eq {
		sub_82427EDC(ctx, base);
		return;
	}
	// 82427E8C: A9430002  lha r10, 2(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82427E90: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82427E94: 41980048  blt cr6, 0x82427edc
	if ctx.cr[6].lt {
		sub_82427EDC(ctx, base);
		return;
	}
	// 82427E98: 89630012  lbz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82427E9C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82427EA0: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82427EA4: 41980040  blt cr6, 0x82427ee4
	if ctx.cr[6].lt {
		sub_82427EE4(ctx, base);
		return;
	}
	// 82427EA8: 2F040020  cmpwi cr6, r4, 0x20
	ctx.cr[6].compare_i32(ctx.r[4].s32, 32, &mut ctx.xer);
	// 82427EAC: 41980030  blt cr6, 0x82427edc
	if ctx.cr[6].lt {
		sub_82427EDC(ctx, base);
		return;
	}
	// 82427EB0: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 82427EB4: 41980028  blt cr6, 0x82427edc
	if ctx.cr[6].lt {
		sub_82427EDC(ctx, base);
		return;
	}
	// 82427EB8: A1630018  lhz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82427EBC: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427EC0: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82427EC4: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427EC8: A163001C  lhz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82427ECC: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82427ED0: A163001E  lhz r11, 0x1e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 82427ED4: B1660002  sth r11, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82427ED8: 48000020  b 0x82427ef8
	sub_82427EE4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427EDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427EDC size=8
    let mut pc: u32 = 0x82427EDC;
    'dispatch: loop {
        match pc {
            0x82427EDC => {
    //   block [0x82427EDC..0x82427EE4)
	// 82427EDC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427EE4 size=28
    let mut pc: u32 = 0x82427EE4;
    'dispatch: loop {
        match pc {
            0x82427EE4 => {
    //   block [0x82427EE4..0x82427F00)
	// 82427EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82427EE8: B1660002  sth r11, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82427EEC: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82427EF0: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427EF4: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427EF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427F00 size=236
    let mut pc: u32 = 0x82427F00;
    'dispatch: loop {
        match pc {
            0x82427F00 => {
    //   block [0x82427F00..0x82427FEC)
	// 82427F00: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82427F04: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82427F08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82427F0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427F10: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 82427F14: B0660000  sth r3, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82427F18: 4198003C  blt cr6, 0x82427f54
	if ctx.cr[6].lt {
	pc = 0x82427F54; continue 'dispatch;
	}
	// 82427F1C: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427F20: 2B038000  cmplwi cr6, r3, 0x8000
	ctx.cr[6].compare_u32(ctx.r[3].u32, 32768 as u32, &mut ctx.xer);
	// 82427F24: 409A0070  bne cr6, 0x82427f94
	if !ctx.cr[6].eq {
	pc = 0x82427F94; continue 'dispatch;
	}
	// 82427F28: ABCB0002  lha r30, 2(r11)
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82427F2C: 2F1E0010  cmpwi cr6, r30, 0x10
	ctx.cr[6].compare_i32(ctx.r[30].s32, 16, &mut ctx.xer);
	// 82427F30: 41980024  blt cr6, 0x82427f54
	if ctx.cr[6].lt {
	pc = 0x82427F54; continue 'dispatch;
	}
	// 82427F34: 886B0012  lbz r3, 0x12(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82427F38: 547F063E  clrlwi r31, r3, 0x18
	ctx.r[31].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82427F3C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82427F40: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 82427F44: 409A0008  bne cr6, 0x82427f4c
	if !ctx.cr[6].eq {
	pc = 0x82427F4C; continue 'dispatch;
	}
	// 82427F48: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82427F4C: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82427F50: 4098000C  bge cr6, 0x82427f5c
	if !ctx.cr[6].lt {
	pc = 0x82427F5C; continue 'dispatch;
	}
	// 82427F54: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427F58: 48000088  b 0x82427fe0
	pc = 0x82427FE0; continue 'dispatch;
	// 82427F5C: 3883FFFC  addi r4, r3, -4
	ctx.r[4].s64 = ctx.r[3].s64 + -4;
	// 82427F60: 7F1E2000  cmpw cr6, r30, r4
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82427F64: 4198FFF0  blt cr6, 0x82427f54
	if ctx.cr[6].lt {
	pc = 0x82427F54; continue 'dispatch;
	}
	// 82427F68: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82427F6C: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 82427F70: 409A0008  bne cr6, 0x82427f78
	if !ctx.cr[6].eq {
	pc = 0x82427F78; continue 'dispatch;
	}
	// 82427F74: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82427F78: 7C645AAE  lhax r3, r4, r11
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as i16) as i64;
	// 82427F7C: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 82427F80: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82427F84: 7CA45A2E  lhzx r5, r4, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82427F88: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82427F8C: B0A60000  sth r5, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 82427F90: 419A000C  beq cr6, 0x82427f9c
	if ctx.cr[6].eq {
	pc = 0x82427F9C; continue 'dispatch;
	}
	// 82427F94: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82427F98: 48000048  b 0x82427fe0
	pc = 0x82427FE0; continue 'dispatch;
	// 82427F9C: 38C40004  addi r6, r4, 4
	ctx.r[6].s64 = ctx.r[4].s64 + 4;
	// 82427FA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427FA4: 7CA65A2E  lhzx r5, r6, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82427FA8: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 82427FAC: B0A70000  sth r5, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 82427FB0: 38E60004  addi r7, r6, 4
	ctx.r[7].s64 = ctx.r[6].s64 + 4;
	// 82427FB4: 7CA6582E  lwzx r5, r6, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82427FB8: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82427FBC: 39070004  addi r8, r7, 4
	ctx.r[8].s64 = ctx.r[7].s64 + 4;
	// 82427FC0: 7CC7582E  lwzx r6, r7, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82427FC4: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82427FC8: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82427FCC: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82427FD0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82427FD4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82427FD8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82427FDC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82427FE0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427FE4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82427FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427FF0 size=308
    let mut pc: u32 = 0x82427FF0;
    'dispatch: loop {
        match pc {
            0x82427FF0 => {
    //   block [0x82427FF0..0x82428124)
	// 82427FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427FF4: 4810D0C1  bl 0x825350b4
	ctx.lr = 0x82427FF8;
	sub_82535080(ctx, base);
	// 82427FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427FFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82428000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82428004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428008: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8242800C: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82428010: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 82428014: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82428018: 4198003C  blt cr6, 0x82428054
	if ctx.cr[6].lt {
	pc = 0x82428054; continue 'dispatch;
	}
	// 8242801C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428020: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 82428024: 409A00AC  bne cr6, 0x824280d0
	if !ctx.cr[6].eq {
	pc = 0x824280D0; continue 'dispatch;
	}
	// 82428028: A93F0002  lha r9, 2(r31)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 8242802C: 2F090010  cmpwi cr6, r9, 0x10
	ctx.cr[6].compare_i32(ctx.r[9].s32, 16, &mut ctx.xer);
	// 82428030: 41980024  blt cr6, 0x82428054
	if ctx.cr[6].lt {
	pc = 0x82428054; continue 'dispatch;
	}
	// 82428034: 897F0012  lbz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82428038: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8242803C: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 82428040: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82428044: 409A0008  bne cr6, 0x8242804c
	if !ctx.cr[6].eq {
	pc = 0x8242804C; continue 'dispatch;
	}
	// 82428048: 39600048  li r11, 0x48
	ctx.r[11].s64 = 72;
	// 8242804C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82428050: 4098000C  bge cr6, 0x8242805c
	if !ctx.cr[6].lt {
	pc = 0x8242805C; continue 'dispatch;
	}
	// 82428054: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82428058: 480000C4  b 0x8242811c
	pc = 0x8242811C; continue 'dispatch;
	// 8242805C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82428060: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82428064: 4198FFF0  blt cr6, 0x82428054
	if ctx.cr[6].lt {
	pc = 0x82428054; continue 'dispatch;
	}
	// 82428068: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8242806C: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82428070: 409A0008  bne cr6, 0x82428078
	if !ctx.cr[6].eq {
	pc = 0x82428078; continue 'dispatch;
	}
	// 82428074: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82428078: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242807C: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82428080: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82428084: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428088: 41820008  beq 0x82428090
	if ctx.cr[0].eq {
	pc = 0x82428090; continue 'dispatch;
	}
	// 8242808C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82428090: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82428094: 7D2BF8AE  lbzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82428098: 3D004149  lis r8, 0x4149
	ctx.r[8].s64 = 1095303168;
	// 8242809C: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 824280A0: 61084E46  ori r8, r8, 0x4e46
	ctx.r[8].u64 = ctx.r[8].u64 | 20038;
	// 824280A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824280A8: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824280AC: 88AA0002  lbz r5, 2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 824280B0: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 824280B4: 894A0003  lbz r10, 3(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824280B8: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824280BC: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 824280C0: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824280C4: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 824280C8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824280CC: 419A000C  beq cr6, 0x824280d8
	if ctx.cr[6].eq {
	pc = 0x824280D8; continue 'dispatch;
	}
	// 824280D0: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 824280D4: 48000048  b 0x8242811c
	pc = 0x8242811C; continue 'dispatch;
	// 824280D8: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824280DC: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 824280E0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824280E4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 824280E8: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 824280EC: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824280F0: 4810CA61  bl 0x82534b50
	ctx.lr = 0x824280F4;
	sub_82534B50(ctx, base);
	// 824280F4: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 824280F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824280FC: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82428100: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82428104: B15B0000  sth r10, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82428108: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242810C: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82428110: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82428114: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428118: B17D0002  sth r11, 2(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8242811C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82428120: 4810CFE4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82428128 size=16
    let mut pc: u32 = 0x82428128;
    'dispatch: loop {
        match pc {
            0x82428128 => {
    //   block [0x82428128..0x82428138)
	// 82428128: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 8242812C: 4098000C  bge cr6, 0x82428138
	if !ctx.cr[6].lt {
		sub_82428138(ctx, base);
		return;
	}
	// 82428130: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82428134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82428138 size=20
    let mut pc: u32 = 0x82428138;
    'dispatch: loop {
        match pc {
            0x82428138 => {
    //   block [0x82428138..0x8242814C)
	// 82428138: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242813C: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82428140: 419A000C  beq cr6, 0x8242814c
	if ctx.cr[6].eq {
		sub_8242814C(ctx, base);
		return;
	}
	// 82428144: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82428148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242814C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242814C size=20
    let mut pc: u32 = 0x8242814C;
    'dispatch: loop {
        match pc {
            0x8242814C => {
    //   block [0x8242814C..0x82428160)
	// 8242814C: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82428154: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82428158: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428160 size=372
    let mut pc: u32 = 0x82428160;
    'dispatch: loop {
        match pc {
            0x82428160 => {
    //   block [0x82428160..0x824282D4)
	// 82428160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82428164: 4810CF4D  bl 0x825350b0
	ctx.lr = 0x82428168;
	sub_82535080(ctx, base);
	// 82428168: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242816C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428170: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428174: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82428178: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242817C: 4BFFE685  bl 0x82426800
	ctx.lr = 0x82428180;
	sub_82426800(ctx, base);
	// 82428180: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82428184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428188: 4BFFE681  bl 0x82426808
	ctx.lr = 0x8242818C;
	sub_82426808(ctx, base);
	// 8242818C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82428190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428194: 4BFFE68D  bl 0x82426820
	ctx.lr = 0x82428198;
	sub_82426820(ctx, base);
	// 82428198: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242819C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824281A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824281A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824281A8: 419A000C  beq cr6, 0x824281b4
	if ctx.cr[6].eq {
	pc = 0x824281B4; continue 'dispatch;
	}
	// 824281AC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824281B0: 409A0020  bne cr6, 0x824281d0
	if !ctx.cr[6].eq {
	pc = 0x824281D0; continue 'dispatch;
	}
	// 824281B4: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824281B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824281BC: 409A0014  bne cr6, 0x824281d0
	if !ctx.cr[6].eq {
	pc = 0x824281D0; continue 'dispatch;
	}
	// 824281C0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 824281C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824281C8: 4BFFE5E9  bl 0x824267b0
	ctx.lr = 0x824281CC;
	sub_824267B0(ctx, base);
	// 824281CC: 48000100  b 0x824282cc
	pc = 0x824282CC; continue 'dispatch;
	// 824281D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824281D4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824281D8: 80BF0050  lwz r5, 0x50(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824281DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824281E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824281E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824281E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824281EC: 4E800421  bctrl
	ctx.lr = 0x824281F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824281F0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824281F4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824281F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824281FC: 40980010  bge cr6, 0x8242820c
	if !ctx.cr[6].lt {
	pc = 0x8242820C; continue 'dispatch;
	}
	// 82428200: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82428204: 386B22E8  addi r3, r11, 0x22e8
	ctx.r[3].s64 = ctx.r[11].s64 + 8936;
	// 82428208: 4BFF9091  bl 0x82421298
	ctx.lr = 0x8242820C;
	sub_82421298(ctx, base);
	// 8242820C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428210: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82428214: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428218: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242821C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82428220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428224: 4E800421  bctrl
	ctx.lr = 0x82428228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428228: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242822C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428230: 4BFFE589  bl 0x824267b8
	ctx.lr = 0x82428234;
	sub_824267B8(ctx, base);
	// 82428234: 7C9CD050  subf r4, r28, r26
	ctx.r[4].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 82428238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242823C: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 82428240: 4BFFE571  bl 0x824267b0
	ctx.lr = 0x82428244;
	sub_824267B0(ctx, base);
	// 82428244: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82428248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242824C: 4BFFE575  bl 0x824267c0
	ctx.lr = 0x82428250;
	sub_824267C0(ctx, base);
	// 82428250: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82428254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428258: 4BFFE521  bl 0x82426778
	ctx.lr = 0x8242825C;
	sub_82426778(ctx, base);
	// 8242825C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428260: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82428264: 409A0054  bne cr6, 0x824282b8
	if !ctx.cr[6].eq {
	pc = 0x824282B8; continue 'dispatch;
	}
	// 82428268: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242826C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82428270: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82428274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428278: 4E800421  bctrl
	ctx.lr = 0x8242827C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242827C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428280: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82428284: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82428288: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242828C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82428290: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82428294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428298: 4E800421  bctrl
	ctx.lr = 0x8242829C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242829C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824282A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824282A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824282A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824282AC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824282B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824282B4: 4E800421  bctrl
	ctx.lr = 0x824282B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824282B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824282BC: 4BFFE655  bl 0x82426910
	ctx.lr = 0x824282C0;
	sub_82426910(ctx, base);
	// 824282C0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824282C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824282C8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 824282CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824282D0: 4810CE30  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824282D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824282D8 size=168
    let mut pc: u32 = 0x824282D8;
    'dispatch: loop {
        match pc {
            0x824282D8 => {
    //   block [0x824282D8..0x82428380)
	// 824282D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824282DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824282E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824282E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824282E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824282EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824282F0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824282F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824282F8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824282FC: 4182006C  beq 0x82428368
	if ctx.cr[0].eq {
	pc = 0x82428368; continue 'dispatch;
	}
	// 82428300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82428304: 419A0064  beq cr6, 0x82428368
	if ctx.cr[6].eq {
	pc = 0x82428368; continue 'dispatch;
	}
	// 82428308: 4BFFE501  bl 0x82426808
	ctx.lr = 0x8242830C;
	sub_82426808(ctx, base);
	// 8242830C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428310: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82428314: 419A0030  beq cr6, 0x82428344
	if ctx.cr[6].eq {
	pc = 0x82428344; continue 'dispatch;
	}
	// 82428318: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8242831C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82428320: 409A0038  bne cr6, 0x82428358
	if !ctx.cr[6].eq {
	pc = 0x82428358; continue 'dispatch;
	}
	// 82428324: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428328: 48005ED9  bl 0x8242e200
	ctx.lr = 0x8242832C;
	sub_8242E200(ctx, base);
	// 8242832C: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82428330: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82428334: 41980010  blt cr6, 0x82428344
	if ctx.cr[6].lt {
	pc = 0x82428344; continue 'dispatch;
	}
	// 82428338: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8242833C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428340: 4BFFE471  bl 0x824267b0
	ctx.lr = 0x82428344;
	sub_824267B0(ctx, base);
	// 82428344: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 82428348: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242834C: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82428350: 4BFF9649  bl 0x82421998
	ctx.lr = 0x82428354;
	sub_82421998(ctx, base);
	// 82428354: 48000014  b 0x82428368
	pc = 0x82428368; continue 'dispatch;
	// 82428358: 7C6B5E70  srawi r11, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 8242835C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428360: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82428364: 4BFF955D  bl 0x824218c0
	ctx.lr = 0x82428368;
	sub_824218C0(ctx, base);
	// 82428368: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242836C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82428370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82428374: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82428378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242837C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428380 size=88
    let mut pc: u32 = 0x82428380;
    'dispatch: loop {
        match pc {
            0x82428380 => {
    //   block [0x82428380..0x824283D8)
	// 82428380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82428384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82428388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242838C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428394: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428398: 4BFFE441  bl 0x824267d8
	ctx.lr = 0x8242839C;
	sub_824267D8(ctx, base);
	// 8242839C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824283A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824283A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824283A8: A8BF0042  lha r5, 0x42(r31)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(66 as u32) ) } as i16) as i64;
	// 824283AC: 419A0014  beq cr6, 0x824283c0
	if ctx.cr[6].eq {
	pc = 0x824283C0; continue 'dispatch;
	}
	// 824283B0: 4BFF2651  bl 0x8241aa00
	ctx.lr = 0x824283B4;
	sub_8241AA00(ctx, base);
	// 824283B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824283B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824283BC: A8BF0044  lha r5, 0x44(r31)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as i16) as i64;
	// 824283C0: 4BFF2641  bl 0x8241aa00
	ctx.lr = 0x824283C4;
	sub_8241AA00(ctx, base);
	// 824283C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824283C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824283CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824283D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824283D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824283D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824283D8 size=772
    let mut pc: u32 = 0x824283D8;
    'dispatch: loop {
        match pc {
            0x824283D8 => {
    //   block [0x824283D8..0x824286DC)
	// 824283D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824283DC: 4810CCD5  bl 0x825350b0
	ctx.lr = 0x824283E0;
	sub_82535080(ctx, base);
	// 824283E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824283E4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824283E8: 897A0098  lbz r11, 0x98(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(152 as u32) ) } as u64;
	// 824283EC: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824283F0: 83FA0014  lwz r31, 0x14(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 824283F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824283F8: 419A02DC  beq cr6, 0x824286d4
	if ctx.cr[6].eq {
	pc = 0x824286D4; continue 'dispatch;
	}
	// 824283FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82428400: B1610052  sth r11, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 82428404: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82428408: 48006B49  bl 0x8242ef50
	ctx.lr = 0x8242840C;
	sub_8242EF50(ctx, base);
	// 8242840C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428410: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82428414: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82428418: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8242841C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428424: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82428428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242842C: 4E800421  bctrl
	ctx.lr = 0x82428430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428430: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428434: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82428438: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8242843C: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 82428440: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428444: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82428448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242844C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428450: 4E800421  bctrl
	ctx.lr = 0x82428454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428458: 4BFFE371  bl 0x824267c8
	ctx.lr = 0x8242845C;
	sub_824267C8(ctx, base);
	// 8242845C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82428460: 40820068  bne 0x824284c8
	if !ctx.cr[0].eq {
	pc = 0x824284C8; continue 'dispatch;
	}
	// 82428464: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82428468: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8242846C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82428470: 4BFFFCB9  bl 0x82428128
	ctx.lr = 0x82428474;
	sub_82428128(ctx, base);
	// 82428474: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82428478: 41820050  beq 0x824284c8
	if ctx.cr[0].eq {
	pc = 0x824284C8; continue 'dispatch;
	}
	// 8242847C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428480: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82428484: 4BFF2B85  bl 0x8241b008
	ctx.lr = 0x82428488;
	sub_8241B008(ctx, base);
	// 82428488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242848C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82428490: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428498: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242849C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824284A0: 4E800421  bctrl
	ctx.lr = 0x824284A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824284A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824284A8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824284AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824284B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824284B4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824284B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824284BC: 4E800421  bctrl
	ctx.lr = 0x824284C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824284C0: 48006A91  bl 0x8242ef50
	ctx.lr = 0x824284C4;
	sub_8242EF50(ctx, base);
	// 824284C4: 48000210  b 0x824286d4
	pc = 0x824286D4; continue 'dispatch;
	// 824284C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824284CC: ABA10050  lha r29, 0x50(r1)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 824284D0: 4BFFE2F9  bl 0x824267c8
	ctx.lr = 0x824284D4;
	sub_824267C8(ctx, base);
	// 824284D4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824284D8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824284DC: 7C9D5850  subf r4, r29, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 824284E0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824284E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824284E8: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824284EC: 409A0024  bne cr6, 0x82428510
	if !ctx.cr[6].eq {
	pc = 0x82428510; continue 'dispatch;
	}
	// 824284F0: 48007601  bl 0x8242faf0
	ctx.lr = 0x824284F4;
	sub_8242FAF0(ctx, base);
	// 824284F4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824284F8: 41820024  beq 0x8242851c
	if ctx.cr[0].eq {
	pc = 0x8242851C; continue 'dispatch;
	}
	// 824284FC: 38A10052  addi r5, r1, 0x52
	ctx.r[5].s64 = ctx.r[1].s64 + 82;
	// 82428500: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82428504: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82428508: 480075E9  bl 0x8242faf0
	ctx.lr = 0x8242850C;
	sub_8242FAF0(ctx, base);
	// 8242850C: 48000028  b 0x82428534
	pc = 0x82428534; continue 'dispatch;
	// 82428510: 4BFFF789  bl 0x82427c98
	ctx.lr = 0x82428514;
	sub_82427C98(ctx, base);
	// 82428514: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82428518: 4082000C  bne 0x82428524
	if !ctx.cr[0].eq {
	pc = 0x82428524; continue 'dispatch;
	}
	// 8242851C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82428520: 48000014  b 0x82428534
	pc = 0x82428534; continue 'dispatch;
	// 82428524: 38A10052  addi r5, r1, 0x52
	ctx.r[5].s64 = ctx.r[1].s64 + 82;
	// 82428528: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8242852C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82428530: 4BFFF769  bl 0x82427c98
	ctx.lr = 0x82428534;
	sub_82427C98(ctx, base);
	// 82428534: A9610050  lha r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 82428538: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8242853C: AB610052  lha r27, 0x52(r1)
	ctx.r[27].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as i16) as i64;
	// 82428540: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82428544: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428548: 419A0050  beq cr6, 0x82428598
	if ctx.cr[6].eq {
	pc = 0x82428598; continue 'dispatch;
	}
	// 8242854C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82428550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428554: 419A0094  beq cr6, 0x824285e8
	if ctx.cr[6].eq {
	pc = 0x824285E8; continue 'dispatch;
	}
	// 82428558: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242855C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82428560: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428568: 4E800421  bctrl
	ctx.lr = 0x8242856C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242856C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428570: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82428574: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242857C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82428580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428584: 4E800421  bctrl
	ctx.lr = 0x82428588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242858C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82428590: 4BFF2A79  bl 0x8241b008
	ctx.lr = 0x82428594;
	sub_8241B008(ctx, base);
	// 82428594: 4BFFFF2C  b 0x824284c0
	pc = 0x824284C0; continue 'dispatch;
	// 82428598: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242859C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824285A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824285A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824285A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824285AC: 4E800421  bctrl
	ctx.lr = 0x824285B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824285B0: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 824285B4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824285B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824285BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824285C0: 48000949  bl 0x82428f08
	ctx.lr = 0x824285C4;
	sub_82428F08(ctx, base);
	// 824285C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824285C8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824285CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824285D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824285D4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824285D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824285DC: 4E800421  bctrl
	ctx.lr = 0x824285E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824285E0: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 824285E4: 4800004C  b 0x82428630
	pc = 0x82428630; continue 'dispatch;
	// 824285E8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824285EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824285F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824285F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824285F8: 4E800421  bctrl
	ctx.lr = 0x824285FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824285FC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82428600: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82428604: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82428608: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8242860C: 480008FD  bl 0x82428f08
	ctx.lr = 0x82428610;
	sub_82428F08(ctx, base);
	// 82428610: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428614: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82428618: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242861C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428620: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82428624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428628: 4E800421  bctrl
	ctx.lr = 0x8242862C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242862C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82428630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428638: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242863C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82428640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428644: 4E800421  bctrl
	ctx.lr = 0x82428648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428648: 48006909  bl 0x8242ef50
	ctx.lr = 0x8242864C;
	sub_8242EF50(ctx, base);
	// 8242864C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428650: 4BFFE121  bl 0x82426770
	ctx.lr = 0x82428654;
	sub_82426770(ctx, base);
	// 82428654: 815A00A4  lwz r10, 0xa4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(164 as u32) ) } as u64;
	// 82428658: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242865C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428660: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82428664: 917A00A4  stw r11, 0xa4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82428668: 4BFFDB21  bl 0x82426188
	ctx.lr = 0x8242866C;
	sub_82426188(ctx, base);
	// 8242866C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428670: 4BFFDAD1  bl 0x82426140
	ctx.lr = 0x82428674;
	sub_82426140(ctx, base);
	// 82428674: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428678: 4BFFEA39  bl 0x824270b0
	ctx.lr = 0x8242867C;
	sub_824270B0(ctx, base);
	// 8242867C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428680: 4BFFB0B9  bl 0x82423738
	ctx.lr = 0x82428684;
	sub_82423738(ctx, base);
	// 82428684: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82428688: 419A0014  beq cr6, 0x8242869c
	if ctx.cr[6].eq {
	pc = 0x8242869C; continue 'dispatch;
	}
	// 8242868C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428690: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82428694: 4BFF2975  bl 0x8241b008
	ctx.lr = 0x82428698;
	sub_8241B008(ctx, base);
	// 82428698: 4800003C  b 0x824286d4
	pc = 0x824286D4; continue 'dispatch;
	// 8242869C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286A0: 809A0048  lwz r4, 0x48(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 824286A4: 4BFFDA85  bl 0x82426128
	ctx.lr = 0x824286A8;
	sub_82426128(ctx, base);
	// 824286A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286AC: 4BFFE145  bl 0x824267f0
	ctx.lr = 0x824286B0;
	sub_824267F0(ctx, base);
	// 824286B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824286B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286B8: 4BFFE0F9  bl 0x824267b0
	ctx.lr = 0x824286BC;
	sub_824267B0(ctx, base);
	// 824286BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824286C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286C4: 4BFFE0FD  bl 0x824267c0
	ctx.lr = 0x824286C8;
	sub_824267C0(ctx, base);
	// 824286C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824286CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286D0: 4BFFE0E9  bl 0x824267b8
	ctx.lr = 0x824286D4;
	sub_824267B8(ctx, base);
	// 824286D4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824286D8: 4810CA28  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824286E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824286E0 size=328
    let mut pc: u32 = 0x824286E0;
    'dispatch: loop {
        match pc {
            0x824286E0 => {
    //   block [0x824286E0..0x82428828)
	// 824286E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824286E4: 4810C9D1  bl 0x825350b4
	ctx.lr = 0x824286E8;
	sub_82535080(ctx, base);
	// 824286E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824286EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824286F0: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824286F4: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824286F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824286FC: 4BFFC1DD  bl 0x824248d8
	ctx.lr = 0x82428700;
	sub_824248D8(ctx, base);
	// 82428700: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82428704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428708: 4BFFC209  bl 0x82424910
	ctx.lr = 0x8242870C;
	sub_82424910(ctx, base);
	// 8242870C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82428710: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82428714: 2F0B2000  cmpwi cr6, r11, 0x2000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8192, &mut ctx.xer);
	// 82428718: 41980008  blt cr6, 0x82428720
	if ctx.cr[6].lt {
	pc = 0x82428720; continue 'dispatch;
	}
	// 8242871C: 39602000  li r11, 0x2000
	ctx.r[11].s64 = 8192;
	// 82428720: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82428724: 40980024  bge cr6, 0x82428748
	if !ctx.cr[6].lt {
	pc = 0x82428748; continue 'dispatch;
	}
	// 82428728: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8242872C: 4BFFE0BD  bl 0x824267e8
	ctx.lr = 0x82428730;
	sub_824267E8(ctx, base);
	// 82428730: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82428734: 40990014  ble cr6, 0x82428748
	if !ctx.cr[6].gt {
	pc = 0x82428748; continue 'dispatch;
	}
	// 82428738: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242873C: 4BFFAFFD  bl 0x82423738
	ctx.lr = 0x82428740;
	sub_82423738(ctx, base);
	// 82428740: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82428744: 409A004C  bne cr6, 0x82428790
	if !ctx.cr[6].eq {
	pc = 0x82428790; continue 'dispatch;
	}
	// 82428748: 897F0070  lbz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8242874C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82428750: 409A0038  bne cr6, 0x82428788
	if !ctx.cr[6].eq {
	pc = 0x82428788; continue 'dispatch;
	}
	// 82428754: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 82428758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242875C: 409A0024  bne cr6, 0x82428780
	if !ctx.cr[6].eq {
	pc = 0x82428780; continue 'dispatch;
	}
	// 82428760: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428768: 4BFFC101  bl 0x82424868
	ctx.lr = 0x8242876C;
	sub_82424868(ctx, base);
	// 8242876C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82428770: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82428774: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 82428778: 816BF40C  lwz r11, -0xbf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3060 as u32) ) } as u64;
	// 8242877C: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82428780: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82428784: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82428788: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242878C: 997F0071  stb r11, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 82428790: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428794: 4BFFAFA5  bl 0x82423738
	ctx.lr = 0x82428798;
	sub_82423738(ctx, base);
	// 82428798: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242879C: 409A0084  bne cr6, 0x82428820
	if !ctx.cr[6].eq {
	pc = 0x82428820; continue 'dispatch;
	}
	// 824287A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824287A4: 4BFF21E5  bl 0x8241a988
	ctx.lr = 0x824287A8;
	sub_8241A988(ctx, base);
	// 824287A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824287AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824287B0: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 824287B4: 557C083C  slwi r28, r11, 1
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824287B8: 40810068  ble 0x82428820
	if !ctx.cr[0].gt {
	pc = 0x82428820; continue 'dispatch;
	}
	// 824287BC: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 824287C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824287C4: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824287C8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824287CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824287D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824287D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824287D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824287DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824287E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824287E4: 4E800421  bctrl
	ctx.lr = 0x824287E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824287E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824287EC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824287F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824287F4: 4810C9DD  bl 0x825351d0
	ctx.lr = 0x824287F8;
	sub_825351D0(ctx, base);
	// 824287F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824287FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82428800: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428808: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242880C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428810: 4E800421  bctrl
	ctx.lr = 0x82428814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428814: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82428818: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8242881C: 4082FFA8  bne 0x824287c4
	if !ctx.cr[0].eq {
	pc = 0x824287C4; continue 'dispatch;
	}
	// 82428820: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82428824: 4810C8E0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428828 size=224
    let mut pc: u32 = 0x82428828;
    'dispatch: loop {
        match pc {
            0x82428828 => {
    //   block [0x82428828..0x82428908)
	// 82428828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242882C: 4810C889  bl 0x825350b4
	ctx.lr = 0x82428830;
	sub_82535080(ctx, base);
	// 82428830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428834: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82428838: 897D006C  lbz r11, 0x6c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 8242883C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82428840: 409A0030  bne cr6, 0x82428870
	if !ctx.cr[6].eq {
	pc = 0x82428870; continue 'dispatch;
	}
	// 82428844: 817D00C0  lwz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 82428848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242884C: 40990024  ble cr6, 0x82428870
	if !ctx.cr[6].gt {
	pc = 0x82428870; continue 'dispatch;
	}
	// 82428850: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428854: 480059AD  bl 0x8242e200
	ctx.lr = 0x82428858;
	sub_8242E200(ctx, base);
	// 82428858: 817D00C0  lwz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 8242885C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82428860: 41980010  blt cr6, 0x82428870
	if ctx.cr[6].lt {
	pc = 0x82428870; continue 'dispatch;
	}
	// 82428864: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82428868: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242886C: 4BFFDF45  bl 0x824267b0
	ctx.lr = 0x82428870;
	sub_824267B0(ctx, base);
	// 82428870: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428874: 4BFFAEC5  bl 0x82423738
	ctx.lr = 0x82428878;
	sub_82423738(ctx, base);
	// 82428878: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242887C: 409A0084  bne cr6, 0x82428900
	if !ctx.cr[6].eq {
	pc = 0x82428900; continue 'dispatch;
	}
	// 82428880: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428884: 4BFFDF55  bl 0x824267d8
	ctx.lr = 0x82428888;
	sub_824267D8(ctx, base);
	// 82428888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242888C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82428890: 93CB0BA4  stw r30, 0xba4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2980 as u32), ctx.r[30].u32 ) };
	// 82428894: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428898: 4BFFC3D1  bl 0x82424c68
	ctx.lr = 0x8242889C;
	sub_82424C68(ctx, base);
	// 8242889C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824288A0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824288A4: 40990040  ble cr6, 0x824288e4
	if !ctx.cr[6].gt {
	pc = 0x824288E4; continue 'dispatch;
	}
	// 824288A8: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 824288AC: 3F608312  lis r27, -0x7cee
	ctx.r[27].s64 = -2095972352;
	// 824288B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824288B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824288B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824288BC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824288C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824288C4: 4E800421  bctrl
	ctx.lr = 0x824288C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824288C8: 907B0BA0  stw r3, 0xba0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2976 as u32), ctx.r[3].u32 ) };
	// 824288CC: 2F030040  cmpwi cr6, r3, 0x40
	ctx.cr[6].compare_i32(ctx.r[3].s32, 64, &mut ctx.xer);
	// 824288D0: 40980014  bge cr6, 0x824288e4
	if !ctx.cr[6].lt {
	pc = 0x824288E4; continue 'dispatch;
	}
	// 824288D4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824288D8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824288DC: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824288E0: 4198FFD0  blt cr6, 0x824288b0
	if ctx.cr[6].lt {
	pc = 0x824288B0; continue 'dispatch;
	}
	// 824288E4: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824288E8: 409A0018  bne cr6, 0x82428900
	if !ctx.cr[6].eq {
	pc = 0x82428900; continue 'dispatch;
	}
	// 824288EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824288F0: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824288F4: 4BFFBF55  bl 0x82424848
	ctx.lr = 0x824288F8;
	sub_82424848(ctx, base);
	// 824288F8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 824288FC: 997D0001  stb r11, 1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82428900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82428904: 4810C800  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428908 size=80
    let mut pc: u32 = 0x82428908;
    'dispatch: loop {
        match pc {
            0x82428908 => {
    //   block [0x82428908..0x82428958)
	// 82428908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242890C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82428910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82428914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242891C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428920: 4BFFBFB9  bl 0x824248d8
	ctx.lr = 0x82428924;
	sub_824248D8(ctx, base);
	// 82428924: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82428928: 906B0BA8  stw r3, 0xba8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2984 as u32), ctx.r[3].u32 ) };
	// 8242892C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428930: 4BFFBFF1  bl 0x82424920
	ctx.lr = 0x82428934;
	sub_82424920(ctx, base);
	// 82428934: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82428938: 419A000C  beq cr6, 0x82428944
	if ctx.cr[6].eq {
	pc = 0x82428944; continue 'dispatch;
	}
	// 8242893C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82428940: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82428944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82428948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242894C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82428950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82428954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428958 size=96
    let mut pc: u32 = 0x82428958;
    'dispatch: loop {
        match pc {
            0x82428958 => {
    //   block [0x82428958..0x824289B8)
	// 82428958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242895C: 4810C761  bl 0x825350bc
	ctx.lr = 0x82428960;
	sub_82535080(ctx, base);
	// 82428960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428968: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8242896C: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 82428970: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82428974: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428978: 41820018  beq 0x82428990
	if ctx.cr[0].eq {
	pc = 0x82428990; continue 'dispatch;
	}
	// 8242897C: 4BFF8F05  bl 0x82421880
	ctx.lr = 0x82428980;
	sub_82421880(ctx, base);
	// 82428980: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82428984: 409A000C  bne cr6, 0x82428990
	if !ctx.cr[6].eq {
	pc = 0x82428990; continue 'dispatch;
	}
	// 82428988: B3BF0060  sth r29, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u16 ) };
	// 8242898C: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 82428990: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82428994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428998: 41820018  beq 0x824289b0
	if ctx.cr[0].eq {
	pc = 0x824289B0; continue 'dispatch;
	}
	// 8242899C: 4BFFEEBD  bl 0x82427858
	ctx.lr = 0x824289A0;
	sub_82427858(ctx, base);
	// 824289A0: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 824289A4: 409A000C  bne cr6, 0x824289b0
	if !ctx.cr[6].eq {
	pc = 0x824289B0; continue 'dispatch;
	}
	// 824289A8: B3BF0060  sth r29, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u16 ) };
	// 824289AC: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 824289B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824289B4: 4810C758  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824289B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824289B8 size=104
    let mut pc: u32 = 0x824289B8;
    'dispatch: loop {
        match pc {
            0x824289B8 => {
    //   block [0x824289B8..0x82428A20)
	// 824289B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824289BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824289C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824289C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824289C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824289CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824289D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824289D4: 419A0038  beq cr6, 0x82428a0c
	if ctx.cr[6].eq {
	pc = 0x82428A0C; continue 'dispatch;
	}
	// 824289D8: 4BFF1E31  bl 0x8241a808
	ctx.lr = 0x824289DC;
	sub_8241A808(ctx, base);
	// 824289DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824289E0: 4182002C  beq 0x82428a0c
	if ctx.cr[0].eq {
	pc = 0x82428A0C; continue 'dispatch;
	}
	// 824289E4: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 824289E8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824289EC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824289F0: 40980010  bge cr6, 0x82428a00
	if !ctx.cr[6].lt {
	pc = 0x82428A00; continue 'dispatch;
	}
	// 824289F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824289F8: 4BFF8E89  bl 0x82421880
	ctx.lr = 0x824289FC;
	sub_82421880(ctx, base);
	// 824289FC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82428A00: 409A000C  bne cr6, 0x82428a0c
	if !ctx.cr[6].eq {
	pc = 0x82428A0C; continue 'dispatch;
	}
	// 82428A04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428A08: 4BFFD731  bl 0x82426138
	ctx.lr = 0x82428A0C;
	sub_82426138(ctx, base);
	// 82428A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82428A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82428A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82428A18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82428A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428A20 size=144
    let mut pc: u32 = 0x82428A20;
    'dispatch: loop {
        match pc {
            0x82428A20 => {
    //   block [0x82428A20..0x82428AB0)
	// 82428A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82428A24: 4810C691  bl 0x825350b4
	ctx.lr = 0x82428A28;
	sub_82535080(ctx, base);
	// 82428A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428A2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82428A30: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A38: 4BFFDDC9  bl 0x82426800
	ctx.lr = 0x82428A3C;
	sub_82426800(ctx, base);
	// 82428A3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82428A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A44: 4BFFDDC5  bl 0x82426808
	ctx.lr = 0x82428A48;
	sub_82426808(ctx, base);
	// 82428A48: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82428A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A50: 4BFFDDD1  bl 0x82426820
	ctx.lr = 0x82428A54;
	sub_82426820(ctx, base);
	// 82428A54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82428A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A5C: 4BFFDEAD  bl 0x82426908
	ctx.lr = 0x82428A60;
	sub_82426908(ctx, base);
	// 82428A60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A68: 4BFFDD51  bl 0x824267b8
	ctx.lr = 0x82428A6C;
	sub_824267B8(ctx, base);
	// 82428A6C: 7C9DD850  subf r4, r29, r27
	ctx.r[4].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 82428A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A74: 909E0090  stw r4, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 82428A78: 4BFFDD39  bl 0x824267b0
	ctx.lr = 0x82428A7C;
	sub_824267B0(ctx, base);
	// 82428A7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82428A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A84: 4BFFDD3D  bl 0x824267c0
	ctx.lr = 0x82428A88;
	sub_824267C0(ctx, base);
	// 82428A88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82428A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428A90: 4BFFDCE9  bl 0x82426778
	ctx.lr = 0x82428A94;
	sub_82426778(ctx, base);
	// 82428A94: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82428A98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82428A9C: 388B8160  addi r4, r11, -0x7ea0
	ctx.r[4].s64 = ctx.r[11].s64 + -32416;
	// 82428AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428AA4: 4BFFDCFD  bl 0x824267a0
	ctx.lr = 0x82428AA8;
	sub_824267A0(ctx, base);
	// 82428AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82428AAC: 4810C658  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82428AB0 size=932
    let mut pc: u32 = 0x82428AB0;
    'dispatch: loop {
        match pc {
            0x82428AB0 => {
    //   block [0x82428AB0..0x82428E54)
	// 82428AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82428AB4: 4810C601  bl 0x825350b4
	ctx.lr = 0x82428AB8;
	sub_82535080(ctx, base);
	// 82428AB8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428AC0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82428AC4: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428AC8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428ACC: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82428AD0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82428AD4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82428AD8: 4182000C  beq 0x82428ae4
	if ctx.cr[0].eq {
	pc = 0x82428AE4; continue 'dispatch;
	}
	// 82428ADC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82428AE0: 409A0058  bne cr6, 0x82428b38
	if !ctx.cr[6].eq {
	pc = 0x82428B38; continue 'dispatch;
	}
	// 82428AE4: 897F00A8  lbz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82428AE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82428AEC: 409A004C  bne cr6, 0x82428b38
	if !ctx.cr[6].eq {
	pc = 0x82428B38; continue 'dispatch;
	}
	// 82428AF0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82428AF4: 4BFF8D8D  bl 0x82421880
	ctx.lr = 0x82428AF8;
	sub_82421880(ctx, base);
	// 82428AF8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82428AFC: 419A0350  beq cr6, 0x82428e4c
	if ctx.cr[6].eq {
	pc = 0x82428E4C; continue 'dispatch;
	}
	// 82428B00: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82428B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428B08: 41820014  beq 0x82428b1c
	if ctx.cr[0].eq {
	pc = 0x82428B1C; continue 'dispatch;
	}
	// 82428B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428B10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82428B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428B18: 4E800421  bctrl
	ctx.lr = 0x82428B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428B20: 80FF00BC  lwz r7, 0xbc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82428B24: 80DF00B8  lwz r6, 0xb8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82428B28: 80BF00B4  lwz r5, 0xb4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82428B2C: 809F00B0  lwz r4, 0xb0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82428B30: 4BFF2A59  bl 0x8241b588
	ctx.lr = 0x82428B34;
	sub_8241B588(ctx, base);
	// 82428B34: 9B7F00A8  stb r27, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[27].u8 ) };
	// 82428B38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428B3C: 4BFFABFD  bl 0x82423738
	ctx.lr = 0x82428B40;
	sub_82423738(ctx, base);
	// 82428B40: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82428B44: 409A02F8  bne cr6, 0x82428e3c
	if !ctx.cr[6].eq {
	pc = 0x82428E3C; continue 'dispatch;
	}
	// 82428B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428B4C: 4BFFDC8D  bl 0x824267d8
	ctx.lr = 0x82428B50;
	sub_824267D8(ctx, base);
	// 82428B50: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82428B54: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82428B58: 7F032000  cmpw cr6, r3, r4
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82428B5C: 4099002C  ble cr6, 0x82428b88
	if !ctx.cr[6].gt {
	pc = 0x82428B88; continue 'dispatch;
	}
	// 82428B60: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82428B64: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82428B68: 4BFF8911  bl 0x82421478
	ctx.lr = 0x82428B6C;
	sub_82421478(ctx, base);
	// 82428B6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82428B70: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82428B74: 386B2314  addi r3, r11, 0x2314
	ctx.r[3].s64 = ctx.r[11].s64 + 8980;
	// 82428B78: 4BFF87A9  bl 0x82421320
	ctx.lr = 0x82428B7C;
	sub_82421320(ctx, base);
	// 82428B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428B80: 4BFF1C51  bl 0x8241a7d0
	ctx.lr = 0x82428B84;
	sub_8241A7D0(ctx, base);
	// 82428B84: 480002C8  b 0x82428e4c
	pc = 0x82428E4C; continue 'dispatch;
	// 82428B88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428B8C: 4BFFDC45  bl 0x824267d0
	ctx.lr = 0x82428B90;
	sub_824267D0(ctx, base);
	// 82428B90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82428B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428B98: 4BFFDC61  bl 0x824267f8
	ctx.lr = 0x82428B9C;
	sub_824267F8(ctx, base);
	// 82428B9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82428BA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428BA4: 4BFFDC25  bl 0x824267c8
	ctx.lr = 0x82428BA8;
	sub_824267C8(ctx, base);
	// 82428BA8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82428BAC: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 82428BB0: 7D7D5BD6  divw r11, r29, r11
	ctx.r[11].s32 = ctx.r[29].s32 / ctx.r[11].s32;
	// 82428BB4: 409A0018  bne cr6, 0x82428bcc
	if !ctx.cr[6].eq {
	pc = 0x82428BCC; continue 'dispatch;
	}
	// 82428BB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428BBC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82428BC0: 4BFFDC29  bl 0x824267e8
	ctx.lr = 0x82428BC4;
	sub_824267E8(ctx, base);
	// 82428BC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82428BC8: 48000028  b 0x82428bf0
	pc = 0x82428BF0; continue 'dispatch;
	// 82428BCC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82428BD0: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 82428BD4: 4199000C  bgt cr6, 0x82428be0
	if ctx.cr[6].gt {
	pc = 0x82428BE0; continue 'dispatch;
	}
	// 82428BD8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82428BDC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82428BE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428BE4: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82428BE8: 4BFFDC01  bl 0x824267e8
	ctx.lr = 0x82428BEC;
	sub_824267E8(ctx, base);
	// 82428BEC: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82428BF0: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82428BF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428BF8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82428BFC: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82428C00: 7C8A59D6  mullw r4, r10, r11
	ctx.r[4].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82428C04: 909F0048  stw r4, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 82428C08: 4BFFD521  bl 0x82426128
	ctx.lr = 0x82428C0C;
	sub_82426128(ctx, base);
	// 82428C0C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82428C10: 409900BC  ble cr6, 0x82428ccc
	if !ctx.cr[6].gt {
	pc = 0x82428CCC; continue 'dispatch;
	}
	// 82428C14: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428C18: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82428C1C: 409A000C  bne cr6, 0x82428c28
	if !ctx.cr[6].eq {
	pc = 0x82428C28; continue 'dispatch;
	}
	// 82428C20: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82428C24: 48000064  b 0x82428c88
	pc = 0x82428C88; continue 'dispatch;
	// 82428C28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428C2C: 4BFFDBFD  bl 0x82426828
	ctx.lr = 0x82428C30;
	sub_82426828(ctx, base);
	// 82428C30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82428C34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82428C38: 7D6A5E70  srawi r10, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82428C3C: 392B07FF  addi r9, r11, 0x7ff
	ctx.r[9].s64 = ctx.r[11].s64 + 2047;
	// 82428C40: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82428C44: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428C48: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82428C4C: 216B0800  subfic r11, r11, 0x800
	ctx.xer.ca = ctx.r[11].u32 <= 2048 as u32;
	ctx.r[11].s64 = (2048 as i64) - ctx.r[11].s64;
	// 82428C50: 7D6A5E70  srawi r10, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82428C54: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82428C58: 7D295E70  srawi r9, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 82428C5C: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428C60: 7C890194  addze r4, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82428C64: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82428C68: 909F008C  stw r4, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82428C6C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82428C70: 4BFF8D29  bl 0x82421998
	ctx.lr = 0x82428C74;
	sub_82421998(ctx, base);
	// 82428C74: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82428C78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82428C7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82428C80: 388B82D8  addi r4, r11, -0x7d28
	ctx.r[4].s64 = ctx.r[11].s64 + -32040;
	// 82428C84: 4BFF8CE5  bl 0x82421968
	ctx.lr = 0x82428C88;
	sub_82421968(ctx, base);
	// 82428C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428C8C: 4BFFDB95  bl 0x82426820
	ctx.lr = 0x82428C90;
	sub_82426820(ctx, base);
	// 82428C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428C94: 4BFFDB6D  bl 0x82426800
	ctx.lr = 0x82428C98;
	sub_82426800(ctx, base);
	// 82428C98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82428C9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428CA0: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 82428CA4: 4BFFDB0D  bl 0x824267b0
	ctx.lr = 0x82428CA8;
	sub_824267B0(ctx, base);
	// 82428CA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428CAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428CB0: 4BFFDB11  bl 0x824267c0
	ctx.lr = 0x82428CB4;
	sub_824267C0(ctx, base);
	// 82428CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428CBC: 4BFFDAFD  bl 0x824267b8
	ctx.lr = 0x82428CC0;
	sub_824267B8(ctx, base);
	// 82428CC0: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82428CC4: 388B8A20  addi r4, r11, -0x75e0
	ctx.r[4].s64 = ctx.r[11].s64 + -30176;
	// 82428CC8: 48000050  b 0x82428d18
	pc = 0x82428D18; continue 'dispatch;
	// 82428CCC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82428CD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428CD4: 41820010  beq 0x82428ce4
	if ctx.cr[0].eq {
	pc = 0x82428CE4; continue 'dispatch;
	}
	// 82428CD8: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 82428CDC: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82428CE0: 4BFF8CB9  bl 0x82421998
	ctx.lr = 0x82428CE4;
	sub_82421998(ctx, base);
	// 82428CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428CE8: 4BFFDB09  bl 0x824267f0
	ctx.lr = 0x82428CEC;
	sub_824267F0(ctx, base);
	// 82428CEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82428CF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428CF4: 4BFFDABD  bl 0x824267b0
	ctx.lr = 0x82428CF8;
	sub_824267B0(ctx, base);
	// 82428CF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428CFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D00: 4BFFDAC1  bl 0x824267c0
	ctx.lr = 0x82428D04;
	sub_824267C0(ctx, base);
	// 82428D04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82428D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D0C: 4BFFDAAD  bl 0x824267b8
	ctx.lr = 0x82428D10;
	sub_824267B8(ctx, base);
	// 82428D10: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82428D14: 388B83D8  addi r4, r11, -0x7c28
	ctx.r[4].s64 = ctx.r[11].s64 + -31784;
	// 82428D18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82428D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D20: 4BFFDA81  bl 0x824267a0
	ctx.lr = 0x82428D24;
	sub_824267A0(ctx, base);
	// 82428D24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D28: 4BFFDAA9  bl 0x824267d0
	ctx.lr = 0x82428D2C;
	sub_824267D0(ctx, base);
	// 82428D2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82428D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D34: 4BFFDAA5  bl 0x824267d8
	ctx.lr = 0x82428D38;
	sub_824267D8(ctx, base);
	// 82428D38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82428D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D40: 4BFFDAB1  bl 0x824267f0
	ctx.lr = 0x82428D44;
	sub_824267F0(ctx, base);
	// 82428D44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82428D48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428D4C: 4BFFDA95  bl 0x824267e0
	ctx.lr = 0x82428D50;
	sub_824267E0(ctx, base);
	// 82428D50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82428D54: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428D58: 480061F9  bl 0x8242ef50
	ctx.lr = 0x82428D5C;
	sub_8242EF50(ctx, base);
	// 82428D5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82428D60: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428D64: 4BFFBC85  bl 0x824249e8
	ctx.lr = 0x82428D68;
	sub_824249E8(ctx, base);
	// 82428D68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82428D6C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428D70: 4BFFBC61  bl 0x824249d0
	ctx.lr = 0x82428D74;
	sub_824249D0(ctx, base);
	// 82428D74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82428D78: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428D7C: 480061D5  bl 0x8242ef50
	ctx.lr = 0x82428D80;
	sub_8242EF50(ctx, base);
	// 82428D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428D84: A89F0040  lha r4, 0x40(r31)
	ctx.r[4].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 82428D88: 4BFF1D19  bl 0x8241aaa0
	ctx.lr = 0x82428D8C;
	sub_8241AAA0(ctx, base);
	// 82428D8C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82428D90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82428D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428D98: 4BFF21A9  bl 0x8241af40
	ctx.lr = 0x82428D9C;
	sub_8241AF40(ctx, base);
	// 82428D9C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82428DA0: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82428DA4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82428DA8: 409A000C  bne cr6, 0x82428db4
	if !ctx.cr[6].eq {
	pc = 0x82428DB4; continue 'dispatch;
	}
	// 82428DAC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82428DB0: 419A000C  beq cr6, 0x82428dbc
	if ctx.cr[6].eq {
	pc = 0x82428DBC; continue 'dispatch;
	}
	// 82428DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428DB8: 4BFF2139  bl 0x8241aef0
	ctx.lr = 0x82428DBC;
	sub_8241AEF0(ctx, base);
	// 82428DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428DC0: 4BFFF5C1  bl 0x82428380
	ctx.lr = 0x82428DC4;
	sub_82428380(ctx, base);
	// 82428DC4: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82428DC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82428DCC: 4182000C  beq 0x82428dd8
	if ctx.cr[0].eq {
	pc = 0x82428DD8; continue 'dispatch;
	}
	// 82428DD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82428DD4: 4BFFE5ED  bl 0x824273c0
	ctx.lr = 0x82428DD8;
	sub_824273C0(ctx, base);
	// 82428DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428DDC: 4BFFD9ED  bl 0x824267c8
	ctx.lr = 0x82428DE0;
	sub_824267C8(ctx, base);
	// 82428DE0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82428DE4: 409A0018  bne cr6, 0x82428dfc
	if !ctx.cr[6].eq {
	pc = 0x82428DFC; continue 'dispatch;
	}
	// 82428DE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82428DEC: 4BFFDB15  bl 0x82426900
	ctx.lr = 0x82428DF0;
	sub_82426900(ctx, base);
	// 82428DF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82428DF4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428DF8: 4800E911  bl 0x82437708
	ctx.lr = 0x82428DFC;
	sub_82437708(ctx, base);
	// 82428DFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82428E00: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82428E04: 4BFFBA45  bl 0x82424848
	ctx.lr = 0x82428E08;
	sub_82424848(ctx, base);
	// 82428E08: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82428E0C: 814B97D8  lwz r10, -0x6828(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26664 as u32) ) } as u64;
	// 82428E10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82428E14: 419A0020  beq cr6, 0x82428e34
	if ctx.cr[6].eq {
	pc = 0x82428E34; continue 'dispatch;
	}
	// 82428E18: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82428E1C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82428E20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82428E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428E28: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82428E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82428E30: 4E800421  bctrl
	ctx.lr = 0x82428E34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82428E34: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82428E38: 48000010  b 0x82428e48
	pc = 0x82428E48; continue 'dispatch;
	// 82428E3C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82428E40: 409A000C  bne cr6, 0x82428e4c
	if !ctx.cr[6].eq {
	pc = 0x82428E4C; continue 'dispatch;
	}
	// 82428E44: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82428E48: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82428E4C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82428E50: 4810C2B4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82428E58 size=172
    let mut pc: u32 = 0x82428E58;
    'dispatch: loop {
        match pc {
            0x82428E58 => {
    //   block [0x82428E58..0x82428F04)
	// 82428E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82428E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82428E60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82428E64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82428E68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82428E6C: 4BFF81C5  bl 0x82421030
	ctx.lr = 0x82428E70;
	sub_82421030(ctx, base);
	// 82428E70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82428E74: 409A0014  bne cr6, 0x82428e88
	if !ctx.cr[6].eq {
	pc = 0x82428E88; continue 'dispatch;
	}
	// 82428E78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82428E7C: 386B2354  addi r3, r11, 0x2354
	ctx.r[3].s64 = ctx.r[11].s64 + 9044;
	// 82428E80: 4BFF8419  bl 0x82421298
	ctx.lr = 0x82428E84;
	sub_82421298(ctx, base);
	// 82428E84: 48000068  b 0x82428eec
	pc = 0x82428EEC; continue 'dispatch;
	// 82428E88: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82428E8C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82428E90: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82428E94: 409A0010  bne cr6, 0x82428ea4
	if !ctx.cr[6].eq {
	pc = 0x82428EA4; continue 'dispatch;
	}
	// 82428E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428E9C: 4BFFF98D  bl 0x82428828
	ctx.lr = 0x82428EA0;
	sub_82428828(ctx, base);
	// 82428EA0: 4800003C  b 0x82428edc
	pc = 0x82428EDC; continue 'dispatch;
	// 82428EA4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82428EA8: 409A0010  bne cr6, 0x82428eb8
	if !ctx.cr[6].eq {
	pc = 0x82428EB8; continue 'dispatch;
	}
	// 82428EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428EB0: 4BFFFC01  bl 0x82428ab0
	ctx.lr = 0x82428EB4;
	sub_82428AB0(ctx, base);
	// 82428EB4: 48000028  b 0x82428edc
	pc = 0x82428EDC; continue 'dispatch;
	// 82428EB8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82428EBC: 409A0010  bne cr6, 0x82428ecc
	if !ctx.cr[6].eq {
	pc = 0x82428ECC; continue 'dispatch;
	}
	// 82428EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428EC4: 4BFFF81D  bl 0x824286e0
	ctx.lr = 0x82428EC8;
	sub_824286E0(ctx, base);
	// 82428EC8: 48000014  b 0x82428edc
	pc = 0x82428EDC; continue 'dispatch;
	// 82428ECC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82428ED0: 409A000C  bne cr6, 0x82428edc
	if !ctx.cr[6].eq {
	pc = 0x82428EDC; continue 'dispatch;
	}
	// 82428ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428ED8: 4BFFFA31  bl 0x82428908
	ctx.lr = 0x82428EDC;
	sub_82428908(ctx, base);
	// 82428EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428EE0: 4BFFFAD9  bl 0x824289b8
	ctx.lr = 0x82428EE4;
	sub_824289B8(ctx, base);
	// 82428EE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82428EE8: 4BFFFA71  bl 0x82428958
	ctx.lr = 0x82428EEC;
	sub_82428958(ctx, base);
	// 82428EEC: 4BFF8185  bl 0x82421070
	ctx.lr = 0x82428EF0;
	sub_82421070(ctx, base);
	// 82428EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82428EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82428EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82428EFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82428F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82428F08 size=68
    let mut pc: u32 = 0x82428F08;
    'dispatch: loop {
        match pc {
            0x82428F08 => {
    //   block [0x82428F08..0x82428F4C)
	// 82428F08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428F0C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82428F10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F14: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82428F18: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82428F1C: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82428F20: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F24: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82428F28: 40990008  ble cr6, 0x82428f30
	if !ctx.cr[6].gt {
	pc = 0x82428F30; continue 'dispatch;
	}
	// 82428F2C: 90850004  stw r4, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82428F30: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F34: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F38: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82428F3C: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82428F40: 4082000C  bne 0x82428f4c
	if !ctx.cr[0].eq {
		sub_82428F4C(ctx, base);
		return;
	}
	// 82428F44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82428F48: 48000010  b 0x82428f58
	sub_82428F4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428F4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82428F4C size=20
    let mut pc: u32 = 0x82428F4C;
    'dispatch: loop {
        match pc {
            0x82428F4C => {
    //   block [0x82428F4C..0x82428F60)
	// 82428F4C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F50: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428F54: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82428F58: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82428F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82428F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82428F60 size=172
    let mut pc: u32 = 0x82428F60;
    'dispatch: loop {
        match pc {
            0x82428F60 => {
    //   block [0x82428F60..0x8242900C)
	// 82428F60: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82428F64: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82428F68: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82428F6C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82428F70: 89030002  lbz r8, 2(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82428F74: 396B3D10  addi r11, r11, 0x3d10
	ctx.r[11].s64 = ctx.r[11].s64 + 15632;
	// 82428F78: 88E30003  lbz r7, 3(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82428F7C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428F80: 88C30004  lbz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82428F84: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82428F88: 88A30005  lbz r5, 5(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 82428F8C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82428F90: 88830006  lbz r4, 6(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82428F94: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82428F98: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82428F9C: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FA0: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 82428FA4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428FA8: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FAC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82428FB0: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FB4: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82428FB8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428FBC: 7CC70774  extsb r7, r6
	ctx.r[7].s64 = ctx.r[6].s8 as i64;
	// 82428FC0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82428FC4: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FC8: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82428FCC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428FD0: 7CA70774  extsb r7, r5
	ctx.r[7].s64 = ctx.r[5].s8 as i64;
	// 82428FD4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82428FD8: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FDC: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82428FE0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428FE4: 7C870774  extsb r7, r4
	ctx.r[7].s64 = ctx.r[4].s8 as i64;
	// 82428FE8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82428FEC: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82428FF0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82428FF4: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82428FF8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82428FFC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82429000: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82429004: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82429008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429010 size=184
    let mut pc: u32 = 0x82429010;
    'dispatch: loop {
        match pc {
            0x82429010 => {
    //   block [0x82429010..0x824290C8)
	// 82429010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429014: 4810C0A1  bl 0x825350b4
	ctx.lr = 0x82429018;
	sub_82535080(ctx, base);
	// 82429018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242901C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82429020: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82429024: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82429028: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8242902C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82429030: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82429034: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429038: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242903C: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82429040: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82429044: 40980078  bge cr6, 0x824290bc
	if !ctx.cr[6].lt {
	pc = 0x824290BC; continue 'dispatch;
	}
	// 82429048: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8242904C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82429050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429054: 4810A13D  bl 0x82533190
	ctx.lr = 0x82429058;
	sub_82533190(ctx, base);
	// 82429058: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242905C: 41820040  beq 0x8242909c
	if ctx.cr[0].eq {
	pc = 0x8242909C; continue 'dispatch;
	}
	// 82429060: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82429064: 419A001C  beq cr6, 0x82429080
	if ctx.cr[6].eq {
	pc = 0x82429080; continue 'dispatch;
	}
	// 82429068: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8242906C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82429070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429074: 4810A11D  bl 0x82533190
	ctx.lr = 0x82429078;
	sub_82533190(ctx, base);
	// 82429078: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242907C: 41820040  beq 0x824290bc
	if ctx.cr[0].eq {
	pc = 0x824290BC; continue 'dispatch;
	}
	// 82429080: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82429084: 4BFFFEDD  bl 0x82428f60
	ctx.lr = 0x82429088;
	sub_82428F60(ctx, base);
	// 82429088: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8242908C: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82429090: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82429094: 4198FFB4  blt cr6, 0x82429048
	if ctx.cr[6].lt {
	pc = 0x82429048; continue 'dispatch;
	}
	// 82429098: 48000018  b 0x824290b0
	pc = 0x824290B0; continue 'dispatch;
	// 8242909C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824290A0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824290A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824290A8: 4BFFFEB9  bl 0x82428f60
	ctx.lr = 0x824290AC;
	sub_82428F60(ctx, base);
	// 824290AC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 824290B0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 824290B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824290B8: 41980008  blt cr6, 0x824290c0
	if ctx.cr[6].lt {
	pc = 0x824290C0; continue 'dispatch;
	}
	// 824290BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824290C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824290C4: 4810C040  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824290C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824290C8 size=24
    let mut pc: u32 = 0x824290C8;
    'dispatch: loop {
        match pc {
            0x824290C8 => {
    //   block [0x824290C8..0x824290E0)
	// 824290C8: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 824290CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824290D0: 614A0018  ori r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u64 | 24;
	// 824290D4: 7D4B502A  ldx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 824290D8: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 824290DC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824290E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824290E0 size=40
    let mut pc: u32 = 0x824290E0;
    'dispatch: loop {
        match pc {
            0x824290E0 => {
    //   block [0x824290E0..0x82429108)
	// 824290E0: 3D6B0002  addis r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 131072;
	// 824290E4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 824290E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 824290EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824290F0: 7D4058A8  ldarx r10, 0, r11
	ctx.reserved.u64 = unsafe { *(base.add(ctx.r[11].u32) as usize) as *const u64 } as u64;
	ctx.r[10].u64 = ctx.reserved.u64.swap_bytes();
	// 824290F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824290F8: 7D4059AD  stdcx. r10, 0, r11
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stdcx64(base as *mut u8, addr, ctx.reserved.u64 as u64, ctx.r[10].u64) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824290FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82429100: 4082FFE8  bne 0x824290e8
	if !ctx.cr[0].eq {
	pc = 0x824290E8; continue 'dispatch;
	}
	// 82429104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429108 size=200
    let mut pc: u32 = 0x82429108;
    'dispatch: loop {
        match pc {
            0x82429108 => {
    //   block [0x82429108..0x824291D0)
	// 82429108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242910C: 4810BFA9  bl 0x825350b4
	ctx.lr = 0x82429110;
	sub_82535080(ctx, base);
	// 82429110: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82429114: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242911C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429120: 3F9F0002  addis r28, r31, 2
	ctx.r[28].s64 = ctx.r[31].s64 + 131072;
	// 82429124: 3F7F0002  addis r27, r31, 2
	ctx.r[27].s64 = ctx.r[31].s64 + 131072;
	// 82429128: 3B9C004C  addi r28, r28, 0x4c
	ctx.r[28].s64 = ctx.r[28].s64 + 76;
	// 8242912C: 3B7B000C  addi r27, r27, 0xc
	ctx.r[27].s64 = ctx.r[27].s64 + 12;
	// 82429130: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82429134: 616B0050  ori r11, r11, 0x50
	ctx.r[11].u64 = ctx.r[11].u64 | 80;
	// 82429138: 611D0010  ori r29, r8, 0x10
	ctx.r[29].u64 = ctx.r[8].u64 | 16;
	// 8242913C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429140: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429144: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82429148: 409A0014  bne cr6, 0x8242915c
	if !ctx.cr[6].eq {
	pc = 0x8242915C; continue 'dispatch;
	}
	// 8242914C: 7C1F5C2E  lfsx f0, r31, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82429150: 7DBFEC2E  lfsx f13, r31, r29
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82429154: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82429158: 419A0068  beq cr6, 0x824291c0
	if ctx.cr[6].eq {
	pc = 0x824291C0; continue 'dispatch;
	}
	// 8242915C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82429160: 7FDF5A14  add r30, r31, r11
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82429164: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429168: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8242916C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82429170: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82429174: C80B2418  lfd f0, 0x2418(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9240 as u32) ) };
	// 82429178: FC2D0032  fmul f1, f13, f0
	ctx.f[1].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 8242917C: 48109E5D  bl 0x82532fd8
	ctx.lr = 0x82429180;
	sub_82532FD8(ctx, base);
	// 82429180: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82429184: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82429188: C82B2020  lfd f1, 0x2020(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8224 as u32) ) };
	// 8242918C: 48109E4D  bl 0x82532fd8
	ctx.lr = 0x82429190;
	sub_82532FD8(ctx, base);
	// 82429190: FDBF0824  fdiv f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[31].f64 / ctx.f[1].f64;
	// 82429194: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82429198: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242919C: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 824291A0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824291A4: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 824291A8: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824291AC: 4815AECD  bl 0x82584078
	ctx.lr = 0x824291B0;
	sub_82584078(ctx, base);
	// 824291B0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824291B4: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824291B8: 7C1FED2E  stfsx f0, r31, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824291BC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824291C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824291C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824291C8: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 824291CC: 4810BF38  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824291D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824291D0 size=152
    let mut pc: u32 = 0x824291D0;
    'dispatch: loop {
        match pc {
            0x824291D0 => {
    //   block [0x824291D0..0x82429268)
	// 824291D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824291D4: 4810BEE1  bl 0x825350b4
	ctx.lr = 0x824291D8;
	sub_82535080(ctx, base);
	// 824291D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824291DC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 824291E0: 3F830002  addis r28, r3, 2
	ctx.r[28].s64 = ctx.r[3].s64 + 131072;
	// 824291E4: 616B0044  ori r11, r11, 0x44
	ctx.r[11].u64 = ctx.r[11].u64 | 68;
	// 824291E8: 3B9C0040  addi r28, r28, 0x40
	ctx.r[28].s64 = ctx.r[28].s64 + 64;
	// 824291EC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824291F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824291F4: 7FC3582E  lwzx r30, r3, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824291F8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824291FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429200: 40990044  ble cr6, 0x82429244
	if !ctx.cr[6].gt {
	pc = 0x82429244; continue 'dispatch;
	}
	// 82429204: 3FE30002  addis r31, r3, 2
	ctx.r[31].s64 = ctx.r[3].s64 + 131072;
	// 82429208: 3BFF0038  addi r31, r31, 0x38
	ctx.r[31].s64 = ctx.r[31].s64 + 56;
	// 8242920C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429210: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82429214: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429218: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242921C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82429220: 4E800421  bctrl
	ctx.lr = 0x82429224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82429224: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82429228: 41980008  blt cr6, 0x82429230
	if ctx.cr[6].lt {
	pc = 0x82429230; continue 'dispatch;
	}
	// 8242922C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429230: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429234: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82429238: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8242923C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82429240: 4198FFCC  blt cr6, 0x8242920c
	if ctx.cr[6].lt {
	pc = 0x8242920C; continue 'dispatch;
	}
	// 82429244: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82429248: 4099000C  ble cr6, 0x82429254
	if !ctx.cr[6].gt {
	pc = 0x82429254; continue 'dispatch;
	}
	// 8242924C: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82429250: 4800000C  b 0x8242925c
	pc = 0x8242925C; continue 'dispatch;
	// 82429254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82429258: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242925C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82429264: 4810BEA0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429268 size=116
    let mut pc: u32 = 0x82429268;
    'dispatch: loop {
        match pc {
            0x82429268 => {
    //   block [0x82429268..0x824292DC)
	// 82429268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82429270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82429274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429278: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242927C: 3BEB9904  addi r31, r11, -0x66fc
	ctx.r[31].s64 = ctx.r[11].s64 + -26364;
	// 82429280: 391FFFFC  addi r8, r31, -4
	ctx.r[8].s64 = ctx.r[31].s64 + -4;
	// 82429284: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82429288: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242928C: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82429290: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82429294: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82429298: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242929C: 4082FFE8  bne 0x82429284
	if !ctx.cr[0].eq {
	pc = 0x82429284; continue 'dispatch;
	}
	// 824292A0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824292A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824292A8: 409A001C  bne cr6, 0x824292c4
	if !ctx.cr[6].eq {
	pc = 0x824292C4; continue 'dispatch;
	}
	// 824292AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824292B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824292B4: 419A0010  beq cr6, 0x824292c4
	if ctx.cr[6].eq {
	pc = 0x824292C4; continue 'dispatch;
	}
	// 824292B8: 48003C81  bl 0x8242cf38
	ctx.lr = 0x824292BC;
	sub_8242CF38(ctx, base);
	// 824292BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824292C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824292C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824292C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824292CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824292D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824292D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824292D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824292E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824292E0 size=672
    let mut pc: u32 = 0x824292E0;
    'dispatch: loop {
        match pc {
            0x824292E0 => {
    //   block [0x824292E0..0x82429580)
	// 824292E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824292E4: 4810BDAD  bl 0x82535090
	ctx.lr = 0x824292E8;
	sub_82535080(ctx, base);
	// 824292E8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824292EC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824292F0: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 824292F4: 3F780002  addis r27, r24, 2
	ctx.r[27].s64 = ctx.r[24].s64 + 131072;
	// 824292F8: 56AB502A  slwi r11, r21, 0xa
	ctx.r[11].u32 = ctx.r[21].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824292FC: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 82429300: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82429304: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82429308: 3A6B0008  addi r19, r11, 8
	ctx.r[19].s64 = ctx.r[11].s64 + 8;
	// 8242930C: 7E97A378  mr r23, r20
	ctx.r[23].u64 = ctx.r[20].u64;
	// 82429310: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429314: 7E799B78  mr r25, r19
	ctx.r[25].u64 = ctx.r[19].u64;
	// 82429318: 556B083D  rlwinm. r11, r11, 1, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242931C: 408101E0  ble 0x824294fc
	if !ctx.cr[0].gt {
	pc = 0x824294FC; continue 'dispatch;
	}
	// 82429320: 3ED80002  addis r22, r24, 2
	ctx.r[22].s64 = ctx.r[24].s64 + 131072;
	// 82429324: 7D725B78  mr r18, r11
	ctx.r[18].u64 = ctx.r[11].u64;
	// 82429328: 3AD60038  addi r22, r22, 0x38
	ctx.r[22].s64 = ctx.r[22].s64 + 56;
	// 8242932C: 80760000  lwz r3, 0(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429330: 56FC083C  slwi r28, r23, 1
	ctx.r[28].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82429334: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82429338: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242933C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82429340: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429344: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82429348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242934C: 4E800421  bctrl
	ctx.lr = 0x82429350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82429350: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429354: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82429358: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8242935C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82429360: 4099004C  ble cr6, 0x824293ac
	if !ctx.cr[6].gt {
	pc = 0x824293AC; continue 'dispatch;
	}
	// 82429364: 3FB80002  addis r29, r24, 2
	ctx.r[29].s64 = ctx.r[24].s64 + 131072;
	// 82429368: 3BE1006C  addi r31, r1, 0x6c
	ctx.r[31].s64 = ctx.r[1].s64 + 108;
	// 8242936C: 3BBD003C  addi r29, r29, 0x3c
	ctx.r[29].s64 = ctx.r[29].s64 + 60;
	// 82429370: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429374: 38DFFFFC  addi r6, r31, -4
	ctx.r[6].s64 = ctx.r[31].s64 + -4;
	// 82429378: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242937C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82429380: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429384: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82429388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242938C: 4E800421  bctrl
	ctx.lr = 0x82429390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82429390: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429394: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82429398: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242939C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824293A0: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824293A4: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824293A8: 4198FFC8  blt cr6, 0x82429370
	if ctx.cr[6].lt {
	pc = 0x82429370; continue 'dispatch;
	}
	// 824293AC: 557AF87E  srwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 824293B0: 7F1AB800  cmpw cr6, r26, r23
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[23].s32, &mut ctx.xer);
	// 824293B4: 41980008  blt cr6, 0x824293bc
	if ctx.cr[6].lt {
	pc = 0x824293BC; continue 'dispatch;
	}
	// 824293B8: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 824293BC: 811B0000  lwz r8, 0(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824293C0: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824293C4: 40810028  ble 0x824293ec
	if !ctx.cr[0].gt {
	pc = 0x824293EC; continue 'dispatch;
	}
	// 824293C8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824293CC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824293D0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 824293D4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824293D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824293DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824293E0: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824293E4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824293E8: 4082FFEC  bne 0x824293d4
	if !ctx.cr[0].eq {
	pc = 0x824293D4; continue 'dispatch;
	}
	// 824293EC: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 824293F0: 419A004C  beq cr6, 0x8242943c
	if ctx.cr[6].eq {
	pc = 0x8242943C; continue 'dispatch;
	}
	// 824293F4: 2F080002  cmpwi cr6, r8, 2
	ctx.cr[6].compare_i32(ctx.r[8].s32, 2, &mut ctx.xer);
	// 824293F8: 409A0078  bne cr6, 0x82429470
	if !ctx.cr[6].eq {
	pc = 0x82429470; continue 'dispatch;
	}
	// 824293FC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82429400: 40990070  ble cr6, 0x82429470
	if !ctx.cr[6].gt {
	pc = 0x82429470; continue 'dispatch;
	}
	// 82429404: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82429408: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242940C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82429410: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82429414: 7D285A2E  lhzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82429418: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242941C: B1390000  sth r9, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82429420: 39390002  addi r9, r25, 2
	ctx.r[9].s64 = ctx.r[25].s64 + 2;
	// 82429424: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429428: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242942C: 3B290002  addi r25, r9, 2
	ctx.r[25].s64 = ctx.r[9].s64 + 2;
	// 82429430: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82429434: 4082FFE0  bne 0x82429414
	if !ctx.cr[0].eq {
	pc = 0x82429414; continue 'dispatch;
	}
	// 82429438: 48000038  b 0x82429470
	pc = 0x82429470; continue 'dispatch;
	// 8242943C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82429440: 40990030  ble cr6, 0x82429470
	if !ctx.cr[6].gt {
	pc = 0x82429470; continue 'dispatch;
	}
	// 82429444: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82429448: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 8242944C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429450: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82429454: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429458: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8242945C: B1390000  sth r9, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82429460: 39390002  addi r9, r25, 2
	ctx.r[9].s64 = ctx.r[25].s64 + 2;
	// 82429464: 3B290002  addi r25, r9, 2
	ctx.r[25].s64 = ctx.r[9].s64 + 2;
	// 82429468: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8242946C: 4082FFE0  bne 0x8242944c
	if !ctx.cr[0].eq {
	pc = 0x8242944C; continue 'dispatch;
	}
	// 82429470: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429474: 575C083C  slwi r28, r26, 1
	ctx.r[28].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82429478: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8242947C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429480: 40990070  ble cr6, 0x824294f0
	if !ctx.cr[6].gt {
	pc = 0x824294F0; continue 'dispatch;
	}
	// 82429484: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 82429488: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 8242948C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82429490: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82429494: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82429498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242949C: 4BFFFA6D  bl 0x82428f08
	ctx.lr = 0x824294A0;
	sub_82428F08(ctx, base);
	// 824294A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824294A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824294A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824294AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824294B0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824294B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824294B8: 4E800421  bctrl
	ctx.lr = 0x824294BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824294BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824294C0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824294C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824294C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824294CC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824294D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824294D4: 4E800421  bctrl
	ctx.lr = 0x824294D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824294D8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824294DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824294E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824294E4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824294E8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824294EC: 4198FFA0  blt cr6, 0x8242948c
	if ctx.cr[6].lt {
	pc = 0x8242948C; continue 'dispatch;
	}
	// 824294F0: 3652FFFF  addic. r18, r18, -1
	ctx.xer.ca = (ctx.r[18].u32 > (!(-1 as u32)));
	ctx.r[18].s64 = ctx.r[18].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 824294F4: 7EFAB850  subf r23, r26, r23
	ctx.r[23].s64 = ctx.r[23].s64 - ctx.r[26].s64;
	// 824294F8: 4082FE34  bne 0x8242932c
	if !ctx.cr[0].eq {
	pc = 0x8242932C; continue 'dispatch;
	}
	// 824294FC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82429500: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82429504: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82429508: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8242950C: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82429510: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82429514: 4200FFF8  bdnz 0x8242950c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242950C; continue 'dispatch;
	}
	// 82429518: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242951C: 92610070  stw r19, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[19].u32 ) };
	// 82429520: 7D57A050  subf r10, r23, r20
	ctx.r[10].s64 = ctx.r[20].s64 - ctx.r[23].s64;
	// 82429524: 92A100C4  stw r21, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[21].u32 ) };
	// 82429528: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8242952C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82429530: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82429534: 7C78582E  lwzx r3, r24, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82429538: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242953C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82429540: 4815A849  bl 0x82583d88
	ctx.lr = 0x82429544;
	sub_82583D88(ctx, base);
	// 82429544: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82429548: 4080002C  bge 0x82429574
	if !ctx.cr[0].lt {
	pc = 0x82429574; continue 'dispatch;
	}
	// 8242954C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429550: 388B2598  addi r4, r11, 0x2598
	ctx.r[4].s64 = ctx.r[11].s64 + 9624;
	// 82429554: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82429558: 3BEB9800  addi r31, r11, -0x6800
	ctx.r[31].s64 = ctx.r[11].s64 + -26624;
	// 8242955C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429560: 4811DA49  bl 0x82546fa8
	ctx.lr = 0x82429564;
	sub_82546FA8(ctx, base);
	// 82429564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429568: 4BFF36A1  bl 0x8241cc08
	ctx.lr = 0x8242956C;
	sub_8241CC08(ctx, base);
	// 8242956C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82429570: 48000008  b 0x82429578
	pc = 0x82429578; continue 'dispatch;
	// 82429574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429578: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 8242957C: 4810BB64  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429580 size=64
    let mut pc: u32 = 0x82429580;
    'dispatch: loop {
        match pc {
            0x82429580 => {
    //   block [0x82429580..0x824295C0)
	// 82429580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82429588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242958C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429590: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 82429594: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429598: 48003A31  bl 0x8242cfc8
	ctx.lr = 0x8242959C;
	sub_8242CFC8(ctx, base);
	// 8242959C: 4815ACAD  bl 0x82584248
	ctx.lr = 0x824295A0;
	sub_82584248(ctx, base);
	// 824295A0: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 824295A4: 48003ABD  bl 0x8242d060
	ctx.lr = 0x824295A8;
	sub_8242D060(ctx, base);
	// 824295A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824295AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824295B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824295B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824295B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824295BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824295C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824295C0 size=172
    let mut pc: u32 = 0x824295C0;
    'dispatch: loop {
        match pc {
            0x824295C0 => {
    //   block [0x824295C0..0x8242966C)
	// 824295C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824295C4: 4810BAF5  bl 0x825350b8
	ctx.lr = 0x824295C8;
	sub_82535080(ctx, base);
	// 824295C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824295CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824295D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824295D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824295D8: 409A0018  bne cr6, 0x824295f0
	if !ctx.cr[6].eq {
	pc = 0x824295F0; continue 'dispatch;
	}
	// 824295DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824295E0: 386B26F8  addi r3, r11, 0x26f8
	ctx.r[3].s64 = ctx.r[11].s64 + 9976;
	// 824295E4: 4BFF3625  bl 0x8241cc08
	ctx.lr = 0x824295E8;
	sub_8241CC08(ctx, base);
	// 824295E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824295EC: 48000078  b 0x82429664
	pc = 0x82429664; continue 'dispatch;
	// 824295F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824295F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824295F8: 409A0010  bne cr6, 0x82429608
	if !ctx.cr[6].eq {
	pc = 0x82429608; continue 'dispatch;
	}
	// 824295FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429600: 386B26D0  addi r3, r11, 0x26d0
	ctx.r[3].s64 = ctx.r[11].s64 + 9936;
	// 82429604: 4BFFFFE0  b 0x824295e4
	pc = 0x824295E4; continue 'dispatch;
	// 82429608: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8242960C: 419A0054  beq cr6, 0x82429660
	if ctx.cr[6].eq {
	pc = 0x82429660; continue 'dispatch;
	}
	// 82429610: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429614: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 82429618: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8242961C: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82429620: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82429624: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82429628: EBBE0000  ld r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8242962C: 4815A97D  bl 0x82583fa8
	ctx.lr = 0x82429630;
	sub_82583FA8(ctx, base);
	// 82429630: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82429634: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82429638: 7F2BE800  cmpd cr6, r11, r29
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[29].s64, &mut ctx.xer);
	// 8242963C: 409A0008  bne cr6, 0x82429644
	if !ctx.cr[6].eq {
	pc = 0x82429644; continue 'dispatch;
	}
	// 82429640: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82429644: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82429648: 794AF0A2  rldicl r10, r10, 0x3e, 0x22
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000003u64;
	// 8242964C: 61290048  ori r9, r9, 0x48
	ctx.r[9].u64 = ctx.r[9].u64 | 72;
	// 82429650: 7D3F4AAA  lwax r9, r31, r9
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32)) } as i32) as i64;
	// 82429654: 7D6959D2  mulld r11, r9, r11
	ctx.r[11].s64 = ctx.r[9].s64 * ctx.r[11].s64;
	// 82429658: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8242965C: F97C0000  std r11, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82429660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429664: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82429668: 4810BAA0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429670 size=272
    let mut pc: u32 = 0x82429670;
    'dispatch: loop {
        match pc {
            0x82429670 => {
    //   block [0x82429670..0x82429780)
	// 82429670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429674: 4810BA49  bl 0x825350bc
	ctx.lr = 0x82429678;
	sub_82535080(ctx, base);
	// 82429678: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8242967C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429680: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429684: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82429688: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242968C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82429690: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82429694: 409A0010  bne cr6, 0x824296a4
	if !ctx.cr[6].eq {
	pc = 0x824296A4; continue 'dispatch;
	}
	// 82429698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242969C: 386B28E0  addi r3, r11, 0x28e0
	ctx.r[3].s64 = ctx.r[11].s64 + 10464;
	// 824296A0: 480000CC  b 0x8242976c
	pc = 0x8242976C; continue 'dispatch;
	// 824296A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824296A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824296AC: 409A0010  bne cr6, 0x824296bc
	if !ctx.cr[6].eq {
	pc = 0x824296BC; continue 'dispatch;
	}
	// 824296B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824296B4: 386B28B8  addi r3, r11, 0x28b8
	ctx.r[3].s64 = ctx.r[11].s64 + 10424;
	// 824296B8: 480000B4  b 0x8242976c
	pc = 0x8242976C; continue 'dispatch;
	// 824296BC: 2B1F0002  cmplwi cr6, r31, 2
	ctx.cr[6].compare_u32(ctx.r[31].u32, 2 as u32, &mut ctx.xer);
	// 824296C0: 419900A4  bgt cr6, 0x82429764
	if ctx.cr[6].gt {
	pc = 0x82429764; continue 'dispatch;
	}
	// 824296C4: 2B1D0006  cmplwi cr6, r29, 6
	ctx.cr[6].compare_u32(ctx.r[29].u32, 6 as u32, &mut ctx.xer);
	// 824296C8: 4199009C  bgt cr6, 0x82429764
	if ctx.cr[6].gt {
	pc = 0x82429764; continue 'dispatch;
	}
	// 824296CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824296D0: C82B2260  lfd f1, 0x2260(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8800 as u32) ) };
	// 824296D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824296D8: C80B2410  lfd f0, 0x2410(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9232 as u32) ) };
	// 824296DC: FC5F0032  fmul f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 824296E0: 4810A091  bl 0x82533770
	ctx.lr = 0x824296E4;
	sub_82533770(ctx, base);
	// 824296E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824296E8: C00B1BB8  lfs f0, 0x1bb8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824296EC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 824296F0: FFE00818  frsp f31, f1
	ctx.f[31].f64 = (ctx.f[1].f64 as f32) as f64;
	// 824296F4: 4199000C  bgt cr6, 0x82429700
	if ctx.cr[6].gt {
	pc = 0x82429700; continue 'dispatch;
	}
	// 824296F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824296FC: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82429700: 1D7F0006  mulli r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 * 6;
	// 82429704: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82429708: 39210068  addi r9, r1, 0x68
	ctx.r[9].s64 = ctx.r[1].s64 + 104;
	// 8242970C: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82429710: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429714: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82429718: 616A0008  ori r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64 | 8;
	// 8242971C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82429720: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82429724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82429728: 9BE10068  stb r31, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u8 ) };
	// 8242972C: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82429730: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82429734: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 82429738: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8242973C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82429740: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82429744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82429748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8242974C: 4815A58D  bl 0x82583cd8
	ctx.lr = 0x82429750;
	sub_82583CD8(ctx, base);
	// 82429750: 397F400B  addi r11, r31, 0x400b
	ctx.r[11].s64 = ctx.r[31].s64 + 16395;
	// 82429754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429758: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242975C: 7FEBF52E  stfsx f31, r11, r30
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 82429760: 48000014  b 0x82429774
	pc = 0x82429774; continue 'dispatch;
	// 82429764: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429768: 386B288C  addi r3, r11, 0x288c
	ctx.r[3].s64 = ctx.r[11].s64 + 10380;
	// 8242976C: 4BFF349D  bl 0x8241cc08
	ctx.lr = 0x82429770;
	sub_8241CC08(ctx, base);
	// 82429770: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82429774: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82429778: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8242977C: 4810B990  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429780 size=80
    let mut pc: u32 = 0x82429780;
    'dispatch: loop {
        match pc {
            0x82429780 => {
    //   block [0x82429780..0x824297D0)
	// 82429780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82429788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242978C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82429790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429794: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82429798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242979C: 3BEB990C  addi r31, r11, -0x66f4
	ctx.r[31].s64 = ctx.r[11].s64 + -26356;
	// 824297A0: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824297A4: 48003825  bl 0x8242cfc8
	ctx.lr = 0x824297A8;
	sub_8242CFC8(ctx, base);
	// 824297A8: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824297AC: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 824297B0: 480038B1  bl 0x8242d060
	ctx.lr = 0x824297B4;
	sub_8242D060(ctx, base);
	// 824297B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824297B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824297BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824297C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824297C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824297C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824297CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824297D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824297D0 size=80
    let mut pc: u32 = 0x824297D0;
    'dispatch: loop {
        match pc {
            0x824297D0 => {
    //   block [0x824297D0..0x82429820)
	// 824297D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824297D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824297D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824297DC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824297E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824297E4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824297E8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824297EC: 3BEB9914  addi r31, r11, -0x66ec
	ctx.r[31].s64 = ctx.r[11].s64 + -26348;
	// 824297F0: 807FFFF0  lwz r3, -0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 824297F4: 480037D5  bl 0x8242cfc8
	ctx.lr = 0x824297F8;
	sub_8242CFC8(ctx, base);
	// 824297F8: 807FFFF0  lwz r3, -0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 824297FC: D3FF0000  stfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82429800: 48003861  bl 0x8242d060
	ctx.lr = 0x82429804;
	sub_8242D060(ctx, base);
	// 82429804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242980C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82429810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82429814: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82429818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242981C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429820 size=152
    let mut pc: u32 = 0x82429820;
    'dispatch: loop {
        match pc {
            0x82429820 => {
    //   block [0x82429820..0x824298B8)
	// 82429820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82429828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242982C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82429830: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82429834: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429838: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242983C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82429840: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429844: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429848: 48003781  bl 0x8242cfc8
	ctx.lr = 0x8242984C;
	sub_8242CFC8(ctx, base);
	// 8242984C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82429850: C82B2260  lfd f1, 0x2260(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8800 as u32) ) };
	// 82429854: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429858: C80B2410  lfd f0, 0x2410(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9232 as u32) ) };
	// 8242985C: FC5F0032  fmul f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 82429860: 48109F11  bl 0x82533770
	ctx.lr = 0x82429864;
	sub_82533770(ctx, base);
	// 82429864: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429868: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8242986C: C00B1BB8  lfs f0, 0x1bb8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82429870: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82429874: 4199000C  bgt cr6, 0x82429880
	if ctx.cr[6].gt {
	pc = 0x82429880; continue 'dispatch;
	}
	// 82429878: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8242987C: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82429880: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429884: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82429888: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242988C: 4815A775  bl 0x82584000
	ctx.lr = 0x82429890;
	sub_82584000(ctx, base);
	// 82429890: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429894: 480037CD  bl 0x8242d060
	ctx.lr = 0x82429898;
	sub_8242D060(ctx, base);
	// 82429898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242989C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824298A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824298A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824298A8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824298AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824298B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824298B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824298B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824298B8 size=136
    let mut pc: u32 = 0x824298B8;
    'dispatch: loop {
        match pc {
            0x824298B8 => {
    //   block [0x824298B8..0x82429940)
	// 824298B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824298BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824298C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824298C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824298C8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824298CC: 3BEB97E0  addi r31, r11, -0x6820
	ctx.r[31].s64 = ctx.r[11].s64 + -26656;
	// 824298D0: 391F0120  addi r8, r31, 0x120
	ctx.r[8].s64 = ctx.r[31].s64 + 288;
	// 824298D4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 824298D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824298DC: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 824298E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824298E4: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824298E8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824298EC: 4082FFE8  bne 0x824298d4
	if !ctx.cr[0].eq {
	pc = 0x824298D4; continue 'dispatch;
	}
	// 824298F0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824298F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824298F8: 409A0030  bne cr6, 0x82429928
	if !ctx.cr[6].eq {
	pc = 0x82429928; continue 'dispatch;
	}
	// 824298FC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82429900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429904: 48003555  bl 0x8242ce58
	ctx.lr = 0x82429908;
	sub_8242CE58(ctx, base);
	// 82429908: 907F0124  stw r3, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[3].u32 ) };
	// 8242990C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82429910: 40820018  bne 0x82429928
	if !ctx.cr[0].eq {
	pc = 0x82429928; continue 'dispatch;
	}
	// 82429914: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429918: 386B2420  addi r3, r11, 0x2420
	ctx.r[3].s64 = ctx.r[11].s64 + 9248;
	// 8242991C: 4BFF32ED  bl 0x8241cc08
	ctx.lr = 0x82429920;
	sub_8241CC08(ctx, base);
	// 82429920: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82429924: 48000008  b 0x8242992c
	pc = 0x8242992C; continue 'dispatch;
	// 82429928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242992C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82429930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82429934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82429938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242993C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429940 size=596
    let mut pc: u32 = 0x82429940;
    'dispatch: loop {
        match pc {
            0x82429940 => {
    //   block [0x82429940..0x82429B94)
	// 82429940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429944: 4810B765  bl 0x825350a8
	ctx.lr = 0x82429948;
	sub_82535080(ctx, base);
	// 82429948: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242994C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82429950: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82429954: 409A0018  bne cr6, 0x8242996c
	if !ctx.cr[6].eq {
	pc = 0x8242996C; continue 'dispatch;
	}
	// 82429958: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242995C: 386B2A90  addi r3, r11, 0x2a90
	ctx.r[3].s64 = ctx.r[11].s64 + 10896;
	// 82429960: 4BFF32A9  bl 0x8241cc08
	ctx.lr = 0x82429964;
	sub_8241CC08(ctx, base);
	// 82429964: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82429968: 48000224  b 0x82429b8c
	pc = 0x82429B8C; continue 'dispatch;
	// 8242996C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82429970: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82429974: 3BCB9908  addi r30, r11, -0x66f8
	ctx.r[30].s64 = ctx.r[11].s64 + -26360;
	// 82429978: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8242997C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82429980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82429984: 409A0034  bne cr6, 0x824299b8
	if !ctx.cr[6].eq {
	pc = 0x824299B8; continue 'dispatch;
	}
	// 82429988: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242998C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82429990: 409A0014  bne cr6, 0x824299a4
	if !ctx.cr[6].eq {
	pc = 0x824299A4; continue 'dispatch;
	}
	// 82429994: 4BFA9DC5  bl 0x823d3758
	ctx.lr = 0x82429998;
	sub_823D3758(ctx, base);
	// 82429998: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8242999C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824299A0: 41820040  beq 0x824299e0
	if ctx.cr[0].eq {
	pc = 0x824299E0; continue 'dispatch;
	}
	// 824299A4: 3CA00002  lis r5, 2
	ctx.r[5].s64 = 131072;
	// 824299A8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824299AC: 60A500C0  ori r5, r5, 0xc0
	ctx.r[5].u64 = ctx.r[5].u64 | 192;
	// 824299B0: 4BFA8921  bl 0x823d22d0
	ctx.lr = 0x824299B4;
	sub_823D22D0(ctx, base);
	// 824299B4: 48000020  b 0x824299d4
	pc = 0x824299D4; continue 'dispatch;
	// 824299B8: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 824299BC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 824299C0: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 824299C4: 608400C0  ori r4, r4, 0xc0
	ctx.r[4].u64 = ctx.r[4].u64 | 192;
	// 824299C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824299CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824299D0: 4E800421  bctrl
	ctx.lr = 0x824299D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824299D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824299D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824299DC: 409A0010  bne cr6, 0x824299ec
	if !ctx.cr[6].eq {
	pc = 0x824299EC; continue 'dispatch;
	}
	// 824299E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824299E4: 386B2A68  addi r3, r11, 0x2a68
	ctx.r[3].s64 = ctx.r[11].s64 + 10856;
	// 824299E8: 4BFFFF78  b 0x82429960
	pc = 0x82429960; continue 'dispatch;
	// 824299EC: 3CA00002  lis r5, 2
	ctx.r[5].s64 = 131072;
	// 824299F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824299F4: 60A500C0  ori r5, r5, 0xc0
	ctx.r[5].u64 = ctx.r[5].u64 | 192;
	// 824299F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824299FC: 4810B7D5  bl 0x825351d0
	ctx.lr = 0x82429A00;
	sub_825351D0(ctx, base);
	// 82429A00: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82429A04: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82429A08: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82429A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82429A10: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82429A14: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82429A18: 4200FFF8  bdnz 0x82429a10
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82429A10; continue 'dispatch;
	}
	// 82429A1C: 391E0004  addi r8, r30, 4
	ctx.r[8].s64 = ctx.r[30].s64 + 4;
	// 82429A20: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82429A24: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82429A28: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 82429A2C: 38FE000C  addi r7, r30, 0xc
	ctx.r[7].s64 = ctx.r[30].s64 + 12;
	// 82429A30: 396B3ED0  addi r11, r11, 0x3ed0
	ctx.r[11].s64 = ctx.r[11].s64 + 16080;
	// 82429A34: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 82429A38: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429A3C: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82429A40: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82429A44: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82429A48: 3B400002  li r26, 2
	ctx.r[26].s64 = 2;
	// 82429A4C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82429A50: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82429A54: 994100A8  stb r10, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[10].u8 ) };
	// 82429A58: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429A5C: 89060000  lbz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429A60: 9B410074  stb r26, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[26].u8 ) };
	// 82429A64: 9B610070  stb r27, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u8 ) };
	// 82429A68: 994100A9  stb r10, 0xa9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(169 as u32), ctx.r[10].u8 ) };
	// 82429A6C: 3D408243  lis r10, -0x7dbd
	ctx.r[10].s64 = -2109538304;
	// 82429A70: 990100AA  stb r8, 0xaa(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(170 as u32), ctx.r[8].u8 ) };
	// 82429A74: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429A78: 394A90C8  addi r10, r10, -0x6f38
	ctx.r[10].s64 = ctx.r[10].s64 + -28472;
	// 82429A7C: 89290000  lbz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429A80: 2B09000C  cmplwi cr6, r9, 0xc
	ctx.cr[6].compare_u32(ctx.r[9].u32, 12 as u32, &mut ctx.xer);
	// 82429A84: 910100B4  stw r8, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[8].u32 ) };
	// 82429A88: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 82429A8C: 914100C0  stw r10, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 82429A90: 611DBB80  ori r29, r8, 0xbb80
	ctx.r[29].u64 = ctx.r[8].u64 | 48000;
	// 82429A94: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82429A98: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82429A9C: 990100AB  stb r8, 0xab(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(171 as u32), ctx.r[8].u8 ) };
	// 82429AA0: 40980010  bge cr6, 0x82429ab0
	if !ctx.cr[6].lt {
	pc = 0x82429AB0; continue 'dispatch;
	}
	// 82429AA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82429AA8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429AAC: 48000008  b 0x82429ab4
	pc = 0x82429AB4; continue 'dispatch;
	// 82429AB0: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82429AB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429AB8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82429ABC: 3C9F0002  addis r4, r31, 2
	ctx.r[4].s64 = ctx.r[31].s64 + 131072;
	// 82429AC0: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82429AC4: 3B8B23B0  addi r28, r11, 0x23b0
	ctx.r[28].s64 = ctx.r[11].s64 + 9136;
	// 82429AC8: 9B610060  stb r27, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 82429ACC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82429AD0: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 82429AD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82429AD8: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82429ADC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82429AE0: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82429AE4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82429AE8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82429AEC: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82429AF0: 4815A7D9  bl 0x825842c8
	ctx.lr = 0x82429AF4;
	sub_825842C8(ctx, base);
	// 82429AF4: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82429AF8: 4080001C  bge 0x82429b14
	if !ctx.cr[0].lt {
	pc = 0x82429B14; continue 'dispatch;
	}
	// 82429AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429B00: 387EFEF8  addi r3, r30, -0x108
	ctx.r[3].s64 = ctx.r[30].s64 + -264;
	// 82429B04: 388B2A28  addi r4, r11, 0x2a28
	ctx.r[4].s64 = ctx.r[11].s64 + 10792;
	// 82429B08: 4811D4A1  bl 0x82546fa8
	ctx.lr = 0x82429B0C;
	sub_82546FA8(ctx, base);
	// 82429B0C: 387EFEF8  addi r3, r30, -0x108
	ctx.r[3].s64 = ctx.r[30].s64 + -264;
	// 82429B10: 4BFFFE50  b 0x82429960
	pc = 0x82429960; continue 'dispatch;
	// 82429B14: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 82429B18: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82429B1C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82429B20: 38630054  addi r3, r3, 0x54
	ctx.r[3].s64 = ctx.r[3].s64 + 84;
	// 82429B24: 4BF970DD  bl 0x823c0c00
	ctx.lr = 0x82429B28;
	sub_823C0C00(ctx, base);
	// 82429B28: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429B2C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82429B30: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82429B34: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82429B38: 616B0030  ori r11, r11, 0x30
	ctx.r[11].u64 = ctx.r[11].u64 | 48;
	// 82429B3C: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 82429B40: 794A0040  clrldi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 82429B44: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82429B48: 3CE00002  lis r7, 2
	ctx.r[7].s64 = 131072;
	// 82429B4C: 6108000C  ori r8, r8, 0xc
	ctx.r[8].u64 = ctx.r[8].u64 | 12;
	// 82429B50: 7F5F492E  stwx r26, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[26].u32) };
	// 82429B54: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82429B58: 7D5F592A  stdx r10, r31, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u64) };
	// 82429B5C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429B60: 60E7004C  ori r7, r7, 0x4c
	ctx.r[7].u64 = ctx.r[7].u64 | 76;
	// 82429B64: 616B0014  ori r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u64 | 20;
	// 82429B68: 7FBF412E  stwx r29, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u32) };
	// 82429B6C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82429B70: 61290048  ori r9, r9, 0x48
	ctx.r[9].u64 = ctx.r[9].u64 | 72;
	// 82429B74: 39000100  li r8, 0x100
	ctx.r[8].s64 = 256;
	// 82429B78: 7FBF392E  stwx r29, r31, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[29].u32) };
	// 82429B7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429B80: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82429B84: 7D1F492E  stwx r8, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82429B88: 93F80000  stw r31, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82429B8C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82429B90: 4810B568  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429B98 size=264
    let mut pc: u32 = 0x82429B98;
    'dispatch: loop {
        match pc {
            0x82429B98 => {
    //   block [0x82429B98..0x82429CA0)
	// 82429B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82429BA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82429BA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82429BA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429BAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429BB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82429BB4: 409A0018  bne cr6, 0x82429bcc
	if !ctx.cr[6].eq {
	pc = 0x82429BCC; continue 'dispatch;
	}
	// 82429BB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429BBC: 386B2AE0  addi r3, r11, 0x2ae0
	ctx.r[3].s64 = ctx.r[11].s64 + 10976;
	// 82429BC0: 4BFF3049  bl 0x8241cc08
	ctx.lr = 0x82429BC4;
	sub_8241CC08(ctx, base);
	// 82429BC4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82429BC8: 480000C0  b 0x82429c88
	pc = 0x82429C88; continue 'dispatch;
	// 82429BCC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429BD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429BD4: 409A0010  bne cr6, 0x82429be4
	if !ctx.cr[6].eq {
	pc = 0x82429BE4; continue 'dispatch;
	}
	// 82429BD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429BDC: 386B2AB8  addi r3, r11, 0x2ab8
	ctx.r[3].s64 = ctx.r[11].s64 + 10936;
	// 82429BE0: 4BFFFFE0  b 0x82429bc0
	pc = 0x82429BC0; continue 'dispatch;
	// 82429BE4: 3FFE0002  addis r31, r30, 2
	ctx.r[31].s64 = ctx.r[30].s64 + 131072;
	// 82429BE8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82429BEC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429BF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82429BF4: 41820024  beq 0x82429c18
	if ctx.cr[0].eq {
	pc = 0x82429C18; continue 'dispatch;
	}
	// 82429BF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82429BFC: 4815A2ED  bl 0x82583ee8
	ctx.lr = 0x82429C00;
	sub_82583EE8(ctx, base);
	// 82429C00: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429C04: 4815A1E5  bl 0x82583de8
	ctx.lr = 0x82429C08;
	sub_82583DE8(ctx, base);
	// 82429C08: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429C0C: 48159F9D  bl 0x82583ba8
	ctx.lr = 0x82429C10;
	sub_82583BA8(ctx, base);
	// 82429C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82429C14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82429C18: 3CA00002  lis r5, 2
	ctx.r[5].s64 = 131072;
	// 82429C1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82429C20: 60A500C0  ori r5, r5, 0xc0
	ctx.r[5].u64 = ctx.r[5].u64 | 192;
	// 82429C24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82429C28: 4810B5A9  bl 0x825351d0
	ctx.lr = 0x82429C2C;
	sub_825351D0(ctx, base);
	// 82429C2C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82429C30: 3BEB9908  addi r31, r11, -0x66f8
	ctx.r[31].s64 = ctx.r[11].s64 + -26360;
	// 82429C34: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82429C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82429C3C: 409A0030  bne cr6, 0x82429c6c
	if !ctx.cr[6].eq {
	pc = 0x82429C6C; continue 'dispatch;
	}
	// 82429C40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82429C48: 409A0014  bne cr6, 0x82429c5c
	if !ctx.cr[6].eq {
	pc = 0x82429C5C; continue 'dispatch;
	}
	// 82429C4C: 4BFA9B0D  bl 0x823d3758
	ctx.lr = 0x82429C50;
	sub_823D3758(ctx, base);
	// 82429C50: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82429C54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82429C58: 4182002C  beq 0x82429c84
	if ctx.cr[0].eq {
	pc = 0x82429C84; continue 'dispatch;
	}
	// 82429C5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82429C60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82429C64: 4BFA8F55  bl 0x823d2bb8
	ctx.lr = 0x82429C68;
	sub_823D2BB8(ctx, base);
	// 82429C68: 4800001C  b 0x82429c84
	pc = 0x82429C84; continue 'dispatch;
	// 82429C6C: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82429C70: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82429C74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82429C78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429C7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82429C80: 4E800421  bctrl
	ctx.lr = 0x82429C84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82429C84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82429C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82429C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82429C94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82429C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82429C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429CA0 size=160
    let mut pc: u32 = 0x82429CA0;
    'dispatch: loop {
        match pc {
            0x82429CA0 => {
    //   block [0x82429CA0..0x82429D40)
	// 82429CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429CA4: 4810B419  bl 0x825350bc
	ctx.lr = 0x82429CA8;
	sub_82535080(ctx, base);
	// 82429CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429CAC: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 82429CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82429CB4: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429CB8: 48003311  bl 0x8242cfc8
	ctx.lr = 0x82429CBC;
	sub_8242CFC8(ctx, base);
	// 82429CBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82429CC0: 409A0018  bne cr6, 0x82429cd8
	if !ctx.cr[6].eq {
	pc = 0x82429CD8; continue 'dispatch;
	}
	// 82429CC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429CC8: 386B247C  addi r3, r11, 0x247c
	ctx.r[3].s64 = ctx.r[11].s64 + 9340;
	// 82429CCC: 4BFF2F3D  bl 0x8241cc08
	ctx.lr = 0x82429CD0;
	sub_8241CC08(ctx, base);
	// 82429CD0: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82429CD4: 48000058  b 0x82429d2c
	pc = 0x82429D2C; continue 'dispatch;
	// 82429CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429CDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429CE0: 409A0010  bne cr6, 0x82429cf0
	if !ctx.cr[6].eq {
	pc = 0x82429CF0; continue 'dispatch;
	}
	// 82429CE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429CE8: 386B2454  addi r3, r11, 0x2454
	ctx.r[3].s64 = ctx.r[11].s64 + 9300;
	// 82429CEC: 4BFFFFE0  b 0x82429ccc
	pc = 0x82429CCC; continue 'dispatch;
	// 82429CF0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429CF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82429CF8: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82429CFC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82429D00: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82429D04: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82429D08: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 82429D0C: 7FDF592A  stdx r30, r31, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u64) };
	// 82429D10: 61290028  ori r9, r9, 0x28
	ctx.r[9].u64 = ctx.r[9].u64 | 40;
	// 82429D14: 610800B8  ori r8, r8, 0xb8
	ctx.r[8].u64 = ctx.r[8].u64 | 184;
	// 82429D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82429D1C: 7FDF512A  stdx r30, r31, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u64) };
	// 82429D20: 7FDF492A  stdx r30, r31, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u64) };
	// 82429D24: 7FDF412E  stwx r30, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u32) };
	// 82429D28: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82429D2C: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429D30: 48003331  bl 0x8242d060
	ctx.lr = 0x82429D34;
	sub_8242D060(ctx, base);
	// 82429D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82429D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82429D3C: 4810B3D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429D40 size=152
    let mut pc: u32 = 0x82429D40;
    'dispatch: loop {
        match pc {
            0x82429D40 => {
    //   block [0x82429D40..0x82429DD8)
	// 82429D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429D44: 4810B375  bl 0x825350b8
	ctx.lr = 0x82429D48;
	sub_82535080(ctx, base);
	// 82429D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429D4C: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 82429D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82429D54: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429D58: 48003271  bl 0x8242cfc8
	ctx.lr = 0x82429D5C;
	sub_8242CFC8(ctx, base);
	// 82429D5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82429D60: 409A0018  bne cr6, 0x82429d78
	if !ctx.cr[6].eq {
	pc = 0x82429D78; continue 'dispatch;
	}
	// 82429D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429D68: 386B24CC  addi r3, r11, 0x24cc
	ctx.r[3].s64 = ctx.r[11].s64 + 9420;
	// 82429D6C: 4BFF2E9D  bl 0x8241cc08
	ctx.lr = 0x82429D70;
	sub_8241CC08(ctx, base);
	// 82429D70: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82429D74: 48000050  b 0x82429dc4
	pc = 0x82429DC4; continue 'dispatch;
	// 82429D78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429D7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429D80: 409A0010  bne cr6, 0x82429d90
	if !ctx.cr[6].eq {
	pc = 0x82429D90; continue 'dispatch;
	}
	// 82429D84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429D88: 386B24A4  addi r3, r11, 0x24a4
	ctx.r[3].s64 = ctx.r[11].s64 + 9380;
	// 82429D8C: 4BFFFFE0  b 0x82429d6c
	pc = 0x82429D6C; continue 'dispatch;
	// 82429D90: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429D94: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 82429D98: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82429D9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82429DA0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82429DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82429DA8: 7FBF592A  stdx r29, r31, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u64) };
	// 82429DAC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429DB0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82429DB4: 4815A135  bl 0x82583ee8
	ctx.lr = 0x82429DB8;
	sub_82583EE8(ctx, base);
	// 82429DB8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429DBC: 4815A02D  bl 0x82583de8
	ctx.lr = 0x82429DC0;
	sub_82583DE8(ctx, base);
	// 82429DC0: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82429DC4: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429DC8: 48003299  bl 0x8242d060
	ctx.lr = 0x82429DCC;
	sub_8242D060(ctx, base);
	// 82429DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82429DD4: 4810B334  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429DD8 size=120
    let mut pc: u32 = 0x82429DD8;
    'dispatch: loop {
        match pc {
            0x82429DD8 => {
    //   block [0x82429DD8..0x82429E50)
	// 82429DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429DDC: 4810B2E1  bl 0x825350bc
	ctx.lr = 0x82429DE0;
	sub_82535080(ctx, base);
	// 82429DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429DE4: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 82429DE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429DEC: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429DF0: 480031D9  bl 0x8242cfc8
	ctx.lr = 0x82429DF4;
	sub_8242CFC8(ctx, base);
	// 82429DF4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82429DF8: 409A0018  bne cr6, 0x82429e10
	if !ctx.cr[6].eq {
	pc = 0x82429E10; continue 'dispatch;
	}
	// 82429DFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429E00: 386B251C  addi r3, r11, 0x251c
	ctx.r[3].s64 = ctx.r[11].s64 + 9500;
	// 82429E04: 4BFF2E05  bl 0x8241cc08
	ctx.lr = 0x82429E08;
	sub_8241CC08(ctx, base);
	// 82429E08: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82429E0C: 48000030  b 0x82429e3c
	pc = 0x82429E3C; continue 'dispatch;
	// 82429E10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429E14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429E18: 409A0010  bne cr6, 0x82429e28
	if !ctx.cr[6].eq {
	pc = 0x82429E28; continue 'dispatch;
	}
	// 82429E1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429E20: 386B24F4  addi r3, r11, 0x24f4
	ctx.r[3].s64 = ctx.r[11].s64 + 9460;
	// 82429E24: 4BFFFFE0  b 0x82429e04
	pc = 0x82429E04; continue 'dispatch;
	// 82429E28: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429E2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82429E30: 616B00B4  ori r11, r11, 0xb4
	ctx.r[11].u64 = ctx.r[11].u64 | 180;
	// 82429E34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82429E38: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82429E3C: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429E40: 48003221  bl 0x8242d060
	ctx.lr = 0x82429E44;
	sub_8242D060(ctx, base);
	// 82429E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429E48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82429E4C: 4810B2C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82429E50 size=116
    let mut pc: u32 = 0x82429E50;
    'dispatch: loop {
        match pc {
            0x82429E50 => {
    //   block [0x82429E50..0x82429EC4)
	// 82429E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429E54: 4810B269  bl 0x825350bc
	ctx.lr = 0x82429E58;
	sub_82535080(ctx, base);
	// 82429E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429E5C: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 82429E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82429E64: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429E68: 48003161  bl 0x8242cfc8
	ctx.lr = 0x82429E6C;
	sub_8242CFC8(ctx, base);
	// 82429E6C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82429E70: 409A0018  bne cr6, 0x82429e88
	if !ctx.cr[6].eq {
	pc = 0x82429E88; continue 'dispatch;
	}
	// 82429E74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429E78: 386B256C  addi r3, r11, 0x256c
	ctx.r[3].s64 = ctx.r[11].s64 + 9580;
	// 82429E7C: 4BFF2D8D  bl 0x8241cc08
	ctx.lr = 0x82429E80;
	sub_8241CC08(ctx, base);
	// 82429E80: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82429E84: 4800002C  b 0x82429eb0
	pc = 0x82429EB0; continue 'dispatch;
	// 82429E88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429E8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429E90: 409A0010  bne cr6, 0x82429ea0
	if !ctx.cr[6].eq {
	pc = 0x82429EA0; continue 'dispatch;
	}
	// 82429E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82429E98: 386B2544  addi r3, r11, 0x2544
	ctx.r[3].s64 = ctx.r[11].s64 + 9540;
	// 82429E9C: 4BFFFFE0  b 0x82429e7c
	pc = 0x82429E7C; continue 'dispatch;
	// 82429EA0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429EA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82429EA8: 616B00B4  ori r11, r11, 0xb4
	ctx.r[11].u64 = ctx.r[11].u64 | 180;
	// 82429EAC: 7FFE592E  stwx r31, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u32) };
	// 82429EB0: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 82429EB4: 480031AD  bl 0x8242d060
	ctx.lr = 0x82429EB8;
	sub_8242D060(ctx, base);
	// 82429EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82429EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82429EC0: 4810B24C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429EC8 size=292
    let mut pc: u32 = 0x82429EC8;
    'dispatch: loop {
        match pc {
            0x82429EC8 => {
    //   block [0x82429EC8..0x82429FEC)
	// 82429EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429ECC: 4810B1DD  bl 0x825350a8
	ctx.lr = 0x82429ED0;
	sub_82535080(ctx, base);
	// 82429ED0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429ED4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82429ED8: 7C8B0E70  srawi r11, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 82429EDC: 3F1D0002  addis r24, r29, 2
	ctx.r[24].s64 = ctx.r[29].s64 + 131072;
	// 82429EE0: 3F3D0002  addis r25, r29, 2
	ctx.r[25].s64 = ctx.r[29].s64 + 131072;
	// 82429EE4: 3B1800B8  addi r24, r24, 0xb8
	ctx.r[24].s64 = ctx.r[24].s64 + 184;
	// 82429EE8: 3B390048  addi r25, r25, 0x48
	ctx.r[25].s64 = ctx.r[25].s64 + 72;
	// 82429EEC: 7FEB0194  addze r31, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[31].s64 = tmp.s64;
	// 82429EF0: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429EF4: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429EF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82429EFC: 7D5F5BD6  divw r10, r31, r11
	ctx.r[10].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 82429F00: 7D5B07B4  extsw r27, r10
	ctx.r[27].s64 = ctx.r[10].s32 as i64;
	// 82429F04: 419A0018  beq cr6, 0x82429f1c
	if ctx.cr[6].eq {
	pc = 0x82429F1C; continue 'dispatch;
	}
	// 82429F08: 7D5F5BD6  divw r10, r31, r11
	ctx.r[10].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 82429F0C: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82429F10: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429F14: 40810008  ble 0x82429f1c
	if !ctx.cr[0].gt {
	pc = 0x82429F1C; continue 'dispatch;
	}
	// 82429F18: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82429F1C: 3FDD0002  addis r30, r29, 2
	ctx.r[30].s64 = ctx.r[29].s64 + 131072;
	// 82429F20: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 82429F24: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82429F28: 216B0080  subfic r11, r11, 0x80
	ctx.xer.ca = ctx.r[11].u32 <= 128 as u32;
	ctx.r[11].s64 = (128 as i64) - ctx.r[11].s64;
	// 82429F2C: 7F3B5800  cmpd cr6, r27, r11
	ctx.cr[6].compare_i64(ctx.r[27].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82429F30: 41980008  blt cr6, 0x82429f38
	if ctx.cr[6].lt {
	pc = 0x82429F38; continue 'dispatch;
	}
	// 82429F34: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82429F38: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82429F3C: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 82429F40: 409900A0  ble cr6, 0x82429fe0
	if !ctx.cr[6].gt {
	pc = 0x82429FE0; continue 'dispatch;
	}
	// 82429F44: 83990000  lwz r28, 0(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429F48: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82429F4C: 41990008  bgt cr6, 0x82429f54
	if ctx.cr[6].gt {
	pc = 0x82429F54; continue 'dispatch;
	}
	// 82429F50: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82429F54: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82429F58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429F5C: 409A001C  bne cr6, 0x82429f78
	if !ctx.cr[6].eq {
	pc = 0x82429F78; continue 'dispatch;
	}
	// 82429F60: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429F64: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82429F68: 616B0030  ori r11, r11, 0x30
	ctx.r[11].u64 = ctx.r[11].u64 | 48;
	// 82429F6C: 7D7D582A  ldx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) };
	// 82429F70: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82429F74: 4098006C  bge cr6, 0x82429fe0
	if !ctx.cr[6].lt {
	pc = 0x82429FE0; continue 'dispatch;
	}
	// 82429F78: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82429F7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82429F80: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82429F84: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82429F88: 48159DA9  bl 0x82583d30
	ctx.lr = 0x82429F8C;
	sub_82583D30(ctx, base);
	// 82429F8C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82429F90: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82429F94: 4182004C  beq 0x82429fe0
	if ctx.cr[0].eq {
	pc = 0x82429FE0; continue 'dispatch;
	}
	// 82429F98: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82429F9C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82429FA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82429FA4: 7D6A3E74  sradi r10, r11, 7
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 7) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 7;
	// 82429FA8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82429FAC: 794A3E24  sldi r10, r10, 7
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(7);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 82429FB0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82429FB4: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82429FB8: 4BFFF329  bl 0x824292e0
	ctx.lr = 0x82429FBC;
	sub_824292E0(ctx, base);
	// 82429FBC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82429FC0: 41800020  blt 0x82429fe0
	if ctx.cr[0].lt {
	pc = 0x82429FE0; continue 'dispatch;
	}
	// 82429FC4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82429FC8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82429FCC: 7FFCF850  subf r31, r28, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82429FD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82429FD4: 7F3AD800  cmpd cr6, r26, r27
	ctx.cr[6].compare_i64(ctx.r[26].s64, ctx.r[27].s64, &mut ctx.xer);
	// 82429FD8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82429FDC: 4198FF68  blt cr6, 0x82429f44
	if ctx.cr[6].lt {
	pc = 0x82429F44; continue 'dispatch;
	}
	// 82429FE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82429FE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82429FE8: 4810B110  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82429FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82429FF0 size=352
    let mut pc: u32 = 0x82429FF0;
    'dispatch: loop {
        match pc {
            0x82429FF0 => {
    //   block [0x82429FF0..0x8242A150)
	// 82429FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82429FF4: 4810B0AD  bl 0x825350a0
	ctx.lr = 0x82429FF8;
	sub_82535080(ctx, base);
	// 82429FF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82429FFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242A000: 7C8B0E70  srawi r11, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8242A004: 3EDD0002  addis r22, r29, 2
	ctx.r[22].s64 = ctx.r[29].s64 + 131072;
	// 8242A008: 3EFD0002  addis r23, r29, 2
	ctx.r[23].s64 = ctx.r[29].s64 + 131072;
	// 8242A00C: 3AD600B8  addi r22, r22, 0xb8
	ctx.r[22].s64 = ctx.r[22].s64 + 184;
	// 8242A010: 3AF70048  addi r23, r23, 0x48
	ctx.r[23].s64 = ctx.r[23].s64 + 72;
	// 8242A014: 7FCB0194  addze r30, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[30].s64 = tmp.s64;
	// 8242A018: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A01C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A020: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242A024: 7D5E5BD6  divw r10, r30, r11
	ctx.r[10].s32 = ctx.r[30].s32 / ctx.r[11].s32;
	// 8242A028: 7D5A07B4  extsw r26, r10
	ctx.r[26].s64 = ctx.r[10].s32 as i64;
	// 8242A02C: 419A0018  beq cr6, 0x8242a044
	if ctx.cr[6].eq {
	pc = 0x8242A044; continue 'dispatch;
	}
	// 8242A030: 7D5E5BD6  divw r10, r30, r11
	ctx.r[10].s32 = ctx.r[30].s32 / ctx.r[11].s32;
	// 8242A034: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242A038: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A03C: 40810008  ble 0x8242a044
	if !ctx.cr[0].gt {
	pc = 0x8242A044; continue 'dispatch;
	}
	// 8242A040: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8242A044: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A048: 3F3D0002  addis r25, r29, 2
	ctx.r[25].s64 = ctx.r[29].s64 + 131072;
	// 8242A04C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8242A050: 3B390028  addi r25, r25, 0x28
	ctx.r[25].s64 = ctx.r[25].s64 + 40;
	// 8242A054: 7D7D582A  ldx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) };
	// 8242A058: E9590000  ld r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	// 8242A05C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8242A060: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8242A064: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242A068: 2F0B0080  cmpwi cr6, r11, 0x80
	ctx.cr[6].compare_i32(ctx.r[11].s32, 128, &mut ctx.xer);
	// 8242A06C: 41980008  blt cr6, 0x8242a074
	if ctx.cr[6].lt {
	pc = 0x8242A074; continue 'dispatch;
	}
	// 8242A070: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 8242A074: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8242A078: 7F3A5800  cmpd cr6, r26, r11
	ctx.cr[6].compare_i64(ctx.r[26].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8242A07C: 41980008  blt cr6, 0x8242a084
	if ctx.cr[6].lt {
	pc = 0x8242A084; continue 'dispatch;
	}
	// 8242A080: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 8242A084: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8242A088: 2F3A0000  cmpdi cr6, r26, 0
	ctx.cr[6].compare_i64(ctx.r[26].s64, 0, &mut ctx.xer);
	// 8242A08C: 409900B8  ble cr6, 0x8242a144
	if !ctx.cr[6].gt {
	pc = 0x8242A144; continue 'dispatch;
	}
	// 8242A090: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A094: 61780018  ori r24, r11, 0x18
	ctx.r[24].u64 = ctx.r[11].u64 | 24;
	// 8242A098: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A09C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8242A0A0: 41990008  bgt cr6, 0x8242a0a8
	if ctx.cr[6].gt {
	pc = 0x8242A0A8; continue 'dispatch;
	}
	// 8242A0A4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8242A0A8: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A0AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A0B0: 409A001C  bne cr6, 0x8242a0cc
	if !ctx.cr[6].eq {
	pc = 0x8242A0CC; continue 'dispatch;
	}
	// 8242A0B4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A0B8: 7D5DC02A  ldx r10, r29, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[24].u32)) };
	// 8242A0BC: 616B0030  ori r11, r11, 0x30
	ctx.r[11].u64 = ctx.r[11].u64 | 48;
	// 8242A0C0: 7D7D582A  ldx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) };
	// 8242A0C4: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8242A0C8: 4098007C  bge cr6, 0x8242a144
	if !ctx.cr[6].lt {
	pc = 0x8242A144; continue 'dispatch;
	}
	// 8242A0CC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A0D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242A0D4: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8242A0D8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242A0DC: 48159C55  bl 0x82583d30
	ctx.lr = 0x8242A0E0;
	sub_82583D30(ctx, base);
	// 8242A0E0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242A0E4: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A0E8: 4182005C  beq 0x8242a144
	if ctx.cr[0].eq {
	pc = 0x8242A144; continue 'dispatch;
	}
	// 8242A0EC: 7FFDC214  add r31, r29, r24
	ctx.r[31].u64 = ctx.r[29].u64 + ctx.r[24].u64;
	// 8242A0F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242A0F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242A0F8: E97F0000  ld r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8242A0FC: 7D6A3E74  sradi r10, r11, 7
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 7) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 7;
	// 8242A100: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8242A104: 794A3E24  sldi r10, r10, 7
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(7);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8242A108: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242A10C: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 8242A110: 4BFFF1D1  bl 0x824292e0
	ctx.lr = 0x8242A114;
	sub_824292E0(ctx, base);
	// 8242A114: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242A118: 4180002C  blt 0x8242a144
	if ctx.cr[0].lt {
	pc = 0x8242A144; continue 'dispatch;
	}
	// 8242A11C: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8242A120: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8242A124: E9790000  ld r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	// 8242A128: 7FDCF050  subf r30, r28, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 8242A12C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242A130: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242A134: 7F3BD000  cmpd cr6, r27, r26
	ctx.cr[6].compare_i64(ctx.r[27].s64, ctx.r[26].s64, &mut ctx.xer);
	// 8242A138: F95F0000  std r10, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8242A13C: F9790000  std r11, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8242A140: 4198FF58  blt cr6, 0x8242a098
	if ctx.cr[6].lt {
	pc = 0x8242A098; continue 'dispatch;
	}
	// 8242A144: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242A148: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8242A14C: 4810AFA4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A150 size=120
    let mut pc: u32 = 0x8242A150;
    'dispatch: loop {
        match pc {
            0x8242A150 => {
    //   block [0x8242A150..0x8242A1C8)
	// 8242A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A154: 4810AF69  bl 0x825350bc
	ctx.lr = 0x8242A158;
	sub_82535080(ctx, base);
	// 8242A158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A15C: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242A160: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A164: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A168: 48002E61  bl 0x8242cfc8
	ctx.lr = 0x8242A16C;
	sub_8242CFC8(ctx, base);
	// 8242A16C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A170: 409A0018  bne cr6, 0x8242a188
	if !ctx.cr[6].eq {
	pc = 0x8242A188; continue 'dispatch;
	}
	// 8242A174: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A178: 386B2608  addi r3, r11, 0x2608
	ctx.r[3].s64 = ctx.r[11].s64 + 9736;
	// 8242A17C: 4BFF2A8D  bl 0x8241cc08
	ctx.lr = 0x8242A180;
	sub_8241CC08(ctx, base);
	// 8242A180: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A184: 48000030  b 0x8242a1b4
	pc = 0x8242A1B4; continue 'dispatch;
	// 8242A188: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A18C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A190: 409A0010  bne cr6, 0x8242a1a0
	if !ctx.cr[6].eq {
	pc = 0x8242A1A0; continue 'dispatch;
	}
	// 8242A194: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A198: 386B25E0  addi r3, r11, 0x25e0
	ctx.r[3].s64 = ctx.r[11].s64 + 9696;
	// 8242A19C: 4BFFFFE0  b 0x8242a17c
	pc = 0x8242A17C; continue 'dispatch;
	// 8242A1A0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A1A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8242A1A8: 616B00B8  ori r11, r11, 0xb8
	ctx.r[11].u64 = ctx.r[11].u64 | 184;
	// 8242A1AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A1B0: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8242A1B4: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A1B8: 48002EA9  bl 0x8242d060
	ctx.lr = 0x8242A1BC;
	sub_8242D060(ctx, base);
	// 8242A1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A1C4: 4810AF48  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A1C8 size=124
    let mut pc: u32 = 0x8242A1C8;
    'dispatch: loop {
        match pc {
            0x8242A1C8 => {
    //   block [0x8242A1C8..0x8242A244)
	// 8242A1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A1CC: 4810AEF1  bl 0x825350bc
	ctx.lr = 0x8242A1D0;
	sub_82535080(ctx, base);
	// 8242A1D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A1D4: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242A1D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A1DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242A1E0: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A1E4: 48002DE5  bl 0x8242cfc8
	ctx.lr = 0x8242A1E8;
	sub_8242CFC8(ctx, base);
	// 8242A1E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A1EC: 409A0018  bne cr6, 0x8242a204
	if !ctx.cr[6].eq {
	pc = 0x8242A204; continue 'dispatch;
	}
	// 8242A1F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A1F4: 386B2658  addi r3, r11, 0x2658
	ctx.r[3].s64 = ctx.r[11].s64 + 9816;
	// 8242A1F8: 4BFF2A11  bl 0x8241cc08
	ctx.lr = 0x8242A1FC;
	sub_8241CC08(ctx, base);
	// 8242A1FC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A200: 48000030  b 0x8242a230
	pc = 0x8242A230; continue 'dispatch;
	// 8242A204: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A208: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A20C: 409A0010  bne cr6, 0x8242a21c
	if !ctx.cr[6].eq {
	pc = 0x8242A21C; continue 'dispatch;
	}
	// 8242A210: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A214: 386B2630  addi r3, r11, 0x2630
	ctx.r[3].s64 = ctx.r[11].s64 + 9776;
	// 8242A218: 4BFFFFE0  b 0x8242a1f8
	pc = 0x8242A1F8; continue 'dispatch;
	// 8242A21C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A220: 419A000C  beq cr6, 0x8242a22c
	if ctx.cr[6].eq {
	pc = 0x8242A22C; continue 'dispatch;
	}
	// 8242A224: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242A228: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242A22C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A230: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A234: 48002E2D  bl 0x8242d060
	ctx.lr = 0x8242A238;
	sub_8242D060(ctx, base);
	// 8242A238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A23C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A240: 4810AECC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A248 size=132
    let mut pc: u32 = 0x8242A248;
    'dispatch: loop {
        match pc {
            0x8242A248 => {
    //   block [0x8242A248..0x8242A2CC)
	// 8242A248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A24C: 4810AE71  bl 0x825350bc
	ctx.lr = 0x8242A250;
	sub_82535080(ctx, base);
	// 8242A250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A254: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242A258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A25C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242A260: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A264: 48002D65  bl 0x8242cfc8
	ctx.lr = 0x8242A268;
	sub_8242CFC8(ctx, base);
	// 8242A268: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A26C: 409A0018  bne cr6, 0x8242a284
	if !ctx.cr[6].eq {
	pc = 0x8242A284; continue 'dispatch;
	}
	// 8242A270: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A274: 386B26A8  addi r3, r11, 0x26a8
	ctx.r[3].s64 = ctx.r[11].s64 + 9896;
	// 8242A278: 4BFF2991  bl 0x8241cc08
	ctx.lr = 0x8242A27C;
	sub_8241CC08(ctx, base);
	// 8242A27C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A280: 48000038  b 0x8242a2b8
	pc = 0x8242A2B8; continue 'dispatch;
	// 8242A284: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A288: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A28C: 409A0010  bne cr6, 0x8242a29c
	if !ctx.cr[6].eq {
	pc = 0x8242A29C; continue 'dispatch;
	}
	// 8242A290: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A294: 386B2680  addi r3, r11, 0x2680
	ctx.r[3].s64 = ctx.r[11].s64 + 9856;
	// 8242A298: 4BFFFFE0  b 0x8242a278
	pc = 0x8242A278; continue 'dispatch;
	// 8242A29C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A2A0: 419A0014  beq cr6, 0x8242a2b4
	if ctx.cr[6].eq {
	pc = 0x8242A2B4; continue 'dispatch;
	}
	// 8242A2A4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A2A8: 616B004C  ori r11, r11, 0x4c
	ctx.r[11].u64 = ctx.r[11].u64 | 76;
	// 8242A2AC: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242A2B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242A2B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A2B8: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A2BC: 48002DA5  bl 0x8242d060
	ctx.lr = 0x8242A2C0;
	sub_8242D060(ctx, base);
	// 8242A2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A2C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A2C8: 4810AE44  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A2D0 size=68
    let mut pc: u32 = 0x8242A2D0;
    'dispatch: loop {
        match pc {
            0x8242A2D0 => {
    //   block [0x8242A2D0..0x8242A314)
	// 8242A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A2D4: 4810ADE9  bl 0x825350bc
	ctx.lr = 0x8242A2D8;
	sub_82535080(ctx, base);
	// 8242A2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A2DC: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242A2E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A2E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242A2E8: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A2EC: 48002CDD  bl 0x8242cfc8
	ctx.lr = 0x8242A2F0;
	sub_8242CFC8(ctx, base);
	// 8242A2F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242A2F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A2F8: 4BFFF2C9  bl 0x824295c0
	ctx.lr = 0x8242A2FC;
	sub_824295C0(ctx, base);
	// 8242A2FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A300: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A304: 48002D5D  bl 0x8242d060
	ctx.lr = 0x8242A308;
	sub_8242D060(ctx, base);
	// 8242A308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A30C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A310: 4810ADFC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A318 size=148
    let mut pc: u32 = 0x8242A318;
    'dispatch: loop {
        match pc {
            0x8242A318 => {
    //   block [0x8242A318..0x8242A3AC)
	// 8242A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A31C: 4810ADA1  bl 0x825350bc
	ctx.lr = 0x8242A320;
	sub_82535080(ctx, base);
	// 8242A320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A324: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242A328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A32C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242A330: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A334: 48002C95  bl 0x8242cfc8
	ctx.lr = 0x8242A338;
	sub_8242CFC8(ctx, base);
	// 8242A338: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A33C: 409A0018  bne cr6, 0x8242a354
	if !ctx.cr[6].eq {
	pc = 0x8242A354; continue 'dispatch;
	}
	// 8242A340: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A344: 386B2748  addi r3, r11, 0x2748
	ctx.r[3].s64 = ctx.r[11].s64 + 10056;
	// 8242A348: 4BFF28C1  bl 0x8241cc08
	ctx.lr = 0x8242A34C;
	sub_8241CC08(ctx, base);
	// 8242A34C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A350: 48000048  b 0x8242a398
	pc = 0x8242A398; continue 'dispatch;
	// 8242A354: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A358: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A35C: 409A0010  bne cr6, 0x8242a36c
	if !ctx.cr[6].eq {
	pc = 0x8242A36C; continue 'dispatch;
	}
	// 8242A360: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A364: 386B2720  addi r3, r11, 0x2720
	ctx.r[3].s64 = ctx.r[11].s64 + 10016;
	// 8242A368: 4BFFFFE0  b 0x8242a348
	pc = 0x8242A348; continue 'dispatch;
	// 8242A36C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A370: 419A0024  beq cr6, 0x8242a394
	if ctx.cr[6].eq {
	pc = 0x8242A394; continue 'dispatch;
	}
	// 8242A374: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A378: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242A37C: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8242A380: 614A0018  ori r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u64 | 24;
	// 8242A384: 7D5F502A  ldx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	// 8242A388: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8242A38C: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8242A390: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8242A394: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A398: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A39C: 48002CC5  bl 0x8242d060
	ctx.lr = 0x8242A3A0;
	sub_8242D060(ctx, base);
	// 8242A3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A3A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A3A8: 4810AD64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A3B0 size=132
    let mut pc: u32 = 0x8242A3B0;
    'dispatch: loop {
        match pc {
            0x8242A3B0 => {
    //   block [0x8242A3B0..0x8242A434)
	// 8242A3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A3B4: 4810AD09  bl 0x825350bc
	ctx.lr = 0x8242A3B8;
	sub_82535080(ctx, base);
	// 8242A3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A3BC: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242A3C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A3C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242A3C8: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A3CC: 48002BFD  bl 0x8242cfc8
	ctx.lr = 0x8242A3D0;
	sub_8242CFC8(ctx, base);
	// 8242A3D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A3D4: 409A0018  bne cr6, 0x8242a3ec
	if !ctx.cr[6].eq {
	pc = 0x8242A3EC; continue 'dispatch;
	}
	// 8242A3D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A3DC: 386B2798  addi r3, r11, 0x2798
	ctx.r[3].s64 = ctx.r[11].s64 + 10136;
	// 8242A3E0: 4BFF2829  bl 0x8241cc08
	ctx.lr = 0x8242A3E4;
	sub_8241CC08(ctx, base);
	// 8242A3E4: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A3E8: 48000038  b 0x8242a420
	pc = 0x8242A420; continue 'dispatch;
	// 8242A3EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A3F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A3F4: 409A0010  bne cr6, 0x8242a404
	if !ctx.cr[6].eq {
	pc = 0x8242A404; continue 'dispatch;
	}
	// 8242A3F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A3FC: 386B2770  addi r3, r11, 0x2770
	ctx.r[3].s64 = ctx.r[11].s64 + 10096;
	// 8242A400: 4BFFFFE0  b 0x8242a3e0
	pc = 0x8242A3E0; continue 'dispatch;
	// 8242A404: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 8242A408: 396B004C  addi r11, r11, 0x4c
	ctx.r[11].s64 = ctx.r[11].s64 + 76;
	// 8242A40C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A410: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8242A414: 419A0008  beq cr6, 0x8242a41c
	if ctx.cr[6].eq {
	pc = 0x8242A41C; continue 'dispatch;
	}
	// 8242A418: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8242A41C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A420: 807D9904  lwz r3, -0x66fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A424: 48002C3D  bl 0x8242d060
	ctx.lr = 0x8242A428;
	sub_8242D060(ctx, base);
	// 8242A428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A42C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A430: 4810ACDC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A438 size=144
    let mut pc: u32 = 0x8242A438;
    'dispatch: loop {
        match pc {
            0x8242A438 => {
    //   block [0x8242A438..0x8242A4C8)
	// 8242A438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A43C: 4810AC7D  bl 0x825350b8
	ctx.lr = 0x8242A440;
	sub_82535080(ctx, base);
	// 8242A440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A444: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8242A448: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A44C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242A450: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A454: 48002B75  bl 0x8242cfc8
	ctx.lr = 0x8242A458;
	sub_8242CFC8(ctx, base);
	// 8242A458: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A45C: 409A0010  bne cr6, 0x8242a46c
	if !ctx.cr[6].eq {
	pc = 0x8242A46C; continue 'dispatch;
	}
	// 8242A460: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A464: 386B2814  addi r3, r11, 0x2814
	ctx.r[3].s64 = ctx.r[11].s64 + 10260;
	// 8242A468: 48000044  b 0x8242a4ac
	pc = 0x8242A4AC; continue 'dispatch;
	// 8242A46C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A470: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A474: 409A0010  bne cr6, 0x8242a484
	if !ctx.cr[6].eq {
	pc = 0x8242A484; continue 'dispatch;
	}
	// 8242A478: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A47C: 386B27EC  addi r3, r11, 0x27ec
	ctx.r[3].s64 = ctx.r[11].s64 + 10220;
	// 8242A480: 4800002C  b 0x8242a4ac
	pc = 0x8242A4AC; continue 'dispatch;
	// 8242A484: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 8242A488: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242A48C: 41990018  bgt cr6, 0x8242a4a4
	if ctx.cr[6].gt {
	pc = 0x8242A4A4; continue 'dispatch;
	}
	// 8242A490: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A494: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A498: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 8242A49C: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8242A4A0: 48000014  b 0x8242a4b4
	pc = 0x8242A4B4; continue 'dispatch;
	// 8242A4A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A4A8: 386B27C0  addi r3, r11, 0x27c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10176;
	// 8242A4AC: 4BFF275D  bl 0x8241cc08
	ctx.lr = 0x8242A4B0;
	sub_8241CC08(ctx, base);
	// 8242A4B0: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A4B4: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A4B8: 48002BA9  bl 0x8242d060
	ctx.lr = 0x8242A4BC;
	sub_8242D060(ctx, base);
	// 8242A4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242A4C4: 4810AC44  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A4C8 size=136
    let mut pc: u32 = 0x8242A4C8;
    'dispatch: loop {
        match pc {
            0x8242A4C8 => {
    //   block [0x8242A4C8..0x8242A550)
	// 8242A4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A4CC: 4810ABED  bl 0x825350b8
	ctx.lr = 0x8242A4D0;
	sub_82535080(ctx, base);
	// 8242A4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A4D4: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8242A4D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A4DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242A4E0: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A4E4: 48002AE5  bl 0x8242cfc8
	ctx.lr = 0x8242A4E8;
	sub_8242CFC8(ctx, base);
	// 8242A4E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A4EC: 409A0018  bne cr6, 0x8242a504
	if !ctx.cr[6].eq {
	pc = 0x8242A504; continue 'dispatch;
	}
	// 8242A4F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A4F4: 386B2864  addi r3, r11, 0x2864
	ctx.r[3].s64 = ctx.r[11].s64 + 10340;
	// 8242A4F8: 4BFF2711  bl 0x8241cc08
	ctx.lr = 0x8242A4FC;
	sub_8241CC08(ctx, base);
	// 8242A4FC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242A500: 4800003C  b 0x8242a53c
	pc = 0x8242A53C; continue 'dispatch;
	// 8242A504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A508: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A50C: 409A0010  bne cr6, 0x8242a51c
	if !ctx.cr[6].eq {
	pc = 0x8242A51C; continue 'dispatch;
	}
	// 8242A510: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A514: 386B283C  addi r3, r11, 0x283c
	ctx.r[3].s64 = ctx.r[11].s64 + 10300;
	// 8242A518: 4BFFFFE0  b 0x8242a4f8
	pc = 0x8242A4F8; continue 'dispatch;
	// 8242A51C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A520: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242A524: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8242A528: 614A0030  ori r10, r10, 0x30
	ctx.r[10].u64 = ctx.r[10].u64 | 48;
	// 8242A52C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242A530: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8242A534: 7D7D5BD2  divd r11, r29, r11
	ctx.r[11].s64 = ctx.r[29].s64 / ctx.r[11].s64;
	// 8242A538: 7D7F512A  stdx r11, r31, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u64) };
	// 8242A53C: 807C9904  lwz r3, -0x66fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A540: 48002B21  bl 0x8242d060
	ctx.lr = 0x8242A544;
	sub_8242D060(ctx, base);
	// 8242A544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242A54C: 4810ABBC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242A550 size=164
    let mut pc: u32 = 0x8242A550;
    'dispatch: loop {
        match pc {
            0x8242A550 => {
    //   block [0x8242A550..0x8242A5F4)
	// 8242A550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242A558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242A55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242A560: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8242A564: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A568: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242A56C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8242A570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A574: 807E9904  lwz r3, -0x66fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A578: 48002A51  bl 0x8242cfc8
	ctx.lr = 0x8242A57C;
	sub_8242CFC8(ctx, base);
	// 8242A57C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A580: 409A0018  bne cr6, 0x8242a598
	if !ctx.cr[6].eq {
	pc = 0x8242A598; continue 'dispatch;
	}
	// 8242A584: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A588: 386B2864  addi r3, r11, 0x2864
	ctx.r[3].s64 = ctx.r[11].s64 + 10340;
	// 8242A58C: 4BFF267D  bl 0x8241cc08
	ctx.lr = 0x8242A590;
	sub_8241CC08(ctx, base);
	// 8242A590: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A594: 48000038  b 0x8242a5cc
	pc = 0x8242A5CC; continue 'dispatch;
	// 8242A598: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A59C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A5A0: 409A0010  bne cr6, 0x8242a5b0
	if !ctx.cr[6].eq {
	pc = 0x8242A5B0; continue 'dispatch;
	}
	// 8242A5A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A5A8: 386B283C  addi r3, r11, 0x283c
	ctx.r[3].s64 = ctx.r[11].s64 + 10300;
	// 8242A5AC: 4BFFFFE0  b 0x8242a58c
	pc = 0x8242A58C; continue 'dispatch;
	// 8242A5B0: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 8242A5B4: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 8242A5B8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8242A5BC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8242A5C0: 419A0008  beq cr6, 0x8242a5c8
	if ctx.cr[6].eq {
	pc = 0x8242A5C8; continue 'dispatch;
	}
	// 8242A5C4: D3EB0000  stfs f31, 0(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8242A5C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A5CC: 807E9904  lwz r3, -0x66fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A5D0: 48002A91  bl 0x8242d060
	ctx.lr = 0x8242A5D4;
	sub_8242D060(ctx, base);
	// 8242A5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A5D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242A5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242A5E4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8242A5E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242A5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242A5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A5F8 size=92
    let mut pc: u32 = 0x8242A5F8;
    'dispatch: loop {
        match pc {
            0x8242A5F8 => {
    //   block [0x8242A5F8..0x8242A654)
	// 8242A5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A5FC: 4810AABD  bl 0x825350b8
	ctx.lr = 0x8242A600;
	sub_82535080(ctx, base);
	// 8242A600: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8242A604: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A608: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242A60C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8242A610: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A614: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242A618: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8242A61C: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A620: 480029A9  bl 0x8242cfc8
	ctx.lr = 0x8242A624;
	sub_8242CFC8(ctx, base);
	// 8242A624: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242A628: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8242A62C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242A630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A634: 4BFFF03D  bl 0x82429670
	ctx.lr = 0x8242A638;
	sub_82429670(ctx, base);
	// 8242A638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A63C: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A640: 48002A21  bl 0x8242d060
	ctx.lr = 0x8242A644;
	sub_8242D060(ctx, base);
	// 8242A644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242A64C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8242A650: 4810AAB8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A658 size=288
    let mut pc: u32 = 0x8242A658;
    'dispatch: loop {
        match pc {
            0x8242A658 => {
    //   block [0x8242A658..0x8242A778)
	// 8242A658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A65C: 4810AA59  bl 0x825350b4
	ctx.lr = 0x8242A660;
	sub_82535080(ctx, base);
	// 8242A660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A664: 3F60828A  lis r27, -0x7d76
	ctx.r[27].s64 = -2104885248;
	// 8242A668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A66C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242A670: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242A674: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8242A678: 807B9904  lwz r3, -0x66fc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A67C: 4800294D  bl 0x8242cfc8
	ctx.lr = 0x8242A680;
	sub_8242CFC8(ctx, base);
	// 8242A680: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A684: 419A00D0  beq cr6, 0x8242a754
	if ctx.cr[6].eq {
	pc = 0x8242A754; continue 'dispatch;
	}
	// 8242A688: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242A68C: 419A00C8  beq cr6, 0x8242a754
	if ctx.cr[6].eq {
	pc = 0x8242A754; continue 'dispatch;
	}
	// 8242A690: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A694: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A698: 409A0010  bne cr6, 0x8242a6a8
	if !ctx.cr[6].eq {
	pc = 0x8242A6A8; continue 'dispatch;
	}
	// 8242A69C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A6A0: 386B2968  addi r3, r11, 0x2968
	ctx.r[3].s64 = ctx.r[11].s64 + 10600;
	// 8242A6A4: 480000B8  b 0x8242a75c
	pc = 0x8242A75C; continue 'dispatch;
	// 8242A6A8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8242A6AC: 419A009C  beq cr6, 0x8242a748
	if ctx.cr[6].eq {
	pc = 0x8242A748; continue 'dispatch;
	}
	// 8242A6B0: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8242A6B4: 41990094  bgt cr6, 0x8242a748
	if ctx.cr[6].gt {
	pc = 0x8242A748; continue 'dispatch;
	}
	// 8242A6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242A6BC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8242A6C0: 40990038  ble cr6, 0x8242a6f8
	if !ctx.cr[6].gt {
	pc = 0x8242A6F8; continue 'dispatch;
	}
	// 8242A6C4: 3D1F0002  addis r8, r31, 2
	ctx.r[8].s64 = ctx.r[31].s64 + 131072;
	// 8242A6C8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8242A6CC: 39080038  addi r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 + 56;
	// 8242A6D0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8242A6D4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242A6D8: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A6DC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242A6E0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8242A6E4: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8242A6E8: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8242A6EC: 4082FFEC  bne 0x8242a6d8
	if !ctx.cr[0].eq {
	pc = 0x8242A6D8; continue 'dispatch;
	}
	// 8242A6F0: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8242A6F4: 40980034  bge cr6, 0x8242a728
	if !ctx.cr[6].lt {
	pc = 0x8242A728; continue 'dispatch;
	}
	// 8242A6F8: 214B0002  subfic r10, r11, 2
	ctx.xer.ca = ctx.r[11].u32 <= 2 as u32;
	ctx.r[10].s64 = (2 as i64) - ctx.r[11].s64;
	// 8242A6FC: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8242A700: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8242A704: 396B800E  addi r11, r11, -0x7ff2
	ctx.r[11].s64 = ctx.r[11].s64 + -32754;
	// 8242A708: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242A70C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242A710: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242A714: 41820014  beq 0x8242a728
	if ctx.cr[0].eq {
	pc = 0x8242A728; continue 'dispatch;
	}
	// 8242A718: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8242A71C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242A720: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242A724: 4200FFF8  bdnz 0x8242a71c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242A71C; continue 'dispatch;
	}
	// 8242A728: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A72C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242A730: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 8242A734: 614A0044  ori r10, r10, 0x44
	ctx.r[10].u64 = ctx.r[10].u64 | 68;
	// 8242A738: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242A73C: 7FBF592E  stwx r29, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8242A740: 7F9F512E  stwx r28, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8242A744: 48000020  b 0x8242a764
	pc = 0x8242A764; continue 'dispatch;
	// 8242A748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A74C: 386B2930  addi r3, r11, 0x2930
	ctx.r[3].s64 = ctx.r[11].s64 + 10544;
	// 8242A750: 4800000C  b 0x8242a75c
	pc = 0x8242A75C; continue 'dispatch;
	// 8242A754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A758: 386B2908  addi r3, r11, 0x2908
	ctx.r[3].s64 = ctx.r[11].s64 + 10504;
	// 8242A75C: 4BFF24AD  bl 0x8241cc08
	ctx.lr = 0x8242A760;
	sub_8241CC08(ctx, base);
	// 8242A760: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242A764: 807B9904  lwz r3, -0x66fc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A768: 480028F9  bl 0x8242d060
	ctx.lr = 0x8242A76C;
	sub_8242D060(ctx, base);
	// 8242A76C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242A774: 4810A990  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A778 size=248
    let mut pc: u32 = 0x8242A778;
    'dispatch: loop {
        match pc {
            0x8242A778 => {
    //   block [0x8242A778..0x8242A870)
	// 8242A778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A77C: 4810A93D  bl 0x825350b8
	ctx.lr = 0x8242A780;
	sub_82535080(ctx, base);
	// 8242A780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A784: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242A788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A78C: 3B8B9800  addi r28, r11, -0x6800
	ctx.r[28].s64 = ctx.r[11].s64 + -26624;
	// 8242A790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242A794: 807C0104  lwz r3, 0x104(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(260 as u32) ) } as u64;
	// 8242A798: 48002831  bl 0x8242cfc8
	ctx.lr = 0x8242A79C;
	sub_8242CFC8(ctx, base);
	// 8242A79C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242A7A0: 409A0018  bne cr6, 0x8242a7b8
	if !ctx.cr[6].eq {
	pc = 0x8242A7B8; continue 'dispatch;
	}
	// 8242A7A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A7A8: 386B2A00  addi r3, r11, 0x2a00
	ctx.r[3].s64 = ctx.r[11].s64 + 10752;
	// 8242A7AC: 4BFF245D  bl 0x8241cc08
	ctx.lr = 0x8242A7B0;
	sub_8241CC08(ctx, base);
	// 8242A7B0: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242A7B4: 480000A8  b 0x8242a85c
	pc = 0x8242A85C; continue 'dispatch;
	// 8242A7B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A7BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A7C0: 409A0010  bne cr6, 0x8242a7d0
	if !ctx.cr[6].eq {
	pc = 0x8242A7D0; continue 'dispatch;
	}
	// 8242A7C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A7C8: 386B29D8  addi r3, r11, 0x29d8
	ctx.r[3].s64 = ctx.r[11].s64 + 10712;
	// 8242A7CC: 4BFFFFE0  b 0x8242a7ac
	pc = 0x8242A7AC; continue 'dispatch;
	// 8242A7D0: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8242A7D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242A7D8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8242A7DC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A7E0: 48159709  bl 0x82583ee8
	ctx.lr = 0x8242A7E4;
	sub_82583EE8(ctx, base);
	// 8242A7E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A7E8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242A7EC: 419A000C  beq cr6, 0x8242a7f8
	if ctx.cr[6].eq {
	pc = 0x8242A7F8; continue 'dispatch;
	}
	// 8242A7F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242A7F4: 4800003C  b 0x8242a830
	pc = 0x8242A830; continue 'dispatch;
	// 8242A7F8: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 8242A7FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8242A800: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8242A804: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 8242A808: 396B0054  addi r11, r11, 0x54
	ctx.r[11].s64 = ctx.r[11].s64 + 84;
	// 8242A80C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8242A810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242A814: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8242A818: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8242A81C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8242A820: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242A824: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8242A828: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 8242A82C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8242A830: 48159451  bl 0x82583c80
	ctx.lr = 0x8242A834;
	sub_82583C80(ctx, base);
	// 8242A834: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8242A838: 40980020  bge cr6, 0x8242a858
	if !ctx.cr[6].lt {
	pc = 0x8242A858; continue 'dispatch;
	}
	// 8242A83C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242A840: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8242A844: 388B2990  addi r4, r11, 0x2990
	ctx.r[4].s64 = ctx.r[11].s64 + 10640;
	// 8242A848: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8242A84C: 4811C75D  bl 0x82546fa8
	ctx.lr = 0x8242A850;
	sub_82546FA8(ctx, base);
	// 8242A850: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8242A854: 4BFFFF58  b 0x8242a7ac
	pc = 0x8242A7AC; continue 'dispatch;
	// 8242A858: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242A85C: 807C0104  lwz r3, 0x104(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(260 as u32) ) } as u64;
	// 8242A860: 48002801  bl 0x8242d060
	ctx.lr = 0x8242A864;
	sub_8242D060(ctx, base);
	// 8242A864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A868: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242A86C: 4810A89C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A870 size=84
    let mut pc: u32 = 0x8242A870;
    'dispatch: loop {
        match pc {
            0x8242A870 => {
    //   block [0x8242A870..0x8242A8C4)
	// 8242A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242A878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242A87C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242A880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A884: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242A888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A88C: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A890: 48002739  bl 0x8242cfc8
	ctx.lr = 0x8242A894;
	sub_8242CFC8(ctx, base);
	// 8242A894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A898: 4BFFF0A9  bl 0x82429940
	ctx.lr = 0x8242A89C;
	sub_82429940(ctx, base);
	// 8242A89C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A8A0: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A8A4: 480027BD  bl 0x8242d060
	ctx.lr = 0x8242A8A8;
	sub_8242D060(ctx, base);
	// 8242A8A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A8AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242A8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242A8B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242A8BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242A8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242A8C8 size=84
    let mut pc: u32 = 0x8242A8C8;
    'dispatch: loop {
        match pc {
            0x8242A8C8 => {
    //   block [0x8242A8C8..0x8242A91C)
	// 8242A8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242A8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242A8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242A8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A8DC: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242A8E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A8E4: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A8E8: 480026E1  bl 0x8242cfc8
	ctx.lr = 0x8242A8EC;
	sub_8242CFC8(ctx, base);
	// 8242A8EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A8F0: 4BFFF2A9  bl 0x82429b98
	ctx.lr = 0x8242A8F4;
	sub_82429B98(ctx, base);
	// 8242A8F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242A8F8: 807F9904  lwz r3, -0x66fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242A8FC: 48002765  bl 0x8242d060
	ctx.lr = 0x8242A900;
	sub_8242D060(ctx, base);
	// 8242A900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242A904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242A910: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242A914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242A918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242A920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242A920 size=332
    let mut pc: u32 = 0x8242A920;
    'dispatch: loop {
        match pc {
            0x8242A920 => {
    //   block [0x8242A920..0x8242AA6C)
	// 8242A920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242A924: 4810A795  bl 0x825350b8
	ctx.lr = 0x8242A928;
	sub_82535080(ctx, base);
	// 8242A928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242A92C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242A930: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242A934: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 8242A938: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 8242A93C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A940: 481595A9  bl 0x82583ee8
	ctx.lr = 0x8242A944;
	sub_82583EE8(ctx, base);
	// 8242A944: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A948: 481594A1  bl 0x82583de8
	ctx.lr = 0x8242A94C;
	sub_82583DE8(ctx, base);
	// 8242A94C: 3F9F0002  addis r28, r31, 2
	ctx.r[28].s64 = ctx.r[31].s64 + 131072;
	// 8242A950: 3B9C00B4  addi r28, r28, 0xb4
	ctx.r[28].s64 = ctx.r[28].s64 + 180;
	// 8242A954: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A95C: 409A0104  bne cr6, 0x8242aa60
	if !ctx.cr[6].eq {
	pc = 0x8242AA60; continue 'dispatch;
	}
	// 8242A960: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A964: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8242A968: 616B004C  ori r11, r11, 0x4c
	ctx.r[11].u64 = ctx.r[11].u64 | 76;
	// 8242A96C: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 8242A970: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242A974: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A978: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242A97C: 409A0024  bne cr6, 0x8242a9a0
	if !ctx.cr[6].eq {
	pc = 0x8242A9A0; continue 'dispatch;
	}
	// 8242A980: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A984: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242A988: 616B0050  ori r11, r11, 0x50
	ctx.r[11].u64 = ctx.r[11].u64 | 80;
	// 8242A98C: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 8242A990: 7C1F5C2E  lfsx f0, r31, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8242A994: 7DBF542E  lfsx f13, r31, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8242A998: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8242A99C: 419A000C  beq cr6, 0x8242a9a8
	if ctx.cr[6].eq {
	pc = 0x8242A9A8; continue 'dispatch;
	}
	// 8242A9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A9A4: 4BFFE765  bl 0x82429108
	ctx.lr = 0x8242A9A8;
	sub_82429108(ctx, base);
	// 8242A9A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242A9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242A9B0: 4BFFE821  bl 0x824291d0
	ctx.lr = 0x8242A9B4;
	sub_824291D0(ctx, base);
	// 8242A9B4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A9B8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242A9BC: 616B00B8  ori r11, r11, 0xb8
	ctx.r[11].u64 = ctx.r[11].u64 | 184;
	// 8242A9C0: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242A9C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242A9C8: 409A003C  bne cr6, 0x8242aa04
	if !ctx.cr[6].eq {
	pc = 0x8242AA04; continue 'dispatch;
	}
	// 8242A9CC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242A9D0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242A9D4: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8242A9D8: 616B0044  ori r11, r11, 0x44
	ctx.r[11].u64 = ctx.r[11].u64 | 68;
	// 8242A9DC: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 8242A9E0: 7D3F582E  lwzx r9, r31, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242A9E4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242A9E8: 7D2A0E70  srawi r10, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8242A9EC: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8242A9F0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242A9F4: 40980008  bge cr6, 0x8242a9fc
	if !ctx.cr[6].lt {
	pc = 0x8242A9FC; continue 'dispatch;
	}
	// 8242A9F8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242A9FC: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242AA00: 41980060  blt cr6, 0x8242aa60
	if ctx.cr[6].lt {
	pc = 0x8242AA60; continue 'dispatch;
	}
	// 8242AA04: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8242AA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AA0C: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 8242AA10: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8242AA14: 2F2B0080  cmpdi cr6, r11, 0x80
	ctx.cr[6].compare_i64(ctx.r[11].s64, 128, &mut ctx.xer);
	// 8242AA18: 4098000C  bge cr6, 0x8242aa24
	if !ctx.cr[6].lt {
	pc = 0x8242AA24; continue 'dispatch;
	}
	// 8242AA1C: 4BFFF4AD  bl 0x82429ec8
	ctx.lr = 0x8242AA20;
	sub_82429EC8(ctx, base);
	// 8242AA20: 48000008  b 0x8242aa28
	pc = 0x8242AA28; continue 'dispatch;
	// 8242AA24: 4BFFF5CD  bl 0x82429ff0
	ctx.lr = 0x8242AA28;
	sub_82429FF0(ctx, base);
	// 8242AA28: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AA2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AA30: 409A0028  bne cr6, 0x8242aa58
	if !ctx.cr[6].eq {
	pc = 0x8242AA58; continue 'dispatch;
	}
	// 8242AA34: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AA38: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8242AA3C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8242AA40: 7D7F582A  ldx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 8242AA44: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8242AA48: 40980010  bge cr6, 0x8242aa58
	if !ctx.cr[6].lt {
	pc = 0x8242AA58; continue 'dispatch;
	}
	// 8242AA4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242AA50: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AA54: 4815944D  bl 0x82583ea0
	ctx.lr = 0x8242AA58;
	sub_82583EA0(ctx, base);
	// 8242AA58: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8242AA5C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242AA60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242AA64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242AA68: 4810A6A0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242AA70 size=356
    let mut pc: u32 = 0x8242AA70;
    'dispatch: loop {
        match pc {
            0x8242AA70 => {
    //   block [0x8242AA70..0x8242ABD4)
	// 8242AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242AA74: 4810A645  bl 0x825350b8
	ctx.lr = 0x8242AA78;
	sub_82535080(ctx, base);
	// 8242AA78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AA7C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8242AA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242AA84: 4BFFE74D  bl 0x824291d0
	ctx.lr = 0x8242AA88;
	sub_824291D0(ctx, base);
	// 8242AA88: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8242AA8C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AA90: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8242AA94: 617D0018  ori r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u64 | 24;
	// 8242AA98: 40990024  ble cr6, 0x8242aabc
	if !ctx.cr[6].gt {
	pc = 0x8242AABC; continue 'dispatch;
	}
	// 8242AA9C: 7D7FE82A  ldx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8242AAA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242AAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AAA8: 2F2B0080  cmpdi cr6, r11, 0x80
	ctx.cr[6].compare_i64(ctx.r[11].s64, 128, &mut ctx.xer);
	// 8242AAAC: 4098000C  bge cr6, 0x8242aab8
	if !ctx.cr[6].lt {
	pc = 0x8242AAB8; continue 'dispatch;
	}
	// 8242AAB0: 4BFFF419  bl 0x82429ec8
	ctx.lr = 0x8242AAB4;
	sub_82429EC8(ctx, base);
	// 8242AAB4: 48000008  b 0x8242aabc
	pc = 0x8242AABC; continue 'dispatch;
	// 8242AAB8: 4BFFF539  bl 0x82429ff0
	ctx.lr = 0x8242AABC;
	sub_82429FF0(ctx, base);
	// 8242AABC: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8242AAC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242AAC4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8242AAC8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AACC: 48159265  bl 0x82583d30
	ctx.lr = 0x8242AAD0;
	sub_82583D30(ctx, base);
	// 8242AAD0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242AAD4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AAD8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AADC: 616B00B4  ori r11, r11, 0xb4
	ctx.r[11].u64 = ctx.r[11].u64 | 180;
	// 8242AAE0: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242AAE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AAE8: 40820030  bne 0x8242ab18
	if !ctx.cr[0].eq {
	pc = 0x8242AB18; continue 'dispatch;
	}
	// 8242AAEC: 409A003C  bne cr6, 0x8242ab28
	if !ctx.cr[6].eq {
	pc = 0x8242AB28; continue 'dispatch;
	}
	// 8242AAF0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AAF4: 7D5FE82A  ldx r10, r31, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8242AAF8: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8242AAFC: 7D7F582A  ldx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 8242AB00: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8242AB04: 40980024  bge cr6, 0x8242ab28
	if !ctx.cr[6].lt {
	pc = 0x8242AB28; continue 'dispatch;
	}
	// 8242AB08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242AB0C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AB10: 48159391  bl 0x82583ea0
	ctx.lr = 0x8242AB14;
	sub_82583EA0(ctx, base);
	// 8242AB14: 48000014  b 0x8242ab28
	pc = 0x8242AB28; continue 'dispatch;
	// 8242AB18: 419A0010  beq cr6, 0x8242ab28
	if ctx.cr[6].eq {
	pc = 0x8242AB28; continue 'dispatch;
	}
	// 8242AB1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242AB20: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AB24: 481593C5  bl 0x82583ee8
	ctx.lr = 0x8242AB28;
	sub_82583EE8(ctx, base);
	// 8242AB28: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AB2C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242AB30: 616B004C  ori r11, r11, 0x4c
	ctx.r[11].u64 = ctx.r[11].u64 | 76;
	// 8242AB34: 614A000C  ori r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 | 12;
	// 8242AB38: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242AB3C: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8242AB40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242AB44: 409A0024  bne cr6, 0x8242ab68
	if !ctx.cr[6].eq {
	pc = 0x8242AB68; continue 'dispatch;
	}
	// 8242AB48: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AB4C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8242AB50: 616B0050  ori r11, r11, 0x50
	ctx.r[11].u64 = ctx.r[11].u64 | 80;
	// 8242AB54: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 8242AB58: 7C1F5C2E  lfsx f0, r31, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8242AB5C: 7DBF542E  lfsx f13, r31, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8242AB60: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8242AB64: 419A000C  beq cr6, 0x8242ab70
	if ctx.cr[6].eq {
	pc = 0x8242AB70; continue 'dispatch;
	}
	// 8242AB68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AB6C: 4BFFE59D  bl 0x82429108
	ctx.lr = 0x8242AB70;
	sub_82429108(ctx, base);
	// 8242AB70: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8242AB74: 3BDE00B8  addi r30, r30, 0xb8
	ctx.r[30].s64 = ctx.r[30].s64 + 184;
	// 8242AB78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AB7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AB80: 419A0048  beq cr6, 0x8242abc8
	if ctx.cr[6].eq {
	pc = 0x8242ABC8; continue 'dispatch;
	}
	// 8242AB84: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8242AB88: 41990040  bgt cr6, 0x8242abc8
	if ctx.cr[6].gt {
	pc = 0x8242ABC8; continue 'dispatch;
	}
	// 8242AB8C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8242AB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AB94: 4BFFEA2D  bl 0x824295c0
	ctx.lr = 0x8242AB98;
	sub_824295C0(ctx, base);
	// 8242AB98: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8242AB9C: 7D5FE82A  ldx r10, r31, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8242ABA0: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8242ABA4: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8242ABA8: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8242ABAC: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8242ABB0: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8242ABB4: 41980014  blt cr6, 0x8242abc8
	if ctx.cr[6].lt {
	pc = 0x8242ABC8; continue 'dispatch;
	}
	// 8242ABB8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242ABBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242ABC0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242ABC4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242ABC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242ABCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242ABD0: 4810A538  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242ABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242ABD8 size=168
    let mut pc: u32 = 0x8242ABD8;
    'dispatch: loop {
        match pc {
            0x8242ABD8 => {
    //   block [0x8242ABD8..0x8242AC80)
	// 8242ABD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242ABDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242ABE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242ABE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242ABE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242ABEC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242ABF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242ABF4: 807E9904  lwz r3, -0x66fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242ABF8: 480023D1  bl 0x8242cfc8
	ctx.lr = 0x8242ABFC;
	sub_8242CFC8(ctx, base);
	// 8242ABFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242AC00: 409A0018  bne cr6, 0x8242ac18
	if !ctx.cr[6].eq {
	pc = 0x8242AC18; continue 'dispatch;
	}
	// 8242AC04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AC08: 386B2B30  addi r3, r11, 0x2b30
	ctx.r[3].s64 = ctx.r[11].s64 + 11056;
	// 8242AC0C: 4BFF1FFD  bl 0x8241cc08
	ctx.lr = 0x8242AC10;
	sub_8241CC08(ctx, base);
	// 8242AC10: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242AC14: 48000048  b 0x8242ac5c
	pc = 0x8242AC5C; continue 'dispatch;
	// 8242AC18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AC1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AC20: 409A0010  bne cr6, 0x8242ac30
	if !ctx.cr[6].eq {
	pc = 0x8242AC30; continue 'dispatch;
	}
	// 8242AC24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AC28: 386B2B08  addi r3, r11, 0x2b08
	ctx.r[3].s64 = ctx.r[11].s64 + 11016;
	// 8242AC2C: 4BFFFFE0  b 0x8242ac0c
	pc = 0x8242AC0C; continue 'dispatch;
	// 8242AC30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242AC34: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242AC38: 419A0018  beq cr6, 0x8242ac50
	if ctx.cr[6].eq {
	pc = 0x8242AC50; continue 'dispatch;
	}
	// 8242AC3C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242AC40: 409A0018  bne cr6, 0x8242ac58
	if !ctx.cr[6].eq {
	pc = 0x8242AC58; continue 'dispatch;
	}
	// 8242AC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AC48: 4BFFFE29  bl 0x8242aa70
	ctx.lr = 0x8242AC4C;
	sub_8242AA70(ctx, base);
	// 8242AC4C: 4800000C  b 0x8242ac58
	pc = 0x8242AC58; continue 'dispatch;
	// 8242AC50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AC54: 4BFFFCCD  bl 0x8242a920
	ctx.lr = 0x8242AC58;
	sub_8242A920(ctx, base);
	// 8242AC58: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242AC5C: 807E9904  lwz r3, -0x66fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26364 as u32) ) } as u64;
	// 8242AC60: 48002401  bl 0x8242d060
	ctx.lr = 0x8242AC64;
	sub_8242D060(ctx, base);
	// 8242AC64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AC68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242AC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AC74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242AC78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242AC80 size=112
    let mut pc: u32 = 0x8242AC80;
    'dispatch: loop {
        match pc {
            0x8242AC80 => {
    //   block [0x8242AC80..0x8242ACF0)
	// 8242AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242AC84: 4810A435  bl 0x825350b8
	ctx.lr = 0x8242AC88;
	sub_82535080(ctx, base);
	// 8242AC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242AC90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242AC94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242AC98: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8242AC9C: 4BFF6395  bl 0x82421030
	ctx.lr = 0x8242ACA0;
	sub_82421030(ctx, base);
	// 8242ACA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242ACA4: 409A0014  bne cr6, 0x8242acb8
	if !ctx.cr[6].eq {
	pc = 0x8242ACB8; continue 'dispatch;
	}
	// 8242ACA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242ACAC: 386B2B80  addi r3, r11, 0x2b80
	ctx.r[3].s64 = ctx.r[11].s64 + 11136;
	// 8242ACB0: 4BFF1F59  bl 0x8241cc08
	ctx.lr = 0x8242ACB4;
	sub_8241CC08(ctx, base);
	// 8242ACB4: 48000030  b 0x8242ace4
	pc = 0x8242ACE4; continue 'dispatch;
	// 8242ACB8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242ACBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242ACC0: 409A0010  bne cr6, 0x8242acd0
	if !ctx.cr[6].eq {
	pc = 0x8242ACD0; continue 'dispatch;
	}
	// 8242ACC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242ACC8: 386B2B58  addi r3, r11, 0x2b58
	ctx.r[3].s64 = ctx.r[11].s64 + 11096;
	// 8242ACCC: 4BFFFFE4  b 0x8242acb0
	pc = 0x8242ACB0; continue 'dispatch;
	// 8242ACD0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8242ACD4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242ACD8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242ACDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242ACE0: 4BFF9FD1  bl 0x82424cb0
	ctx.lr = 0x8242ACE4;
	sub_82424CB0(ctx, base);
	// 8242ACE4: 4BFF638D  bl 0x82421070
	ctx.lr = 0x8242ACE8;
	sub_82421070(ctx, base);
	// 8242ACE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242ACEC: 4810A41C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242ACF0 size=116
    let mut pc: u32 = 0x8242ACF0;
    'dispatch: loop {
        match pc {
            0x8242ACF0 => {
    //   block [0x8242ACF0..0x8242AD64)
	// 8242ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242ACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242ACF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242ACFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AD00: 4BFF6331  bl 0x82421030
	ctx.lr = 0x8242AD04;
	sub_82421030(ctx, base);
	// 8242AD04: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242AD08: 3BEB9928  addi r31, r11, -0x66d8
	ctx.r[31].s64 = ctx.r[11].s64 + -26328;
	// 8242AD0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AD10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242AD14: 419A0014  beq cr6, 0x8242ad28
	if ctx.cr[6].eq {
	pc = 0x8242AD28; continue 'dispatch;
	}
	// 8242AD18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242AD1C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242AD20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242AD24: 4E800421  bctrl
	ctx.lr = 0x8242AD28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242AD28: 4BFFCAC9  bl 0x824277f0
	ctx.lr = 0x8242AD2C;
	sub_824277F0(ctx, base);
	// 8242AD2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242AD30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242AD34: 419A0018  beq cr6, 0x8242ad4c
	if ctx.cr[6].eq {
	pc = 0x8242AD4C; continue 'dispatch;
	}
	// 8242AD38: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8242AD3C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242AD40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AD44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242AD48: 4E800421  bctrl
	ctx.lr = 0x8242AD4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242AD4C: 4BFF6325  bl 0x82421070
	ctx.lr = 0x8242AD50;
	sub_82421070(ctx, base);
	// 8242AD50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242AD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242AD68 size=32
    let mut pc: u32 = 0x8242AD68;
    'dispatch: loop {
        match pc {
            0x8242AD68 => {
    //   block [0x8242AD68..0x8242AD88)
	// 8242AD68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242AD6C: 409A001C  bne cr6, 0x8242ad88
	if !ctx.cr[6].eq {
		sub_8242AD88(ctx, base);
		return;
	}
	// 8242AD70: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242AD74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242AD78: 916A9938  stw r11, -0x66c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26312 as u32), ctx.r[11].u32 ) };
	// 8242AD7C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242AD80: 916A993C  stw r11, -0x66c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26308 as u32), ctx.r[11].u32 ) };
	// 8242AD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242AD88 size=20
    let mut pc: u32 = 0x8242AD88;
    'dispatch: loop {
        match pc {
            0x8242AD88 => {
    //   block [0x8242AD88..0x8242AD9C)
	// 8242AD88: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242AD8C: 906B9938  stw r3, -0x66c8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26312 as u32), ctx.r[3].u32 ) };
	// 8242AD90: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242AD94: 908B993C  stw r4, -0x66c4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26308 as u32), ctx.r[4].u32 ) };
	// 8242AD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242ADA0 size=144
    let mut pc: u32 = 0x8242ADA0;
    'dispatch: loop {
        match pc {
            0x8242ADA0 => {
    //   block [0x8242ADA0..0x8242AE30)
	// 8242ADA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242ADA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242ADA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242ADAC: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8242ADB0: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 8242ADB4: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8242ADB8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8242ADBC: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 8242ADC0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 8242ADC4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8242ADC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242ADCC: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 8242ADD0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8242ADD4: 39210088  addi r9, r1, 0x88
	ctx.r[9].s64 = ctx.r[1].s64 + 136;
	// 8242ADD8: 3BEB0AA0  addi r31, r11, 0xaa0
	ctx.r[31].s64 = ctx.r[11].s64 + 2720;
	// 8242ADDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8242ADE0: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 8242ADE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242ADE8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242ADEC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242ADF0: 4BFF8841  bl 0x82423630
	ctx.lr = 0x8242ADF4;
	sub_82423630(ctx, base);
	// 8242ADF4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242ADF8: 814B9938  lwz r10, -0x66c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26312 as u32) ) } as u64;
	// 8242ADFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242AE00: 419A001C  beq cr6, 0x8242ae1c
	if ctx.cr[6].eq {
	pc = 0x8242AE1C; continue 'dispatch;
	}
	// 8242AE04: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242AE08: 816B9938  lwz r11, -0x66c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26312 as u32) ) } as u64;
	// 8242AE0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8242AE10: 806A993C  lwz r3, -0x66c4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26308 as u32) ) } as u64;
	// 8242AE14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242AE18: 4E800421  bctrl
	ctx.lr = 0x8242AE1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242AE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242AE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242AE30 size=128
    let mut pc: u32 = 0x8242AE30;
    'dispatch: loop {
        match pc {
            0x8242AE30 => {
    //   block [0x8242AE30..0x8242AEB0)
	// 8242AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242AE38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242AE3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AE40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AE44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242AE48: 816B2BDC  lwz r11, 0x2bdc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11228 as u32) ) } as u64;
	// 8242AE4C: 48004105  bl 0x8242ef50
	ctx.lr = 0x8242AE50;
	sub_8242EF50(ctx, base);
	// 8242AE50: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242AE54: 817F9940  lwz r11, -0x66c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26304 as u32) ) } as u64;
	// 8242AE58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AE5C: 409A0030  bne cr6, 0x8242ae8c
	if !ctx.cr[6].eq {
	pc = 0x8242AE8C; continue 'dispatch;
	}
	// 8242AE60: 4BFF60D9  bl 0x82420f38
	ctx.lr = 0x8242AE64;
	sub_82420F38(ctx, base);
	// 8242AE64: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242AE68: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8242AE6C: 386B7CA0  addi r3, r11, 0x7ca0
	ctx.r[3].s64 = ctx.r[11].s64 + 31904;
	// 8242AE70: 60A58E00  ori r5, r5, 0x8e00
	ctx.r[5].u64 = ctx.r[5].u64 | 36352;
	// 8242AE74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242AE78: 4810A359  bl 0x825351d0
	ctx.lr = 0x8242AE7C;
	sub_825351D0(ctx, base);
	// 8242AE7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242AE80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242AE84: 4BFFFEE5  bl 0x8242ad68
	ctx.lr = 0x8242AE88;
	sub_8242AD68(ctx, base);
	// 8242AE88: 817F9940  lwz r11, -0x66c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26304 as u32) ) } as u64;
	// 8242AE8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242AE90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242AE94: 917F9940  stw r11, -0x66c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26304 as u32), ctx.r[11].u32 ) };
	// 8242AE98: 480040B9  bl 0x8242ef50
	ctx.lr = 0x8242AE9C;
	sub_8242EF50(ctx, base);
	// 8242AE9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242AEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AEA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242AEB0 size=152
    let mut pc: u32 = 0x8242AEB0;
    'dispatch: loop {
        match pc {
            0x8242AEB0 => {
    //   block [0x8242AEB0..0x8242AF48)
	// 8242AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242AEB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242AEBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242AEC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AEC4: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242AEC8: 816A9940  lwz r11, -0x66c0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26304 as u32) ) } as u64;
	// 8242AECC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242AED0: 916A9940  stw r11, -0x66c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26304 as u32), ctx.r[11].u32 ) };
	// 8242AED4: 4082005C  bne 0x8242af30
	if !ctx.cr[0].eq {
	pc = 0x8242AF30; continue 'dispatch;
	}
	// 8242AED8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242AEDC: 3BCB7CA0  addi r30, r11, 0x7ca0
	ctx.r[30].s64 = ctx.r[11].s64 + 31904;
	// 8242AEE0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8242AEE4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242AEE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242AEEC: 409A000C  bne cr6, 0x8242aef8
	if !ctx.cr[6].eq {
	pc = 0x8242AEF8; continue 'dispatch;
	}
	// 8242AEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242AEF4: 4BFFCBE5  bl 0x82427ad8
	ctx.lr = 0x8242AEF8;
	sub_82427AD8(ctx, base);
	// 8242AEF8: 3D7E0001  addis r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 65536;
	// 8242AEFC: 3BFF0238  addi r31, r31, 0x238
	ctx.r[31].s64 = ctx.r[31].s64 + 568;
	// 8242AF00: 396B8E00  addi r11, r11, -0x7200
	ctx.r[11].s64 = ctx.r[11].s64 + -29184;
	// 8242AF04: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242AF08: 4198FFDC  blt cr6, 0x8242aee4
	if ctx.cr[6].lt {
	pc = 0x8242AEE4; continue 'dispatch;
	}
	// 8242AF0C: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8242AF10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242AF14: 60A58E00  ori r5, r5, 0x8e00
	ctx.r[5].u64 = ctx.r[5].u64 | 36352;
	// 8242AF18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242AF1C: 4810A2B5  bl 0x825351d0
	ctx.lr = 0x8242AF20;
	sub_825351D0(ctx, base);
	// 8242AF20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242AF24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242AF28: 4BFFFE41  bl 0x8242ad68
	ctx.lr = 0x8242AF2C;
	sub_8242AD68(ctx, base);
	// 8242AF2C: 4BFF608D  bl 0x82420fb8
	ctx.lr = 0x8242AF30;
	sub_82420FB8(ctx, base);
	// 8242AF30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242AF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AF3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242AF40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242AF48 size=80
    let mut pc: u32 = 0x8242AF48;
    'dispatch: loop {
        match pc {
            0x8242AF48 => {
    //   block [0x8242AF48..0x8242AF98)
	// 8242AF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242AF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242AF50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242AF54: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242AF58: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8242AF5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242AF60: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8242AF64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242AF68: 4BFF85C1  bl 0x82423528
	ctx.lr = 0x8242AF6C;
	sub_82423528(ctx, base);
	// 8242AF6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8242AF70: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8242AF74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242AF78: 4BFF8641  bl 0x824235b8
	ctx.lr = 0x8242AF7C;
	sub_824235B8(ctx, base);
	// 8242AF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242AF80: 480027F9  bl 0x8242d778
	ctx.lr = 0x8242AF84;
	sub_8242D778(ctx, base);
	// 8242AF84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8242AF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242AF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242AF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242AF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242AF98 size=28
    let mut pc: u32 = 0x8242AF98;
    'dispatch: loop {
        match pc {
            0x8242AF98 => {
    //   block [0x8242AF98..0x8242AFB4)
	// 8242AF98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242AF9C: 409A0018  bne cr6, 0x8242afb4
	if !ctx.cr[6].eq {
		sub_8242AFB4(ctx, base);
		return;
	}
	// 8242AFA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AFA4: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242AFA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AFAC: 386B2C44  addi r3, r11, 0x2c44
	ctx.r[3].s64 = ctx.r[11].s64 + 11332;
	// 8242AFB0: 4BFFFF98  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242AFB4 size=32
    let mut pc: u32 = 0x8242AFB4;
    'dispatch: loop {
        match pc {
            0x8242AFB4 => {
    //   block [0x8242AFB4..0x8242AFD4)
	// 8242AFB4: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242AFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242AFBC: 409A0018  bne cr6, 0x8242afd4
	if !ctx.cr[6].eq {
		sub_8242AFD4(ctx, base);
		return;
	}
	// 8242AFC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AFC4: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242AFC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242AFCC: 386B2C38  addi r3, r11, 0x2c38
	ctx.r[3].s64 = ctx.r[11].s64 + 11320;
	// 8242AFD0: 4BFFFF78  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242AFD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242AFD4 size=112
    let mut pc: u32 = 0x8242AFD4;
    'dispatch: loop {
        match pc {
            0x8242AFD4 => {
    //   block [0x8242AFD4..0x8242B044)
	// 8242AFD4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242AFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242AFDC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242AFE0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8242AFE4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242AFE8: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 8242AFEC: 40810030  ble 0x8242b01c
	if !ctx.cr[0].gt {
	pc = 0x8242B01C; continue 'dispatch;
	}
	// 8242AFF0: 39480008  addi r10, r8, 8
	ctx.r[10].s64 = ctx.r[8].s64 + 8;
	// 8242AFF4: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 8242AFF8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242AFFC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242B000: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8242B004: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 8242B008: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8242B00C: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242B010: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8242B014: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8242B018: 4198FFDC  blt cr6, 0x8242aff4
	if ctx.cr[6].lt {
	pc = 0x8242AFF4; continue 'dispatch;
	}
	// 8242B01C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242B020: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8242B024: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B028: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8242B02C: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8242B030: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8242B034: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8242B038: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8242B03C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8242B040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B048 size=284
    let mut pc: u32 = 0x8242B048;
    'dispatch: loop {
        match pc {
            0x8242B048 => {
    //   block [0x8242B048..0x8242B164)
	// 8242B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242B054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B05C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8242B060: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8242B064: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8242B068: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242B06C: 409A001C  bne cr6, 0x8242b088
	if !ctx.cr[6].eq {
	pc = 0x8242B088; continue 'dispatch;
	}
	// 8242B070: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B074: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B078: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B07C: 386B2C74  addi r3, r11, 0x2c74
	ctx.r[3].s64 = ctx.r[11].s64 + 11380;
	// 8242B080: 4BFFFEC9  bl 0x8242af48
	ctx.lr = 0x8242B084;
	sub_8242AF48(ctx, base);
	// 8242B084: 480000C8  b 0x8242b14c
	pc = 0x8242B14C; continue 'dispatch;
	// 8242B088: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B08C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242B090: 409A0018  bne cr6, 0x8242b0a8
	if !ctx.cr[6].eq {
	pc = 0x8242B0A8; continue 'dispatch;
	}
	// 8242B094: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B098: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B09C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B0A0: 386B2C68  addi r3, r11, 0x2c68
	ctx.r[3].s64 = ctx.r[11].s64 + 11368;
	// 8242B0A4: 4BFFFFDC  b 0x8242b080
	pc = 0x8242B080; continue 'dispatch;
	// 8242B0A8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8242B0AC: 41990078  bgt cr6, 0x8242b124
	if ctx.cr[6].gt {
	pc = 0x8242B124; continue 'dispatch;
	}
	// 8242B0B0: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 8242B0B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242B0B8: 7FEB182E  lwzx r31, r11, r3
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242B0BC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B0C0: 41820080  beq 0x8242b140
	if ctx.cr[0].eq {
	pc = 0x8242B140; continue 'dispatch;
	}
	// 8242B0C4: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8242B0C8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8242B0CC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242B0D0: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242B0D4: 41990020  bgt cr6, 0x8242b0f4
	if ctx.cr[6].gt {
	pc = 0x8242B0F4; continue 'dispatch;
	}
	// 8242B0D8: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8242B0DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B0E0: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 8242B0E4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242B0E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B0EC: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8242B0F0: 4800005C  b 0x8242b14c
	pc = 0x8242B14C; continue 'dispatch;
	// 8242B0F4: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242B0F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242B0FC: 409A0044  bne cr6, 0x8242b140
	if !ctx.cr[6].eq {
	pc = 0x8242B140; continue 'dispatch;
	}
	// 8242B100: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8242B104: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242B108: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242B10C: 4BFFDDFD  bl 0x82428f08
	ctx.lr = 0x8242B110;
	sub_82428F08(ctx, base);
	// 8242B110: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8242B114: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8242B118: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8242B11C: F95F0008  std r10, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8242B120: 4800002C  b 0x8242b14c
	pc = 0x8242B14C; continue 'dispatch;
	// 8242B124: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B128: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B12C: 41820014  beq 0x8242b140
	if ctx.cr[0].eq {
	pc = 0x8242B140; continue 'dispatch;
	}
	// 8242B130: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B134: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B13C: 4E800421  bctrl
	ctx.lr = 0x8242B140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242B140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242B144: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B148: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242B14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242B150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B158: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242B15C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B168 size=28
    let mut pc: u32 = 0x8242B168;
    'dispatch: loop {
        match pc {
            0x8242B168 => {
    //   block [0x8242B168..0x8242B184)
	// 8242B168: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242B16C: 409A0018  bne cr6, 0x8242b184
	if !ctx.cr[6].eq {
		sub_8242B184(ctx, base);
		return;
	}
	// 8242B170: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B174: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B178: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B17C: 386B2C8C  addi r3, r11, 0x2c8c
	ctx.r[3].s64 = ctx.r[11].s64 + 11404;
	// 8242B180: 4BFFFDC8  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B184 size=32
    let mut pc: u32 = 0x8242B184;
    'dispatch: loop {
        match pc {
            0x8242B184 => {
    //   block [0x8242B184..0x8242B1A4)
	// 8242B184: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B18C: 409A0018  bne cr6, 0x8242b1a4
	if !ctx.cr[6].eq {
		sub_8242B1A4(ctx, base);
		return;
	}
	// 8242B190: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B194: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B198: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B19C: 386B2C80  addi r3, r11, 0x2c80
	ctx.r[3].s64 = ctx.r[11].s64 + 11392;
	// 8242B1A0: 4BFFFDA8  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B1A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B1A4 size=20
    let mut pc: u32 = 0x8242B1A4;
    'dispatch: loop {
        match pc {
            0x8242B1A4 => {
    //   block [0x8242B1A4..0x8242B1B8)
	// 8242B1A4: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 8242B1A8: 419900CC  bgt cr6, 0x8242b274
	if ctx.cr[6].gt {
		sub_8242B274(ctx, base);
		return;
	}
	// 8242B1AC: 80C50004  lwz r6, 4(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B1B0: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242B1B4: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B1B8 size=12
    let mut pc: u32 = 0x8242B1B8;
    'dispatch: loop {
        match pc {
            0x8242B1B8 => {
    //   block [0x8242B1B8..0x8242B1C4)
	// 8242B1B8: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B1BC: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B1C0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B1C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B1C4 size=20
    let mut pc: u32 = 0x8242B1C4;
    'dispatch: loop {
        match pc {
            0x8242B1C4 => {
    //   block [0x8242B1C4..0x8242B1D8)
	// 8242B1C4: 39640006  addi r11, r4, 6
	ctx.r[11].s64 = ctx.r[4].s64 + 6;
	// 8242B1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242B1CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242B1D0: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8242B1D4: 4800000C  b 0x8242b1e0
	sub_8242B1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B1D8 size=72
    let mut pc: u32 = 0x8242B1D8;
    'dispatch: loop {
        match pc {
            0x8242B1D8 => {
    //   block [0x8242B1D8..0x8242B220)
	// 8242B1D8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8242B1DC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242B1E0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B1E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B1E8: 4082FFF0  bne 0x8242b1d8
	if !ctx.cr[0].eq {
	pc = 0x8242B1D8; continue 'dispatch;
	}
	// 8242B1EC: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242B1F0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242B1F4: 409A002C  bne cr6, 0x8242b220
	if !ctx.cr[6].eq {
		sub_8242B220(ctx, base);
		return;
	}
	// 8242B1F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242B1FC: 419A0024  beq cr6, 0x8242b220
	if ctx.cr[6].eq {
		sub_8242B220(ctx, base);
		return;
	}
	// 8242B200: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242B204: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242B208: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8242B20C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8242B210: 409A0010  bne cr6, 0x8242b220
	if !ctx.cr[6].eq {
		sub_8242B220(ctx, base);
		return;
	}
	// 8242B214: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8242B218: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8242B21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B220 size=24
    let mut pc: u32 = 0x8242B220;
    'dispatch: loop {
        match pc {
            0x8242B220 => {
    //   block [0x8242B220..0x8242B238)
	// 8242B220: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242B224: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B228: 40820020  bne 0x8242b248
	if !ctx.cr[0].eq {
		sub_8242B248(ctx, base);
		return;
	}
	// 8242B22C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B234: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B238 size=16
    let mut pc: u32 = 0x8242B238;
    'dispatch: loop {
        match pc {
            0x8242B238 => {
    //   block [0x8242B238..0x8242B248)
	// 8242B238: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B23C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B240: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B244: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B248 size=44
    let mut pc: u32 = 0x8242B248;
    'dispatch: loop {
        match pc {
            0x8242B248 => {
    //   block [0x8242B248..0x8242B274)
	// 8242B248: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B24C: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 8242B250: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8242B254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242B258: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242B25C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B260: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242B264: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B268: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242B26C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B274 size=12
    let mut pc: u32 = 0x8242B274;
    'dispatch: loop {
        match pc {
            0x8242B274 => {
    //   block [0x8242B274..0x8242B280)
	// 8242B274: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B278: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B27C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B280 size=16
    let mut pc: u32 = 0x8242B280;
    'dispatch: loop {
        match pc {
            0x8242B280 => {
    //   block [0x8242B280..0x8242B290)
	// 8242B280: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B284: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B28C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B290 size=28
    let mut pc: u32 = 0x8242B290;
    'dispatch: loop {
        match pc {
            0x8242B290 => {
    //   block [0x8242B290..0x8242B2AC)
	// 8242B290: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242B294: 409A0018  bne cr6, 0x8242b2ac
	if !ctx.cr[6].eq {
		sub_8242B2AC(ctx, base);
		return;
	}
	// 8242B298: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B29C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B2A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B2A4: 386B2CA4  addi r3, r11, 0x2ca4
	ctx.r[3].s64 = ctx.r[11].s64 + 11428;
	// 8242B2A8: 4BFFFCA0  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B2AC size=32
    let mut pc: u32 = 0x8242B2AC;
    'dispatch: loop {
        match pc {
            0x8242B2AC => {
    //   block [0x8242B2AC..0x8242B2CC)
	// 8242B2AC: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B2B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B2B4: 409A0018  bne cr6, 0x8242b2cc
	if !ctx.cr[6].eq {
		sub_8242B2CC(ctx, base);
		return;
	}
	// 8242B2B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B2BC: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B2C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B2C4: 386B2C98  addi r3, r11, 0x2c98
	ctx.r[3].s64 = ctx.r[11].s64 + 11416;
	// 8242B2C8: 4BFFFC80  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B2CC size=20
    let mut pc: u32 = 0x8242B2CC;
    'dispatch: loop {
        match pc {
            0x8242B2CC => {
    //   block [0x8242B2CC..0x8242B2E0)
	// 8242B2CC: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 8242B2D0: 419900C0  bgt cr6, 0x8242b390
	if ctx.cr[6].gt {
		sub_8242B390(ctx, base);
		return;
	}
	// 8242B2D4: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B2D8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242B2DC: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B2E0 size=12
    let mut pc: u32 = 0x8242B2E0;
    'dispatch: loop {
        match pc {
            0x8242B2E0 => {
    //   block [0x8242B2E0..0x8242B2EC)
	// 8242B2E0: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B2E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B2E8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B2EC size=72
    let mut pc: u32 = 0x8242B2EC;
    'dispatch: loop {
        match pc {
            0x8242B2EC => {
    //   block [0x8242B2EC..0x8242B334)
	// 8242B2EC: 39640006  addi r11, r4, 6
	ctx.r[11].s64 = ctx.r[4].s64 + 6;
	// 8242B2F0: 88E30005  lbz r7, 5(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242B2F4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242B2F8: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 8242B2FC: 7D6A182E  lwzx r11, r10, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242B300: 409A0034  bne cr6, 0x8242b334
	if !ctx.cr[6].eq {
		sub_8242B334(ctx, base);
		return;
	}
	// 8242B304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B308: 419A002C  beq cr6, 0x8242b334
	if ctx.cr[6].eq {
		sub_8242B334(ctx, base);
		return;
	}
	// 8242B30C: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242B310: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8242B314: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8242B318: 409A001C  bne cr6, 0x8242b334
	if !ctx.cr[6].eq {
		sub_8242B334(ctx, base);
		return;
	}
	// 8242B31C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8242B320: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242B324: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B328: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8242B32C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8242B330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B334 size=24
    let mut pc: u32 = 0x8242B334;
    'dispatch: loop {
        match pc {
            0x8242B334 => {
    //   block [0x8242B334..0x8242B34C)
	// 8242B334: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242B338: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B33C: 40820020  bne 0x8242b35c
	if !ctx.cr[0].eq {
		sub_8242B35C(ctx, base);
		return;
	}
	// 8242B340: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B344: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B348: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B34C size=16
    let mut pc: u32 = 0x8242B34C;
    'dispatch: loop {
        match pc {
            0x8242B34C => {
    //   block [0x8242B34C..0x8242B35C)
	// 8242B34C: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B350: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B358: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B35C size=52
    let mut pc: u32 = 0x8242B35C;
    'dispatch: loop {
        match pc {
            0x8242B35C => {
    //   block [0x8242B35C..0x8242B390)
	// 8242B35C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8242B364: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 8242B368: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8242B36C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8242B370: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B374: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242B378: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B37C: 91270004  stw r9, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8242B380: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242B384: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242B388: 7D6A192E  stwx r11, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 8242B38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B390 size=12
    let mut pc: u32 = 0x8242B390;
    'dispatch: loop {
        match pc {
            0x8242B390 => {
    //   block [0x8242B390..0x8242B39C)
	// 8242B390: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B394: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B398: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B39C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B39C size=16
    let mut pc: u32 = 0x8242B39C;
    'dispatch: loop {
        match pc {
            0x8242B39C => {
    //   block [0x8242B39C..0x8242B3AC)
	// 8242B39C: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B3A0: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B3A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B3A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B3B0 size=100
    let mut pc: u32 = 0x8242B3B0;
    'dispatch: loop {
        match pc {
            0x8242B3B0 => {
    //   block [0x8242B3B0..0x8242B414)
	// 8242B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B3B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242B3BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B3C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B3C4: 480023BD  bl 0x8242d780
	ctx.lr = 0x8242B3C8;
	sub_8242D780(ctx, base);
	// 8242B3C8: 480024B1  bl 0x8242d878
	ctx.lr = 0x8242B3CC;
	sub_8242D878(ctx, base);
	// 8242B3CC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242B3D0: 83FE9944  lwz r31, -0x66bc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 8242B3D4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8242B3D8: 409A0018  bne cr6, 0x8242b3f0
	if !ctx.cr[6].eq {
	pc = 0x8242B3F0; continue 'dispatch;
	}
	// 8242B3DC: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242B3E0: 38A01200  li r5, 0x1200
	ctx.r[5].s64 = 4608;
	// 8242B3E4: 386B6AA0  addi r3, r11, 0x6aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 27296;
	// 8242B3E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242B3EC: 48109DE5  bl 0x825351d0
	ctx.lr = 0x8242B3F0;
	sub_825351D0(ctx, base);
	// 8242B3F0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8242B3F4: 917E9944  stw r11, -0x66bc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-26300 as u32), ctx.r[11].u32 ) };
	// 8242B3F8: 480024C1  bl 0x8242d8b8
	ctx.lr = 0x8242B3FC;
	sub_8242D8B8(ctx, base);
	// 8242B3FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242B40C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B418 size=80
    let mut pc: u32 = 0x8242B418;
    'dispatch: loop {
        match pc {
            0x8242B418 => {
    //   block [0x8242B418..0x8242B468)
	// 8242B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B424: 48002455  bl 0x8242d878
	ctx.lr = 0x8242B428;
	sub_8242D878(ctx, base);
	// 8242B428: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242B42C: 816A9944  lwz r11, -0x66bc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 8242B430: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242B434: 916A9944  stw r11, -0x66bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26300 as u32), ctx.r[11].u32 ) };
	// 8242B438: 40820018  bne 0x8242b450
	if !ctx.cr[0].eq {
	pc = 0x8242B450; continue 'dispatch;
	}
	// 8242B43C: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242B440: 38A01200  li r5, 0x1200
	ctx.r[5].s64 = 4608;
	// 8242B444: 386B6AA0  addi r3, r11, 0x6aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 27296;
	// 8242B448: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242B44C: 48109D85  bl 0x825351d0
	ctx.lr = 0x8242B450;
	sub_825351D0(ctx, base);
	// 8242B450: 48002469  bl 0x8242d8b8
	ctx.lr = 0x8242B454;
	sub_8242D8B8(ctx, base);
	// 8242B454: 480023AD  bl 0x8242d800
	ctx.lr = 0x8242B458;
	sub_8242D800(ctx, base);
	// 8242B458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B468 size=136
    let mut pc: u32 = 0x8242B468;
    'dispatch: loop {
        match pc {
            0x8242B468 => {
    //   block [0x8242B468..0x8242B4F0)
	// 8242B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B47C: 480023FD  bl 0x8242d878
	ctx.lr = 0x8242B480;
	sub_8242D878(ctx, base);
	// 8242B480: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242B484: 409A001C  bne cr6, 0x8242b4a0
	if !ctx.cr[6].eq {
	pc = 0x8242B4A0; continue 'dispatch;
	}
	// 8242B488: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B48C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B490: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B494: 386B2BFC  addi r3, r11, 0x2bfc
	ctx.r[3].s64 = ctx.r[11].s64 + 11260;
	// 8242B498: 4BFFFAB1  bl 0x8242af48
	ctx.lr = 0x8242B49C;
	sub_8242AF48(ctx, base);
	// 8242B49C: 4800003C  b 0x8242b4d8
	pc = 0x8242B4D8; continue 'dispatch;
	// 8242B4A0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B4A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B4A8: 409A0018  bne cr6, 0x8242b4c0
	if !ctx.cr[6].eq {
	pc = 0x8242B4C0; continue 'dispatch;
	}
	// 8242B4AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B4B0: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B4B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B4B8: 386B2BF0  addi r3, r11, 0x2bf0
	ctx.r[3].s64 = ctx.r[11].s64 + 11248;
	// 8242B4BC: 4BFFFFDC  b 0x8242b498
	pc = 0x8242B498; continue 'dispatch;
	// 8242B4C0: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 8242B4C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242B4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B4CC: 48109D05  bl 0x825351d0
	ctx.lr = 0x8242B4D0;
	sub_825351D0(ctx, base);
	// 8242B4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242B4D4: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8242B4D8: 480023E1  bl 0x8242d8b8
	ctx.lr = 0x8242B4DC;
	sub_8242D8B8(ctx, base);
	// 8242B4DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B4E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B4F0 size=124
    let mut pc: u32 = 0x8242B4F0;
    'dispatch: loop {
        match pc {
            0x8242B4F0 => {
    //   block [0x8242B4F0..0x8242B56C)
	// 8242B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B4F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B4FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B504: 48002375  bl 0x8242d878
	ctx.lr = 0x8242B508;
	sub_8242D878(ctx, base);
	// 8242B508: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242B50C: 409A0020  bne cr6, 0x8242b52c
	if !ctx.cr[6].eq {
	pc = 0x8242B52C; continue 'dispatch;
	}
	// 8242B510: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B514: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B51C: 386B2C14  addi r3, r11, 0x2c14
	ctx.r[3].s64 = ctx.r[11].s64 + 11284;
	// 8242B520: 4BFFFA29  bl 0x8242af48
	ctx.lr = 0x8242B524;
	sub_8242AF48(ctx, base);
	// 8242B524: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242B528: 48000028  b 0x8242b550
	pc = 0x8242B550; continue 'dispatch;
	// 8242B52C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B534: 409A0018  bne cr6, 0x8242b54c
	if !ctx.cr[6].eq {
	pc = 0x8242B54C; continue 'dispatch;
	}
	// 8242B538: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B53C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B540: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B544: 386B2C08  addi r3, r11, 0x2c08
	ctx.r[3].s64 = ctx.r[11].s64 + 11272;
	// 8242B548: 4BFFFFD8  b 0x8242b520
	pc = 0x8242B520; continue 'dispatch;
	// 8242B54C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242B550: 48002369  bl 0x8242d8b8
	ctx.lr = 0x8242B554;
	sub_8242D8B8(ctx, base);
	// 8242B554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B570 size=112
    let mut pc: u32 = 0x8242B570;
    'dispatch: loop {
        match pc {
            0x8242B570 => {
    //   block [0x8242B570..0x8242B5E0)
	// 8242B570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B574: 48109B49  bl 0x825350bc
	ctx.lr = 0x8242B578;
	sub_82535080(ctx, base);
	// 8242B578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B57C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242B584: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242B588: 480022F1  bl 0x8242d878
	ctx.lr = 0x8242B58C;
	sub_8242D878(ctx, base);
	// 8242B58C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242B590: 409A001C  bne cr6, 0x8242b5ac
	if !ctx.cr[6].eq {
	pc = 0x8242B5AC; continue 'dispatch;
	}
	// 8242B594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B598: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B59C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B5A0: 386B2C2C  addi r3, r11, 0x2c2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11308;
	// 8242B5A4: 4BFFF9A5  bl 0x8242af48
	ctx.lr = 0x8242B5A8;
	sub_8242AF48(ctx, base);
	// 8242B5A8: 4800002C  b 0x8242b5d4
	pc = 0x8242B5D4; continue 'dispatch;
	// 8242B5AC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B5B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B5B4: 409A0018  bne cr6, 0x8242b5cc
	if !ctx.cr[6].eq {
	pc = 0x8242B5CC; continue 'dispatch;
	}
	// 8242B5B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B5BC: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B5C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B5C4: 386B2C20  addi r3, r11, 0x2c20
	ctx.r[3].s64 = ctx.r[11].s64 + 11296;
	// 8242B5C8: 4BFFFFDC  b 0x8242b5a4
	pc = 0x8242B5A4; continue 'dispatch;
	// 8242B5CC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8242B5D0: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8242B5D4: 480022E5  bl 0x8242d8b8
	ctx.lr = 0x8242B5D8;
	sub_8242D8B8(ctx, base);
	// 8242B5D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B5DC: 48109B30  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B5E0 size=56
    let mut pc: u32 = 0x8242B5E0;
    'dispatch: loop {
        match pc {
            0x8242B5E0 => {
    //   block [0x8242B5E0..0x8242B618)
	// 8242B5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B5F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B5F4: 48002285  bl 0x8242d878
	ctx.lr = 0x8242B5F8;
	sub_8242D878(ctx, base);
	// 8242B5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B5FC: 4BFFF99D  bl 0x8242af98
	ctx.lr = 0x8242B600;
	sub_8242AF98(ctx, base);
	// 8242B600: 480022B9  bl 0x8242d8b8
	ctx.lr = 0x8242B604;
	sub_8242D8B8(ctx, base);
	// 8242B604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B618 size=212
    let mut pc: u32 = 0x8242B618;
    'dispatch: loop {
        match pc {
            0x8242B618 => {
    //   block [0x8242B618..0x8242B6EC)
	// 8242B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242B624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B62C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242B630: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242B634: 48002245  bl 0x8242d878
	ctx.lr = 0x8242B638;
	sub_8242D878(ctx, base);
	// 8242B638: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242B63C: 409A001C  bne cr6, 0x8242b658
	if !ctx.cr[6].eq {
	pc = 0x8242B658; continue 'dispatch;
	}
	// 8242B640: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B644: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B64C: 386B2C5C  addi r3, r11, 0x2c5c
	ctx.r[3].s64 = ctx.r[11].s64 + 11356;
	// 8242B650: 4BFFF8F9  bl 0x8242af48
	ctx.lr = 0x8242B654;
	sub_8242AF48(ctx, base);
	// 8242B654: 48000074  b 0x8242b6c8
	pc = 0x8242B6C8; continue 'dispatch;
	// 8242B658: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B65C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B660: 409A0018  bne cr6, 0x8242b678
	if !ctx.cr[6].eq {
	pc = 0x8242B678; continue 'dispatch;
	}
	// 8242B664: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B668: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B66C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B670: 386B2C50  addi r3, r11, 0x2c50
	ctx.r[3].s64 = ctx.r[11].s64 + 11344;
	// 8242B674: 4BFFFFDC  b 0x8242b650
	pc = 0x8242B650; continue 'dispatch;
	// 8242B678: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8242B67C: 41990030  bgt cr6, 0x8242b6ac
	if ctx.cr[6].gt {
	pc = 0x8242B6AC; continue 'dispatch;
	}
	// 8242B680: 397F0006  addi r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 + 6;
	// 8242B684: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242B688: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242B68C: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8242B690: 48000010  b 0x8242b6a0
	pc = 0x8242B6A0; continue 'dispatch;
	// 8242B694: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242B698: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242B69C: 7FEAFA14  add r31, r10, r31
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8242B6A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B6A4: 4082FFF0  bne 0x8242b694
	if !ctx.cr[0].eq {
	pc = 0x8242B694; continue 'dispatch;
	}
	// 8242B6A8: 48000024  b 0x8242b6cc
	pc = 0x8242B6CC; continue 'dispatch;
	// 8242B6AC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B6B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B6B4: 41820014  beq 0x8242b6c8
	if ctx.cr[0].eq {
	pc = 0x8242B6C8; continue 'dispatch;
	}
	// 8242B6B8: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B6BC: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B6C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B6C4: 4E800421  bctrl
	ctx.lr = 0x8242B6C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242B6C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242B6CC: 480021ED  bl 0x8242d8b8
	ctx.lr = 0x8242B6D0;
	sub_8242D8B8(ctx, base);
	// 8242B6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B6D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B6E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242B6E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B6F0 size=64
    let mut pc: u32 = 0x8242B6F0;
    'dispatch: loop {
        match pc {
            0x8242B6F0 => {
    //   block [0x8242B6F0..0x8242B730)
	// 8242B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B6F4: 481099C5  bl 0x825350b8
	ctx.lr = 0x8242B6F8;
	sub_82535080(ctx, base);
	// 8242B6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B700: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242B704: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242B708: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8242B70C: 4800216D  bl 0x8242d878
	ctx.lr = 0x8242B710;
	sub_8242D878(ctx, base);
	// 8242B710: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8242B714: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242B718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242B71C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B720: 4BFFF929  bl 0x8242b048
	ctx.lr = 0x8242B724;
	sub_8242B048(ctx, base);
	// 8242B724: 48002195  bl 0x8242d8b8
	ctx.lr = 0x8242B728;
	sub_8242D8B8(ctx, base);
	// 8242B728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242B72C: 481099DC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B730 size=56
    let mut pc: u32 = 0x8242B730;
    'dispatch: loop {
        match pc {
            0x8242B730 => {
    //   block [0x8242B730..0x8242B768)
	// 8242B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B734: 48109989  bl 0x825350bc
	ctx.lr = 0x8242B738;
	sub_82535080(ctx, base);
	// 8242B738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B73C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B740: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242B744: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242B748: 48002131  bl 0x8242d878
	ctx.lr = 0x8242B74C;
	sub_8242D878(ctx, base);
	// 8242B74C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242B750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242B754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B758: 4BFFFA11  bl 0x8242b168
	ctx.lr = 0x8242B75C;
	sub_8242B168(ctx, base);
	// 8242B75C: 4800215D  bl 0x8242d8b8
	ctx.lr = 0x8242B760;
	sub_8242D8B8(ctx, base);
	// 8242B760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B764: 481099A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B768 size=56
    let mut pc: u32 = 0x8242B768;
    'dispatch: loop {
        match pc {
            0x8242B768 => {
    //   block [0x8242B768..0x8242B7A0)
	// 8242B768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B76C: 48109951  bl 0x825350bc
	ctx.lr = 0x8242B770;
	sub_82535080(ctx, base);
	// 8242B770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242B77C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242B780: 480020F9  bl 0x8242d878
	ctx.lr = 0x8242B784;
	sub_8242D878(ctx, base);
	// 8242B784: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242B788: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242B78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B790: 4BFFFB01  bl 0x8242b290
	ctx.lr = 0x8242B794;
	sub_8242B290(ctx, base);
	// 8242B794: 48002125  bl 0x8242d8b8
	ctx.lr = 0x8242B798;
	sub_8242D8B8(ctx, base);
	// 8242B798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B79C: 48109970  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B7A0 size=240
    let mut pc: u32 = 0x8242B7A0;
    'dispatch: loop {
        match pc {
            0x8242B7A0 => {
    //   block [0x8242B7A0..0x8242B890)
	// 8242B7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B7A4: 48109915  bl 0x825350b8
	ctx.lr = 0x8242B7A8;
	sub_82535080(ctx, base);
	// 8242B7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B7B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242B7B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8242B7B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8242B7BC: 480020BD  bl 0x8242d878
	ctx.lr = 0x8242B7C0;
	sub_8242D878(ctx, base);
	// 8242B7C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242B7C4: 409A001C  bne cr6, 0x8242b7e0
	if !ctx.cr[6].eq {
	pc = 0x8242B7E0; continue 'dispatch;
	}
	// 8242B7C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B7CC: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242B7D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B7D4: 386B2CBC  addi r3, r11, 0x2cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 11452;
	// 8242B7D8: 4BFFF771  bl 0x8242af48
	ctx.lr = 0x8242B7DC;
	sub_8242AF48(ctx, base);
	// 8242B7DC: 480000A0  b 0x8242b87c
	pc = 0x8242B87C; continue 'dispatch;
	// 8242B7E0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242B7E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242B7E8: 409A0018  bne cr6, 0x8242b800
	if !ctx.cr[6].eq {
	pc = 0x8242B800; continue 'dispatch;
	}
	// 8242B7EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B7F0: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242B7F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B7F8: 386B2CB0  addi r3, r11, 0x2cb0
	ctx.r[3].s64 = ctx.r[11].s64 + 11440;
	// 8242B7FC: 4BFFFFDC  b 0x8242b7d8
	pc = 0x8242B7D8; continue 'dispatch;
	// 8242B800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242B804: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 8242B808: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B80C: 41990054  bgt cr6, 0x8242b860
	if ctx.cr[6].gt {
	pc = 0x8242B860; continue 'dispatch;
	}
	// 8242B810: 397E0006  addi r11, r30, 6
	ctx.r[11].s64 = ctx.r[30].s64 + 6;
	// 8242B814: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242B818: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8242B81C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B820: 4182005C  beq 0x8242b87c
	if ctx.cr[0].eq {
	pc = 0x8242B87C; continue 'dispatch;
	}
	// 8242B824: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8242B828: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8242B82C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242B830: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242B834: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242B838: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8242B83C: 409A0014  bne cr6, 0x8242b850
	if !ctx.cr[6].eq {
	pc = 0x8242B850; continue 'dispatch;
	}
	// 8242B840: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8242B844: 41980038  blt cr6, 0x8242b87c
	if ctx.cr[6].lt {
	pc = 0x8242B87C; continue 'dispatch;
	}
	// 8242B848: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8242B84C: 48000034  b 0x8242b880
	pc = 0x8242B880; continue 'dispatch;
	// 8242B850: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 8242B854: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8242B858: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8242B85C: 48000024  b 0x8242b880
	pc = 0x8242B880; continue 'dispatch;
	// 8242B860: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242B864: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242B868: 41820014  beq 0x8242b87c
	if ctx.cr[0].eq {
	pc = 0x8242B87C; continue 'dispatch;
	}
	// 8242B86C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242B870: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8242B874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242B878: 4E800421  bctrl
	ctx.lr = 0x8242B87C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242B87C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242B880: 48002039  bl 0x8242d8b8
	ctx.lr = 0x8242B884;
	sub_8242D8B8(ctx, base);
	// 8242B884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242B888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242B88C: 4810987C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242B890 size=12
    let mut pc: u32 = 0x8242B890;
    'dispatch: loop {
        match pc {
            0x8242B890 => {
    //   block [0x8242B890..0x8242B89C)
	// 8242B890: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8242B894: 906B3F14  stw r3, 0x3f14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16148 as u32), ctx.r[3].u32 ) };
	// 8242B898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B8A0 size=64
    let mut pc: u32 = 0x8242B8A0;
    'dispatch: loop {
        match pc {
            0x8242B8A0 => {
    //   block [0x8242B8A0..0x8242B8E0)
	// 8242B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B8AC: 4BFFA46D  bl 0x82425d18
	ctx.lr = 0x8242B8B0;
	sub_82425D18(ctx, base);
	// 8242B8B0: 4BFFA471  bl 0x82425d20
	ctx.lr = 0x8242B8B4;
	sub_82425D20(ctx, base);
	// 8242B8B4: 4BFFA475  bl 0x82425d28
	ctx.lr = 0x8242B8B8;
	sub_82425D28(ctx, base);
	// 8242B8B8: 4BFFA479  bl 0x82425d30
	ctx.lr = 0x8242B8BC;
	sub_82425D30(ctx, base);
	// 8242B8BC: 4BFFA47D  bl 0x82425d38
	ctx.lr = 0x8242B8C0;
	sub_82425D38(ctx, base);
	// 8242B8C0: 4BFFA481  bl 0x82425d40
	ctx.lr = 0x8242B8C4;
	sub_82425D40(ctx, base);
	// 8242B8C4: 4BFFA485  bl 0x82425d48
	ctx.lr = 0x8242B8C8;
	sub_82425D48(ctx, base);
	// 8242B8C8: 4BFFA489  bl 0x82425d50
	ctx.lr = 0x8242B8CC;
	sub_82425D50(ctx, base);
	// 8242B8CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242B8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B8E0 size=112
    let mut pc: u32 = 0x8242B8E0;
    'dispatch: loop {
        match pc {
            0x8242B8E0 => {
    //   block [0x8242B8E0..0x8242B950)
	// 8242B8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B8EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8242B8F0: 816B3F14  lwz r11, 0x3f14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16148 as u32) ) } as u64;
	// 8242B8F4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8242B8F8: 409A0014  bne cr6, 0x8242b90c
	if !ctx.cr[6].eq {
	pc = 0x8242B90C; continue 'dispatch;
	}
	// 8242B8FC: 4BFF0815  bl 0x8241c110
	ctx.lr = 0x8242B900;
	sub_8241C110(ctx, base);
	// 8242B900: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8242B904: 419A002C  beq cr6, 0x8242b930
	if ctx.cr[6].eq {
	pc = 0x8242B930; continue 'dispatch;
	}
	// 8242B908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242B90C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242B910: 419A0028  beq cr6, 0x8242b938
	if ctx.cr[6].eq {
	pc = 0x8242B938; continue 'dispatch;
	}
	// 8242B914: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242B918: 419A0018  beq cr6, 0x8242b930
	if ctx.cr[6].eq {
	pc = 0x8242B930; continue 'dispatch;
	}
	// 8242B91C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8242B920: 409A001C  bne cr6, 0x8242b93c
	if !ctx.cr[6].eq {
	pc = 0x8242B93C; continue 'dispatch;
	}
	// 8242B924: 4BFFA425  bl 0x82425d48
	ctx.lr = 0x8242B928;
	sub_82425D48(ctx, base);
	// 8242B928: 4BFFA429  bl 0x82425d50
	ctx.lr = 0x8242B92C;
	sub_82425D50(ctx, base);
	// 8242B92C: 48000010  b 0x8242b93c
	pc = 0x8242B93C; continue 'dispatch;
	// 8242B930: 4BFFA411  bl 0x82425d40
	ctx.lr = 0x8242B934;
	sub_82425D40(ctx, base);
	// 8242B934: 48000008  b 0x8242b93c
	pc = 0x8242B93C; continue 'dispatch;
	// 8242B938: 4BFFFF69  bl 0x8242b8a0
	ctx.lr = 0x8242B93C;
	sub_8242B8A0(ctx, base);
	// 8242B93C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242B940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242B944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242B950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242B950 size=176
    let mut pc: u32 = 0x8242B950;
    'dispatch: loop {
        match pc {
            0x8242B950 => {
    //   block [0x8242B950..0x8242BA00)
	// 8242B950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242B954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242B958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242B95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242B960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242B964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242B968: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8242B96C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242B970: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242B974: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8242B978: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242B97C: 40990018  ble cr6, 0x8242b994
	if !ctx.cr[6].gt {
	pc = 0x8242B994; continue 'dispatch;
	}
	// 8242B980: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B984: 386B2CF0  addi r3, r11, 0x2cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 11504;
	// 8242B988: 4BFF1281  bl 0x8241cc08
	ctx.lr = 0x8242B98C;
	sub_8241CC08(ctx, base);
	// 8242B98C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242B990: 48000058  b 0x8242b9e8
	pc = 0x8242B9E8; continue 'dispatch;
	// 8242B994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8242B998: 3D000200  lis r8, 0x200
	ctx.r[8].s64 = 33554432;
	// 8242B99C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8242B9A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8242B9A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8242B9A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242B9AC: 4BF95585  bl 0x823c0f30
	ctx.lr = 0x8242B9B0;
	sub_823C0F30(ctx, base);
	// 8242B9B0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8242B9B4: 7C7EF92E  stwx r3, r30, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[3].u32) };
	// 8242B9B8: 409A0014  bne cr6, 0x8242b9cc
	if !ctx.cr[6].eq {
	pc = 0x8242B9CC; continue 'dispatch;
	}
	// 8242B9BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242B9C0: 386B2CC8  addi r3, r11, 0x2cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 11464;
	// 8242B9C4: 4BFF1245  bl 0x8241cc08
	ctx.lr = 0x8242B9C8;
	sub_8241CC08(ctx, base);
	// 8242B9C8: 4800001C  b 0x8242b9e4
	pc = 0x8242B9E4; continue 'dispatch;
	// 8242B9CC: 815F012C  lwz r10, 0x12c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 8242B9D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242B9D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242B9D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242B9DC: 915F012C  stw r10, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[10].u32 ) };
	// 8242B9E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8242B9E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242B9E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242B9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242B9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242B9F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242B9F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242B9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BA00 size=220
    let mut pc: u32 = 0x8242BA00;
    'dispatch: loop {
        match pc {
            0x8242BA00 => {
    //   block [0x8242BA00..0x8242BADC)
	// 8242BA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BA04: 481096B5  bl 0x825350b8
	ctx.lr = 0x8242BA08;
	sub_82535080(ctx, base);
	// 8242BA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BA0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242BA10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242BA14: 409A0018  bne cr6, 0x8242ba2c
	if !ctx.cr[6].eq {
	pc = 0x8242BA2C; continue 'dispatch;
	}
	// 8242BA18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BA1C: 386B2D1C  addi r3, r11, 0x2d1c
	ctx.r[3].s64 = ctx.r[11].s64 + 11548;
	// 8242BA20: 4BFF11E9  bl 0x8241cc08
	ctx.lr = 0x8242BA24;
	sub_8241CC08(ctx, base);
	// 8242BA24: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242BA28: 480000AC  b 0x8242bad4
	pc = 0x8242BAD4; continue 'dispatch;
	// 8242BA2C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BA30: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8242BA34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242BA38: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242BA3C: 4182000C  beq 0x8242ba48
	if ctx.cr[0].eq {
	pc = 0x8242BA48; continue 'dispatch;
	}
	// 8242BA40: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242BA44: 4800000C  b 0x8242ba50
	pc = 0x8242BA50; continue 'dispatch;
	// 8242BA48: 3D20828A  lis r9, -0x7d76
	ctx.r[9].s64 = -2104885248;
	// 8242BA4C: 9169996C  stw r11, -0x6694(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26260 as u32), ctx.r[11].u32 ) };
	// 8242BA50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242BA54: 419A0008  beq cr6, 0x8242ba5c
	if ctx.cr[6].eq {
	pc = 0x8242BA5C; continue 'dispatch;
	}
	// 8242BA58: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242BA5C: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 8242BA60: 3BDF0130  addi r30, r31, 0x130
	ctx.r[30].s64 = ctx.r[31].s64 + 304;
	// 8242BA64: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8242BA68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242BA6C: 4099002C  ble cr6, 0x8242ba98
	if !ctx.cr[6].gt {
	pc = 0x8242BA98; continue 'dispatch;
	}
	// 8242BA70: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BA74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242BA78: 4182000C  beq 0x8242ba84
	if ctx.cr[0].eq {
	pc = 0x8242BA84; continue 'dispatch;
	}
	// 8242BA7C: 4BF94EDD  bl 0x823c0958
	ctx.lr = 0x8242BA80;
	sub_823C0958(ctx, base);
	// 8242BA80: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8242BA84: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 8242BA88: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8242BA8C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8242BA90: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242BA94: 4198FFDC  blt cr6, 0x8242ba70
	if ctx.cr[6].lt {
	pc = 0x8242BA70; continue 'dispatch;
	}
	// 8242BA98: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242BA9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242BAA0: 419A0008  beq cr6, 0x8242baa8
	if ctx.cr[6].eq {
	pc = 0x8242BAA8; continue 'dispatch;
	}
	// 8242BAA4: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242BAA8: 38A00130  li r5, 0x130
	ctx.r[5].s64 = 304;
	// 8242BAAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242BAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BAB4: 4810971D  bl 0x825351d0
	ctx.lr = 0x8242BAB8;
	sub_825351D0(ctx, base);
	// 8242BAB8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8242BABC: 419A0014  beq cr6, 0x8242bad0
	if ctx.cr[6].eq {
	pc = 0x8242BAD0; continue 'dispatch;
	}
	// 8242BAC0: 4BFA7C99  bl 0x823d3758
	ctx.lr = 0x8242BAC4;
	sub_823D3758(ctx, base);
	// 8242BAC4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242BAC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242BACC: 4BFA70ED  bl 0x823d2bb8
	ctx.lr = 0x8242BAD0;
	sub_823D2BB8(ctx, base);
	// 8242BAD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242BAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242BAD8: 48109630  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242BAE0 size=24
    let mut pc: u32 = 0x8242BAE0;
    'dispatch: loop {
        match pc {
            0x8242BAE0 => {
    //   block [0x8242BAE0..0x8242BAF8)
	// 8242BAE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242BAE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242BAE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BAEC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8242BAF0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242BAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BAF8 size=188
    let mut pc: u32 = 0x8242BAF8;
    'dispatch: loop {
        match pc {
            0x8242BAF8 => {
    //   block [0x8242BAF8..0x8242BBB4)
	// 8242BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BAFC: 481095C1  bl 0x825350bc
	ctx.lr = 0x8242BB00;
	sub_82535080(ctx, base);
	// 8242BB00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BB04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242BB08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242BB0C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242BB10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242BB14: 419A0014  beq cr6, 0x8242bb28
	if ctx.cr[6].eq {
	pc = 0x8242BB28; continue 'dispatch;
	}
	// 8242BB18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242BB1C: 419A000C  beq cr6, 0x8242bb28
	if ctx.cr[6].eq {
	pc = 0x8242BB28; continue 'dispatch;
	}
	// 8242BB20: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242BB24: 409A0010  bne cr6, 0x8242bb34
	if !ctx.cr[6].eq {
	pc = 0x8242BB34; continue 'dispatch;
	}
	// 8242BB28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BB2C: 386B2D44  addi r3, r11, 0x2d44
	ctx.r[3].s64 = ctx.r[11].s64 + 11588;
	// 8242BB30: 4BFF10D9  bl 0x8241cc08
	ctx.lr = 0x8242BB34;
	sub_8241CC08(ctx, base);
	// 8242BB34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242BB38: 80BF0124  lwz r5, 0x124(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8242BB3C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8242BB40: 4810C181  bl 0x82537cc0
	ctx.lr = 0x8242BB44;
	sub_82537CC0(ctx, base);
	// 8242BB44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BB48: 40820058  bne 0x8242bba0
	if !ctx.cr[0].eq {
	pc = 0x8242BBA0; continue 'dispatch;
	}
	// 8242BB4C: 817F0128  lwz r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 8242BB50: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8242BB54: 409A000C  bne cr6, 0x8242bb60
	if !ctx.cr[6].eq {
	pc = 0x8242BB60; continue 'dispatch;
	}
	// 8242BB58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242BB5C: 48000048  b 0x8242bba4
	pc = 0x8242BBA4; continue 'dispatch;
	// 8242BB60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242BB64: 4811B51D  bl 0x82547080
	ctx.lr = 0x8242BB68;
	sub_82547080(ctx, base);
	// 8242BB68: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8242BB6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242BB70: 48000014  b 0x8242bb84
	pc = 0x8242BB84; continue 'dispatch;
	// 8242BB74: 7D2BF0AE  lbzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8242BB78: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 8242BB7C: 409A0008  bne cr6, 0x8242bb84
	if !ctx.cr[6].eq {
	pc = 0x8242BB84; continue 'dispatch;
	}
	// 8242BB80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242BB84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242BB88: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8242BB8C: 4198FFE8  blt cr6, 0x8242bb74
	if ctx.cr[6].lt {
	pc = 0x8242BB74; continue 'dispatch;
	}
	// 8242BB90: 817F0128  lwz r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 8242BB94: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242BB98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242BB9C: 40990008  ble cr6, 0x8242bba4
	if !ctx.cr[6].gt {
	pc = 0x8242BBA4; continue 'dispatch;
	}
	// 8242BBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242BBA4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242BBA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242BBAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242BBB0: 4810955C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BBB8 size=188
    let mut pc: u32 = 0x8242BBB8;
    'dispatch: loop {
        match pc {
            0x8242BBB8 => {
    //   block [0x8242BBB8..0x8242BC74)
	// 8242BBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242BBC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242BBC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BBC8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242BBCC: 3BEB9948  addi r31, r11, -0x66b8
	ctx.r[31].s64 = ctx.r[11].s64 + -26296;
	// 8242BBD0: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 8242BBD4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242BBD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242BBDC: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242BBE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242BBE4: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242BBE8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242BBEC: 4082FFE8  bne 0x8242bbd4
	if !ctx.cr[0].eq {
	pc = 0x8242BBD4; continue 'dispatch;
	}
	// 8242BBF0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242BBF4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242BBF8: 409A0064  bne cr6, 0x8242bc5c
	if !ctx.cr[6].eq {
	pc = 0x8242BC5C; continue 'dispatch;
	}
	// 8242BBFC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8242BC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BC04: 48001255  bl 0x8242ce58
	ctx.lr = 0x8242BC08;
	sub_8242CE58(ctx, base);
	// 8242BC08: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8242BC0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242BC10: 40820018  bne 0x8242bc28
	if !ctx.cr[0].eq {
	pc = 0x8242BC28; continue 'dispatch;
	}
	// 8242BC14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BC18: 386B2E2C  addi r3, r11, 0x2e2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11820;
	// 8242BC1C: 4BFF0FED  bl 0x8241cc08
	ctx.lr = 0x8242BC20;
	sub_8241CC08(ctx, base);
	// 8242BC20: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242BC24: 4800003C  b 0x8242bc60
	pc = 0x8242BC60; continue 'dispatch;
	// 8242BC28: 480013A1  bl 0x8242cfc8
	ctx.lr = 0x8242BC2C;
	sub_8242CFC8(ctx, base);
	// 8242BC2C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BC30: 40800010  bge 0x8242bc40
	if !ctx.cr[0].lt {
	pc = 0x8242BC40; continue 'dispatch;
	}
	// 8242BC34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BC38: 386B2D70  addi r3, r11, 0x2d70
	ctx.r[3].s64 = ctx.r[11].s64 + 11632;
	// 8242BC3C: 4BFF0FCD  bl 0x8241cc08
	ctx.lr = 0x8242BC40;
	sub_8241CC08(ctx, base);
	// 8242BC40: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242BC44: 4800141D  bl 0x8242d060
	ctx.lr = 0x8242BC48;
	sub_8242D060(ctx, base);
	// 8242BC48: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BC4C: 40800010  bge 0x8242bc5c
	if !ctx.cr[0].lt {
	pc = 0x8242BC5C; continue 'dispatch;
	}
	// 8242BC50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BC54: 386B2DD0  addi r3, r11, 0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11728;
	// 8242BC58: 4BFF0FB1  bl 0x8241cc08
	ctx.lr = 0x8242BC5C;
	sub_8241CC08(ctx, base);
	// 8242BC5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242BC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242BC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242BC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242BC6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242BC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BC78 size=208
    let mut pc: u32 = 0x8242BC78;
    'dispatch: loop {
        match pc {
            0x8242BC78 => {
    //   block [0x8242BC78..0x8242BD48)
	// 8242BC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242BC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242BC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242BC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BC8C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242BC90: 3BEB9970  addi r31, r11, -0x6690
	ctx.r[31].s64 = ctx.r[11].s64 + -26256;
	// 8242BC94: 391FFFF8  addi r8, r31, -8
	ctx.r[8].s64 = ctx.r[31].s64 + -8;
	// 8242BC98: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242BC9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242BCA0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242BCA4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242BCA8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242BCAC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242BCB0: 4082FFE8  bne 0x8242bc98
	if !ctx.cr[0].eq {
	pc = 0x8242BC98; continue 'dispatch;
	}
	// 8242BCB4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242BCB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242BCBC: 409A0070  bne cr6, 0x8242bd2c
	if !ctx.cr[6].eq {
	pc = 0x8242BD2C; continue 'dispatch;
	}
	// 8242BCC0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BCC4: 48001305  bl 0x8242cfc8
	ctx.lr = 0x8242BCC8;
	sub_8242CFC8(ctx, base);
	// 8242BCC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BCCC: 40800010  bge 0x8242bcdc
	if !ctx.cr[0].lt {
	pc = 0x8242BCDC; continue 'dispatch;
	}
	// 8242BCD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BCD4: 386B2D70  addi r3, r11, 0x2d70
	ctx.r[3].s64 = ctx.r[11].s64 + 11632;
	// 8242BCD8: 4BFF0F31  bl 0x8241cc08
	ctx.lr = 0x8242BCDC;
	sub_8241CC08(ctx, base);
	// 8242BCDC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242BCE0: 4800000C  b 0x8242bcec
	pc = 0x8242BCEC; continue 'dispatch;
	// 8242BCE4: 807E996C  lwz r3, -0x6694(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242BCE8: 4BFFFD19  bl 0x8242ba00
	ctx.lr = 0x8242BCEC;
	sub_8242BA00(ctx, base);
	// 8242BCEC: 817E996C  lwz r11, -0x6694(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242BCF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242BCF4: 409AFFF0  bne cr6, 0x8242bce4
	if !ctx.cr[6].eq {
	pc = 0x8242BCE4; continue 'dispatch;
	}
	// 8242BCF8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BCFC: 48001365  bl 0x8242d060
	ctx.lr = 0x8242BD00;
	sub_8242D060(ctx, base);
	// 8242BD00: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BD04: 40800010  bge 0x8242bd14
	if !ctx.cr[0].lt {
	pc = 0x8242BD14; continue 'dispatch;
	}
	// 8242BD08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BD0C: 386B2DD0  addi r3, r11, 0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11728;
	// 8242BD10: 4BFF0EF9  bl 0x8241cc08
	ctx.lr = 0x8242BD14;
	sub_8241CC08(ctx, base);
	// 8242BD14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BD18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242BD1C: 419A0010  beq cr6, 0x8242bd2c
	if ctx.cr[6].eq {
	pc = 0x8242BD2C; continue 'dispatch;
	}
	// 8242BD20: 48001219  bl 0x8242cf38
	ctx.lr = 0x8242BD24;
	sub_8242CF38(ctx, base);
	// 8242BD24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242BD28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242BD2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242BD30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242BD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242BD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242BD3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242BD40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242BD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BD48 size=268
    let mut pc: u32 = 0x8242BD48;
    'dispatch: loop {
        match pc {
            0x8242BD48 => {
    //   block [0x8242BD48..0x8242BE54)
	// 8242BD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BD4C: 4810936D  bl 0x825350b8
	ctx.lr = 0x8242BD50;
	sub_82535080(ctx, base);
	// 8242BD50: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BD54: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8242BD58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242BD5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242BD60: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8242BD64: 807C9970  lwz r3, -0x6690(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242BD68: 48001261  bl 0x8242cfc8
	ctx.lr = 0x8242BD6C;
	sub_8242CFC8(ctx, base);
	// 8242BD6C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BD70: 40800010  bge 0x8242bd80
	if !ctx.cr[0].lt {
	pc = 0x8242BD80; continue 'dispatch;
	}
	// 8242BD74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BD78: 386B2D70  addi r3, r11, 0x2d70
	ctx.r[3].s64 = ctx.r[11].s64 + 11632;
	// 8242BD7C: 4BFF0E8D  bl 0x8241cc08
	ctx.lr = 0x8242BD80;
	sub_8241CC08(ctx, base);
	// 8242BD80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242BD84: 409A0018  bne cr6, 0x8242bd9c
	if !ctx.cr[6].eq {
	pc = 0x8242BD9C; continue 'dispatch;
	}
	// 8242BD88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BD8C: 386B2E88  addi r3, r11, 0x2e88
	ctx.r[3].s64 = ctx.r[11].s64 + 11912;
	// 8242BD90: 4BFF0E79  bl 0x8241cc08
	ctx.lr = 0x8242BD94;
	sub_8241CC08(ctx, base);
	// 8242BD94: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242BD98: 48000094  b 0x8242be2c
	pc = 0x8242BE2C; continue 'dispatch;
	// 8242BD9C: 39600130  li r11, 0x130
	ctx.r[11].s64 = 304;
	// 8242BDA0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8242BDA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242BDA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8242BDAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242BDB0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8242BDB4: 4800050D  bl 0x8242c2c0
	ctx.lr = 0x8242BDB8;
	sub_8242C2C0(ctx, base);
	// 8242BDB8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242BDBC: 38800118  li r4, 0x118
	ctx.r[4].s64 = 280;
	// 8242BDC0: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 8242BDC4: 48003DB5  bl 0x8242fb78
	ctx.lr = 0x8242BDC8;
	sub_8242FB78(ctx, base);
	// 8242BDC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BDCC: 40800010  bge 0x8242bddc
	if !ctx.cr[0].lt {
	pc = 0x8242BDDC; continue 'dispatch;
	}
	// 8242BDD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BDD4: 386B2E5C  addi r3, r11, 0x2e5c
	ctx.r[3].s64 = ctx.r[11].s64 + 11868;
	// 8242BDD8: 4BFFFFB8  b 0x8242bd90
	pc = 0x8242BD90; continue 'dispatch;
	// 8242BDDC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8242BDE0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242BDE4: 48003E65  bl 0x8242fc48
	ctx.lr = 0x8242BDE8;
	sub_8242FC48(ctx, base);
	// 8242BDE8: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 8242BDEC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8242BDF0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242BDF4: 388BBAE0  addi r4, r11, -0x4520
	ctx.r[4].s64 = ctx.r[11].s64 + -17696;
	// 8242BDF8: 48003E99  bl 0x8242fc90
	ctx.lr = 0x8242BDFC;
	sub_8242FC90(ctx, base);
	// 8242BDFC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8242BE00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8242BE04: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242BE08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8242BE0C: 4800408D  bl 0x8242fe98
	ctx.lr = 0x8242BE10;
	sub_8242FE98(ctx, base);
	// 8242BE10: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242BE14: 48003E05  bl 0x8242fc18
	ctx.lr = 0x8242BE18;
	sub_8242FC18(ctx, base);
	// 8242BE18: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242BE1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242BE20: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242BE24: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8242BE28: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242BE2C: 807C9970  lwz r3, -0x6690(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242BE30: 48001231  bl 0x8242d060
	ctx.lr = 0x8242BE34;
	sub_8242D060(ctx, base);
	// 8242BE34: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BE38: 40800010  bge 0x8242be48
	if !ctx.cr[0].lt {
	pc = 0x8242BE48; continue 'dispatch;
	}
	// 8242BE3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BE40: 386B2DD0  addi r3, r11, 0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11728;
	// 8242BE44: 4BFF0DC5  bl 0x8241cc08
	ctx.lr = 0x8242BE48;
	sub_8241CC08(ctx, base);
	// 8242BE48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BE4C: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8242BE50: 481092B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BE58 size=208
    let mut pc: u32 = 0x8242BE58;
    'dispatch: loop {
        match pc {
            0x8242BE58 => {
    //   block [0x8242BE58..0x8242BF28)
	// 8242BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BE5C: 4810925D  bl 0x825350b8
	ctx.lr = 0x8242BE60;
	sub_82535080(ctx, base);
	// 8242BE60: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BE64: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8242BE68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242BE6C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242BE70: 807C9970  lwz r3, -0x6690(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242BE74: 48001155  bl 0x8242cfc8
	ctx.lr = 0x8242BE78;
	sub_8242CFC8(ctx, base);
	// 8242BE78: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BE7C: 40800010  bge 0x8242be8c
	if !ctx.cr[0].lt {
	pc = 0x8242BE8C; continue 'dispatch;
	}
	// 8242BE80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BE84: 386B2D70  addi r3, r11, 0x2d70
	ctx.r[3].s64 = ctx.r[11].s64 + 11632;
	// 8242BE88: 4BFF0D81  bl 0x8241cc08
	ctx.lr = 0x8242BE8C;
	sub_8241CC08(ctx, base);
	// 8242BE8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242BE90: 419A0060  beq cr6, 0x8242bef0
	if ctx.cr[6].eq {
	pc = 0x8242BEF0; continue 'dispatch;
	}
	// 8242BE94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242BE98: 419A0058  beq cr6, 0x8242bef0
	if ctx.cr[6].eq {
	pc = 0x8242BEF0; continue 'dispatch;
	}
	// 8242BE9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242BEA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242BEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BEA8: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8242BEAC: 48000415  bl 0x8242c2c0
	ctx.lr = 0x8242BEB0;
	sub_8242C2C0(ctx, base);
	// 8242BEB0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242BEB4: 83EB996C  lwz r31, -0x6694(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242BEB8: 4800002C  b 0x8242bee4
	pc = 0x8242BEE4; continue 'dispatch;
	// 8242BEBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242BEC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242BEC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BEC8: 4BFFFC31  bl 0x8242baf8
	ctx.lr = 0x8242BECC;
	sub_8242BAF8(ctx, base);
	// 8242BECC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8242BED0: 41800030  blt 0x8242bf00
	if ctx.cr[0].lt {
	pc = 0x8242BF00; continue 'dispatch;
	}
	// 8242BED4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242BED8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242BEDC: 409A0024  bne cr6, 0x8242bf00
	if !ctx.cr[6].eq {
	pc = 0x8242BF00; continue 'dispatch;
	}
	// 8242BEE0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242BEE4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242BEE8: 4082FFD4  bne 0x8242bebc
	if !ctx.cr[0].eq {
	pc = 0x8242BEBC; continue 'dispatch;
	}
	// 8242BEEC: 48000014  b 0x8242bf00
	pc = 0x8242BF00; continue 'dispatch;
	// 8242BEF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BEF4: 386B2EB0  addi r3, r11, 0x2eb0
	ctx.r[3].s64 = ctx.r[11].s64 + 11952;
	// 8242BEF8: 4BFF0D11  bl 0x8241cc08
	ctx.lr = 0x8242BEFC;
	sub_8241CC08(ctx, base);
	// 8242BEFC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242BF00: 807C9970  lwz r3, -0x6690(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242BF04: 4800115D  bl 0x8242d060
	ctx.lr = 0x8242BF08;
	sub_8242D060(ctx, base);
	// 8242BF08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BF0C: 40800010  bge 0x8242bf1c
	if !ctx.cr[0].lt {
	pc = 0x8242BF1C; continue 'dispatch;
	}
	// 8242BF10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BF14: 386B2DD0  addi r3, r11, 0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11728;
	// 8242BF18: 4BFF0CF1  bl 0x8241cc08
	ctx.lr = 0x8242BF1C;
	sub_8241CC08(ctx, base);
	// 8242BF1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242BF20: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8242BF24: 481091E4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242BF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242BF28 size=516
    let mut pc: u32 = 0x8242BF28;
    'dispatch: loop {
        match pc {
            0x8242BF28 => {
    //   block [0x8242BF28..0x8242C12C)
	// 8242BF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242BF2C: 4810917D  bl 0x825350a8
	ctx.lr = 0x8242BF30;
	sub_82535080(ctx, base);
	// 8242BF30: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242BF34: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8242BF38: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8242BF3C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8242BF40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8242BF44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8242BF48: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8242BF4C: 409A0018  bne cr6, 0x8242bf64
	if !ctx.cr[6].eq {
	pc = 0x8242BF64; continue 'dispatch;
	}
	// 8242BF50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BF54: 386B2F58  addi r3, r11, 0x2f58
	ctx.r[3].s64 = ctx.r[11].s64 + 12120;
	// 8242BF58: 4BFF0CB1  bl 0x8241cc08
	ctx.lr = 0x8242BF5C;
	sub_8241CC08(ctx, base);
	// 8242BF5C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242BF60: 480001C4  b 0x8242c124
	pc = 0x8242C124; continue 'dispatch;
	// 8242BF64: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8242BF68: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242BF6C: 38800118  li r4, 0x118
	ctx.r[4].s64 = 280;
	// 8242BF70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8242BF74: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8242BF78: 48003C01  bl 0x8242fb78
	ctx.lr = 0x8242BF7C;
	sub_8242FB78(ctx, base);
	// 8242BF7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242BF80: 40800010  bge 0x8242bf90
	if !ctx.cr[0].lt {
	pc = 0x8242BF90; continue 'dispatch;
	}
	// 8242BF84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BF88: 386B2F2C  addi r3, r11, 0x2f2c
	ctx.r[3].s64 = ctx.r[11].s64 + 12076;
	// 8242BF8C: 4BFFFFCC  b 0x8242bf58
	pc = 0x8242BF58; continue 'dispatch;
	// 8242BF90: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8242BF94: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242BF98: 48003CB1  bl 0x8242fc48
	ctx.lr = 0x8242BF9C;
	sub_8242FC48(ctx, base);
	// 8242BF9C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8242BFA0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242BFA4: 409A005C  bne cr6, 0x8242c000
	if !ctx.cr[6].eq {
	pc = 0x8242C000; continue 'dispatch;
	}
	// 8242BFA8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8242BFAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242BFB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8242BFB4: 4BFFFD95  bl 0x8242bd48
	ctx.lr = 0x8242BFB8;
	sub_8242BD48(ctx, base);
	// 8242BFB8: 4BFA77A1  bl 0x823d3758
	ctx.lr = 0x8242BFBC;
	sub_823D3758(ctx, base);
	// 8242BFBC: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242BFC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242BFC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242BFC8: 4BFA6309  bl 0x823d22d0
	ctx.lr = 0x8242BFCC;
	sub_823D22D0(ctx, base);
	// 8242BFCC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8242BFD0: 40820010  bne 0x8242bfe0
	if !ctx.cr[0].eq {
	pc = 0x8242BFE0; continue 'dispatch;
	}
	// 8242BFD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242BFD8: 386B2F04  addi r3, r11, 0x2f04
	ctx.r[3].s64 = ctx.r[11].s64 + 12036;
	// 8242BFDC: 4BFFFF7C  b 0x8242bf58
	pc = 0x8242BF58; continue 'dispatch;
	// 8242BFE0: 397E0003  addi r11, r30, 3
	ctx.r[11].s64 = ctx.r[30].s64 + 3;
	// 8242BFE4: 38A00130  li r5, 0x130
	ctx.r[5].s64 = 304;
	// 8242BFE8: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8242BFEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242BFF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242BFF4: 481091DD  bl 0x825351d0
	ctx.lr = 0x8242BFF8;
	sub_825351D0(ctx, base);
	// 8242BFF8: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 8242BFFC: 48000034  b 0x8242c030
	pc = 0x8242C030; continue 'dispatch;
	// 8242C000: 2B1D0134  cmplwi cr6, r29, 0x134
	ctx.cr[6].compare_u32(ctx.r[29].u32, 308 as u32, &mut ctx.xer);
	// 8242C004: 40980010  bge cr6, 0x8242c014
	if !ctx.cr[6].lt {
	pc = 0x8242C014; continue 'dispatch;
	}
	// 8242C008: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C00C: 386B2ED8  addi r3, r11, 0x2ed8
	ctx.r[3].s64 = ctx.r[11].s64 + 11992;
	// 8242C010: 4BFFFF48  b 0x8242bf58
	pc = 0x8242BF58; continue 'dispatch;
	// 8242C014: 397E0003  addi r11, r30, 3
	ctx.r[11].s64 = ctx.r[30].s64 + 3;
	// 8242C018: 38A00130  li r5, 0x130
	ctx.r[5].s64 = 304;
	// 8242C01C: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8242C020: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242C028: 481091A9  bl 0x825351d0
	ctx.lr = 0x8242C02C;
	sub_825351D0(ctx, base);
	// 8242C02C: 933F000C  stw r25, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 8242C030: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8242C034: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8242C038: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8242C03C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8242C040: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8242C044: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 8242C048: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242C04C: 48000275  bl 0x8242c2c0
	ctx.lr = 0x8242C050;
	sub_8242C2C0(ctx, base);
	// 8242C050: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C054: 4811B02D  bl 0x82547080
	ctx.lr = 0x8242C058;
	sub_82547080(ctx, base);
	// 8242C058: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8242C05C: 894B001F  lbz r10, 0x1f(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(31 as u32) ) } as u64;
	// 8242C060: 2B0A005C  cmplwi cr6, r10, 0x5c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 92 as u32, &mut ctx.xer);
	// 8242C064: 409A000C  bne cr6, 0x8242c070
	if !ctx.cr[6].eq {
	pc = 0x8242C070; continue 'dispatch;
	}
	// 8242C068: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8242C06C: 9B2B001F  stb r25, 0x1f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(31 as u32), ctx.r[25].u8 ) };
	// 8242C070: 39400130  li r10, 0x130
	ctx.r[10].s64 = 304;
	// 8242C074: 907F0124  stw r3, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[3].u32 ) };
	// 8242C078: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 8242C07C: 939F0128  stw r28, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[28].u32 ) };
	// 8242C080: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8242C084: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 8242C088: 388BB950  addi r4, r11, -0x46b0
	ctx.r[4].s64 = ctx.r[11].s64 + -18096;
	// 8242C08C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8242C090: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C094: 48003BFD  bl 0x8242fc90
	ctx.lr = 0x8242C098;
	sub_8242FC90(ctx, base);
	// 8242C098: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8242C09C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C0A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242C0A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242C0A8: 48003DF1  bl 0x8242fe98
	ctx.lr = 0x8242C0AC;
	sub_8242FE98(ctx, base);
	// 8242C0AC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C0B0: 48003B69  bl 0x8242fc18
	ctx.lr = 0x8242C0B4;
	sub_8242FC18(ctx, base);
	// 8242C0B4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8242C0B8: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242C0BC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8242C0C0: 807E9970  lwz r3, -0x6690(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242C0C4: 48000F05  bl 0x8242cfc8
	ctx.lr = 0x8242C0C8;
	sub_8242CFC8(ctx, base);
	// 8242C0C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242C0CC: 40800010  bge 0x8242c0dc
	if !ctx.cr[0].lt {
	pc = 0x8242C0DC; continue 'dispatch;
	}
	// 8242C0D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C0D4: 386B2D70  addi r3, r11, 0x2d70
	ctx.r[3].s64 = ctx.r[11].s64 + 11632;
	// 8242C0D8: 4BFF0B31  bl 0x8241cc08
	ctx.lr = 0x8242C0DC;
	sub_8241CC08(ctx, base);
	// 8242C0DC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C0E0: 814B996C  lwz r10, -0x6694(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242C0E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242C0E8: 419A0014  beq cr6, 0x8242c0fc
	if ctx.cr[6].eq {
	pc = 0x8242C0FC; continue 'dispatch;
	}
	// 8242C0EC: 814B996C  lwz r10, -0x6694(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242C0F0: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8242C0F4: 814B996C  lwz r10, -0x6694(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26260 as u32) ) } as u64;
	// 8242C0F8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242C0FC: 807E9970  lwz r3, -0x6690(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-26256 as u32) ) } as u64;
	// 8242C100: 93EB996C  stw r31, -0x6694(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26260 as u32), ctx.r[31].u32 ) };
	// 8242C104: 48000F5D  bl 0x8242d060
	ctx.lr = 0x8242C108;
	sub_8242D060(ctx, base);
	// 8242C108: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242C10C: 40800010  bge 0x8242c11c
	if !ctx.cr[0].lt {
	pc = 0x8242C11C; continue 'dispatch;
	}
	// 8242C110: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C114: 386B2DD0  addi r3, r11, 0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11728;
	// 8242C118: 4BFF0AF1  bl 0x8241cc08
	ctx.lr = 0x8242C11C;
	sub_8241CC08(ctx, base);
	// 8242C11C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C120: 93F80000  stw r31, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8242C124: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 8242C128: 48108FD0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C130 size=120
    let mut pc: u32 = 0x8242C130;
    'dispatch: loop {
        match pc {
            0x8242C130 => {
    //   block [0x8242C130..0x8242C1A8)
	// 8242C130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242C13C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C144: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242C148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242C14C: 419A0040  beq cr6, 0x8242c18c
	if ctx.cr[6].eq {
	pc = 0x8242C18C; continue 'dispatch;
	}
	// 8242C150: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242C154: 419A0038  beq cr6, 0x8242c18c
	if ctx.cr[6].eq {
	pc = 0x8242C18C; continue 'dispatch;
	}
	// 8242C158: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242C15C: 4811AF4D  bl 0x825470a8
	ctx.lr = 0x8242C160;
	sub_825470A8(ctx, base);
	// 8242C160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C164: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8242C168: 409A0010  bne cr6, 0x8242c178
	if !ctx.cr[6].eq {
	pc = 0x8242C178; continue 'dispatch;
	}
	// 8242C16C: 4BF94515  bl 0x823c0680
	ctx.lr = 0x8242C170;
	sub_823C0680(ctx, base);
	// 8242C170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C174: 40820018  bne 0x8242c18c
	if !ctx.cr[0].eq {
	pc = 0x8242C18C; continue 'dispatch;
	}
	// 8242C178: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C17C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242C180: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8242C184: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242C188: 48000008  b 0x8242c190
	pc = 0x8242C190; continue 'dispatch;
	// 8242C18C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C190: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C19C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242C1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C1A8 size=136
    let mut pc: u32 = 0x8242C1A8;
    'dispatch: loop {
        match pc {
            0x8242C1A8 => {
    //   block [0x8242C1A8..0x8242C230)
	// 8242C1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242C1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C1BC: F8810088  std r4, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u64 ) };
	// 8242C1C0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8242C1C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242C1C8: 409A000C  bne cr6, 0x8242c1d4
	if !ctx.cr[6].eq {
	pc = 0x8242C1D4; continue 'dispatch;
	}
	// 8242C1CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C1D0: 48000048  b 0x8242c218
	pc = 0x8242C218; continue 'dispatch;
	// 8242C1D4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8242C1D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242C1DC: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8242C1E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8242C1E4: 4811A185  bl 0x82546368
	ctx.lr = 0x8242C1E8;
	sub_82546368(ctx, base);
	// 8242C1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C1EC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8242C1F0: 409A0010  bne cr6, 0x8242c200
	if !ctx.cr[6].eq {
	pc = 0x8242C200; continue 'dispatch;
	}
	// 8242C1F4: 4BF9448D  bl 0x823c0680
	ctx.lr = 0x8242C1F8;
	sub_823C0680(ctx, base);
	// 8242C1F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C1FC: 4082FFD0  bne 0x8242c1cc
	if !ctx.cr[0].eq {
	pc = 0x8242C1CC; continue 'dispatch;
	}
	// 8242C200: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242C204: 419A0010  beq cr6, 0x8242c214
	if ctx.cr[6].eq {
	pc = 0x8242C214; continue 'dispatch;
	}
	// 8242C208: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C20C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8242C210: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242C214: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242C218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C224: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242C228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C230 size=52
    let mut pc: u32 = 0x8242C230;
    'dispatch: loop {
        match pc {
            0x8242C230 => {
    //   block [0x8242C230..0x8242C264)
	// 8242C230: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C234: 386B9978  addi r3, r11, -0x6688
	ctx.r[3].s64 = ctx.r[11].s64 + -26248;
	// 8242C238: 39030108  addi r8, r3, 0x108
	ctx.r[8].s64 = ctx.r[3].s64 + 264;
	// 8242C23C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242C240: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242C244: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242C248: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242C24C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242C250: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242C254: 4082FFE8  bne 0x8242c23c
	if !ctx.cr[0].eq {
	pc = 0x8242C23C; continue 'dispatch;
	}
	// 8242C258: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242C25C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242C260: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C264 size=12
    let mut pc: u32 = 0x8242C264;
    'dispatch: loop {
        match pc {
            0x8242C264 => {
    //   block [0x8242C264..0x8242C270)
	// 8242C264: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8242C268: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C26C: 48108F64  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C270 size=4
    let mut pc: u32 = 0x8242C270;
    'dispatch: loop {
        match pc {
            0x8242C270 => {
    //   block [0x8242C270..0x8242C274)
	// 8242C270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C278 size=52
    let mut pc: u32 = 0x8242C278;
    'dispatch: loop {
        match pc {
            0x8242C278 => {
    //   block [0x8242C278..0x8242C2AC)
	// 8242C278: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C27C: 386B9978  addi r3, r11, -0x6688
	ctx.r[3].s64 = ctx.r[11].s64 + -26248;
	// 8242C280: 39030108  addi r8, r3, 0x108
	ctx.r[8].s64 = ctx.r[3].s64 + 264;
	// 8242C284: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242C288: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242C28C: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242C290: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242C294: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242C298: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242C29C: 4082FFE8  bne 0x8242c284
	if !ctx.cr[0].eq {
	pc = 0x8242C284; continue 'dispatch;
	}
	// 8242C2A0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242C2A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C2A8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C2AC size=12
    let mut pc: u32 = 0x8242C2AC;
    'dispatch: loop {
        match pc {
            0x8242C2AC => {
    //   block [0x8242C2AC..0x8242C2B8)
	// 8242C2AC: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8242C2B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C2B4: 48108F1C  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242C2B8 size=4
    let mut pc: u32 = 0x8242C2B8;
    'dispatch: loop {
        match pc {
            0x8242C2B8 => {
    //   block [0x8242C2B8..0x8242C2BC)
	// 8242C2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C2C0 size=392
    let mut pc: u32 = 0x8242C2C0;
    'dispatch: loop {
        match pc {
            0x8242C2C0 => {
    //   block [0x8242C2C0..0x8242C448)
	// 8242C2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C2C4: 48108DF9  bl 0x825350bc
	ctx.lr = 0x8242C2C8;
	sub_82535080(ctx, base);
	// 8242C2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C2CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242C2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242C2D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242C2D8: 409A001C  bne cr6, 0x8242c2f4
	if !ctx.cr[6].eq {
	pc = 0x8242C2F4; continue 'dispatch;
	}
	// 8242C2DC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C2E0: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C2E4: 38AB9978  addi r5, r11, -0x6688
	ctx.r[5].s64 = ctx.r[11].s64 + -26248;
	// 8242C2E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C2EC: 4BFF723D  bl 0x82423528
	ctx.lr = 0x8242C2F0;
	sub_82423528(ctx, base);
	// 8242C2F0: 48000150  b 0x8242c440
	pc = 0x8242C440; continue 'dispatch;
	// 8242C2F4: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C2F8: 7D690775  extsb. r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242C2FC: 4182FFE0  beq 0x8242c2dc
	if ctx.cr[0].eq {
	pc = 0x8242C2DC; continue 'dispatch;
	}
	// 8242C300: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242C304: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242C308: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C30C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C310: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8242C314: 409AFFF4  bne cr6, 0x8242c308
	if !ctx.cr[6].eq {
	pc = 0x8242C308; continue 'dispatch;
	}
	// 8242C318: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242C31C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242C320: 556B003F  rotlwi. r11, r11, 0
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C324: 40810018  ble 0x8242c33c
	if !ctx.cr[0].gt {
	pc = 0x8242C33C; continue 'dispatch;
	}
	// 8242C328: 7D4BE8AE  lbzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8242C32C: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 8242C330: 419A000C  beq cr6, 0x8242c33c
	if ctx.cr[6].eq {
	pc = 0x8242C33C; continue 'dispatch;
	}
	// 8242C334: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C338: 4181FFF0  bgt 0x8242c328
	if ctx.cr[0].gt {
	pc = 0x8242C328; continue 'dispatch;
	}
	// 8242C33C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C340: 409A0070  bne cr6, 0x8242c3b0
	if !ctx.cr[6].eq {
	pc = 0x8242C3B0; continue 'dispatch;
	}
	// 8242C344: 2F09005C  cmpwi cr6, r9, 0x5c
	ctx.cr[6].compare_i32(ctx.r[9].s32, 92, &mut ctx.xer);
	// 8242C348: 419A0024  beq cr6, 0x8242c36c
	if ctx.cr[6].eq {
	pc = 0x8242C36C; continue 'dispatch;
	}
	// 8242C34C: 2F09002F  cmpwi cr6, r9, 0x2f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 47, &mut ctx.xer);
	// 8242C350: 419A001C  beq cr6, 0x8242c36c
	if ctx.cr[6].eq {
	pc = 0x8242C36C; continue 'dispatch;
	}
	// 8242C354: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C358: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C35C: 38AB9978  addi r5, r11, -0x6688
	ctx.r[5].s64 = ctx.r[11].s64 + -26248;
	// 8242C360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C364: 4BFF71C5  bl 0x82423528
	ctx.lr = 0x8242C368;
	sub_82423528(ctx, base);
	// 8242C368: 48000034  b 0x8242c39c
	pc = 0x8242C39C; continue 'dispatch;
	// 8242C36C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C370: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C374: 3BEB9978  addi r31, r11, -0x6688
	ctx.r[31].s64 = ctx.r[11].s64 + -26248;
	// 8242C378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C37C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8242C380: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8242C384: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 8242C388: 4BFF71D1  bl 0x82423558
	ctx.lr = 0x8242C38C;
	sub_82423558(ctx, base);
	// 8242C38C: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8242C390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242C394: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8242C398: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8242C39C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C3A0: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C3A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242C3A8: 4BFF7211  bl 0x824235b8
	ctx.lr = 0x8242C3AC;
	sub_824235B8(ctx, base);
	// 8242C3AC: 48000048  b 0x8242c3f4
	pc = 0x8242C3F4; continue 'dispatch;
	// 8242C3B0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8242C3B4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8242C3B8: 386B018C  addi r3, r11, 0x18c
	ctx.r[3].s64 = ctx.r[11].s64 + 396;
	// 8242C3BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242C3C0: 4810B901  bl 0x82537cc0
	ctx.lr = 0x8242C3C4;
	sub_82537CC0(ctx, base);
	// 8242C3C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242C3C8: 4082001C  bne 0x8242c3e4
	if !ctx.cr[0].eq {
	pc = 0x8242C3E4; continue 'dispatch;
	}
	// 8242C3CC: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 8242C3D0: 38BD0004  addi r5, r29, 4
	ctx.r[5].s64 = ctx.r[29].s64 + 4;
	// 8242C3D4: 38800103  li r4, 0x103
	ctx.r[4].s64 = 259;
	// 8242C3D8: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 8242C3DC: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8242C3E0: 48000010  b 0x8242c3f0
	pc = 0x8242C3F0; continue 'dispatch;
	// 8242C3E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242C3E8: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C3EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C3F0: 4BFF7139  bl 0x82423528
	ctx.lr = 0x8242C3F4;
	sub_82423528(ctx, base);
	// 8242C3F4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8242C3F8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242C3FC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C400: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C404: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242C408: 409AFFF4  bne cr6, 0x8242c3fc
	if !ctx.cr[6].eq {
	pc = 0x8242C3FC; continue 'dispatch;
	}
	// 8242C40C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242C410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242C414: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242C418: 554A003F  rotlwi. r10, r10, 0
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242C41C: 41800024  blt 0x8242c440
	if ctx.cr[0].lt {
	pc = 0x8242C440; continue 'dispatch;
	}
	// 8242C420: 7D2BF0AE  lbzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8242C424: 2B09002F  cmplwi cr6, r9, 0x2f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 47 as u32, &mut ctx.xer);
	// 8242C428: 409A000C  bne cr6, 0x8242c434
	if !ctx.cr[6].eq {
	pc = 0x8242C434; continue 'dispatch;
	}
	// 8242C42C: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	// 8242C430: 7D2BF1AE  stbx r9, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u8) };
	// 8242C434: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C438: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242C43C: 4099FFE4  ble cr6, 0x8242c420
	if !ctx.cr[6].gt {
	pc = 0x8242C420; continue 'dispatch;
	}
	// 8242C440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C444: 48108CC8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C448 size=100
    let mut pc: u32 = 0x8242C448;
    'dispatch: loop {
        match pc {
            0x8242C448 => {
    //   block [0x8242C448..0x8242C4AC)
	// 8242C448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C45C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C460: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8242C464: 386B2F80  addi r3, r11, 0x2f80
	ctx.r[3].s64 = ctx.r[11].s64 + 12160;
	// 8242C468: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8242C46C: 4810B855  bl 0x82537cc0
	ctx.lr = 0x8242C470;
	sub_82537CC0(ctx, base);
	// 8242C470: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242C474: 41820024  beq 0x8242c498
	if ctx.cr[0].eq {
	pc = 0x8242C498; continue 'dispatch;
	}
	// 8242C478: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8242C47C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8242C480: 386B018C  addi r3, r11, 0x18c
	ctx.r[3].s64 = ctx.r[11].s64 + 396;
	// 8242C484: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8242C488: 4810B839  bl 0x82537cc0
	ctx.lr = 0x8242C48C;
	sub_82537CC0(ctx, base);
	// 8242C48C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8242C490: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8242C494: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8242C498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242C49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C4A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C4B0 size=372
    let mut pc: u32 = 0x8242C4B0;
    'dispatch: loop {
        match pc {
            0x8242C4B0 => {
    //   block [0x8242C4B0..0x8242C624)
	// 8242C4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C4B4: 48108C05  bl 0x825350b8
	ctx.lr = 0x8242C4B8;
	sub_82535080(ctx, base);
	// 8242C4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C4BC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C4C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242C4C4: 3BCB9978  addi r30, r11, -0x6688
	ctx.r[30].s64 = ctx.r[11].s64 + -26248;
	// 8242C4C8: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8242C4CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C4D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C4D4: 48108CFD  bl 0x825351d0
	ctx.lr = 0x8242C4D8;
	sub_825351D0(ctx, base);
	// 8242C4D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242C4DC: 409A0020  bne cr6, 0x8242c4fc
	if !ctx.cr[6].eq {
	pc = 0x8242C4FC; continue 'dispatch;
	}
	// 8242C4E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C4E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C4E8: 388B2FDC  addi r4, r11, 0x2fdc
	ctx.r[4].s64 = ctx.r[11].s64 + 12252;
	// 8242C4EC: 4811AABD  bl 0x82546fa8
	ctx.lr = 0x8242C4F0;
	sub_82546FA8(ctx, base);
	// 8242C4F0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242C4F4: 917E0104  stw r11, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 8242C4F8: 48000124  b 0x8242c61c
	pc = 0x8242C61C; continue 'dispatch;
	// 8242C4FC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242C500: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242C504: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C508: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C50C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242C510: 409AFFF4  bne cr6, 0x8242c504
	if !ctx.cr[6].eq {
	pc = 0x8242C504; continue 'dispatch;
	}
	// 8242C514: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242C518: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242C51C: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8242C520: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8242C524: 40980014  bge cr6, 0x8242c538
	if !ctx.cr[6].lt {
	pc = 0x8242C538; continue 'dispatch;
	}
	// 8242C528: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C52C: 386B2FB8  addi r3, r11, 0x2fb8
	ctx.r[3].s64 = ctx.r[11].s64 + 12216;
	// 8242C530: 4BFF06D9  bl 0x8241cc08
	ctx.lr = 0x8242C534;
	sub_8241CC08(ctx, base);
	// 8242C534: 480000E8  b 0x8242c61c
	pc = 0x8242C61C; continue 'dispatch;
	// 8242C538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242C53C: 3B80005C  li r28, 0x5c
	ctx.r[28].s64 = 92;
	// 8242C540: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8242C544: 41980044  blt cr6, 0x8242c588
	if ctx.cr[6].lt {
	pc = 0x8242C588; continue 'dispatch;
	}
	// 8242C548: 7D4BE8AE  lbzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8242C54C: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 8242C550: 409A0008  bne cr6, 0x8242c558
	if !ctx.cr[6].eq {
	pc = 0x8242C558; continue 'dispatch;
	}
	// 8242C554: 7F8BE9AE  stbx r28, r11, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u8) };
	// 8242C558: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C55C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8242C560: 4099FFE8  ble cr6, 0x8242c548
	if !ctx.cr[6].gt {
	pc = 0x8242C548; continue 'dispatch;
	}
	// 8242C564: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8242C568: 41980020  blt cr6, 0x8242c588
	if ctx.cr[6].lt {
	pc = 0x8242C588; continue 'dispatch;
	}
	// 8242C56C: 7D7FE8AE  lbzx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8242C570: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 8242C574: 419A000C  beq cr6, 0x8242c580
	if ctx.cr[6].eq {
	pc = 0x8242C580; continue 'dispatch;
	}
	// 8242C578: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8242C57C: 4080FFF0  bge 0x8242c56c
	if !ctx.cr[0].lt {
	pc = 0x8242C56C; continue 'dispatch;
	}
	// 8242C580: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8242C584: 41990010  bgt cr6, 0x8242c594
	if ctx.cr[6].gt {
	pc = 0x8242C594; continue 'dispatch;
	}
	// 8242C588: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C58C: 386B2F84  addi r3, r11, 0x2f84
	ctx.r[3].s64 = ctx.r[11].s64 + 12164;
	// 8242C590: 4BFFFFA0  b 0x8242c530
	pc = 0x8242C530; continue 'dispatch;
	// 8242C594: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242C598: 4BFFFEB1  bl 0x8242c448
	ctx.lr = 0x8242C59C;
	sub_8242C448(ctx, base);
	// 8242C59C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242C5A0: 40820024  bne 0x8242c5c4
	if !ctx.cr[0].eq {
	pc = 0x8242C5C4; continue 'dispatch;
	}
	// 8242C5A4: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 8242C5A8: 7CBFEA14  add r5, r31, r29
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8242C5AC: 38800103  li r4, 0x103
	ctx.r[4].s64 = 259;
	// 8242C5B0: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 8242C5B4: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8242C5B8: 4BFF6F71  bl 0x82423528
	ctx.lr = 0x8242C5BC;
	sub_82423528(ctx, base);
	// 8242C5BC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8242C5C0: 48000014  b 0x8242c5d4
	pc = 0x8242C5D4; continue 'dispatch;
	// 8242C5C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242C5C8: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242C5CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242C5D0: 4BFF6F59  bl 0x82423528
	ctx.lr = 0x8242C5D4;
	sub_82423528(ctx, base);
	// 8242C5D4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8242C5D8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242C5DC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C5E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C5E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242C5E8: 409AFFF4  bne cr6, 0x8242c5dc
	if !ctx.cr[6].eq {
	pc = 0x8242C5DC; continue 'dispatch;
	}
	// 8242C5EC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242C5F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242C5F4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242C5F8: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8242C5FC: 892AFFFF  lbz r9, -1(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8242C600: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 8242C604: 419A0014  beq cr6, 0x8242c618
	if ctx.cr[6].eq {
	pc = 0x8242C618; continue 'dispatch;
	}
	// 8242C608: 9B8A0000  stb r28, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8242C60C: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 8242C610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242C614: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 8242C618: 93FE0104  stw r31, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8242C61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242C620: 48108AE8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C628 size=96
    let mut pc: u32 = 0x8242C628;
    'dispatch: loop {
        match pc {
            0x8242C628 => {
    //   block [0x8242C628..0x8242C688)
	// 8242C628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C638: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C63C: 394B2FE0  addi r10, r11, 0x2fe0
	ctx.r[10].s64 = ctx.r[11].s64 + 12256;
	// 8242C640: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C644: 3BEB9A84  addi r31, r11, -0x657c
	ctx.r[31].s64 = ctx.r[11].s64 + -25980;
	// 8242C648: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 8242C64C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242C650: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242C654: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242C658: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242C65C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8242C660: 409A0014  bne cr6, 0x8242c674
	if !ctx.cr[6].eq {
	pc = 0x8242C674; continue 'dispatch;
	}
	// 8242C664: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8242C668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242C66C: 480007ED  bl 0x8242ce58
	ctx.lr = 0x8242C670;
	sub_8242CE58(ctx, base);
	// 8242C670: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8242C674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242C678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C688 size=84
    let mut pc: u32 = 0x8242C688;
    'dispatch: loop {
        match pc {
            0x8242C688 => {
    //   block [0x8242C688..0x8242C6DC)
	// 8242C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C698: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242C69C: 3BEB9AA8  addi r31, r11, -0x6558
	ctx.r[31].s64 = ctx.r[11].s64 + -25944;
	// 8242C6A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242C6A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C6A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242C6AC: 4082001C  bne 0x8242c6c8
	if !ctx.cr[0].eq {
	pc = 0x8242C6C8; continue 'dispatch;
	}
	// 8242C6B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C6B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242C6B8: 419A0010  beq cr6, 0x8242c6c8
	if ctx.cr[6].eq {
	pc = 0x8242C6C8; continue 'dispatch;
	}
	// 8242C6BC: 4800087D  bl 0x8242cf38
	ctx.lr = 0x8242C6C0;
	sub_8242CF38(ctx, base);
	// 8242C6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242C6C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242C6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242C6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C6D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C6E0 size=200
    let mut pc: u32 = 0x8242C6E0;
    'dispatch: loop {
        match pc {
            0x8242C6E0 => {
    //   block [0x8242C6E0..0x8242C7A8)
	// 8242C6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C6E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242C6EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C6F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C6F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C6F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242C6FC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242C700: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8242C704: 48000068  b 0x8242c76c
	pc = 0x8242C76C; continue 'dispatch;
	// 8242C708: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C70C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242C710: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8242C714: 419A001C  beq cr6, 0x8242c730
	if ctx.cr[6].eq {
	pc = 0x8242C730; continue 'dispatch;
	}
	// 8242C718: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242C71C: 4BF954AD  bl 0x823c1bc8
	ctx.lr = 0x8242C720;
	sub_823C1BC8(ctx, base);
	// 8242C720: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8242C724: 419A000C  beq cr6, 0x8242c730
	if ctx.cr[6].eq {
	pc = 0x8242C730; continue 'dispatch;
	}
	// 8242C728: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242C72C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8242C730: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242C734: 48000895  bl 0x8242cfc8
	ctx.lr = 0x8242C738;
	sub_8242CFC8(ctx, base);
	// 8242C738: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242C73C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242C740: 409A0024  bne cr6, 0x8242c764
	if !ctx.cr[6].eq {
	pc = 0x8242C764; continue 'dispatch;
	}
	// 8242C744: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8242C748: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C74C: 41820010  beq 0x8242c75c
	if ctx.cr[0].eq {
	pc = 0x8242C75C; continue 'dispatch;
	}
	// 8242C750: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8242C754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242C758: 4E800421  bctrl
	ctx.lr = 0x8242C75C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242C75C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242C760: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242C764: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242C768: 480008F9  bl 0x8242d060
	ctx.lr = 0x8242C76C;
	sub_8242D060(ctx, base);
	// 8242C76C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242C770: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242C774: 4BF94DA5  bl 0x823c1518
	ctx.lr = 0x8242C778;
	sub_823C1518(ctx, base);
	// 8242C778: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242C77C: 4BF94CC5  bl 0x823c1440
	ctx.lr = 0x8242C780;
	sub_823C1440(ctx, base);
	// 8242C780: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242C784: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C788: 419AFF80  beq cr6, 0x8242c708
	if ctx.cr[6].eq {
	pc = 0x8242C708; continue 'dispatch;
	}
	// 8242C78C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C790: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C79C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242C7A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C7A8 size=336
    let mut pc: u32 = 0x8242C7A8;
    'dispatch: loop {
        match pc {
            0x8242C7A8 => {
    //   block [0x8242C7A8..0x8242C8F8)
	// 8242C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C7B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C7B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C7B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242C7BC: 409A0018  bne cr6, 0x8242c7d4
	if !ctx.cr[6].eq {
	pc = 0x8242C7D4; continue 'dispatch;
	}
	// 8242C7C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C7C4: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242C7C8: 4BFF0441  bl 0x8241cc08
	ctx.lr = 0x8242C7CC;
	sub_8241CC08(ctx, base);
	// 8242C7CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C7D0: 48000114  b 0x8242c8e4
	pc = 0x8242C8E4; continue 'dispatch;
	// 8242C7D4: 2B040058  cmplwi cr6, r4, 0x58
	ctx.cr[6].compare_u32(ctx.r[4].u32, 88 as u32, &mut ctx.xer);
	// 8242C7D8: 40980010  bge cr6, 0x8242c7e8
	if !ctx.cr[6].lt {
	pc = 0x8242C7E8; continue 'dispatch;
	}
	// 8242C7DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C7E0: 386B30E8  addi r3, r11, 0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + 12520;
	// 8242C7E4: 4BFFFFE4  b 0x8242c7c8
	pc = 0x8242C7C8; continue 'dispatch;
	// 8242C7E8: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8242C7EC: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8242C7F0: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8242C7F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C7F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242C7FC: 481089D5  bl 0x825351d0
	ctx.lr = 0x8242C800;
	sub_825351D0(ctx, base);
	// 8242C800: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8242C804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8242C808: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242C80C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C810: 4BF94C69  bl 0x823c1478
	ctx.lr = 0x8242C814;
	sub_823C1478(ctx, base);
	// 8242C814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C818: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8242C81C: 40820010  bne 0x8242c82c
	if !ctx.cr[0].eq {
	pc = 0x8242C82C; continue 'dispatch;
	}
	// 8242C820: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C824: 386B30BC  addi r3, r11, 0x30bc
	ctx.r[3].s64 = ctx.r[11].s64 + 12476;
	// 8242C828: 4BFFFFA0  b 0x8242c7c8
	pc = 0x8242C7C8; continue 'dispatch;
	// 8242C82C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8242C830: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8242C834: 48000625  bl 0x8242ce58
	ctx.lr = 0x8242C838;
	sub_8242CE58(ctx, base);
	// 8242C838: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C83C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8242C840: 4082001C  bne 0x8242c85c
	if !ctx.cr[0].eq {
	pc = 0x8242C85C; continue 'dispatch;
	}
	// 8242C844: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C848: 386B308C  addi r3, r11, 0x308c
	ctx.r[3].s64 = ctx.r[11].s64 + 12428;
	// 8242C84C: 4BFF03BD  bl 0x8241cc08
	ctx.lr = 0x8242C850;
	sub_8241CC08(ctx, base);
	// 8242C850: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242C854: 4BF94105  bl 0x823c0958
	ctx.lr = 0x8242C858;
	sub_823C0958(ctx, base);
	// 8242C858: 4BFFFF74  b 0x8242c7cc
	pc = 0x8242C7CC; continue 'dispatch;
	// 8242C85C: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 8242C860: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8242C864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8242C868: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8242C86C: 38ABC6E0  addi r5, r11, -0x3920
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	// 8242C870: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242C874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242C878: 4BF95559  bl 0x823c1dd0
	ctx.lr = 0x8242C87C;
	sub_823C1DD0(ctx, base);
	// 8242C87C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242C880: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8242C884: 4082002C  bne 0x8242c8b0
	if !ctx.cr[0].eq {
	pc = 0x8242C8B0; continue 'dispatch;
	}
	// 8242C888: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C88C: 386B3064  addi r3, r11, 0x3064
	ctx.r[3].s64 = ctx.r[11].s64 + 12388;
	// 8242C890: 4BFF0379  bl 0x8241cc08
	ctx.lr = 0x8242C894;
	sub_8241CC08(ctx, base);
	// 8242C894: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242C898: 4BF940C1  bl 0x823c0958
	ctx.lr = 0x8242C89C;
	sub_823C0958(ctx, base);
	// 8242C89C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242C8A0: 48000699  bl 0x8242cf38
	ctx.lr = 0x8242C8A4;
	sub_8242CF38(ctx, base);
	// 8242C8A4: 4BFFFF28  b 0x8242c7cc
	pc = 0x8242C7CC; continue 'dispatch;
	// 8242C8A8: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 8242C8AC: 4BF93B1D  bl 0x823c03c8
	ctx.lr = 0x8242C8B0;
	sub_823C03C8(ctx, base);
	// 8242C8B0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242C8B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C8B8: 419AFFF0  beq cr6, 0x8242c8a8
	if ctx.cr[6].eq {
	pc = 0x8242C8A8; continue 'dispatch;
	}
	// 8242C8BC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242C8C0: 4BF951A1  bl 0x823c1a60
	ctx.lr = 0x8242C8C4;
	sub_823C1A60(ctx, base);
	// 8242C8C4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8242C8C8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8242C8CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8242C8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242C8D4: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8242C8D8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8242C8DC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8242C8E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242C8E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C8F8 size=148
    let mut pc: u32 = 0x8242C8F8;
    'dispatch: loop {
        match pc {
            0x8242C8F8 => {
    //   block [0x8242C8F8..0x8242C98C)
	// 8242C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242C900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242C904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242C908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C90C: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242C910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C914: 807E9AA8  lwz r3, -0x6558(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242C918: 480006B1  bl 0x8242cfc8
	ctx.lr = 0x8242C91C;
	sub_8242CFC8(ctx, base);
	// 8242C91C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242C920: 409A0018  bne cr6, 0x8242c938
	if !ctx.cr[6].eq {
	pc = 0x8242C938; continue 'dispatch;
	}
	// 8242C924: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C928: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242C92C: 4BFF02DD  bl 0x8241cc08
	ctx.lr = 0x8242C930;
	sub_8241CC08(ctx, base);
	// 8242C930: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242C934: 48000034  b 0x8242c968
	pc = 0x8242C968; continue 'dispatch;
	// 8242C938: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C93C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C940: 409A0010  bne cr6, 0x8242c950
	if !ctx.cr[6].eq {
	pc = 0x8242C950; continue 'dispatch;
	}
	// 8242C944: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C948: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242C94C: 4BFFFFE0  b 0x8242c92c
	pc = 0x8242C92C; continue 'dispatch;
	// 8242C950: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242C954: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242C958: 409A000C  bne cr6, 0x8242c964
	if !ctx.cr[6].eq {
	pc = 0x8242C964; continue 'dispatch;
	}
	// 8242C95C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242C960: 4BF94BC1  bl 0x823c1520
	ctx.lr = 0x8242C964;
	sub_823C1520(ctx, base);
	// 8242C964: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242C968: 807E9AA8  lwz r3, -0x6558(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242C96C: 480006F5  bl 0x8242d060
	ctx.lr = 0x8242C970;
	sub_8242D060(ctx, base);
	// 8242C970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242C974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242C978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242C97C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242C980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242C984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242C988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242C990 size=200
    let mut pc: u32 = 0x8242C990;
    'dispatch: loop {
        match pc {
            0x8242C990 => {
    //   block [0x8242C990..0x8242CA58)
	// 8242C990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242C994: 48108725  bl 0x825350b8
	ctx.lr = 0x8242C998;
	sub_82535080(ctx, base);
	// 8242C998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242C99C: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8242C9A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242C9A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242C9A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242C9AC: 807C9AA8  lwz r3, -0x6558(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242C9B0: 48000619  bl 0x8242cfc8
	ctx.lr = 0x8242C9B4;
	sub_8242CFC8(ctx, base);
	// 8242C9B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242C9B8: 409A0018  bne cr6, 0x8242c9d0
	if !ctx.cr[6].eq {
	pc = 0x8242C9D0; continue 'dispatch;
	}
	// 8242C9BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C9C0: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242C9C4: 4BFF0245  bl 0x8241cc08
	ctx.lr = 0x8242C9C8;
	sub_8241CC08(ctx, base);
	// 8242C9C8: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8242C9CC: 48000078  b 0x8242ca44
	pc = 0x8242CA44; continue 'dispatch;
	// 8242C9D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242C9D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C9D8: 409A0010  bne cr6, 0x8242c9e8
	if !ctx.cr[6].eq {
	pc = 0x8242C9E8; continue 'dispatch;
	}
	// 8242C9DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242C9E0: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242C9E4: 4BFFFFE0  b 0x8242c9c4
	pc = 0x8242C9C4; continue 'dispatch;
	// 8242C9E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242C9EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242C9F0: 409AFFD8  bne cr6, 0x8242c9c8
	if !ctx.cr[6].eq {
	pc = 0x8242C9C8; continue 'dispatch;
	}
	// 8242C9F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242C9F8: 480005D1  bl 0x8242cfc8
	ctx.lr = 0x8242C9FC;
	sub_8242CFC8(ctx, base);
	// 8242C9FC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242CA00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8242CA04: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8242CA08: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 8242CA0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242CA10: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8242CA14: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 8242CA18: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242CA1C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242CA20: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8242CA24: 409A0008  bne cr6, 0x8242ca2c
	if !ctx.cr[6].eq {
	pc = 0x8242CA2C; continue 'dispatch;
	}
	// 8242CA28: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8242CA2C: 83DF003C  lwz r30, 0x3c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242CA30: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242CA34: 4800062D  bl 0x8242d060
	ctx.lr = 0x8242CA38;
	sub_8242D060(ctx, base);
	// 8242CA38: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CA3C: 4BF94AE5  bl 0x823c1520
	ctx.lr = 0x8242CA40;
	sub_823C1520(ctx, base);
	// 8242CA40: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8242CA44: 807C9AA8  lwz r3, -0x6558(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CA48: 48000619  bl 0x8242d060
	ctx.lr = 0x8242CA4C;
	sub_8242D060(ctx, base);
	// 8242CA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242CA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242CA54: 481086B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CA58 size=240
    let mut pc: u32 = 0x8242CA58;
    'dispatch: loop {
        match pc {
            0x8242CA58 => {
    //   block [0x8242CA58..0x8242CB48)
	// 8242CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242CA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242CA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242CA6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242CA70: 409A0014  bne cr6, 0x8242ca84
	if !ctx.cr[6].eq {
	pc = 0x8242CA84; continue 'dispatch;
	}
	// 8242CA74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CA78: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242CA7C: 4BFF018D  bl 0x8241cc08
	ctx.lr = 0x8242CA80;
	sub_8241CC08(ctx, base);
	// 8242CA80: 480000B4  b 0x8242cb34
	pc = 0x8242CB34; continue 'dispatch;
	// 8242CA84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242CA88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CA8C: 409A0010  bne cr6, 0x8242ca9c
	if !ctx.cr[6].eq {
	pc = 0x8242CA9C; continue 'dispatch;
	}
	// 8242CA90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CA94: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242CA98: 4BFFFFE4  b 0x8242ca7c
	pc = 0x8242CA7C; continue 'dispatch;
	// 8242CA9C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242CAA0: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242CAA4: 409A0090  bne cr6, 0x8242cb34
	if !ctx.cr[6].eq {
	pc = 0x8242CB34; continue 'dispatch;
	}
	// 8242CAA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CAAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CAB0: 419A0084  beq cr6, 0x8242cb34
	if ctx.cr[6].eq {
	pc = 0x8242CB34; continue 'dispatch;
	}
	// 8242CAB4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CAB8: 4BF94A69  bl 0x823c1520
	ctx.lr = 0x8242CABC;
	sub_823C1520(ctx, base);
	// 8242CABC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242CAC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CAC4: 409A0014  bne cr6, 0x8242cad8
	if !ctx.cr[6].eq {
	pc = 0x8242CAD8; continue 'dispatch;
	}
	// 8242CAC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CACC: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242CAD0: 4BFF0139  bl 0x8241cc08
	ctx.lr = 0x8242CAD4;
	sub_8241CC08(ctx, base);
	// 8242CAD4: 48000024  b 0x8242caf8
	pc = 0x8242CAF8; continue 'dispatch;
	// 8242CAD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CADC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242CAE0: 409A000C  bne cr6, 0x8242caec
	if !ctx.cr[6].eq {
	pc = 0x8242CAEC; continue 'dispatch;
	}
	// 8242CAE4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CAE8: 4BF94A39  bl 0x823c1520
	ctx.lr = 0x8242CAEC;
	sub_823C1520(ctx, base);
	// 8242CAEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CAF0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242CAF4: 41820040  beq 0x8242cb34
	if ctx.cr[0].eq {
	pc = 0x8242CB34; continue 'dispatch;
	}
	// 8242CAF8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242CAFC: 480004CD  bl 0x8242cfc8
	ctx.lr = 0x8242CB00;
	sub_8242CFC8(ctx, base);
	// 8242CB00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CB04: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242CB08: 409A0024  bne cr6, 0x8242cb2c
	if !ctx.cr[6].eq {
	pc = 0x8242CB2C; continue 'dispatch;
	}
	// 8242CB0C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8242CB10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242CB14: 41820010  beq 0x8242cb24
	if ctx.cr[0].eq {
	pc = 0x8242CB24; continue 'dispatch;
	}
	// 8242CB18: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8242CB1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242CB20: 4E800421  bctrl
	ctx.lr = 0x8242CB24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242CB24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242CB28: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242CB2C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242CB30: 48000531  bl 0x8242d060
	ctx.lr = 0x8242CB34;
	sub_8242D060(ctx, base);
	// 8242CB34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242CB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242CB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242CB40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242CB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CB48 size=144
    let mut pc: u32 = 0x8242CB48;
    'dispatch: loop {
        match pc {
            0x8242CB48 => {
    //   block [0x8242CB48..0x8242CBD8)
	// 8242CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CB4C: 48108571  bl 0x825350bc
	ctx.lr = 0x8242CB50;
	sub_82535080(ctx, base);
	// 8242CB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CB54: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242CB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242CB5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242CB60: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CB64: 48000465  bl 0x8242cfc8
	ctx.lr = 0x8242CB68;
	sub_8242CFC8(ctx, base);
	// 8242CB68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242CB6C: 409A0014  bne cr6, 0x8242cb80
	if !ctx.cr[6].eq {
	pc = 0x8242CB80; continue 'dispatch;
	}
	// 8242CB70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CB74: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242CB78: 4BFF0091  bl 0x8241cc08
	ctx.lr = 0x8242CB7C;
	sub_8241CC08(ctx, base);
	// 8242CB7C: 48000044  b 0x8242cbc0
	pc = 0x8242CBC0; continue 'dispatch;
	// 8242CB80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242CB84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CB88: 409A0010  bne cr6, 0x8242cb98
	if !ctx.cr[6].eq {
	pc = 0x8242CB98; continue 'dispatch;
	}
	// 8242CB8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CB90: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242CB94: 4BFFFFE4  b 0x8242cb78
	pc = 0x8242CB78; continue 'dispatch;
	// 8242CB98: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242CB9C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8242CBA0: 419A000C  beq cr6, 0x8242cbac
	if ctx.cr[6].eq {
	pc = 0x8242CBAC; continue 'dispatch;
	}
	// 8242CBA4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8242CBA8: 4800001C  b 0x8242cbc4
	pc = 0x8242CBC4; continue 'dispatch;
	// 8242CBAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CBB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CBB4: 419AFFF0  beq cr6, 0x8242cba4
	if ctx.cr[6].eq {
	pc = 0x8242CBA4; continue 'dispatch;
	}
	// 8242CBB8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CBBC: 4BF94965  bl 0x823c1520
	ctx.lr = 0x8242CBC0;
	sub_823C1520(ctx, base);
	// 8242CBC0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242CBC4: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CBC8: 48000499  bl 0x8242d060
	ctx.lr = 0x8242CBCC;
	sub_8242D060(ctx, base);
	// 8242CBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242CBD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242CBD4: 48108538  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CBD8 size=132
    let mut pc: u32 = 0x8242CBD8;
    'dispatch: loop {
        match pc {
            0x8242CBD8 => {
    //   block [0x8242CBD8..0x8242CC5C)
	// 8242CBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CBDC: 481084E1  bl 0x825350bc
	ctx.lr = 0x8242CBE0;
	sub_82535080(ctx, base);
	// 8242CBE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CBE4: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242CBE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242CBEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242CBF0: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CBF4: 480003D5  bl 0x8242cfc8
	ctx.lr = 0x8242CBF8;
	sub_8242CFC8(ctx, base);
	// 8242CBF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242CBFC: 409A0014  bne cr6, 0x8242cc10
	if !ctx.cr[6].eq {
	pc = 0x8242CC10; continue 'dispatch;
	}
	// 8242CC00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CC04: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242CC08: 4BFF0001  bl 0x8241cc08
	ctx.lr = 0x8242CC0C;
	sub_8241CC08(ctx, base);
	// 8242CC0C: 48000040  b 0x8242cc4c
	pc = 0x8242CC4C; continue 'dispatch;
	// 8242CC10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242CC14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CC18: 409A0010  bne cr6, 0x8242cc28
	if !ctx.cr[6].eq {
	pc = 0x8242CC28; continue 'dispatch;
	}
	// 8242CC1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CC20: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242CC24: 4BFFFFE4  b 0x8242cc08
	pc = 0x8242CC08; continue 'dispatch;
	// 8242CC28: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8242CC2C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8242CC30: 419A001C  beq cr6, 0x8242cc4c
	if ctx.cr[6].eq {
	pc = 0x8242CC4C; continue 'dispatch;
	}
	// 8242CC34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242CC38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242CC3C: 4BF94DA5  bl 0x823c19e0
	ctx.lr = 0x8242CC40;
	sub_823C19E0(ctx, base);
	// 8242CC40: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242CC44: 4BF94E1D  bl 0x823c1a60
	ctx.lr = 0x8242CC48;
	sub_823C1A60(ctx, base);
	// 8242CC48: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 8242CC4C: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CC50: 48000411  bl 0x8242d060
	ctx.lr = 0x8242CC54;
	sub_8242D060(ctx, base);
	// 8242CC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242CC58: 481084B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CC60 size=128
    let mut pc: u32 = 0x8242CC60;
    'dispatch: loop {
        match pc {
            0x8242CC60 => {
    //   block [0x8242CC60..0x8242CCE0)
	// 8242CC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CC64: 48108459  bl 0x825350bc
	ctx.lr = 0x8242CC68;
	sub_82535080(ctx, base);
	// 8242CC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CC6C: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242CC70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242CC74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242CC78: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CC7C: 4800034D  bl 0x8242cfc8
	ctx.lr = 0x8242CC80;
	sub_8242CFC8(ctx, base);
	// 8242CC80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242CC84: 409A0014  bne cr6, 0x8242cc98
	if !ctx.cr[6].eq {
	pc = 0x8242CC98; continue 'dispatch;
	}
	// 8242CC88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CC8C: 386B304C  addi r3, r11, 0x304c
	ctx.r[3].s64 = ctx.r[11].s64 + 12364;
	// 8242CC90: 4BFEFF79  bl 0x8241cc08
	ctx.lr = 0x8242CC94;
	sub_8241CC08(ctx, base);
	// 8242CC94: 4800003C  b 0x8242ccd0
	pc = 0x8242CCD0; continue 'dispatch;
	// 8242CC98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242CC9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CCA0: 409A0010  bne cr6, 0x8242ccb0
	if !ctx.cr[6].eq {
	pc = 0x8242CCB0; continue 'dispatch;
	}
	// 8242CCA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CCA8: 386B3030  addi r3, r11, 0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + 12336;
	// 8242CCAC: 4BFFFFE4  b 0x8242cc90
	pc = 0x8242CC90; continue 'dispatch;
	// 8242CCB0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242CCB4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8242CCB8: 419A0018  beq cr6, 0x8242ccd0
	if ctx.cr[6].eq {
	pc = 0x8242CCD0; continue 'dispatch;
	}
	// 8242CCBC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CCC0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8242CCC4: 4BF9485D  bl 0x823c1520
	ctx.lr = 0x8242CCC8;
	sub_823C1520(ctx, base);
	// 8242CCC8: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 8242CCCC: 4BF936FD  bl 0x823c03c8
	ctx.lr = 0x8242CCD0;
	sub_823C03C8(ctx, base);
	// 8242CCD0: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CCD4: 4800038D  bl 0x8242d060
	ctx.lr = 0x8242CCD8;
	sub_8242D060(ctx, base);
	// 8242CCD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242CCDC: 48108430  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CCE0 size=68
    let mut pc: u32 = 0x8242CCE0;
    'dispatch: loop {
        match pc {
            0x8242CCE0 => {
    //   block [0x8242CCE0..0x8242CD24)
	// 8242CCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CCE4: 481083D9  bl 0x825350bc
	ctx.lr = 0x8242CCE8;
	sub_82535080(ctx, base);
	// 8242CCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CCEC: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242CCF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242CCF4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242CCF8: 807F9AA8  lwz r3, -0x6558(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CCFC: 480002CD  bl 0x8242cfc8
	ctx.lr = 0x8242CD00;
	sub_8242CFC8(ctx, base);
	// 8242CD00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242CD04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CD08: 4BFFFAA1  bl 0x8242c7a8
	ctx.lr = 0x8242CD0C;
	sub_8242C7A8(ctx, base);
	// 8242CD0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242CD10: 807F9AA8  lwz r3, -0x6558(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CD14: 4800034D  bl 0x8242d060
	ctx.lr = 0x8242CD18;
	sub_8242D060(ctx, base);
	// 8242CD18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242CD20: 481083EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CD28 size=60
    let mut pc: u32 = 0x8242CD28;
    'dispatch: loop {
        match pc {
            0x8242CD28 => {
    //   block [0x8242CD28..0x8242CD64)
	// 8242CD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CD2C: 48108391  bl 0x825350bc
	ctx.lr = 0x8242CD30;
	sub_82535080(ctx, base);
	// 8242CD30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CD34: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242CD38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242CD3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242CD40: 807F9AA8  lwz r3, -0x6558(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CD44: 48000285  bl 0x8242cfc8
	ctx.lr = 0x8242CD48;
	sub_8242CFC8(ctx, base);
	// 8242CD48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242CD4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CD50: 4BFFFD09  bl 0x8242ca58
	ctx.lr = 0x8242CD54;
	sub_8242CA58(ctx, base);
	// 8242CD54: 807F9AA8  lwz r3, -0x6558(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CD58: 48000309  bl 0x8242d060
	ctx.lr = 0x8242CD5C;
	sub_8242D060(ctx, base);
	// 8242CD5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242CD60: 481083AC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CD68 size=228
    let mut pc: u32 = 0x8242CD68;
    'dispatch: loop {
        match pc {
            0x8242CD68 => {
    //   block [0x8242CD68..0x8242CE4C)
	// 8242CD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CD6C: 48108351  bl 0x825350bc
	ctx.lr = 0x8242CD70;
	sub_82535080(ctx, base);
	// 8242CD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CD74: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8242CD78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242CD7C: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CD80: 48000249  bl 0x8242cfc8
	ctx.lr = 0x8242CD84;
	sub_8242CFC8(ctx, base);
	// 8242CD84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242CD88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242CD8C: 419A0010  beq cr6, 0x8242cd9c
	if ctx.cr[6].eq {
	pc = 0x8242CD9C; continue 'dispatch;
	}
	// 8242CD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242CD94: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242CD98: 4BFFFCC1  bl 0x8242ca58
	ctx.lr = 0x8242CD9C;
	sub_8242CA58(ctx, base);
	// 8242CD9C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8242CDA0: 48000018  b 0x8242cdb8
	pc = 0x8242CDB8; continue 'dispatch;
	// 8242CDA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242CDA8: 2B0B0103  cmplwi cr6, r11, 0x103
	ctx.cr[6].compare_u32(ctx.r[11].u32, 259 as u32, &mut ctx.xer);
	// 8242CDAC: 409A002C  bne cr6, 0x8242cdd8
	if !ctx.cr[6].eq {
	pc = 0x8242CDD8; continue 'dispatch;
	}
	// 8242CDB0: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 8242CDB4: 4BF93615  bl 0x823c03c8
	ctx.lr = 0x8242CDB8;
	sub_823C03C8(ctx, base);
	// 8242CDB8: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 8242CDBC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CDC0: 4BF94761  bl 0x823c1520
	ctx.lr = 0x8242CDC4;
	sub_823C1520(ctx, base);
	// 8242CDC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242CDC8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242CDCC: 4BF94D55  bl 0x823c1b20
	ctx.lr = 0x8242CDD0;
	sub_823C1B20(ctx, base);
	// 8242CDD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242CDD4: 4082FFD0  bne 0x8242cda4
	if !ctx.cr[0].eq {
	pc = 0x8242CDA4; continue 'dispatch;
	}
	// 8242CDD8: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8242CDDC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242CDE0: 4BF94739  bl 0x823c1518
	ctx.lr = 0x8242CDE4;
	sub_823C1518(ctx, base);
	// 8242CDE4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CDE8: 4BF94659  bl 0x823c1440
	ctx.lr = 0x8242CDEC;
	sub_823C1440(ctx, base);
	// 8242CDEC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242CDF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242CDF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242CDF8: 4182000C  beq 0x8242ce04
	if ctx.cr[0].eq {
	pc = 0x8242CE04; continue 'dispatch;
	}
	// 8242CDFC: 4BF93B5D  bl 0x823c0958
	ctx.lr = 0x8242CE00;
	sub_823C0958(ctx, base);
	// 8242CE00: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8242CE04: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242CE08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242CE0C: 4182000C  beq 0x8242ce18
	if ctx.cr[0].eq {
	pc = 0x8242CE18; continue 'dispatch;
	}
	// 8242CE10: 48000129  bl 0x8242cf38
	ctx.lr = 0x8242CE14;
	sub_8242CF38(ctx, base);
	// 8242CE14: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8242CE18: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242CE1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242CE20: 4182000C  beq 0x8242ce2c
	if ctx.cr[0].eq {
	pc = 0x8242CE2C; continue 'dispatch;
	}
	// 8242CE24: 4BF93B35  bl 0x823c0958
	ctx.lr = 0x8242CE28;
	sub_823C0958(ctx, base);
	// 8242CE28: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8242CE2C: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8242CE30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242CE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242CE38: 48108399  bl 0x825351d0
	ctx.lr = 0x8242CE3C;
	sub_825351D0(ctx, base);
	// 8242CE3C: 807D9AA8  lwz r3, -0x6558(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-25944 as u32) ) } as u64;
	// 8242CE40: 48000221  bl 0x8242d060
	ctx.lr = 0x8242CE44;
	sub_8242D060(ctx, base);
	// 8242CE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242CE48: 481082C4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242CE50 size=8
    let mut pc: u32 = 0x8242CE50;
    'dispatch: loop {
        match pc {
            0x8242CE50 => {
    //   block [0x8242CE50..0x8242CE58)
	// 8242CE50: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8242CE54: 820DA2C0  lwz r16, -0x5d40(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23872 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CE58 size=200
    let mut pc: u32 = 0x8242CE58;
    'dispatch: loop {
        match pc {
            0x8242CE58 => {
    //   block [0x8242CE58..0x8242CF20)
	// 8242CE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242CE60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242CE64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242CE68: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8242CE6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CE70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CE74: 396B3108  addi r11, r11, 0x3108
	ctx.r[11].s64 = ctx.r[11].s64 + 12552;
	// 8242CE78: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242CE7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242CE80: 916A9AB0  stw r11, -0x6550(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25936 as u32), ctx.r[11].u32 ) };
	// 8242CE84: 409A0018  bne cr6, 0x8242ce9c
	if !ctx.cr[6].eq {
	pc = 0x8242CE9C; continue 'dispatch;
	}
	// 8242CE88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CE8C: 386B31BC  addi r3, r11, 0x31bc
	ctx.r[3].s64 = ctx.r[11].s64 + 12732;
	// 8242CE90: 4BFEFD79  bl 0x8241cc08
	ctx.lr = 0x8242CE94;
	sub_8241CC08(ctx, base);
	// 8242CE94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242CE98: 48000070  b 0x8242cf08
	pc = 0x8242CF08; continue 'dispatch;
	// 8242CE9C: 2B040020  cmplwi cr6, r4, 0x20
	ctx.cr[6].compare_u32(ctx.r[4].u32, 32 as u32, &mut ctx.xer);
	// 8242CEA0: 40980010  bge cr6, 0x8242ceb0
	if !ctx.cr[6].lt {
	pc = 0x8242CEB0; continue 'dispatch;
	}
	// 8242CEA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CEA8: 386B318C  addi r3, r11, 0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + 12684;
	// 8242CEAC: 4BFFFFE4  b 0x8242ce90
	pc = 0x8242CE90; continue 'dispatch;
	// 8242CEB0: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8242CEB4: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8242CEB8: 557E003A  rlwinm r30, r11, 0, 0, 0x1d
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8242CEBC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8242CEC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242CEC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CEC8: 48108309  bl 0x825351d0
	ctx.lr = 0x8242CECC;
	sub_825351D0(ctx, base);
	// 8242CECC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CED0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CED8: 482E03B5  bl 0x8270d28c
	ctx.lr = 0x8242CEDC;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8242CEDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CEE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CEE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CEE8: 4800001C  b 0x8242cf04
	pc = 0x8242CF04; continue 'dispatch;
	// 8242CEEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CEF0: 386B3158  addi r3, r11, 0x3158
	ctx.r[3].s64 = ctx.r[11].s64 + 12632;
	// 8242CEF4: 4BFEFD15  bl 0x8241cc08
	ctx.lr = 0x8242CEF8;
	sub_8241CC08(ctx, base);
	// 8242CEF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242CEFC: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242CF00: 48000008  b 0x8242cf08
	pc = 0x8242CF08; continue 'dispatch;
	// 8242CF04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242CF08: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8242CF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242CF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242CF14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242CF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242CF20 size=12
    let mut pc: u32 = 0x8242CF20;
    'dispatch: loop {
        match pc {
            0x8242CF20 => {
    //   block [0x8242CF20..0x8242CF2C)
	// 8242CF20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242CF24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242CF30 size=8
    let mut pc: u32 = 0x8242CF30;
    'dispatch: loop {
        match pc {
            0x8242CF30 => {
    //   block [0x8242CF30..0x8242CF38)
	// 8242CF30: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8242CF34: 820DA2D8  lwz r16, -0x5d28(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CF38 size=124
    let mut pc: u32 = 0x8242CF38;
    'dispatch: loop {
        match pc {
            0x8242CF38 => {
    //   block [0x8242CF38..0x8242CFB4)
	// 8242CF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242CF40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242CF44: 3BE1FFA0  addi r31, r1, -0x60
	ctx.r[31].s64 = ctx.r[1].s64 + -96;
	// 8242CF48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CF4C: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 8242CF50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242CF54: 409A0014  bne cr6, 0x8242cf68
	if !ctx.cr[6].eq {
	pc = 0x8242CF68; continue 'dispatch;
	}
	// 8242CF58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CF5C: 386B321C  addi r3, r11, 0x321c
	ctx.r[3].s64 = ctx.r[11].s64 + 12828;
	// 8242CF60: 4BFEFCA9  bl 0x8241cc08
	ctx.lr = 0x8242CF64;
	sub_8241CC08(ctx, base);
	// 8242CF64: 4800003C  b 0x8242cfa0
	pc = 0x8242CFA0; continue 'dispatch;
	// 8242CF68: 60000000  nop
	// 8242CF6C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF70: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF78: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF7C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CF80: 48000014  b 0x8242cf94
	pc = 0x8242CF94; continue 'dispatch;
	// 8242CF84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CF88: 386B31EC  addi r3, r11, 0x31ec
	ctx.r[3].s64 = ctx.r[11].s64 + 12780;
	// 8242CF8C: 4BFEFC7D  bl 0x8241cc08
	ctx.lr = 0x8242CF90;
	sub_8241CC08(ctx, base);
	// 8242CF90: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8242CF94: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8242CF98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242CF9C: 48108235  bl 0x825351d0
	ctx.lr = 0x8242CFA0;
	sub_825351D0(ctx, base);
	// 8242CFA0: 383F0060  addi r1, r31, 0x60
	ctx.r[1].s64 = ctx.r[31].s64 + 96;
	// 8242CFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242CFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242CFAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242CFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242CFB4 size=12
    let mut pc: u32 = 0x8242CFB4;
    'dispatch: loop {
        match pc {
            0x8242CFB4 => {
    //   block [0x8242CFB4..0x8242CFC0)
	// 8242CFB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242CFB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242CFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242CFC0 size=8
    let mut pc: u32 = 0x8242CFC0;
    'dispatch: loop {
        match pc {
            0x8242CFC0 => {
    //   block [0x8242CFC0..0x8242CFC8)
	// 8242CFC0: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8242CFC4: 820DA2F0  lwz r16, -0x5d10(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23824 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242CFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242CFC8 size=132
    let mut pc: u32 = 0x8242CFC8;
    'dispatch: loop {
        match pc {
            0x8242CFC8 => {
    //   block [0x8242CFC8..0x8242D04C)
	// 8242CFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242CFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242CFD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242CFD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242CFD8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8242CFDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242CFE0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242CFE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242CFE8: 409A0018  bne cr6, 0x8242d000
	if !ctx.cr[6].eq {
	pc = 0x8242D000; continue 'dispatch;
	}
	// 8242CFEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242CFF0: 386B3274  addi r3, r11, 0x3274
	ctx.r[3].s64 = ctx.r[11].s64 + 12916;
	// 8242CFF4: 4BFEFC15  bl 0x8241cc08
	ctx.lr = 0x8242CFF8;
	sub_8241CC08(ctx, base);
	// 8242CFF8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242CFFC: 48000038  b 0x8242d034
	pc = 0x8242D034; continue 'dispatch;
	// 8242D000: 60000000  nop
	// 8242D004: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D008: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D00C: 482E0251  bl 0x8270d25c
	ctx.lr = 0x8242D010;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8242D010: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D014: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D018: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D01C: 48000014  b 0x8242d030
	pc = 0x8242D030; continue 'dispatch;
	// 8242D020: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D024: 386B3248  addi r3, r11, 0x3248
	ctx.r[3].s64 = ctx.r[11].s64 + 12872;
	// 8242D028: 4BFEFBE1  bl 0x8241cc08
	ctx.lr = 0x8242D02C;
	sub_8241CC08(ctx, base);
	// 8242D02C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242D030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242D034: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8242D038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D04C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D04C size=12
    let mut pc: u32 = 0x8242D04C;
    'dispatch: loop {
        match pc {
            0x8242D04C => {
    //   block [0x8242D04C..0x8242D058)
	// 8242D04C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242D050: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D058 size=8
    let mut pc: u32 = 0x8242D058;
    'dispatch: loop {
        match pc {
            0x8242D058 => {
    //   block [0x8242D058..0x8242D060)
	// 8242D058: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8242D05C: 820DA308  lwz r16, -0x5cf8(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D060 size=132
    let mut pc: u32 = 0x8242D060;
    'dispatch: loop {
        match pc {
            0x8242D060 => {
    //   block [0x8242D060..0x8242D0E4)
	// 8242D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242D06C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D070: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8242D074: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D078: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242D07C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242D080: 409A0018  bne cr6, 0x8242d098
	if !ctx.cr[6].eq {
	pc = 0x8242D098; continue 'dispatch;
	}
	// 8242D084: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D088: 386B32C8  addi r3, r11, 0x32c8
	ctx.r[3].s64 = ctx.r[11].s64 + 13000;
	// 8242D08C: 4BFEFB7D  bl 0x8241cc08
	ctx.lr = 0x8242D090;
	sub_8241CC08(ctx, base);
	// 8242D090: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242D094: 48000038  b 0x8242d0cc
	pc = 0x8242D0CC; continue 'dispatch;
	// 8242D098: 60000000  nop
	// 8242D09C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0A4: 482E01C9  bl 0x8270d26c
	ctx.lr = 0x8242D0A8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8242D0A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0B4: 48000014  b 0x8242d0c8
	pc = 0x8242D0C8; continue 'dispatch;
	// 8242D0B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D0BC: 386B329C  addi r3, r11, 0x329c
	ctx.r[3].s64 = ctx.r[11].s64 + 12956;
	// 8242D0C0: 4BFEFB49  bl 0x8241cc08
	ctx.lr = 0x8242D0C4;
	sub_8241CC08(ctx, base);
	// 8242D0C4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8242D0C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242D0CC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8242D0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D0E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D0E4 size=12
    let mut pc: u32 = 0x8242D0E4;
    'dispatch: loop {
        match pc {
            0x8242D0E4 => {
    //   block [0x8242D0E4..0x8242D0F0)
	// 8242D0E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242D0E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8242D0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242D0F0 size=288
    let mut pc: u32 = 0x8242D0F0;
    'dispatch: loop {
        match pc {
            0x8242D0F0 => {
    //   block [0x8242D0F0..0x8242D210)
	// 8242D0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D0F4: 48107FC9  bl 0x825350bc
	ctx.lr = 0x8242D0F8;
	sub_82535080(ctx, base);
	// 8242D0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D100: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D104: 4BFF19DD  bl 0x8241eae0
	ctx.lr = 0x8242D108;
	sub_8241EAE0(ctx, base);
	// 8242D108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242D10C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D110: 4BFF19D1  bl 0x8241eae0
	ctx.lr = 0x8242D114;
	sub_8241EAE0(ctx, base);
	// 8242D114: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8242D118: 419A00E8  beq cr6, 0x8242d200
	if ctx.cr[6].eq {
	pc = 0x8242D200; continue 'dispatch;
	}
	// 8242D11C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242D120: 419A00E0  beq cr6, 0x8242d200
	if ctx.cr[6].eq {
	pc = 0x8242D200; continue 'dispatch;
	}
	// 8242D124: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8242D128: 419A0078  beq cr6, 0x8242d1a0
	if ctx.cr[6].eq {
	pc = 0x8242D1A0; continue 'dispatch;
	}
	// 8242D12C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8242D130: 419A0070  beq cr6, 0x8242d1a0
	if ctx.cr[6].eq {
	pc = 0x8242D1A0; continue 'dispatch;
	}
	// 8242D134: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 8242D138: 809F033C  lwz r4, 0x33c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8242D13C: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8242D140: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D144: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8242D148: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D14C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242D150: 396B00D1  addi r11, r11, 0xd1
	ctx.r[11].s64 = ctx.r[11].s64 + 209;
	// 8242D154: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242D158: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8242D15C: 4BFF15FD  bl 0x8241e758
	ctx.lr = 0x8242D160;
	sub_8241E758(ctx, base);
	// 8242D160: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 8242D164: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8242D168: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D16C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D170: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8242D174: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8242D178: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D17C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242D180: 396B00D1  addi r11, r11, 0xd1
	ctx.r[11].s64 = ctx.r[11].s64 + 209;
	// 8242D184: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242D188: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8242D18C: 4BFF1555  bl 0x8241e6e0
	ctx.lr = 0x8242D190;
	sub_8241E6E0(ctx, base);
	// 8242D190: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 8242D194: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 8242D198: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D19C: 917F0340  stw r11, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[11].u32 ) };
	// 8242D1A0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D1A4: 4BFF1495  bl 0x8241e638
	ctx.lr = 0x8242D1A8;
	sub_8241E638(ctx, base);
	// 8242D1A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242D1AC: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D1B0: 4BFF1489  bl 0x8241e638
	ctx.lr = 0x8242D1B4;
	sub_8241E638(ctx, base);
	// 8242D1B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242D1B8: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D1BC: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8242D1C0: 796B5D24  sldi r11, r11, 0xb
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 8242D1C4: F97F0330  std r11, 0x330(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(816 as u32), ctx.r[11].u64 ) };
	// 8242D1C8: 4BFF1919  bl 0x8241eae0
	ctx.lr = 0x8242D1CC;
	sub_8241EAE0(ctx, base);
	// 8242D1CC: 817F0338  lwz r11, 0x338(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(824 as u32) ) } as u64;
	// 8242D1D0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242D1D4: 41980034  blt cr6, 0x8242d208
	if ctx.cr[6].lt {
	pc = 0x8242D208; continue 'dispatch;
	}
	// 8242D1D8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242D1DC: 4198002C  blt cr6, 0x8242d208
	if ctx.cr[6].lt {
	pc = 0x8242D208; continue 'dispatch;
	}
	// 8242D1E0: 817F033C  lwz r11, 0x33c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8242D1E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D1E8: 41990020  bgt cr6, 0x8242d208
	if ctx.cr[6].gt {
	pc = 0x8242D208; continue 'dispatch;
	}
	// 8242D1EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8242D1F0: 409A0018  bne cr6, 0x8242d208
	if !ctx.cr[6].eq {
	pc = 0x8242D208; continue 'dispatch;
	}
	// 8242D1F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242D1F8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8242D1FC: 4800000C  b 0x8242d208
	pc = 0x8242D208; continue 'dispatch;
	// 8242D200: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242D204: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242D208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D20C: 48107F00  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D210 size=180
    let mut pc: u32 = 0x8242D210;
    'dispatch: loop {
        match pc {
            0x8242D210 => {
    //   block [0x8242D210..0x8242D2C4)
	// 8242D210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242D21C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D228: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D22C: 4BFF18B5  bl 0x8241eae0
	ctx.lr = 0x8242D230;
	sub_8241EAE0(ctx, base);
	// 8242D230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242D234: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D238: 4BFF18A9  bl 0x8241eae0
	ctx.lr = 0x8242D23C;
	sub_8241EAE0(ctx, base);
	// 8242D23C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8242D240: 419A0064  beq cr6, 0x8242d2a4
	if ctx.cr[6].eq {
	pc = 0x8242D2A4; continue 'dispatch;
	}
	// 8242D244: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242D248: 419A005C  beq cr6, 0x8242d2a4
	if ctx.cr[6].eq {
	pc = 0x8242D2A4; continue 'dispatch;
	}
	// 8242D24C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D250: 4BFF1B49  bl 0x8241ed98
	ctx.lr = 0x8242D254;
	sub_8241ED98(ctx, base);
	// 8242D254: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D258: 41820014  beq 0x8242d26c
	if ctx.cr[0].eq {
	pc = 0x8242D26C; continue 'dispatch;
	}
	// 8242D25C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242D260: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D264: 4BFF166D  bl 0x8241e8d0
	ctx.lr = 0x8242D268;
	sub_8241E8D0(ctx, base);
	// 8242D268: 48000044  b 0x8242d2ac
	pc = 0x8242D2AC; continue 'dispatch;
	// 8242D26C: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D270: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242D274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D278: 4182000C  beq 0x8242d284
	if ctx.cr[0].eq {
	pc = 0x8242D284; continue 'dispatch;
	}
	// 8242D27C: 4BFF1255  bl 0x8241e4d0
	ctx.lr = 0x8242D280;
	sub_8241E4D0(ctx, base);
	// 8242D280: 93DF031C  stw r30, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[30].u32 ) };
	// 8242D284: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D288: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D28C: 4182000C  beq 0x8242d298
	if ctx.cr[0].eq {
	pc = 0x8242D298; continue 'dispatch;
	}
	// 8242D290: 4BFF1241  bl 0x8241e4d0
	ctx.lr = 0x8242D294;
	sub_8241E4D0(ctx, base);
	// 8242D294: 93DF0320  stw r30, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[30].u32 ) };
	// 8242D298: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8242D29C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8242D2A0: 4800000C  b 0x8242d2ac
	pc = 0x8242D2AC; continue 'dispatch;
	// 8242D2A4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242D2A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242D2AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D2B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D2BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D2C8 size=164
    let mut pc: u32 = 0x8242D2C8;
    'dispatch: loop {
        match pc {
            0x8242D2C8 => {
    //   block [0x8242D2C8..0x8242D36C)
	// 8242D2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D2CC: 48107DF1  bl 0x825350bc
	ctx.lr = 0x8242D2D0;
	sub_82535080(ctx, base);
	// 8242D2D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D2D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D2D8: 3BDF0114  addi r30, r31, 0x114
	ctx.r[30].s64 = ctx.r[31].s64 + 276;
	// 8242D2DC: 3BBF0218  addi r29, r31, 0x218
	ctx.r[29].s64 = ctx.r[31].s64 + 536;
	// 8242D2E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242D2E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242D2E8: 48119D09  bl 0x82546ff0
	ctx.lr = 0x8242D2EC;
	sub_82546FF0(ctx, base);
	// 8242D2EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D2F0: 4182006C  beq 0x8242d35c
	if ctx.cr[0].eq {
	pc = 0x8242D35C; continue 'dispatch;
	}
	// 8242D2F4: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 8242D2F8: 387F034C  addi r3, r31, 0x34c
	ctx.r[3].s64 = ctx.r[31].s64 + 844;
	// 8242D2FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242D300: 419A0010  beq cr6, 0x8242d310
	if ctx.cr[6].eq {
	pc = 0x8242D310; continue 'dispatch;
	}
	// 8242D304: 817F0350  lwz r11, 0x350(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(848 as u32) ) } as u64;
	// 8242D308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242D30C: 409A0024  bne cr6, 0x8242d330
	if !ctx.cr[6].eq {
	pc = 0x8242D330; continue 'dispatch;
	}
	// 8242D310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242D314: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242D318: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242D31C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8242D320: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8242D324: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8242D328: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8242D32C: 93DF0350  stw r30, 0x350(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(848 as u32), ctx.r[30].u32 ) };
	// 8242D330: 817F0368  lwz r11, 0x368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(872 as u32) ) } as u64;
	// 8242D334: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D338: 409A0018  bne cr6, 0x8242d350
	if !ctx.cr[6].eq {
	pc = 0x8242D350; continue 'dispatch;
	}
	// 8242D33C: 4BFF165D  bl 0x8241e998
	ctx.lr = 0x8242D340;
	sub_8241E998(ctx, base);
	// 8242D340: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D344: 41800020  blt 0x8242d364
	if ctx.cr[0].lt {
	pc = 0x8242D364; continue 'dispatch;
	}
	// 8242D348: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242D34C: 917F0368  stw r11, 0x368(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(872 as u32), ctx.r[11].u32 ) };
	// 8242D350: 817F0354  lwz r11, 0x354(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(852 as u32) ) } as u64;
	// 8242D354: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D358: 419A000C  beq cr6, 0x8242d364
	if ctx.cr[6].eq {
	pc = 0x8242D364; continue 'dispatch;
	}
	// 8242D35C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242D360: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242D364: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D368: 48107DA4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D370 size=124
    let mut pc: u32 = 0x8242D370;
    'dispatch: loop {
        match pc {
            0x8242D370 => {
    //   block [0x8242D370..0x8242D3EC)
	// 8242D370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242D37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D388: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242D38C: 409A0014  bne cr6, 0x8242d3a0
	if !ctx.cr[6].eq {
	pc = 0x8242D3A0; continue 'dispatch;
	}
	// 8242D390: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D394: 386B32F0  addi r3, r11, 0x32f0
	ctx.r[3].s64 = ctx.r[11].s64 + 13040;
	// 8242D398: 4BFEF871  bl 0x8241cc08
	ctx.lr = 0x8242D39C;
	sub_8241CC08(ctx, base);
	// 8242D39C: 48000038  b 0x8242d3d4
	pc = 0x8242D3D4; continue 'dispatch;
	// 8242D3A0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D3A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242D3A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D3AC: 4182000C  beq 0x8242d3b8
	if ctx.cr[0].eq {
	pc = 0x8242D3B8; continue 'dispatch;
	}
	// 8242D3B0: 4BFF1121  bl 0x8241e4d0
	ctx.lr = 0x8242D3B4;
	sub_8241E4D0(ctx, base);
	// 8242D3B4: 93DF031C  stw r30, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[30].u32 ) };
	// 8242D3B8: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D3BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D3C0: 4182000C  beq 0x8242d3cc
	if ctx.cr[0].eq {
	pc = 0x8242D3CC; continue 'dispatch;
	}
	// 8242D3C4: 4BFF110D  bl 0x8241e4d0
	ctx.lr = 0x8242D3C8;
	sub_8241E4D0(ctx, base);
	// 8242D3C8: 93DF0320  stw r30, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[30].u32 ) };
	// 8242D3CC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8242D3D0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8242D3D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D3E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D3E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D3F0 size=140
    let mut pc: u32 = 0x8242D3F0;
    'dispatch: loop {
        match pc {
            0x8242D3F0 => {
    //   block [0x8242D3F0..0x8242D47C)
	// 8242D3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D3F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D3FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D400: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D404: 3BEB9AB4  addi r31, r11, -0x654c
	ctx.r[31].s64 = ctx.r[11].s64 + -25932;
	// 8242D408: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 8242D40C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242D410: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242D414: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242D418: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242D41C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242D420: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242D424: 4082FFE8  bne 0x8242d40c
	if !ctx.cr[0].eq {
	pc = 0x8242D40C; continue 'dispatch;
	}
	// 8242D428: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242D42C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242D430: 409A0038  bne cr6, 0x8242d468
	if !ctx.cr[6].eq {
	pc = 0x8242D468; continue 'dispatch;
	}
	// 8242D434: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8242D438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D43C: 4BFFFA1D  bl 0x8242ce58
	ctx.lr = 0x8242D440;
	sub_8242CE58(ctx, base);
	// 8242D440: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8242D444: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D448: 40820014  bne 0x8242d45c
	if !ctx.cr[0].eq {
	pc = 0x8242D45C; continue 'dispatch;
	}
	// 8242D44C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D450: 386B1088  addi r3, r11, 0x1088
	ctx.r[3].s64 = ctx.r[11].s64 + 4232;
	// 8242D454: 4BFEF7B5  bl 0x8241cc08
	ctx.lr = 0x8242D458;
	sub_8241CC08(ctx, base);
	// 8242D458: 48000010  b 0x8242d468
	pc = 0x8242D468; continue 'dispatch;
	// 8242D45C: 4BFFFB6D  bl 0x8242cfc8
	ctx.lr = 0x8242D460;
	sub_8242CFC8(ctx, base);
	// 8242D460: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242D464: 4BFFFBFD  bl 0x8242d060
	ctx.lr = 0x8242D468;
	sub_8242D060(ctx, base);
	// 8242D468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D480 size=204
    let mut pc: u32 = 0x8242D480;
    'dispatch: loop {
        match pc {
            0x8242D480 => {
    //   block [0x8242D480..0x8242D54C)
	// 8242D480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242D48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D498: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242D49C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242D4A0: 419A008C  beq cr6, 0x8242d52c
	if ctx.cr[6].eq {
	pc = 0x8242D52C; continue 'dispatch;
	}
	// 8242D4A4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8242D4A8: 419A0028  beq cr6, 0x8242d4d0
	if ctx.cr[6].eq {
	pc = 0x8242D4D0; continue 'dispatch;
	}
	// 8242D4AC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8242D4B0: 419A0014  beq cr6, 0x8242d4c4
	if ctx.cr[6].eq {
	pc = 0x8242D4C4; continue 'dispatch;
	}
	// 8242D4B4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8242D4B8: 409A007C  bne cr6, 0x8242d534
	if !ctx.cr[6].eq {
	pc = 0x8242D534; continue 'dispatch;
	}
	// 8242D4BC: 4BFFFE0D  bl 0x8242d2c8
	ctx.lr = 0x8242D4C0;
	sub_8242D2C8(ctx, base);
	// 8242D4C0: 48000074  b 0x8242d534
	pc = 0x8242D534; continue 'dispatch;
	// 8242D4C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D4C8: 4BFFFD49  bl 0x8242d210
	ctx.lr = 0x8242D4CC;
	sub_8242D210(ctx, base);
	// 8242D4CC: 48000068  b 0x8242d534
	pc = 0x8242D534; continue 'dispatch;
	// 8242D4D0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D4D4: 4BFF160D  bl 0x8241eae0
	ctx.lr = 0x8242D4D8;
	sub_8241EAE0(ctx, base);
	// 8242D4D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242D4DC: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D4E0: 4BFF1601  bl 0x8241eae0
	ctx.lr = 0x8242D4E4;
	sub_8241EAE0(ctx, base);
	// 8242D4E4: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8242D4E8: 419A0038  beq cr6, 0x8242d520
	if ctx.cr[6].eq {
	pc = 0x8242D520; continue 'dispatch;
	}
	// 8242D4EC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242D4F0: 419A0030  beq cr6, 0x8242d520
	if ctx.cr[6].eq {
	pc = 0x8242D520; continue 'dispatch;
	}
	// 8242D4F4: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D4F8: 4BFEFFF1  bl 0x8241d4e8
	ctx.lr = 0x8242D4FC;
	sub_8241D4E8(ctx, base);
	// 8242D4FC: E89F0328  ld r4, 0x328(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) };
	// 8242D500: 7F232000  cmpd cr6, r3, r4
	ctx.cr[6].compare_i64(ctx.r[3].s64, ctx.r[4].s64, &mut ctx.xer);
	// 8242D504: 419A0010  beq cr6, 0x8242d514
	if ctx.cr[6].eq {
	pc = 0x8242D514; continue 'dispatch;
	}
	// 8242D508: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D50C: 4BFF12C5  bl 0x8241e7d0
	ctx.lr = 0x8242D510;
	sub_8241E7D0(ctx, base);
	// 8242D510: 48000024  b 0x8242d534
	pc = 0x8242D534; continue 'dispatch;
	// 8242D514: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242D518: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8242D51C: 48000018  b 0x8242d534
	pc = 0x8242D534; continue 'dispatch;
	// 8242D520: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242D524: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242D528: 4800000C  b 0x8242d534
	pc = 0x8242D534; continue 'dispatch;
	// 8242D52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D530: 4BFFFBC1  bl 0x8242d0f0
	ctx.lr = 0x8242D534;
	sub_8242D0F0(ctx, base);
	// 8242D534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D550 size=208
    let mut pc: u32 = 0x8242D550;
    'dispatch: loop {
        match pc {
            0x8242D550 => {
    //   block [0x8242D550..0x8242D620)
	// 8242D550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D55C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242D564: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242D568: 409A0014  bne cr6, 0x8242d57c
	if !ctx.cr[6].eq {
	pc = 0x8242D57C; continue 'dispatch;
	}
	// 8242D56C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D570: 386B3340  addi r3, r11, 0x3340
	ctx.r[3].s64 = ctx.r[11].s64 + 13120;
	// 8242D574: 4BFEF695  bl 0x8241cc08
	ctx.lr = 0x8242D578;
	sub_8241CC08(ctx, base);
	// 8242D578: 48000094  b 0x8242d60c
	pc = 0x8242D60C; continue 'dispatch;
	// 8242D57C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242D580: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D584: 409A0088  bne cr6, 0x8242d60c
	if !ctx.cr[6].eq {
	pc = 0x8242D60C; continue 'dispatch;
	}
	// 8242D588: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242D58C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242D590: 419A0018  beq cr6, 0x8242d5a8
	if ctx.cr[6].eq {
	pc = 0x8242D5A8; continue 'dispatch;
	}
	// 8242D594: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242D598: 409A0074  bne cr6, 0x8242d60c
	if !ctx.cr[6].eq {
	pc = 0x8242D60C; continue 'dispatch;
	}
	// 8242D59C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D5A0: 4BFFFEE1  bl 0x8242d480
	ctx.lr = 0x8242D5A4;
	sub_8242D480(ctx, base);
	// 8242D5A4: 48000068  b 0x8242d60c
	pc = 0x8242D60C; continue 'dispatch;
	// 8242D5A8: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D5AC: 4BFF17ED  bl 0x8241ed98
	ctx.lr = 0x8242D5B0;
	sub_8241ED98(ctx, base);
	// 8242D5B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D5B4: 41820058  beq 0x8242d60c
	if ctx.cr[0].eq {
	pc = 0x8242D60C; continue 'dispatch;
	}
	// 8242D5B8: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242D5BC: 4BFF17DD  bl 0x8241ed98
	ctx.lr = 0x8242D5C0;
	sub_8241ED98(ctx, base);
	// 8242D5C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D5C4: 41820048  beq 0x8242d60c
	if ctx.cr[0].eq {
	pc = 0x8242D60C; continue 'dispatch;
	}
	// 8242D5C8: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D5CC: 4BFEFF1D  bl 0x8241d4e8
	ctx.lr = 0x8242D5D0;
	sub_8241D4E8(ctx, base);
	// 8242D5D0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8242D5D4: 80BF0344  lwz r5, 0x344(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(836 as u32) ) } as u64;
	// 8242D5D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242D5DC: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8242D5E0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8242D5E4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242D5E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8242D5EC: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 8242D5F0: F97F0328  std r11, 0x328(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[11].u64 ) };
	// 8242D5F4: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 8242D5F8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8242D5FC: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 8242D600: 917F0338  stw r11, 0x338(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(824 as u32), ctx.r[11].u32 ) };
	// 8242D604: 4BFF10DD  bl 0x8241e6e0
	ctx.lr = 0x8242D608;
	sub_8241E6E0(ctx, base);
	// 8242D608: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 8242D60C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D620 size=240
    let mut pc: u32 = 0x8242D620;
    'dispatch: loop {
        match pc {
            0x8242D620 => {
    //   block [0x8242D620..0x8242D710)
	// 8242D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D624: 48107A95  bl 0x825350b8
	ctx.lr = 0x8242D628;
	sub_82535080(ctx, base);
	// 8242D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D62C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D630: 3BCB9ADC  addi r30, r11, -0x6524
	ctx.r[30].s64 = ctx.r[11].s64 + -25892;
	// 8242D634: 391EFFF8  addi r8, r30, -8
	ctx.r[8].s64 = ctx.r[30].s64 + -8;
	// 8242D638: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8242D63C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242D640: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8242D644: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242D648: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8242D64C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8242D650: 4082FFE8  bne 0x8242d638
	if !ctx.cr[0].eq {
	pc = 0x8242D638; continue 'dispatch;
	}
	// 8242D654: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242D658: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D65C: 409A00AC  bne cr6, 0x8242d708
	if !ctx.cr[6].eq {
	pc = 0x8242D708; continue 'dispatch;
	}
	// 8242D660: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242D664: 4BFFF965  bl 0x8242cfc8
	ctx.lr = 0x8242D668;
	sub_8242CFC8(ctx, base);
	// 8242D668: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D66C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242D670: 419A0078  beq cr6, 0x8242d6e8
	if ctx.cr[6].eq {
	pc = 0x8242D6E8; continue 'dispatch;
	}
	// 8242D674: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D678: 3B8B3318  addi r28, r11, 0x3318
	ctx.r[28].s64 = ctx.r[11].s64 + 13080;
	// 8242D67C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 8242D680: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242D684: 409A0014  bne cr6, 0x8242d698
	if !ctx.cr[6].eq {
	pc = 0x8242D698; continue 'dispatch;
	}
	// 8242D688: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8242D68C: 4BFEF57D  bl 0x8241cc08
	ctx.lr = 0x8242D690;
	sub_8241CC08(ctx, base);
	// 8242D690: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D694: 4800004C  b 0x8242d6e0
	pc = 0x8242D6E0; continue 'dispatch;
	// 8242D698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D69C: 4BFFFCD5  bl 0x8242d370
	ctx.lr = 0x8242D6A0;
	sub_8242D370(ctx, base);
	// 8242D6A0: 815D036C  lwz r10, 0x36c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(876 as u32) ) } as u64;
	// 8242D6A4: 817D0370  lwz r11, 0x370(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(880 as u32) ) } as u64;
	// 8242D6A8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D6AC: 41820010  beq 0x8242d6bc
	if ctx.cr[0].eq {
	pc = 0x8242D6BC; continue 'dispatch;
	}
	// 8242D6B0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D6B4: 916A0370  stw r11, 0x370(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(880 as u32), ctx.r[11].u32 ) };
	// 8242D6B8: 4800000C  b 0x8242d6c4
	pc = 0x8242D6C4; continue 'dispatch;
	// 8242D6BC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8242D6C0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8242D6C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242D6C8: 419A0008  beq cr6, 0x8242d6d0
	if ctx.cr[6].eq {
	pc = 0x8242D6D0; continue 'dispatch;
	}
	// 8242D6CC: 914B036C  stw r10, 0x36c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(876 as u32), ctx.r[10].u32 ) };
	// 8242D6D0: 38A00378  li r5, 0x378
	ctx.r[5].s64 = 888;
	// 8242D6D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242D6D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242D6DC: 48107AF5  bl 0x825351d0
	ctx.lr = 0x8242D6E0;
	sub_825351D0(ctx, base);
	// 8242D6E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242D6E4: 409AFF98  bne cr6, 0x8242d67c
	if !ctx.cr[6].eq {
	pc = 0x8242D67C; continue 'dispatch;
	}
	// 8242D6E8: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242D6EC: 4BFFF975  bl 0x8242d060
	ctx.lr = 0x8242D6F0;
	sub_8242D060(ctx, base);
	// 8242D6F0: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242D6F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242D6F8: 419A0010  beq cr6, 0x8242d708
	if ctx.cr[6].eq {
	pc = 0x8242D708; continue 'dispatch;
	}
	// 8242D6FC: 4BFFF83D  bl 0x8242cf38
	ctx.lr = 0x8242D700;
	sub_8242CF38(ctx, base);
	// 8242D700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242D704: 917EFFFC  stw r11, -4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 8242D708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242D70C: 481079FC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D710 size=100
    let mut pc: u32 = 0x8242D710;
    'dispatch: loop {
        match pc {
            0x8242D710 => {
    //   block [0x8242D710..0x8242D774)
	// 8242D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242D71C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D724: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D728: 3BEB9ADC  addi r31, r11, -0x6524
	ctx.r[31].s64 = ctx.r[11].s64 + -25892;
	// 8242D72C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242D730: 4BFFF899  bl 0x8242cfc8
	ctx.lr = 0x8242D734;
	sub_8242CFC8(ctx, base);
	// 8242D734: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D738: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242D73C: 419A0018  beq cr6, 0x8242d754
	if ctx.cr[6].eq {
	pc = 0x8242D754; continue 'dispatch;
	}
	// 8242D740: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242D744: 4BFFFE0D  bl 0x8242d550
	ctx.lr = 0x8242D748;
	sub_8242D550(ctx, base);
	// 8242D748: 83DE0370  lwz r30, 0x370(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(880 as u32) ) } as u64;
	// 8242D74C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D750: 4082FFF0  bne 0x8242d740
	if !ctx.cr[0].eq {
	pc = 0x8242D740; continue 'dispatch;
	}
	// 8242D754: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242D758: 4BFFF909  bl 0x8242d060
	ctx.lr = 0x8242D75C;
	sub_8242D060(ctx, base);
	// 8242D75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242D760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242D76C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D778 size=4
    let mut pc: u32 = 0x8242D778;
    'dispatch: loop {
        match pc {
            0x8242D778 => {
    //   block [0x8242D778..0x8242D77C)
	// 8242D778: 4BFF80A8  b 0x82425820
	sub_82425820(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D780 size=128
    let mut pc: u32 = 0x8242D780;
    'dispatch: loop {
        match pc {
            0x8242D780 => {
    //   block [0x8242D780..0x8242D800)
	// 8242D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D78C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D790: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D794: 3BEB9AE0  addi r31, r11, -0x6520
	ctx.r[31].s64 = ctx.r[11].s64 + -25888;
	// 8242D798: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 8242D79C: 395F002C  addi r10, r31, 0x2c
	ctx.r[10].s64 = ctx.r[31].s64 + 44;
	// 8242D7A0: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 8242D7A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D7A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D7AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242D7B0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D7B4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242D7B8: 409A0034  bne cr6, 0x8242d7ec
	if !ctx.cr[6].eq {
	pc = 0x8242D7EC; continue 'dispatch;
	}
	// 8242D7BC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242D7C4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8242D7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242D7CC: 914B9B04  stw r10, -0x64fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25852 as u32), ctx.r[10].u32 ) };
	// 8242D7D0: 4BFFF689  bl 0x8242ce58
	ctx.lr = 0x8242D7D4;
	sub_8242CE58(ctx, base);
	// 8242D7D4: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 8242D7D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242D7DC: 40820010  bne 0x8242d7ec
	if !ctx.cr[0].eq {
	pc = 0x8242D7EC; continue 'dispatch;
	}
	// 8242D7E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D7E4: 386B3368  addi r3, r11, 0x3368
	ctx.r[3].s64 = ctx.r[11].s64 + 13160;
	// 8242D7E8: 4BFEF421  bl 0x8241cc08
	ctx.lr = 0x8242D7EC;
	sub_8241CC08(ctx, base);
	// 8242D7EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D7F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D800 size=116
    let mut pc: u32 = 0x8242D800;
    'dispatch: loop {
        match pc {
            0x8242D800 => {
    //   block [0x8242D800..0x8242D874)
	// 8242D800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242D80C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D810: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D814: 3BEB9B00  addi r31, r11, -0x6500
	ctx.r[31].s64 = ctx.r[11].s64 + -25856;
	// 8242D818: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8242D81C: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 8242D820: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8242D824: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D828: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242D82C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242D830: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D834: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242D838: 409A0028  bne cr6, 0x8242d860
	if !ctx.cr[6].eq {
	pc = 0x8242D860; continue 'dispatch;
	}
	// 8242D83C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D840: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242D848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242D84C: 914B9B04  stw r10, -0x64fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25852 as u32), ctx.r[10].u32 ) };
	// 8242D850: 419A0010  beq cr6, 0x8242d860
	if ctx.cr[6].eq {
	pc = 0x8242D860; continue 'dispatch;
	}
	// 8242D854: 4BFFF6E5  bl 0x8242cf38
	ctx.lr = 0x8242D858;
	sub_8242CF38(ctx, base);
	// 8242D858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242D85C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242D860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D86C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242D870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D878 size=60
    let mut pc: u32 = 0x8242D878;
    'dispatch: loop {
        match pc {
            0x8242D878 => {
    //   block [0x8242D878..0x8242D8B4)
	// 8242D878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D884: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D888: 806B9B00  lwz r3, -0x6500(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25856 as u32) ) } as u64;
	// 8242D88C: 4BFFF73D  bl 0x8242cfc8
	ctx.lr = 0x8242D890;
	sub_8242CFC8(ctx, base);
	// 8242D890: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D894: 40800010  bge 0x8242d8a4
	if !ctx.cr[0].lt {
	pc = 0x8242D8A4; continue 'dispatch;
	}
	// 8242D898: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D89C: 386B33A0  addi r3, r11, 0x33a0
	ctx.r[3].s64 = ctx.r[11].s64 + 13216;
	// 8242D8A0: 4BFEF369  bl 0x8241cc08
	ctx.lr = 0x8242D8A4;
	sub_8241CC08(ctx, base);
	// 8242D8A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242D8B8 size=60
    let mut pc: u32 = 0x8242D8B8;
    'dispatch: loop {
        match pc {
            0x8242D8B8 => {
    //   block [0x8242D8B8..0x8242D8F4)
	// 8242D8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242D8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242D8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242D8C4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242D8C8: 806B9B00  lwz r3, -0x6500(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25856 as u32) ) } as u64;
	// 8242D8CC: 4BFFF795  bl 0x8242d060
	ctx.lr = 0x8242D8D0;
	sub_8242D060(ctx, base);
	// 8242D8D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242D8D4: 40800010  bge 0x8242d8e4
	if !ctx.cr[0].lt {
	pc = 0x8242D8E4; continue 'dispatch;
	}
	// 8242D8D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242D8DC: 386B3400  addi r3, r11, 0x3400
	ctx.r[3].s64 = ctx.r[11].s64 + 13312;
	// 8242D8E0: 4BFEF329  bl 0x8241cc08
	ctx.lr = 0x8242D8E4;
	sub_8241CC08(ctx, base);
	// 8242D8E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242D8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242D8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242D8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D8F8 size=184
    let mut pc: u32 = 0x8242D8F8;
    'dispatch: loop {
        match pc {
            0x8242D8F8 => {
    //   block [0x8242D8F8..0x8242D9B0)
	// 8242D8F8: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D8FC: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8242D900: 99440000  stb r10, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8242D904: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D908: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D90C: 99440001  stb r10, 1(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8242D910: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D914: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D918: 99440002  stb r10, 2(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8242D91C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D920: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242D924: 99440003  stb r10, 3(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 8242D928: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D92C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D930: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D934: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242D938: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242D93C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242D940: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D944: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D948: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242D94C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D950: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8242D954: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242D958: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D95C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D960: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D964: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242D968: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242D96C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242D970: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D974: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D978: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242D97C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242D980: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8242D984: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8242D988: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D98C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D990: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D994: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D998: B144000C  sth r10, 0xc(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 8242D99C: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242D9A0: 9944003C  stb r10, 0x3c(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), ctx.r[10].u8 ) };
	// 8242D9A4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242D9A8: 9964003D  stb r11, 0x3d(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 8242D9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242D9B0 size=236
    let mut pc: u32 = 0x8242D9B0;
    'dispatch: loop {
        match pc {
            0x8242D9B0 => {
    //   block [0x8242D9B0..0x8242DA9C)
	// 8242D9B0: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D9B4: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 8242D9B8: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D9BC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D9C0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D9C4: B1440000  sth r10, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8242D9C8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D9CC: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D9D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242D9D4: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D9D8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D9DC: B1440002  sth r10, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 8242D9E0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242D9E4: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242D9E8: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242D9EC: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242D9F0: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242D9F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242D9F8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242D9FC: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA00: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242DA04: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA08: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8242DA0C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242DA10: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DA14: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DA18: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DA1C: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242DA20: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242DA24: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242DA28: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DA2C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA30: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242DA34: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA38: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8242DA3C: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8242DA40: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DA44: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DA48: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DA4C: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242DA50: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242DA54: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DA58: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA5C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242DA60: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA64: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8242DA68: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8242DA6C: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242DA70: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242DA74: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DA78: 890B0006  lbz r8, 6(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8242DA7C: 896B0007  lbz r11, 7(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 8242DA80: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DA84: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA88: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242DA8C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242DA90: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8242DA94: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8242DA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242DAA0 size=180
    let mut pc: u32 = 0x8242DAA0;
    'dispatch: loop {
        match pc {
            0x8242DAA0 => {
    //   block [0x8242DAA0..0x8242DB54)
	// 8242DAA0: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DAA4: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 8242DAA8: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DAAC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DAB0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DAB4: B1440000  sth r10, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8242DAB8: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 8242DABC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242DAC0: 40810018  ble 0x8242dad8
	if !ctx.cr[0].gt {
	pc = 0x8242DAD8; continue 'dispatch;
	}
	// 8242DAC4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DAC8: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DACC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DAD0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DAD4: B1440002  sth r10, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 8242DAD8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242DADC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DAE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DAE4: 99440004  stb r10, 4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 8242DAE8: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8242DAEC: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DAF0: 4081000C  ble 0x8242dafc
	if !ctx.cr[0].gt {
	pc = 0x8242DAFC; continue 'dispatch;
	}
	// 8242DAF4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DAF8: 99440005  stb r10, 5(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 8242DAFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DB00: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DB08: 99440006  stb r10, 6(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 8242DB0C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8242DB10: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DB14: 4081000C  ble 0x8242db20
	if !ctx.cr[0].gt {
	pc = 0x8242DB20; continue 'dispatch;
	}
	// 8242DB18: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DB1C: 99440007  stb r10, 7(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(7 as u32), ctx.r[10].u8 ) };
	// 8242DB20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DB24: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DB28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DB2C: 99440008  stb r10, 8(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8242DB30: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8242DB34: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DB38: 4081000C  ble 0x8242db44
	if !ctx.cr[0].gt {
	pc = 0x8242DB44; continue 'dispatch;
	}
	// 8242DB3C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DB40: 99440009  stb r10, 9(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 8242DB44: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DB48: 9944000A  stb r10, 0xa(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(10 as u32), ctx.r[10].u8 ) };
	// 8242DB4C: 7D490775  extsb. r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242DB50: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DB54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242DB54 size=12
    let mut pc: u32 = 0x8242DB54;
    'dispatch: loop {
        match pc {
            0x8242DB54 => {
    //   block [0x8242DB54..0x8242DB60)
	// 8242DB54: 896B0002  lbz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242DB58: 9964000B  stb r11, 0xb(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(11 as u32), ctx.r[11].u8 ) };
	// 8242DB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242DB60 size=392
    let mut pc: u32 = 0x8242DB60;
    'dispatch: loop {
        match pc {
            0x8242DB60 => {
    //   block [0x8242DB60..0x8242DCE8)
	// 8242DB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242DB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242DB68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242DB6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242DB70: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8242DB74: 419A000C  beq cr6, 0x8242db80
	if ctx.cr[6].eq {
	pc = 0x8242DB80; continue 'dispatch;
	}
	// 8242DB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242DB7C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242DB80: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8242DB84: 4098000C  bge cr6, 0x8242db90
	if !ctx.cr[6].lt {
	pc = 0x8242DB90; continue 'dispatch;
	}
	// 8242DB88: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242DB8C: 48000148  b 0x8242dcd4
	pc = 0x8242DCD4; continue 'dispatch;
	// 8242DB90: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DB94: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 8242DB98: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DB9C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DBA0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242DBA4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8242DBA8: 2B0A8000  cmplwi cr6, r10, 0x8000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32768 as u32, &mut ctx.xer);
	// 8242DBAC: 419A000C  beq cr6, 0x8242dbb8
	if ctx.cr[6].eq {
	pc = 0x8242DBB8; continue 'dispatch;
	}
	// 8242DBB0: 3860FFFC  li r3, -4
	ctx.r[3].s64 = -4;
	// 8242DBB4: 48000120  b 0x8242dcd4
	pc = 0x8242DCD4; continue 'dispatch;
	// 8242DBB8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DBBC: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8242DBC0: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DBC4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8242DBC8: 554B403E  rotlwi r11, r10, 8
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242DBCC: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8242DBD0: 419A0008  beq cr6, 0x8242dbd8
	if ctx.cr[6].eq {
	pc = 0x8242DBD8; continue 'dispatch;
	}
	// 8242DBD4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242DBD8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8242DBDC: 419A00F4  beq cr6, 0x8242dcd0
	if ctx.cr[6].eq {
	pc = 0x8242DCD0; continue 'dispatch;
	}
	// 8242DBE0: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 8242DBE4: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242DBE8: 4098000C  bge cr6, 0x8242dbf4
	if !ctx.cr[6].lt {
	pc = 0x8242DBF4; continue 'dispatch;
	}
	// 8242DBEC: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8242DBF0: 480000E4  b 0x8242dcd4
	pc = 0x8242DCD4; continue 'dispatch;
	// 8242DBF4: 38ABFFFA  addi r5, r11, -6
	ctx.r[5].s64 = ctx.r[11].s64 + -6;
	// 8242DBF8: 2F050010  cmpwi cr6, r5, 0x10
	ctx.cr[6].compare_i32(ctx.r[5].s32, 16, &mut ctx.xer);
	// 8242DBFC: 4198FFF0  blt cr6, 0x8242dbec
	if ctx.cr[6].lt {
	pc = 0x8242DBEC; continue 'dispatch;
	}
	// 8242DC00: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8242DC04: 4BFFFCF5  bl 0x8242d8f8
	ctx.lr = 0x8242DC08;
	sub_8242D8F8(ctx, base);
	// 8242DC08: 8946003C  lbz r10, 0x3c(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242DC0C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 8242DC10: 3BE5FFF0  addi r31, r5, -0x10
	ctx.r[31].s64 = ctx.r[5].s64 + -16;
	// 8242DC14: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 8242DC18: 409A000C  bne cr6, 0x8242dc24
	if !ctx.cr[6].eq {
	pc = 0x8242DC24; continue 'dispatch;
	}
	// 8242DC1C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8242DC20: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 8242DC24: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8242DC28: 419800A8  blt cr6, 0x8242dcd0
	if ctx.cr[6].lt {
	pc = 0x8242DCD0; continue 'dispatch;
	}
	// 8242DC2C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8242DC30: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8242DC34: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242DC38: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242DC3C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8242DC40: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8242DC44: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8242DC48: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242DC4C: B166000E  sth r11, 0xe(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8242DC50: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 8242DC54: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242DC58: 41980078  blt cr6, 0x8242dcd0
	if ctx.cr[6].lt {
	pc = 0x8242DCD0; continue 'dispatch;
	}
	// 8242DC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8242DC60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DC64: 40990020  ble cr6, 0x8242dc84
	if !ctx.cr[6].gt {
	pc = 0x8242DC84; continue 'dispatch;
	}
	// 8242DC68: 38860010  addi r4, r6, 0x10
	ctx.r[4].s64 = ctx.r[6].s64 + 16;
	// 8242DC6C: 4BFFFD45  bl 0x8242d9b0
	ctx.lr = 0x8242DC70;
	sub_8242D9B0(ctx, base);
	// 8242DC70: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8242DC74: 38840014  addi r4, r4, 0x14
	ctx.r[4].s64 = ctx.r[4].s64 + 20;
	// 8242DC78: A966000E  lha r11, 0xe(r6)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as i16) as i64;
	// 8242DC7C: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242DC80: 4198FFEC  blt cr6, 0x8242dc6c
	if ctx.cr[6].lt {
	pc = 0x8242DC6C; continue 'dispatch;
	}
	// 8242DC84: 89460003  lbz r10, 3(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242DC88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8242DC8C: A966000E  lha r11, 0xe(r6)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as i16) as i64;
	// 8242DC90: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 8242DC94: 7CEBF850  subf r7, r11, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8242DC98: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DC9C: 40810034  ble 0x8242dcd0
	if !ctx.cr[0].gt {
	pc = 0x8242DCD0; continue 'dispatch;
	}
	// 8242DCA0: 38860024  addi r4, r6, 0x24
	ctx.r[4].s64 = ctx.r[6].s64 + 36;
	// 8242DCA4: 2F07000C  cmpwi cr6, r7, 0xc
	ctx.cr[6].compare_i32(ctx.r[7].s32, 12, &mut ctx.xer);
	// 8242DCA8: 41980028  blt cr6, 0x8242dcd0
	if ctx.cr[6].lt {
	pc = 0x8242DCD0; continue 'dispatch;
	}
	// 8242DCAC: 4BFFFDF5  bl 0x8242daa0
	ctx.lr = 0x8242DCB0;
	sub_8242DAA0(ctx, base);
	// 8242DCB0: 89660003  lbz r11, 3(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242DCB4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8242DCB8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242DCBC: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8242DCC0: 38E7FFF4  addi r7, r7, -0xc
	ctx.r[7].s64 = ctx.r[7].s64 + -12;
	// 8242DCC4: 3884000C  addi r4, r4, 0xc
	ctx.r[4].s64 = ctx.r[4].s64 + 12;
	// 8242DCC8: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242DCCC: 4198FFD8  blt cr6, 0x8242dca4
	if ctx.cr[6].lt {
	pc = 0x8242DCA4; continue 'dispatch;
	}
	// 8242DCD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242DCD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242DCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242DCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242DCE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242DCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242DCE8 size=280
    let mut pc: u32 = 0x8242DCE8;
    'dispatch: loop {
        match pc {
            0x8242DCE8 => {
    //   block [0x8242DCE8..0x8242DE00)
	// 8242DCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242DCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242DCF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242DCF4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242DCF8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8242DCFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242DD00: 419A00E0  beq cr6, 0x8242dde0
	if ctx.cr[6].eq {
	pc = 0x8242DDE0; continue 'dispatch;
	}
	// 8242DD04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242DD08: 419A00D8  beq cr6, 0x8242dde0
	if ctx.cr[6].eq {
	pc = 0x8242DDE0; continue 'dispatch;
	}
	// 8242DD0C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8242DD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242DD14: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8242DD18: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8242DD1C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8242DD20: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8242DD24: F94B0010  std r10, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8242DD28: F94B0018  std r10, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 8242DD2C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8242DD30: F9490000  std r10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8242DD34: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8242DD38: 4200FFF8  bdnz 0x8242dd30
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242DD30; continue 'dispatch;
	}
	// 8242DD3C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8242DD40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242DD44: 4BFFFE1D  bl 0x8242db60
	ctx.lr = 0x8242DD48;
	sub_8242DB60(ctx, base);
	// 8242DD48: 2F03FFFC  cmpwi cr6, r3, -4
	ctx.cr[6].compare_i32(ctx.r[3].s32, -4, &mut ctx.xer);
	// 8242DD4C: 409A0010  bne cr6, 0x8242dd5c
	if !ctx.cr[6].eq {
	pc = 0x8242DD5C; continue 'dispatch;
	}
	// 8242DD50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242DD54: 386B34C8  addi r3, r11, 0x34c8
	ctx.r[3].s64 = ctx.r[11].s64 + 13512;
	// 8242DD58: 48000090  b 0x8242dde8
	pc = 0x8242DDE8; continue 'dispatch;
	// 8242DD5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8242DD60: 40980010  bge cr6, 0x8242dd70
	if !ctx.cr[6].lt {
	pc = 0x8242DD70; continue 'dispatch;
	}
	// 8242DD64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242DD68: 386B3490  addi r3, r11, 0x3490
	ctx.r[3].s64 = ctx.r[11].s64 + 13456;
	// 8242DD6C: 4800007C  b 0x8242dde8
	pc = 0x8242DDE8; continue 'dispatch;
	// 8242DD70: 896100BC  lbz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 8242DD74: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8242DD78: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8242DD7C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8242DD80: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242DD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242DD88: 396BFFFA  addi r11, r11, -6
	ctx.r[11].s64 = ctx.r[11].s64 + -6;
	// 8242DD8C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8242DD90: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8242DD94: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8242DD98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242DD9C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8242DDA0: 89610083  lbz r11, 0x83(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(131 as u32) ) } as u64;
	// 8242DDA4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242DDA8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8242DDAC: 89610082  lbz r11, 0x82(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(130 as u32) ) } as u64;
	// 8242DDB0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242DDB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8242DDB8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8242DDBC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8242DDC0: A961008E  lha r11, 0x8e(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(142 as u32) ) } as i16) as i64;
	// 8242DDC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8242DDC8: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242DDCC: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8242DDD0: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8242DDD4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8242DDD8: 48106D79  bl 0x82534b50
	ctx.lr = 0x8242DDDC;
	sub_82534B50(ctx, base);
	// 8242DDDC: 48000010  b 0x8242ddec
	pc = 0x8242DDEC; continue 'dispatch;
	// 8242DDE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242DDE4: 386B345C  addi r3, r11, 0x345c
	ctx.r[3].s64 = ctx.r[11].s64 + 13404;
	// 8242DDE8: 4BFF34B1  bl 0x82421298
	ctx.lr = 0x8242DDEC;
	sub_82421298(ctx, base);
	// 8242DDEC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8242DDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242DDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242DDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242DDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8242DE00 size=332
    let mut pc: u32 = 0x8242DE00;
    'dispatch: loop {
        match pc {
            0x8242DE00 => {
    //   block [0x8242DE00..0x8242DF4C)
	// 8242DE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242DE04: 481072B9  bl 0x825350bc
	ctx.lr = 0x8242DE08;
	sub_82535080(ctx, base);
	// 8242DE08: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242DE0C: 814B9B10  lwz r10, -0x64f0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25840 as u32) ) } as u64;
	// 8242DE10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242DE14: 409A000C  bne cr6, 0x8242de20
	if !ctx.cr[6].eq {
	pc = 0x8242DE20; continue 'dispatch;
	}
	// 8242DE18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8242DE1C: 914B9B10  stw r10, -0x64f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25840 as u32), ctx.r[10].u32 ) };
	// 8242DE20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242DE24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242DE28: B3C50000  sth r30, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8242DE2C: B3C60000  sth r30, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8242DE30: B3C70000  sth r30, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8242DE34: 409A000C  bne cr6, 0x8242de40
	if !ctx.cr[6].eq {
	pc = 0x8242DE40; continue 'dispatch;
	}
	// 8242DE38: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242DE3C: 40990108  ble cr6, 0x8242df44
	if !ctx.cr[6].gt {
	pc = 0x8242DF44; continue 'dispatch;
	}
	// 8242DE40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242DE44: 3BE04A1D  li r31, 0x4a1d
	ctx.r[31].s64 = 18973;
	// 8242DE48: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8242DE4C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242DE50: 396B3538  addi r11, r11, 0x3538
	ctx.r[11].s64 = ctx.r[11].s64 + 13624;
	// 8242DE54: 40990044  ble cr6, 0x8242de98
	if !ctx.cr[6].gt {
	pc = 0x8242DE98; continue 'dispatch;
	}
	// 8242DE58: 7D2A18AE  lbzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242DE5C: 390B0100  addi r8, r11, 0x100
	ctx.r[8].s64 = ctx.r[11].s64 + 256;
	// 8242DE60: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 8242DE64: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8242DE68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242DE6C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242DE70: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242DE74: 7D2942AE  lhax r9, r9, r8
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as i16) as i64;
	// 8242DE78: 7D29F9D6  mullw r9, r9, r31
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[31].s32 as i64);
	// 8242DE7C: 7D285670  srawi r8, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 8242DE80: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 8242DE84: 5508502A  slwi r8, r8, 0xa
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8242DE88: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8242DE8C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242DE90: 7FE95A2E  lhzx r31, r9, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242DE94: 4198FFC4  blt cr6, 0x8242de58
	if ctx.cr[6].lt {
	pc = 0x8242DE58; continue 'dispatch;
	}
	// 8242DE98: 390053FF  li r8, 0x53ff
	ctx.r[8].s64 = 21503;
	// 8242DE9C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8242DEA0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242DEA4: 40990044  ble cr6, 0x8242dee8
	if !ctx.cr[6].gt {
	pc = 0x8242DEE8; continue 'dispatch;
	}
	// 8242DEA8: 7D2A18AE  lbzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242DEAC: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 8242DEB0: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 8242DEB4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8242DEB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242DEBC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242DEC0: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242DEC4: 7D29EAAE  lhax r9, r9, r29
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as i16) as i64;
	// 8242DEC8: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8242DECC: 7D285670  srawi r8, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 8242DED0: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 8242DED4: 5508502A  slwi r8, r8, 0xa
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8242DED8: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8242DEDC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242DEE0: 7D095A2E  lhzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242DEE4: 4198FFC4  blt cr6, 0x8242dea8
	if ctx.cr[6].lt {
	pc = 0x8242DEA8; continue 'dispatch;
	}
	// 8242DEE8: 39205DC1  li r9, 0x5dc1
	ctx.r[9].s64 = 24001;
	// 8242DEEC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8242DEF0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242DEF4: 40990044  ble cr6, 0x8242df38
	if !ctx.cr[6].gt {
	pc = 0x8242DF38; continue 'dispatch;
	}
	// 8242DEF8: 7FCA18AE  lbzx r30, r10, r3
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242DEFC: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 8242DF00: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 8242DF04: 7FDE0774  extsb r30, r30
	ctx.r[30].s64 = ctx.r[30].s8 as i64;
	// 8242DF08: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242DF0C: 57DE083C  slwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8242DF10: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242DF14: 7FDEEAAE  lhax r30, r30, r29
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as i16) as i64;
	// 8242DF18: 7D3E49D6  mullw r9, r30, r9
	ctx.r[9].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8242DF1C: 7D3E5670  srawi r30, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 8242DF20: 7FDE0194  addze r30, r30
	tmp.s64 = ctx.r[30].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[30].u32);
	ctx.r[30].s64 = tmp.s64;
	// 8242DF24: 57DE502A  slwi r30, r30, 0xa
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(10);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8242DF28: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 8242DF2C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242DF30: 7D295A2E  lhzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8242DF34: 4198FFC4  blt cr6, 0x8242def8
	if ctx.cr[6].lt {
	pc = 0x8242DEF8; continue 'dispatch;
	}
	// 8242DF38: B3E50000  sth r31, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 8242DF3C: B1060000  sth r8, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8242DF40: B1270000  sth r9, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8242DF44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242DF48: 481071C4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242DF50 size=12
    let mut pc: u32 = 0x8242DF50;
    'dispatch: loop {
        match pc {
            0x8242DF50 => {
    //   block [0x8242DF50..0x8242DF5C)
	// 8242DF50: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242DF54: 806B9B2C  lwz r3, -0x64d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25812 as u32) ) } as u64;
	// 8242DF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242DF60 size=80
    let mut pc: u32 = 0x8242DF60;
    'dispatch: loop {
        match pc {
            0x8242DF60 => {
    //   block [0x8242DF60..0x8242DFB0)
	// 8242DF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242DF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242DF68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242DF6C: 48001FE5  bl 0x8242ff50
	ctx.lr = 0x8242DF70;
	sub_8242FF50(ctx, base);
	// 8242DF70: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242DF74: 38A01D80  li r5, 0x1d80
	ctx.r[5].s64 = 7552;
	// 8242DF78: 386B4D20  addi r3, r11, 0x4d20
	ctx.r[3].s64 = ctx.r[11].s64 + 19744;
	// 8242DF7C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242DF80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242DF84: 814B9B10  lwz r10, -0x64f0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25840 as u32) ) } as u64;
	// 8242DF88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242DF8C: 914B9B10  stw r10, -0x64f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25840 as u32), ctx.r[10].u32 ) };
	// 8242DF90: 48107241  bl 0x825351d0
	ctx.lr = 0x8242DF94;
	sub_825351D0(ctx, base);
	// 8242DF94: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242DF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242DF9C: 916A9B2C  stw r11, -0x64d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25812 as u32), ctx.r[11].u32 ) };
	// 8242DFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242DFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242DFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242DFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242DFB0 size=48
    let mut pc: u32 = 0x8242DFB0;
    'dispatch: loop {
        match pc {
            0x8242DFB0 => {
    //   block [0x8242DFB0..0x8242DFE0)
	// 8242DFB0: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8242DFB4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242DFB8: 8143008C  lwz r10, 0x8c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8242DFBC: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242DFC0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242DFC4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242DFC8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242DFCC: 81430088  lwz r10, 0x88(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 8242DFD0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242DFD4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242DFD8: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242DFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242DFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242DFE0 size=28
    let mut pc: u32 = 0x8242DFE0;
    'dispatch: loop {
        match pc {
            0x8242DFE0 => {
    //   block [0x8242DFE0..0x8242DFFC)
	// 8242DFE0: 8143008C  lwz r10, 0x8c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8242DFE4: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 8242DFE8: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 8242DFEC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8242DFF0: 9143008C  stw r10, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8242DFF4: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8242DFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E000 size=92
    let mut pc: u32 = 0x8242E000;
    'dispatch: loop {
        match pc {
            0x8242E000 => {
    //   block [0x8242E000..0x8242E05C)
	// 8242E000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E008: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242E00C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E018: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242E01C: 419A0028  beq cr6, 0x8242e044
	if ctx.cr[6].eq {
	pc = 0x8242E044; continue 'dispatch;
	}
	// 8242E020: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242E024: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E028: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8242E02C: 4800207D  bl 0x824300a8
	ctx.lr = 0x8242E030;
	sub_824300A8(ctx, base);
	// 8242E030: 38A000EC  li r5, 0xec
	ctx.r[5].s64 = 236;
	// 8242E034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242E038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E03C: 48107195  bl 0x825351d0
	ctx.lr = 0x8242E040;
	sub_825351D0(ctx, base);
	// 8242E040: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8242E044: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242E048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E050: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242E054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E060 size=164
    let mut pc: u32 = 0x8242E060;
    'dispatch: loop {
        match pc {
            0x8242E060 => {
    //   block [0x8242E060..0x8242E104)
	// 8242E060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E064: 48107059  bl 0x825350bc
	ctx.lr = 0x8242E068;
	sub_82535080(ctx, base);
	// 8242E068: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8242E06C: A0E3009A  lhz r7, 0x9a(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(154 as u32) ) } as u64;
	// 8242E070: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242E074: 80C3003C  lwz r6, 0x3c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242E078: 6145BB80  ori r5, r10, 0xbb80
	ctx.r[5].u64 = ctx.r[10].u64 | 48000;
	// 8242E07C: 80830040  lwz r4, 0x40(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242E080: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8242E084: 83E30044  lwz r31, 0x44(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242E088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8242E08C: 615DFFFF  ori r29, r10, 0xffff
	ctx.r[29].u64 = ctx.r[10].u64 | 65535;
	// 8242E090: B0E30098  sth r7, 0x98(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[7].u16 ) };
	// 8242E094: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8242E098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242E09C: 3920007F  li r9, 0x7f
	ctx.r[9].s64 = 127;
	// 8242E0A0: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 8242E0A4: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 8242E0A8: 90C3005C  stw r6, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8242E0AC: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 8242E0B0: 9903000E  stb r8, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[8].u8 ) };
	// 8242E0B4: 93A30018  stw r29, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8242E0B8: 91030050  stw r8, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8242E0BC: 9923000F  stb r9, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[9].u8 ) };
	// 8242E0C0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8242E0C4: 9BC3000D  stb r30, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[30].u8 ) };
	// 8242E0C8: 91230054  stw r9, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8242E0CC: 91430058  stw r10, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8242E0D0: 90830060  stw r4, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 8242E0D4: 93E30064  stw r31, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8242E0D8: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8242E0DC: B163001C  sth r11, 0x1c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 8242E0E0: B1630024  sth r11, 0x24(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 8242E0E4: B1630026  sth r11, 0x26(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 8242E0E8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8242E0EC: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8242E0F0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8242E0F4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8242E0F8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8242E0FC: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8242E100: 4810700C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E108 size=12
    let mut pc: u32 = 0x8242E108;
    'dispatch: loop {
        match pc {
            0x8242E108 => {
    //   block [0x8242E108..0x8242E114)
	// 8242E108: 90830078  stw r4, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 8242E10C: 90A3007C  stw r5, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 8242E110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E118 size=8
    let mut pc: u32 = 0x8242E118;
    'dispatch: loop {
        match pc {
            0x8242E118 => {
    //   block [0x8242E118..0x8242E120)
	// 8242E118: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242E11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E120 size=8
    let mut pc: u32 = 0x8242E120;
    'dispatch: loop {
        match pc {
            0x8242E120 => {
    //   block [0x8242E120..0x8242E128)
	// 8242E120: A8630098  lha r3, 0x98(r3)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 8242E124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E128 size=64
    let mut pc: u32 = 0x8242E128;
    'dispatch: loop {
        match pc {
            0x8242E128 => {
    //   block [0x8242E128..0x8242E168)
	// 8242E128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242E138: 409A0018  bne cr6, 0x8242e150
	if !ctx.cr[6].eq {
	pc = 0x8242E150; continue 'dispatch;
	}
	// 8242E13C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242E140: 386B3D38  addi r3, r11, 0x3d38
	ctx.r[3].s64 = ctx.r[11].s64 + 15672;
	// 8242E144: 4BFF3155  bl 0x82421298
	ctx.lr = 0x8242E148;
	sub_82421298(ctx, base);
	// 8242E148: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242E14C: 4800000C  b 0x8242e158
	pc = 0x8242E158; continue 'dispatch;
	// 8242E150: 8963000E  lbz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242E154: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8242E158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E168 size=12
    let mut pc: u32 = 0x8242E168;
    'dispatch: loop {
        match pc {
            0x8242E168 => {
    //   block [0x8242E168..0x8242E174)
	// 8242E168: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8242E16C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8242E170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E178 size=40
    let mut pc: u32 = 0x8242E178;
    'dispatch: loop {
        match pc {
            0x8242E178 => {
    //   block [0x8242E178..0x8242E1A0)
	// 8242E178: A9630098  lha r11, 0x98(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 8242E17C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242E180: 41820048  beq 0x8242e1c8
	if ctx.cr[0].eq {
		sub_8242E1C8(ctx, base);
		return;
	}
	// 8242E184: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242E188: 409A0028  bne cr6, 0x8242e1b0
	if !ctx.cr[6].eq {
		sub_8242E1B0(ctx, base);
		return;
	}
	// 8242E18C: A963009C  lha r11, 0x9c(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as i16) as i64;
	// 8242E190: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242E194: 409A000C  bne cr6, 0x8242e1a0
	if !ctx.cr[6].eq {
		sub_8242E1A0(ctx, base);
		return;
	}
	// 8242E198: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8242E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1A0 size=16
    let mut pc: u32 = 0x8242E1A0;
    'dispatch: loop {
        match pc {
            0x8242E1A0 => {
    //   block [0x8242E1A0..0x8242E1B0)
	// 8242E1A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242E1A4: 409A0024  bne cr6, 0x8242e1c8
	if !ctx.cr[6].eq {
		sub_8242E1C8(ctx, base);
		return;
	}
	// 8242E1A8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8242E1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1B0 size=24
    let mut pc: u32 = 0x8242E1B0;
    'dispatch: loop {
        match pc {
            0x8242E1B0 => {
    //   block [0x8242E1B0..0x8242E1C8)
	// 8242E1B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242E1B4: 409A0014  bne cr6, 0x8242e1c8
	if !ctx.cr[6].eq {
		sub_8242E1C8(ctx, base);
		return;
	}
	// 8242E1B8: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8242E1BC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8242E1C0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8242E1C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1C8 size=8
    let mut pc: u32 = 0x8242E1C8;
    'dispatch: loop {
        match pc {
            0x8242E1C8 => {
    //   block [0x8242E1C8..0x8242E1D0)
	// 8242E1C8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8242E1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1D0 size=8
    let mut pc: u32 = 0x8242E1D0;
    'dispatch: loop {
        match pc {
            0x8242E1D0 => {
    //   block [0x8242E1D0..0x8242E1D8)
	// 8242E1D0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242E1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1D8 size=8
    let mut pc: u32 = 0x8242E1D8;
    'dispatch: loop {
        match pc {
            0x8242E1D8 => {
    //   block [0x8242E1D8..0x8242E1E0)
	// 8242E1D8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242E1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1E0 size=8
    let mut pc: u32 = 0x8242E1E0;
    'dispatch: loop {
        match pc {
            0x8242E1E0 => {
    //   block [0x8242E1E0..0x8242E1E8)
	// 8242E1E0: A8630024  lha r3, 0x24(r3)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as i16) as i64;
	// 8242E1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1E8 size=8
    let mut pc: u32 = 0x8242E1E8;
    'dispatch: loop {
        match pc {
            0x8242E1E8 => {
    //   block [0x8242E1E8..0x8242E1F0)
	// 8242E1E8: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242E1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1F0 size=8
    let mut pc: u32 = 0x8242E1F0;
    'dispatch: loop {
        match pc {
            0x8242E1F0 => {
    //   block [0x8242E1F0..0x8242E1F8)
	// 8242E1F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242E1F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E1F8 size=8
    let mut pc: u32 = 0x8242E1F8;
    'dispatch: loop {
        match pc {
            0x8242E1F8 => {
    //   block [0x8242E1F8..0x8242E200)
	// 8242E1F8: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242E1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E200 size=8
    let mut pc: u32 = 0x8242E200;
    'dispatch: loop {
        match pc {
            0x8242E200 => {
    //   block [0x8242E200..0x8242E208)
	// 8242E200: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8242E204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E208 size=8
    let mut pc: u32 = 0x8242E208;
    'dispatch: loop {
        match pc {
            0x8242E208 => {
    //   block [0x8242E208..0x8242E210)
	// 8242E208: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8242E20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E210 size=8
    let mut pc: u32 = 0x8242E210;
    'dispatch: loop {
        match pc {
            0x8242E210 => {
    //   block [0x8242E210..0x8242E218)
	// 8242E210: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 8242E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E218 size=8
    let mut pc: u32 = 0x8242E218;
    'dispatch: loop {
        match pc {
            0x8242E218 => {
    //   block [0x8242E218..0x8242E220)
	// 8242E218: A06300D4  lhz r3, 0xd4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 8242E21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E220 size=16
    let mut pc: u32 = 0x8242E220;
    'dispatch: loop {
        match pc {
            0x8242E220 => {
    //   block [0x8242E220..0x8242E230)
	// 8242E220: 3964006B  addi r11, r4, 0x6b
	ctx.r[11].s64 = ctx.r[4].s64 + 107;
	// 8242E224: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E228: 7C6B1A2E  lhzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8242E22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E230 size=76
    let mut pc: u32 = 0x8242E230;
    'dispatch: loop {
        match pc {
            0x8242E230 => {
    //   block [0x8242E230..0x8242E27C)
	// 8242E230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E23C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E244: 38BF00B0  addi r5, r31, 0xb0
	ctx.r[5].s64 = ctx.r[31].s64 + 176;
	// 8242E248: 389F00AC  addi r4, r31, 0xac
	ctx.r[4].s64 = ctx.r[31].s64 + 172;
	// 8242E24C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E250: 48001E01  bl 0x82430050
	ctx.lr = 0x8242E254;
	sub_82430050(ctx, base);
	// 8242E254: 38DF00AA  addi r6, r31, 0xaa
	ctx.r[6].s64 = ctx.r[31].s64 + 170;
	// 8242E258: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E25C: 38BF00A8  addi r5, r31, 0xa8
	ctx.r[5].s64 = ctx.r[31].s64 + 168;
	// 8242E260: 389F00A6  addi r4, r31, 0xa6
	ctx.r[4].s64 = ctx.r[31].s64 + 166;
	// 8242E264: 48001E25  bl 0x82430088
	ctx.lr = 0x8242E268;
	sub_82430088(ctx, base);
	// 8242E268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E280 size=76
    let mut pc: u32 = 0x8242E280;
    'dispatch: loop {
        match pc {
            0x8242E280 => {
    //   block [0x8242E280..0x8242E2CC)
	// 8242E280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E28C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E294: 38BF00B0  addi r5, r31, 0xb0
	ctx.r[5].s64 = ctx.r[31].s64 + 176;
	// 8242E298: 389F00AC  addi r4, r31, 0xac
	ctx.r[4].s64 = ctx.r[31].s64 + 172;
	// 8242E29C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E2A0: 48001D89  bl 0x82430028
	ctx.lr = 0x8242E2A4;
	sub_82430028(ctx, base);
	// 8242E2A4: A0DF00AA  lhz r6, 0xaa(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(170 as u32) ) } as u64;
	// 8242E2A8: A0BF00A8  lhz r5, 0xa8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8242E2AC: A09F00A6  lhz r4, 0xa6(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(166 as u32) ) } as u64;
	// 8242E2B0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E2B4: 48001DC5  bl 0x82430078
	ctx.lr = 0x8242E2B8;
	sub_82430078(ctx, base);
	// 8242E2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E2D0 size=120
    let mut pc: u32 = 0x8242E2D0;
    'dispatch: loop {
        match pc {
            0x8242E2D0 => {
    //   block [0x8242E2D0..0x8242E348)
	// 8242E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E2DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E2E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8242E2E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242E2EC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242E2F0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E2F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242E2F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242E2FC: 409AFFF4  bne cr6, 0x8242e2f0
	if !ctx.cr[6].eq {
	pc = 0x8242E2F0; continue 'dispatch;
	}
	// 8242E300: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242E304: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8242E308: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242E30C: 38C10052  addi r6, r1, 0x52
	ctx.r[6].s64 = ctx.r[1].s64 + 82;
	// 8242E310: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8242E314: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242E318: 4BFFFAE9  bl 0x8242de00
	ctx.lr = 0x8242E31C;
	sub_8242DE00(ctx, base);
	// 8242E31C: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242E320: A1410052  lhz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8242E324: A1210054  lhz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242E328: B17F00A0  sth r11, 0xa0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u16 ) };
	// 8242E32C: B15F00A2  sth r10, 0xa2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(162 as u32), ctx.r[10].u16 ) };
	// 8242E330: B13F00A4  sth r9, 0xa4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[9].u16 ) };
	// 8242E334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242E338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E348 size=224
    let mut pc: u32 = 0x8242E348;
    'dispatch: loop {
        match pc {
            0x8242E348 => {
    //   block [0x8242E348..0x8242E428)
	// 8242E348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E34C: 48106D71  bl 0x825350bc
	ctx.lr = 0x8242E350;
	sub_82535080(ctx, base);
	// 8242E350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E354: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8242E358: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8242E35C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8242E360: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 8242E364: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8242E368: 419800A4  blt cr6, 0x8242e40c
	if ctx.cr[6].lt {
	pc = 0x8242E40C; continue 'dispatch;
	}
	// 8242E36C: 54AB063E  clrlwi r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 8242E370: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 8242E374: 41980034  blt cr6, 0x8242e3a8
	if ctx.cr[6].lt {
	pc = 0x8242E3A8; continue 'dispatch;
	}
	// 8242E378: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242E37C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8242E380: 38AB3D60  addi r5, r11, 0x3d60
	ctx.r[5].s64 = ctx.r[11].s64 + 15712;
	// 8242E384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242E388: 4BFF5261  bl 0x824235e8
	ctx.lr = 0x8242E38C;
	sub_824235E8(ctx, base);
	// 8242E38C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8242E390: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8242E394: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8242E398: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8242E39C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242E3A0: 4BFFFA61  bl 0x8242de00
	ctx.lr = 0x8242E3A4;
	sub_8242DE00(ctx, base);
	// 8242E3A4: 48000078  b 0x8242e41c
	pc = 0x8242E41C; continue 'dispatch;
	// 8242E3A8: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8242E3AC: 41980060  blt cr6, 0x8242e40c
	if ctx.cr[6].lt {
	pc = 0x8242E40C; continue 'dispatch;
	}
	// 8242E3B0: A16300A0  lhz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 8242E3B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242E3B8: 409A003C  bne cr6, 0x8242e3f4
	if !ctx.cr[6].eq {
	pc = 0x8242E3F4; continue 'dispatch;
	}
	// 8242E3BC: A16300A2  lhz r11, 0xa2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(162 as u32) ) } as u64;
	// 8242E3C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242E3C4: 409A0030  bne cr6, 0x8242e3f4
	if !ctx.cr[6].eq {
	pc = 0x8242E3F4; continue 'dispatch;
	}
	// 8242E3C8: A16300A4  lhz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8242E3CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242E3D0: 409A0024  bne cr6, 0x8242e3f4
	if !ctx.cr[6].eq {
	pc = 0x8242E3F4; continue 'dispatch;
	}
	// 8242E3D4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242E3D8: 396B9B28  addi r11, r11, -0x64d8
	ctx.r[11].s64 = ctx.r[11].s64 + -25816;
	// 8242E3DC: A14BFFF8  lhz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E3E0: B14300A0  sth r10, 0xa0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 8242E3E4: A14BFFFC  lhz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8242E3E8: B14300A2  sth r10, 0xa2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[10].u16 ) };
	// 8242E3EC: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E3F0: B16300A4  sth r11, 0xa4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u16 ) };
	// 8242E3F4: A16300A0  lhz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 8242E3F8: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E3FC: A16300A2  lhz r11, 0xa2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(162 as u32) ) } as u64;
	// 8242E400: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E404: A16300A4  lhz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8242E408: 48000010  b 0x8242e418
	pc = 0x8242E418; continue 'dispatch;
	// 8242E40C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242E410: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E414: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E418: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E41C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242E420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242E424: 48106CE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E428 size=8
    let mut pc: u32 = 0x8242E428;
    'dispatch: loop {
        match pc {
            0x8242E428 => {
    //   block [0x8242E428..0x8242E430)
	// 8242E428: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242E42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E430 size=28
    let mut pc: u32 = 0x8242E430;
    'dispatch: loop {
        match pc {
            0x8242E430 => {
    //   block [0x8242E430..0x8242E44C)
	// 8242E430: A1630098  lhz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 8242E434: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 8242E438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242E43C: 409A0010  bne cr6, 0x8242e44c
	if !ctx.cr[6].eq {
		sub_8242E44C(ctx, base);
		return;
	}
	// 8242E440: 8963000F  lbz r11, 0xf(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 8242E444: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 8242E448: 48000020  b 0x8242e468
	sub_8242E44C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8242E44C size=64
    let mut pc: u32 = 0x8242E44C;
    'dispatch: loop {
        match pc {
            0x8242E44C => {
    //   block [0x8242E44C..0x8242E48C)
	// 8242E44C: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8242E450: 8923000E  lbz r9, 0xe(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242E454: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242E458: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8242E45C: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 8242E460: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8242E464: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8242E468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242E46C: 7D4553D6  divw r10, r5, r10
	ctx.r[10].s32 = ctx.r[5].s32 / ctx.r[10].s32;
	// 8242E470: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8242E474: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8242E478: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8242E47C: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8242E480: 916300E0  stw r11, 0xe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8242E484: 916300DC  stw r11, 0xdc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8242E488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E490 size=12
    let mut pc: u32 = 0x8242E490;
    'dispatch: loop {
        match pc {
            0x8242E490 => {
    //   block [0x8242E490..0x8242E49C)
	// 8242E490: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242E494: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242E498: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E49C size=12
    let mut pc: u32 = 0x8242E49C;
    'dispatch: loop {
        match pc {
            0x8242E49C => {
    //   block [0x8242E49C..0x8242E4A8)
	// 8242E49C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242E4A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242E4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E4A8 size=56
    let mut pc: u32 = 0x8242E4A8;
    'dispatch: loop {
        match pc {
            0x8242E4A8 => {
    //   block [0x8242E4A8..0x8242E4E0)
	// 8242E4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E4B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E4B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E4BC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E4C0: 48001CA1  bl 0x82430160
	ctx.lr = 0x8242E4C4;
	sub_82430160(ctx, base);
	// 8242E4C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242E4C8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242E4CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E4D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E4E0 size=72
    let mut pc: u32 = 0x8242E4E0;
    'dispatch: loop {
        match pc {
            0x8242E4E0 => {
    //   block [0x8242E4E0..0x8242E528)
	// 8242E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E4E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E4EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E4F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E4F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242E4F8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8242E4FC: 409A0018  bne cr6, 0x8242e514
	if !ctx.cr[6].eq {
	pc = 0x8242E514; continue 'dispatch;
	}
	// 8242E500: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E504: 48001C75  bl 0x82430178
	ctx.lr = 0x8242E508;
	sub_82430178(ctx, base);
	// 8242E508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242E50C: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8242E510: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242E514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E528 size=8
    let mut pc: u32 = 0x8242E528;
    'dispatch: loop {
        match pc {
            0x8242E528 => {
    //   block [0x8242E528..0x8242E530)
	// 8242E528: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242E52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E530 size=8
    let mut pc: u32 = 0x8242E530;
    'dispatch: loop {
        match pc {
            0x8242E530 => {
    //   block [0x8242E530..0x8242E538)
	// 8242E530: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242E534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E538 size=88
    let mut pc: u32 = 0x8242E538;
    'dispatch: loop {
        match pc {
            0x8242E538 => {
    //   block [0x8242E538..0x8242E590)
	// 8242E538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E548: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242E54C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8242E550: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8242E554: 812B0068  lwz r9, 0x68(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8242E558: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E55C: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8242E560: 808B0048  lwz r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8242E564: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E56C: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8242E570: 48001B61  bl 0x824300d0
	ctx.lr = 0x8242E574;
	sub_824300D0(ctx, base);
	// 8242E574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E578: 48001BC9  bl 0x82430140
	ctx.lr = 0x8242E57C;
	sub_82430140(ctx, base);
	// 8242E57C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E590 size=96
    let mut pc: u32 = 0x8242E590;
    'dispatch: loop {
        match pc {
            0x8242E590 => {
    //   block [0x8242E590..0x8242E5F0)
	// 8242E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E59C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E5A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242E5A4: 5485083C  slwi r5, r4, 1
	ctx.r[5].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8242E5A8: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8242E5AC: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E5B0: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8242E5B4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242E5B8: 808B0048  lwz r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8242E5BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E5C0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8242E5C4: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8242E5C8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E5CC: 7CEB3214  add r7, r11, r6
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8242E5D0: 48001B39  bl 0x82430108
	ctx.lr = 0x8242E5D4;
	sub_82430108(ctx, base);
	// 8242E5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E5D8: 48001B69  bl 0x82430140
	ctx.lr = 0x8242E5DC;
	sub_82430140(ctx, base);
	// 8242E5DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242E5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E5E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E5F0 size=204
    let mut pc: u32 = 0x8242E5F0;
    'dispatch: loop {
        match pc {
            0x8242E5F0 => {
    //   block [0x8242E5F0..0x8242E6BC)
	// 8242E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E5F4: 48106AC9  bl 0x825350bc
	ctx.lr = 0x8242E5F8;
	sub_82535080(ctx, base);
	// 8242E5F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E5FC: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 8242E600: 80C30070  lwz r6, 0x70(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 8242E604: 81230068  lwz r9, 0x68(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 8242E608: 7D465A14  add r10, r6, r11
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 8242E60C: 81030060  lwz r8, 0x60(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8242E610: 83A30050  lwz r29, 0x50(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242E614: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242E618: 8083004C  lwz r4, 0x4c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242E61C: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 8242E620: 80A3006C  lwz r5, 0x6c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8242E624: 7FCA5BD6  divw r30, r10, r11
	ctx.r[30].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8242E628: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8242E62C: 7FDE59D6  mullw r30, r30, r11
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242E630: 7FEA5BD6  divw r31, r10, r11
	ctx.r[31].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8242E634: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 8242E638: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8242E63C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242E640: 7CE75BD6  divw r7, r7, r11
	ctx.r[7].s32 = ctx.r[7].s32 / ctx.r[11].s32;
	// 8242E644: 7C84EBD6  divw r4, r4, r29
	ctx.r[4].s32 = ctx.r[4].s32 / ctx.r[29].s32;
	// 8242E648: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242E64C: 7F1F3800  cmpw cr6, r31, r7
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8242E650: 4098001C  bge cr6, 0x8242e66c
	if !ctx.cr[6].lt {
	pc = 0x8242E66C; continue 'dispatch;
	}
	// 8242E654: 7FC759D6  mullw r30, r7, r11
	ctx.r[30].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242E658: 7FCAF050  subf r30, r10, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 8242E65C: 7D3E4A14  add r9, r30, r9
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 8242E660: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242E664: 40980008  bge cr6, 0x8242e66c
	if !ctx.cr[6].lt {
	pc = 0x8242E66C; continue 'dispatch;
	}
	// 8242E668: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8242E66C: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8242E670: 40980008  bge cr6, 0x8242e678
	if !ctx.cr[6].lt {
	pc = 0x8242E678; continue 'dispatch;
	}
	// 8242E674: 7CAA2A14  add r5, r10, r5
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 8242E678: 7D655BD6  divw r11, r5, r11
	ctx.r[11].s32 = ctx.r[5].s32 / ctx.r[11].s32;
	// 8242E67C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242E680: 40990008  ble cr6, 0x8242e688
	if !ctx.cr[6].gt {
	pc = 0x8242E688; continue 'dispatch;
	}
	// 8242E684: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8242E688: 7F04F800  cmpw cr6, r4, r31
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8242E68C: 40990008  ble cr6, 0x8242e694
	if !ctx.cr[6].gt {
	pc = 0x8242E694; continue 'dispatch;
	}
	// 8242E690: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8242E694: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8242E698: 40990008  ble cr6, 0x8242e6a0
	if !ctx.cr[6].gt {
	pc = 0x8242E6A0; continue 'dispatch;
	}
	// 8242E69C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8242E6A0: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8242E6A4: 409A000C  bne cr6, 0x8242e6b0
	if !ctx.cr[6].eq {
	pc = 0x8242E6B0; continue 'dispatch;
	}
	// 8242E6A8: 4BFFFEE9  bl 0x8242e590
	ctx.lr = 0x8242E6AC;
	sub_8242E590(ctx, base);
	// 8242E6AC: 48000008  b 0x8242e6b4
	pc = 0x8242E6B4; continue 'dispatch;
	// 8242E6B0: 4BFFFE89  bl 0x8242e538
	ctx.lr = 0x8242E6B4;
	sub_8242E538(ctx, base);
	// 8242E6B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242E6B8: 48106A54  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E6C0 size=72
    let mut pc: u32 = 0x8242E6C0;
    'dispatch: loop {
        match pc {
            0x8242E6C0 => {
    //   block [0x8242E6C0..0x8242E708)
	// 8242E6C0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8242E6C4: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E6C8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8242E6CC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8242E6D0: 40990018  ble cr6, 0x8242e6e8
	if !ctx.cr[6].gt {
	pc = 0x8242E6E8; continue 'dispatch;
	}
	// 8242E6D4: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8242E6D8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E6DC: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 8242E6E0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242E6E4: 4200FFF4  bdnz 0x8242e6d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242E6D8; continue 'dispatch;
	}
	// 8242E6E8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8242E6EC: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 8242E6F0: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242E6F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E6F8: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8242E6FC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8242E700: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8242E704: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242E708 size=24
    let mut pc: u32 = 0x8242E708;
    'dispatch: loop {
        match pc {
            0x8242E708 => {
    //   block [0x8242E708..0x8242E720)
	// 8242E708: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8242E70C: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E710: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 8242E714: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242E718: 4200FFF4  bdnz 0x8242e70c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242E70C; continue 'dispatch;
	}
	// 8242E71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E720 size=224
    let mut pc: u32 = 0x8242E720;
    'dispatch: loop {
        match pc {
            0x8242E720 => {
    //   block [0x8242E720..0x8242E800)
	// 8242E720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E724: 48106981  bl 0x825350a4
	ctx.lr = 0x8242E728;
	sub_82535080(ctx, base);
	// 8242E728: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E72C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242E730: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8242E734: 839E0040  lwz r28, 0x40(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242E738: 82FE0044  lwz r23, 0x44(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242E73C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E740: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242E744: 835F000C  lwz r26, 0xc(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242E748: 837F0028  lwz r27, 0x28(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242E74C: 833F0020  lwz r25, 0x20(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242E750: 831F0014  lwz r24, 0x14(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242E754: 4BFFFA7D  bl 0x8242e1d0
	ctx.lr = 0x8242E758;
	sub_8242E1D0(ctx, base);
	// 8242E758: 7D5BEA14  add r10, r27, r29
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 8242E75C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E760: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 8242E764: 7D43E9D6  mullw r10, r3, r29
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8242E768: 7D09EBD6  divw r8, r9, r29
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[29].s32;
	// 8242E76C: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8242E770: 7D6859D6  mullw r11, r8, r11
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242E774: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8242E778: 4199001C  bgt cr6, 0x8242e794
	if ctx.cr[6].gt {
	pc = 0x8242E794; continue 'dispatch;
	}
	// 8242E77C: 7D69EBD6  divw r11, r9, r29
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[29].s32;
	// 8242E780: 7D6BE9D6  mullw r11, r11, r29
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8242E784: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8242E788: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8242E78C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8242E790: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8242E794: 7D23D1D6  mullw r9, r3, r26
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[26].s32 as i64);
	// 8242E798: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8242E79C: 7D6ACA14  add r11, r10, r25
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 8242E7A0: 913E0094  stw r9, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 8242E7A4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8242E7A8: 41980050  blt cr6, 0x8242e7f8
	if ctx.cr[6].lt {
	pc = 0x8242E7F8; continue 'dispatch;
	}
	// 8242E7AC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E7B0: 7CDC5850  subf r6, r28, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8242E7B4: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8242E7B8: 409A0018  bne cr6, 0x8242e7d0
	if !ctx.cr[6].eq {
	pc = 0x8242E7D0; continue 'dispatch;
	}
	// 8242E7BC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8242E7C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242E7C4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8242E7C8: 4BFFFEF9  bl 0x8242e6c0
	ctx.lr = 0x8242E7CC;
	sub_8242E6C0(ctx, base);
	// 8242E7CC: 4800002C  b 0x8242e7f8
	pc = 0x8242E7F8; continue 'dispatch;
	// 8242E7D0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8242E7D4: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242E7D8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8242E7DC: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 8242E7E0: 40990018  ble cr6, 0x8242e7f8
	if !ctx.cr[6].gt {
	pc = 0x8242E7F8; continue 'dispatch;
	}
	// 8242E7E4: 7D4BC050  subf r10, r11, r24
	ctx.r[10].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 8242E7E8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E7EC: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 8242E7F0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242E7F4: 4200FFF4  bdnz 0x8242e7e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8242E7E8; continue 'dispatch;
	}
	// 8242E7F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8242E7FC: 481068F8  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E800 size=208
    let mut pc: u32 = 0x8242E800;
    'dispatch: loop {
        match pc {
            0x8242E800 => {
    //   block [0x8242E800..0x8242E8D0)
	// 8242E800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242E808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242E80C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242E810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E818: 3BDF0048  addi r30, r31, 0x48
	ctx.r[30].s64 = ctx.r[31].s64 + 72;
	// 8242E81C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242E820: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242E824: 409A0040  bne cr6, 0x8242e864
	if !ctx.cr[6].eq {
	pc = 0x8242E864; continue 'dispatch;
	}
	// 8242E828: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E82C: 4800189D  bl 0x824300c8
	ctx.lr = 0x8242E830;
	sub_824300C8(ctx, base);
	// 8242E830: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242E834: 40820030  bne 0x8242e864
	if !ctx.cr[0].eq {
	pc = 0x8242E864; continue 'dispatch;
	}
	// 8242E838: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8242E83C: 38DE0028  addi r6, r30, 0x28
	ctx.r[6].s64 = ctx.r[30].s64 + 40;
	// 8242E840: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8242E844: 38BE0024  addi r5, r30, 0x24
	ctx.r[5].s64 = ctx.r[30].s64 + 36;
	// 8242E848: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 8242E84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242E850: 4E800421  bctrl
	ctx.lr = 0x8242E854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242E854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E858: 4BFFFD99  bl 0x8242e5f0
	ctx.lr = 0x8242E85C;
	sub_8242E5F0(ctx, base);
	// 8242E85C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8242E860: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242E864: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242E868: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242E86C: 409A004C  bne cr6, 0x8242e8b8
	if !ctx.cr[6].eq {
	pc = 0x8242E8B8; continue 'dispatch;
	}
	// 8242E870: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E874: 4800191D  bl 0x82430190
	ctx.lr = 0x8242E878;
	sub_82430190(ctx, base);
	// 8242E878: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E87C: 4800184D  bl 0x824300c8
	ctx.lr = 0x8242E880;
	sub_824300C8(ctx, base);
	// 8242E880: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242E884: 409A0034  bne cr6, 0x8242e8b8
	if !ctx.cr[6].eq {
	pc = 0x8242E8B8; continue 'dispatch;
	}
	// 8242E888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E88C: 4BFFFE95  bl 0x8242e720
	ctx.lr = 0x8242E890;
	sub_8242E720(ctx, base);
	// 8242E890: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242E894: 480018E5  bl 0x82430178
	ctx.lr = 0x8242E898;
	sub_82430178(ctx, base);
	// 8242E898: 80BF0090  lwz r5, 0x90(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242E89C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242E8A0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8242E8A4: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8242E8A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242E8AC: 4E800421  bctrl
	ctx.lr = 0x8242E8B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242E8B0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242E8B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242E8B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242E8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242E8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242E8C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242E8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242E8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E8D0 size=248
    let mut pc: u32 = 0x8242E8D0;
    'dispatch: loop {
        match pc {
            0x8242E8D0 => {
    //   block [0x8242E8D0..0x8242E9C8)
	// 8242E8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E8D4: 481067DD  bl 0x825350b0
	ctx.lr = 0x8242E8D8;
	sub_82535080(ctx, base);
	// 8242E8D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E8DC: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242E8E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242E8E4: 396B4D20  addi r11, r11, 0x4d20
	ctx.r[11].s64 = ctx.r[11].s64 + 19744;
	// 8242E8E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8242E8EC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8242E8F0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8242E8F4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8242E8F8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8242E8FC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8242E900: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242E904: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8242E908: 419A0018  beq cr6, 0x8242e920
	if ctx.cr[6].eq {
	pc = 0x8242E920; continue 'dispatch;
	}
	// 8242E90C: 392900EC  addi r9, r9, 0xec
	ctx.r[9].s64 = ctx.r[9].s64 + 236;
	// 8242E910: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 8242E914: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242E918: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242E91C: 4198FFE4  blt cr6, 0x8242e900
	if ctx.cr[6].lt {
	pc = 0x8242E900; continue 'dispatch;
	}
	// 8242E920: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8242E924: 409A000C  bne cr6, 0x8242e930
	if !ctx.cr[6].eq {
	pc = 0x8242E930; continue 'dispatch;
	}
	// 8242E928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242E92C: 48000094  b 0x8242e9c0
	pc = 0x8242E9C0; continue 'dispatch;
	// 8242E930: 1D4A00EC  mulli r10, r10, 0xec
	ctx.r[10].s64 = ctx.r[10].s64 * 236;
	// 8242E934: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8242E938: 38A000EC  li r5, 0xec
	ctx.r[5].s64 = 236;
	// 8242E93C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242E940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E944: 4810688D  bl 0x825351d0
	ctx.lr = 0x8242E948;
	sub_825351D0(ctx, base);
	// 8242E948: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242E94C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242E950: 48001619  bl 0x8242ff68
	ctx.lr = 0x8242E954;
	sub_8242FF68(ctx, base);
	// 8242E954: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242E958: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8242E95C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242E960: 4082000C  bne 0x8242e96c
	if !ctx.cr[0].eq {
	pc = 0x8242E96C; continue 'dispatch;
	}
	// 8242E964: 4BFFF69D  bl 0x8242e000
	ctx.lr = 0x8242E968;
	sub_8242E000(ctx, base);
	// 8242E968: 4BFFFFC0  b 0x8242e928
	pc = 0x8242E928; continue 'dispatch;
	// 8242E96C: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 8242E970: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8242E974: 3D408243  lis r10, -0x7dbd
	ctx.r[10].s64 = -2109538304;
	// 8242E978: 939F003C  stw r28, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[28].u32 ) };
	// 8242E97C: 392BDFE0  addi r9, r11, -0x2020
	ctx.r[9].s64 = ctx.r[11].s64 + -8224;
	// 8242E980: 937F0040  stw r27, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[27].u32 ) };
	// 8242E984: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 8242E988: 935F0044  stw r26, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[26].u32 ) };
	// 8242E98C: 394ADFB0  addi r10, r10, -0x2050
	ctx.r[10].s64 = ctx.r[10].s64 + -8272;
	// 8242E990: 93FF007C  stw r31, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 8242E994: 93FF0084  stw r31, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 8242E998: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 8242E99C: 913F0080  stw r9, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 8242E9A0: B3DF00D4  sth r30, 0xd4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u16 ) };
	// 8242E9A4: 915F0078  stw r10, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8242E9A8: B17F00D6  sth r11, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[11].u16 ) };
	// 8242E9AC: B17F00D8  sth r11, 0xd8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u16 ) };
	// 8242E9B0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8242E9B4: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 8242E9B8: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 8242E9BC: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 8242E9C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242E9C4: 4810673C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242E9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242E9C8 size=744
    let mut pc: u32 = 0x8242E9C8;
    'dispatch: loop {
        match pc {
            0x8242E9C8 => {
    //   block [0x8242E9C8..0x8242ECB0)
	// 8242E9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242E9CC: 481066D5  bl 0x825350a0
	ctx.lr = 0x8242E9D0;
	sub_82535080(ctx, base);
	// 8242E9D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242E9D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242E9D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242E9DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8242E9E0: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242E9E4: 41820030  beq 0x8242ea14
	if ctx.cr[0].eq {
	pc = 0x8242EA14; continue 'dispatch;
	}
	// 8242E9E8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242E9EC: 816B9B2C  lwz r11, -0x64d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25812 as u32) ) } as u64;
	// 8242E9F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242E9F4: 409A0020  bne cr6, 0x8242ea14
	if !ctx.cr[6].eq {
	pc = 0x8242EA14; continue 'dispatch;
	}
	// 8242E9F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242E9FC: 388B3DCC  addi r4, r11, 0x3dcc
	ctx.r[4].s64 = ctx.r[11].s64 + 15820;
	// 8242EA00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242EA04: 386B3DAC  addi r3, r11, 0x3dac
	ctx.r[3].s64 = ctx.r[11].s64 + 15788;
	// 8242EA08: 4BFF2919  bl 0x82421320
	ctx.lr = 0x8242EA0C;
	sub_82421320(ctx, base);
	// 8242EA0C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242EA10: 48000298  b 0x8242eca8
	pc = 0x8242ECA8; continue 'dispatch;
	// 8242EA14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242EA18: 3ADF0010  addi r22, r31, 0x10
	ctx.r[22].s64 = ctx.r[31].s64 + 16;
	// 8242EA1C: 3B3F0018  addi r25, r31, 0x18
	ctx.r[25].s64 = ctx.r[31].s64 + 24;
	// 8242EA20: 3B5F0014  addi r26, r31, 0x14
	ctx.r[26].s64 = ctx.r[31].s64 + 20;
	// 8242EA24: 3B1F000E  addi r24, r31, 0xe
	ctx.r[24].s64 = ctx.r[31].s64 + 14;
	// 8242EA28: 3AFF000F  addi r23, r31, 0xf
	ctx.r[23].s64 = ctx.r[31].s64 + 15;
	// 8242EA2C: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8242EA30: 3B7F000D  addi r27, r31, 0xd
	ctx.r[27].s64 = ctx.r[31].s64 + 13;
	// 8242EA34: 92C1005C  stw r22, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[22].u32 ) };
	// 8242EA38: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8242EA3C: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 8242EA40: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8242EA44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EA48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EA4C: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 8242EA50: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 8242EA54: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 8242EA58: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8242EA5C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8242EA60: 4BFF9289  bl 0x82427ce8
	ctx.lr = 0x8242EA64;
	sub_82427CE8(ctx, base);
	// 8242EA64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EA68: 4080000C  bge 0x8242ea74
	if !ctx.cr[0].lt {
	pc = 0x8242EA74; continue 'dispatch;
	}
	// 8242EA6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242EA70: 48000238  b 0x8242eca8
	pc = 0x8242ECA8; continue 'dispatch;
	// 8242EA74: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EA78: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242EA7C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8242EA80: 409900E8  ble cr6, 0x8242eb68
	if !ctx.cr[6].gt {
	pc = 0x8242EB68; continue 'dispatch;
	}
	// 8242EA84: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8242EA88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242EA8C: 409A0018  bne cr6, 0x8242eaa4
	if !ctx.cr[6].eq {
	pc = 0x8242EAA4; continue 'dispatch;
	}
	// 8242EA90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242EA94: 388B3D88  addi r4, r11, 0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + 15752;
	// 8242EA98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242EA9C: 386B3D68  addi r3, r11, 0x3d68
	ctx.r[3].s64 = ctx.r[11].s64 + 15720;
	// 8242EAA0: 4BFFFF68  b 0x8242ea08
	pc = 0x8242EA08; continue 'dispatch;
	// 8242EAA4: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EAA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242EAAC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8242EAB0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242EAB4: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 8242EAB8: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8242EABC: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8242EAC0: B3DF001C  sth r30, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u16 ) };
	// 8242EAC4: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242EAC8: 995B0000  stb r10, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8242EACC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8242EAD0: B3DF0024  sth r30, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u16 ) };
	// 8242EAD4: 38A10061  addi r5, r1, 0x61
	ctx.r[5].s64 = ctx.r[1].s64 + 97;
	// 8242EAD8: 91360000  stw r9, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8242EADC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EAE0: B3DF0026  sth r30, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[30].u16 ) };
	// 8242EAE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EAE8: B11F0098  sth r8, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[8].u16 ) };
	// 8242EAEC: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8242EAF0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8242EAF4: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8242EAF8: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8242EAFC: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8242EB00: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 8242EB04: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8242EB08: 4BFF9321  bl 0x82427e28
	ctx.lr = 0x8242EB0C;
	sub_82427E28(ctx, base);
	// 8242EB0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EB10: 4180FF5C  blt 0x8242ea6c
	if ctx.cr[0].lt {
	pc = 0x8242EA6C; continue 'dispatch;
	}
	// 8242EB14: 3921007E  addi r9, r1, 0x7e
	ctx.r[9].s64 = ctx.r[1].s64 + 126;
	// 8242EB18: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EB1C: 3901007C  addi r8, r1, 0x7c
	ctx.r[8].s64 = ctx.r[1].s64 + 124;
	// 8242EB20: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8242EB24: 38E1007A  addi r7, r1, 0x7a
	ctx.r[7].s64 = ctx.r[1].s64 + 122;
	// 8242EB28: 88810061  lbz r4, 0x61(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 8242EB2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EB30: B3C10078  sth r30, 0x78(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u16 ) };
	// 8242EB34: 4BFFF815  bl 0x8242e348
	ctx.lr = 0x8242EB38;
	sub_8242E348(ctx, base);
	// 8242EB38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EB3C: 4180FED0  blt 0x8242ea0c
	if ctx.cr[0].lt {
	pc = 0x8242EA0C; continue 'dispatch;
	}
	// 8242EB40: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242EB44: 814B9B1C  lwz r10, -0x64e4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25828 as u32) ) } as u64;
	// 8242EB48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242EB4C: 419A011C  beq cr6, 0x8242ec68
	if ctx.cr[6].eq {
	pc = 0x8242EC68; continue 'dispatch;
	}
	// 8242EB50: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8242EB54: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8242EB58: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242EB5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242EB60: 4E800421  bctrl
	ctx.lr = 0x8242EB64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242EB64: 48000104  b 0x8242ec68
	pc = 0x8242EC68; continue 'dispatch;
	// 8242EB68: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8242EB6C: 38A10061  addi r5, r1, 0x61
	ctx.r[5].s64 = ctx.r[1].s64 + 97;
	// 8242EB70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EB74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EB78: 4BFF92B1  bl 0x82427e28
	ctx.lr = 0x8242EB7C;
	sub_82427E28(ctx, base);
	// 8242EB7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EB80: 4180FEEC  blt 0x8242ea6c
	if ctx.cr[0].lt {
	pc = 0x8242EA6C; continue 'dispatch;
	}
	// 8242EB84: 39210062  addi r9, r1, 0x62
	ctx.r[9].s64 = ctx.r[1].s64 + 98;
	// 8242EB88: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EB8C: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 8242EB90: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8242EB94: 38E10066  addi r7, r1, 0x66
	ctx.r[7].s64 = ctx.r[1].s64 + 102;
	// 8242EB98: 88810061  lbz r4, 0x61(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 8242EB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EBA0: 4BFFF7A9  bl 0x8242e348
	ctx.lr = 0x8242EBA4;
	sub_8242E348(ctx, base);
	// 8242EBA4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EBA8: 4180FE64  blt 0x8242ea0c
	if ctx.cr[0].lt {
	pc = 0x8242EA0C; continue 'dispatch;
	}
	// 8242EBAC: A0C10062  lhz r6, 0x62(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 8242EBB0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242EBB4: A0A10064  lhz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8242EBB8: A0810066  lhz r4, 0x66(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 8242EBBC: 480014BD  bl 0x82430078
	ctx.lr = 0x8242EBC0;
	sub_82430078(ctx, base);
	// 8242EBC0: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 8242EBC4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EBC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EBCC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8242EBD0: 4BFF9219  bl 0x82427de8
	ctx.lr = 0x8242EBD4;
	sub_82427DE8(ctx, base);
	// 8242EBD4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EBD8: 4180FE94  blt 0x8242ea6c
	if ctx.cr[0].lt {
	pc = 0x8242EA6C; continue 'dispatch;
	}
	// 8242EBDC: 38C1006C  addi r6, r1, 0x6c
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	// 8242EBE0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8242EBE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EBE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EBEC: 4BFF928D  bl 0x82427e78
	ctx.lr = 0x8242EBF0;
	sub_82427E78(ctx, base);
	// 8242EBF0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EBF4: 4180FE78  blt 0x8242ea6c
	if ctx.cr[0].lt {
	pc = 0x8242EA6C; continue 'dispatch;
	}
	// 8242EBF8: A8BE0000  lha r5, 0(r30)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8242EBFC: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EC00: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242EC04: 4800140D  bl 0x82430010
	ctx.lr = 0x8242EC08;
	sub_82430010(ctx, base);
	// 8242EC08: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 8242EC0C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242EC10: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8242EC14: 48001415  bl 0x82430028
	ctx.lr = 0x8242EC18;
	sub_82430028(ctx, base);
	// 8242EC18: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 8242EC1C: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 8242EC20: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 8242EC24: 391F0028  addi r8, r31, 0x28
	ctx.r[8].s64 = ctx.r[31].s64 + 40;
	// 8242EC28: 38FF0026  addi r7, r31, 0x26
	ctx.r[7].s64 = ctx.r[31].s64 + 38;
	// 8242EC2C: 38DF0024  addi r6, r31, 0x24
	ctx.r[6].s64 = ctx.r[31].s64 + 36;
	// 8242EC30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8242EC34: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 8242EC38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EC3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EC40: 4BFF92C1  bl 0x82427f00
	ctx.lr = 0x8242EC44;
	sub_82427F00(ctx, base);
	// 8242EC44: 391F00D6  addi r8, r31, 0xd6
	ctx.r[8].s64 = ctx.r[31].s64 + 214;
	// 8242EC48: 38FF00D4  addi r7, r31, 0xd4
	ctx.r[7].s64 = ctx.r[31].s64 + 212;
	// 8242EC4C: 38DF00C4  addi r6, r31, 0xc4
	ctx.r[6].s64 = ctx.r[31].s64 + 196;
	// 8242EC50: 38BF00C0  addi r5, r31, 0xc0
	ctx.r[5].s64 = ctx.r[31].s64 + 192;
	// 8242EC54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8242EC58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242EC5C: 4BFF9395  bl 0x82427ff0
	ctx.lr = 0x8242EC60;
	sub_82427FF0(ctx, base);
	// 8242EC60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242EC64: B3DF0098  sth r30, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u16 ) };
	// 8242EC68: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EC6C: 89570000  lbz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EC70: 81360000  lwz r9, 0(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EC74: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242EC78: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242EC7C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242EC80: 80FF0040  lwz r7, 0x40(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242EC84: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242EC88: A8610068  lha r3, 0x68(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as i16) as i64;
	// 8242EC8C: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 8242EC90: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8242EC94: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8242EC98: 913F0058  stw r9, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8242EC9C: 911F005C  stw r8, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8242ECA0: 90FF0060  stw r7, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 8242ECA4: 90DF0064  stw r6, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 8242ECA8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8242ECAC: 48106444  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242ECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242ECB0 size=256
    let mut pc: u32 = 0x8242ECB0;
    'dispatch: loop {
        match pc {
            0x8242ECB0 => {
    //   block [0x8242ECB0..0x8242EDB0)
	// 8242ECB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242ECB4: 48106409  bl 0x825350bc
	ctx.lr = 0x8242ECB8;
	sub_82535080(ctx, base);
	// 8242ECB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242ECBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242ECC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242ECC4: 3940FF80  li r10, -0x80
	ctx.r[10].s64 = -128;
	// 8242ECC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242ECCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8242ECD0: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8242ECD4: B17F00D4  sth r11, 0xd4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u16 ) };
	// 8242ECD8: B15F00D6  sth r10, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[10].u16 ) };
	// 8242ECDC: B15F00D8  sth r10, 0xd8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[10].u16 ) };
	// 8242ECE0: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8242ECE4: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8242ECE8: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 8242ECEC: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 8242ECF0: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242ECF4: 895E0001  lbz r10, 1(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242ECF8: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8242ECFC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8242ED00: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8242ED04: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8242ED08: 409A000C  bne cr6, 0x8242ed14
	if !ctx.cr[6].eq {
	pc = 0x8242ED14; continue 'dispatch;
	}
	// 8242ED0C: 4BFFFCBD  bl 0x8242e9c8
	ctx.lr = 0x8242ED10;
	sub_8242E9C8(ctx, base);
	// 8242ED10: 48000098  b 0x8242eda8
	pc = 0x8242EDA8; continue 'dispatch;
	// 8242ED14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242ED18: 48002761  bl 0x82431478
	ctx.lr = 0x8242ED1C;
	sub_82431478(ctx, base);
	// 8242ED1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242ED20: 41820018  beq 0x8242ed38
	if ctx.cr[0].eq {
	pc = 0x8242ED38; continue 'dispatch;
	}
	// 8242ED24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242ED28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242ED2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242ED30: 48002669  bl 0x82431398
	ctx.lr = 0x8242ED34;
	sub_82431398(ctx, base);
	// 8242ED34: 48000074  b 0x8242eda8
	pc = 0x8242EDA8; continue 'dispatch;
	// 8242ED38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242ED3C: 48000D05  bl 0x8242fa40
	ctx.lr = 0x8242ED40;
	sub_8242FA40(ctx, base);
	// 8242ED40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242ED44: 41820018  beq 0x8242ed5c
	if ctx.cr[0].eq {
	pc = 0x8242ED5C; continue 'dispatch;
	}
	// 8242ED48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242ED4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242ED50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242ED54: 4800079D  bl 0x8242f4f0
	ctx.lr = 0x8242ED58;
	sub_8242F4F0(ctx, base);
	// 8242ED58: 48000050  b 0x8242eda8
	pc = 0x8242EDA8; continue 'dispatch;
	// 8242ED5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242ED60: 48001FF1  bl 0x82430d50
	ctx.lr = 0x8242ED64;
	sub_82430D50(ctx, base);
	// 8242ED64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242ED68: 41820018  beq 0x8242ed80
	if ctx.cr[0].eq {
	pc = 0x8242ED80; continue 'dispatch;
	}
	// 8242ED6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242ED70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242ED74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242ED78: 48002149  bl 0x82430ec0
	ctx.lr = 0x8242ED7C;
	sub_82430EC0(ctx, base);
	// 8242ED7C: 4800002C  b 0x8242eda8
	pc = 0x8242EDA8; continue 'dispatch;
	// 8242ED80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242ED84: 48001735  bl 0x824304b8
	ctx.lr = 0x8242ED88;
	sub_824304B8(ctx, base);
	// 8242ED88: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242ED8C: 41820018  beq 0x8242eda4
	if ctx.cr[0].eq {
	pc = 0x8242EDA4; continue 'dispatch;
	}
	// 8242ED90: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242ED94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242ED98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242ED9C: 4800188D  bl 0x82430628
	ctx.lr = 0x8242EDA0;
	sub_82430628(ctx, base);
	// 8242EDA0: 48000008  b 0x8242eda8
	pc = 0x8242EDA8; continue 'dispatch;
	// 8242EDA4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242EDA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242EDAC: 48106360  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242EDB0 size=232
    let mut pc: u32 = 0x8242EDB0;
    'dispatch: loop {
        match pc {
            0x8242EDB0 => {
    //   block [0x8242EDB0..0x8242EE98)
	// 8242EDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242EDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242EDB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242EDBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242EDC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242EDC4: A97F0098  lha r11, 0x98(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 8242EDC8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242EDCC: 4082000C  bne 0x8242edd8
	if !ctx.cr[0].eq {
	pc = 0x8242EDD8; continue 'dispatch;
	}
	// 8242EDD0: 4BFFFA31  bl 0x8242e800
	ctx.lr = 0x8242EDD4;
	sub_8242E800(ctx, base);
	// 8242EDD4: 48000064  b 0x8242ee38
	pc = 0x8242EE38; continue 'dispatch;
	// 8242EDD8: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8242EDDC: 409A0010  bne cr6, 0x8242edec
	if !ctx.cr[6].eq {
	pc = 0x8242EDEC; continue 'dispatch;
	}
	// 8242EDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EDE4: 48000135  bl 0x8242ef18
	ctx.lr = 0x8242EDE8;
	sub_8242EF18(ctx, base);
	// 8242EDE8: 48000050  b 0x8242ee38
	pc = 0x8242EE38; continue 'dispatch;
	// 8242EDEC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242EDF0: 409A0010  bne cr6, 0x8242ee00
	if !ctx.cr[6].eq {
	pc = 0x8242EE00; continue 'dispatch;
	}
	// 8242EDF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EDF8: 480021C1  bl 0x82430fb8
	ctx.lr = 0x8242EDFC;
	sub_82430FB8(ctx, base);
	// 8242EDFC: 4800003C  b 0x8242ee38
	pc = 0x8242EE38; continue 'dispatch;
	// 8242EE00: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8242EE04: 409A0010  bne cr6, 0x8242ee14
	if !ctx.cr[6].eq {
	pc = 0x8242EE14; continue 'dispatch;
	}
	// 8242EE08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EE0C: 48002465  bl 0x82431270
	ctx.lr = 0x8242EE10;
	sub_82431270(ctx, base);
	// 8242EE10: 48000028  b 0x8242ee38
	pc = 0x8242EE38; continue 'dispatch;
	// 8242EE14: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8242EE18: 409A0010  bne cr6, 0x8242ee28
	if !ctx.cr[6].eq {
	pc = 0x8242EE28; continue 'dispatch;
	}
	// 8242EE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EE20: 48001C01  bl 0x82430a20
	ctx.lr = 0x8242EE24;
	sub_82430A20(ctx, base);
	// 8242EE24: 48000014  b 0x8242ee38
	pc = 0x8242EE38; continue 'dispatch;
	// 8242EE28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242EE2C: 409A000C  bne cr6, 0x8242ee38
	if !ctx.cr[6].eq {
	pc = 0x8242EE38; continue 'dispatch;
	}
	// 8242EE30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242EE34: 48000C8D  bl 0x8242fac0
	ctx.lr = 0x8242EE38;
	sub_8242FAC0(ctx, base);
	// 8242EE38: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8242EE3C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242EE40: 41820044  beq 0x8242ee84
	if ctx.cr[0].eq {
	pc = 0x8242EE84; continue 'dispatch;
	}
	// 8242EE44: 813F0094  lwz r9, 0x94(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242EE48: 811F00DC  lwz r8, 0xdc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8242EE4C: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242EE50: 7C884851  subf. r4, r8, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242EE54: 4080000C  bge 0x8242ee60
	if !ctx.cr[0].lt {
	pc = 0x8242EE60; continue 'dispatch;
	}
	// 8242EE58: 3C848000  addis r4, r4, -0x8000
	ctx.r[4].s64 = ctx.r[4].s64 + -2147483648;
	// 8242EE5C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8242EE60: 893F000E  lbz r9, 0xe(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242EE64: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8242EE68: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8242EE6C: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242EE70: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8242EE74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8242EE78: 4E800421  bctrl
	ctx.lr = 0x8242EE7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242EE7C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242EE80: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8242EE84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242EE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242EE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242EE90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EE98 size=12
    let mut pc: u32 = 0x8242EE98;
    'dispatch: loop {
        match pc {
            0x8242EE98 => {
    //   block [0x8242EE98..0x8242EEA4)
	// 8242EE98: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 8242EE9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242EEA0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EEA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EEA4 size=16
    let mut pc: u32 = 0x8242EEA4;
    'dispatch: loop {
        match pc {
            0x8242EEA4 => {
    //   block [0x8242EEA4..0x8242EEB4)
	// 8242EEA4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242EEA8: 816B9B30  lwz r11, -0x64d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25808 as u32) ) } as u64;
	// 8242EEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242EEB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EEB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EEB4 size=4
    let mut pc: u32 = 0x8242EEB4;
    'dispatch: loop {
        match pc {
            0x8242EEB4 => {
    //   block [0x8242EEB4..0x8242EEB8)
	// 8242EEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242EEB8 size=96
    let mut pc: u32 = 0x8242EEB8;
    'dispatch: loop {
        match pc {
            0x8242EEB8 => {
    //   block [0x8242EEB8..0x8242EF18)
	// 8242EEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242EEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242EEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242EEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242EEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242EECC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242EED0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242EED4: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 8242EED8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242EEDC: 41820014  beq 0x8242eef0
	if ctx.cr[0].eq {
	pc = 0x8242EEF0; continue 'dispatch;
	}
	// 8242EEE0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242EEE4: 816B9B34  lwz r11, -0x64cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25804 as u32) ) } as u64;
	// 8242EEE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242EEEC: 4E800421  bctrl
	ctx.lr = 0x8242EEF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242EEF0: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 8242EEF4: 93FE00B8  stw r31, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 8242EEF8: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 8242EEFC: 917E00BC  stw r11, 0xbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8242EF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242EF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242EF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242EF0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242EF10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF18 size=16
    let mut pc: u32 = 0x8242EF18;
    'dispatch: loop {
        match pc {
            0x8242EF18 => {
    //   block [0x8242EF18..0x8242EF28)
	// 8242EF18: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242EF1C: 816B9B3C  lwz r11, -0x64c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25796 as u32) ) } as u64;
	// 8242EF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242EF24: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF28 size=12
    let mut pc: u32 = 0x8242EF28;
    'dispatch: loop {
        match pc {
            0x8242EF28 => {
    //   block [0x8242EF28..0x8242EF34)
	// 8242EF28: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 8242EF2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242EF30: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF34 size=16
    let mut pc: u32 = 0x8242EF34;
    'dispatch: loop {
        match pc {
            0x8242EF34 => {
    //   block [0x8242EF34..0x8242EF44)
	// 8242EF34: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242EF38: 816B9B38  lwz r11, -0x64c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25800 as u32) ) } as u64;
	// 8242EF3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242EF40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF44 size=4
    let mut pc: u32 = 0x8242EF44;
    'dispatch: loop {
        match pc {
            0x8242EF44 => {
    //   block [0x8242EF44..0x8242EF48)
	// 8242EF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF48 size=4
    let mut pc: u32 = 0x8242EF48;
    'dispatch: loop {
        match pc {
            0x8242EF48 => {
    //   block [0x8242EF48..0x8242EF4C)
	// 8242EF48: 4BFF2128  b 0x82421070
	sub_82421070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242EF50 size=4
    let mut pc: u32 = 0x8242EF50;
    'dispatch: loop {
        match pc {
            0x8242EF50 => {
    //   block [0x8242EF50..0x8242EF54)
	// 8242EF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242EF58 size=268
    let mut pc: u32 = 0x8242EF58;
    'dispatch: loop {
        match pc {
            0x8242EF58 => {
    //   block [0x8242EF58..0x8242F064)
	// 8242EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242EF5C: 48106161  bl 0x825350bc
	ctx.lr = 0x8242EF60;
	sub_82535080(ctx, base);
	// 8242EF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242EF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242EF68: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242EF6C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242EF70: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242EF74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242EF78: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242EF7C: 409900E0  ble cr6, 0x8242f05c
	if !ctx.cr[6].gt {
	pc = 0x8242F05C; continue 'dispatch;
	}
	// 8242EF80: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242EF84: 4BFF33ED  bl 0x82422370
	ctx.lr = 0x8242EF88;
	sub_82422370(ctx, base);
	// 8242EF88: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242EF8C: 4BFF3615  bl 0x824225a0
	ctx.lr = 0x8242EF90;
	sub_824225A0(ctx, base);
	// 8242EF90: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242EF94: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8242EF98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242EF9C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242EFA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242EFA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242EFA8: 409AFFF4  bne cr6, 0x8242ef9c
	if !ctx.cr[6].eq {
	pc = 0x8242EF9C; continue 'dispatch;
	}
	// 8242EFAC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242EFB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8242EFB4: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8242EFB8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8242EFBC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242EFC0: 5529003F  rotlwi. r9, r9, 0
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242EFC4: 41820018  beq 0x8242efdc
	if ctx.cr[0].eq {
	pc = 0x8242EFDC; continue 'dispatch;
	}
	// 8242EFC8: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8242EFCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242EFD0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8242EFD4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8242EFD8: 4198FFF0  blt cr6, 0x8242efc8
	if ctx.cr[6].lt {
	pc = 0x8242EFC8; continue 'dispatch;
	}
	// 8242EFDC: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242EFE0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8242EFE4: 419A0014  beq cr6, 0x8242eff8
	if ctx.cr[6].eq {
	pc = 0x8242EFF8; continue 'dispatch;
	}
	// 8242EFE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242EFEC: 386B3DF0  addi r3, r11, 0x3df0
	ctx.r[3].s64 = ctx.r[11].s64 + 15856;
	// 8242EFF0: 4BFFBDB1  bl 0x8242ada0
	ctx.lr = 0x8242EFF4;
	sub_8242ADA0(ctx, base);
	// 8242EFF4: 48000068  b 0x8242f05c
	pc = 0x8242F05C; continue 'dispatch;
	// 8242EFF8: E97E004E  lwa r11, 0x4c(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as i32) as i64;
	// 8242EFFC: 80DE0048  lwz r6, 0x48(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8242F000: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8242F004: 80BE0044  lwz r5, 0x44(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242F008: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F00C: 4BFF31F5  bl 0x82422200
	ctx.lr = 0x8242F010;
	sub_82422200(ctx, base);
	// 8242F010: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242F014: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F018: 4BFF2981  bl 0x82421998
	ctx.lr = 0x8242F01C;
	sub_82421998(ctx, base);
	// 8242F01C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242F020: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8242F024: 93BE0054  stw r29, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8242F028: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242F02C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242F030: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F034: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 8242F038: 4BFF3049  bl 0x82422080
	ctx.lr = 0x8242F03C;
	sub_82422080(ctx, base);
	// 8242F03C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242F040: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F044: 4BFF287D  bl 0x824218c0
	ctx.lr = 0x8242F048;
	sub_824218C0(ctx, base);
	// 8242F048: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F04C: 4BFF321D  bl 0x82422268
	ctx.lr = 0x8242F050;
	sub_82422268(ctx, base);
	// 8242F050: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242F054: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 8242F058: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8242F05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242F060: 481060AC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F068 size=160
    let mut pc: u32 = 0x8242F068;
    'dispatch: loop {
        match pc {
            0x8242F068 => {
    //   block [0x8242F068..0x8242F108)
	// 8242F068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242F070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242F074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242F078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242F080: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F084: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242F088: 40820014  bne 0x8242f09c
	if !ctx.cr[0].eq {
	pc = 0x8242F09C; continue 'dispatch;
	}
	// 8242F08C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242F090: 386B3E30  addi r3, r11, 0x3e30
	ctx.r[3].s64 = ctx.r[11].s64 + 15920;
	// 8242F094: 4BFFBD0D  bl 0x8242ada0
	ctx.lr = 0x8242F098;
	sub_8242ADA0(ctx, base);
	// 8242F098: 48000058  b 0x8242f0f0
	pc = 0x8242F0F0; continue 'dispatch;
	// 8242F09C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F0A0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F0A4: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242F0A8: 4BFF27D9  bl 0x82421880
	ctx.lr = 0x8242F0AC;
	sub_82421880(ctx, base);
	// 8242F0AC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8242F0B0: 419A0034  beq cr6, 0x8242f0e4
	if ctx.cr[6].eq {
	pc = 0x8242F0E4; continue 'dispatch;
	}
	// 8242F0B4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8242F0B8: 419A0018  beq cr6, 0x8242f0d0
	if ctx.cr[6].eq {
	pc = 0x8242F0D0; continue 'dispatch;
	}
	// 8242F0BC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8242F0C0: 409A0030  bne cr6, 0x8242f0f0
	if !ctx.cr[6].eq {
	pc = 0x8242F0F0; continue 'dispatch;
	}
	// 8242F0C4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242F0C8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8242F0CC: 48000024  b 0x8242f0f0
	pc = 0x8242F0F0; continue 'dispatch;
	// 8242F0D0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8242F0D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8242F0D8: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8242F0DC: 915E0050  stw r10, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8242F0E0: 48000010  b 0x8242f0f0
	pc = 0x8242F0F0; continue 'dispatch;
	// 8242F0E4: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F0E8: 4BFF2831  bl 0x82421918
	ctx.lr = 0x8242F0EC;
	sub_82421918(ctx, base);
	// 8242F0EC: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8242F0F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242F0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242F0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242F0FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242F100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242F104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242F108 size=184
    let mut pc: u32 = 0x8242F108;
    'dispatch: loop {
        match pc {
            0x8242F108 => {
    //   block [0x8242F108..0x8242F1C0)
	// 8242F108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F10C: 48105FA9  bl 0x825350b4
	ctx.lr = 0x8242F110;
	sub_82535080(ctx, base);
	// 8242F110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242F118: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242F11C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8242F120: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8242F124: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8242F128: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8242F12C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242F130: 419A0088  beq cr6, 0x8242f1b8
	if ctx.cr[6].eq {
	pc = 0x8242F1B8; continue 'dispatch;
	}
	// 8242F134: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242F138: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242F13C: 409A0020  bne cr6, 0x8242f15c
	if !ctx.cr[6].eq {
	pc = 0x8242F15C; continue 'dispatch;
	}
	// 8242F140: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F144: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F148: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242F14C: 83CB003C  lwz r30, 0x3c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242F150: 83AB0044  lwz r29, 0x44(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242F154: 838B0048  lwz r28, 0x48(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8242F158: 836B004C  lwz r27, 0x4c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8242F15C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F160: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242F164: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242F168: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F16C: 7D492670  srawi r9, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 8242F170: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8242F174: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242F178: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8242F17C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8242F180: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8242F184: 41810010  bgt 0x8242f194
	if ctx.cr[0].gt {
	pc = 0x8242F194; continue 'dispatch;
	}
	// 8242F188: 4BFF84A1  bl 0x82427628
	ctx.lr = 0x8242F18C;
	sub_82427628(ctx, base);
	// 8242F18C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242F190: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8242F194: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242F198: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242F19C: 409A001C  bne cr6, 0x8242f1b8
	if !ctx.cr[6].eq {
	pc = 0x8242F1B8; continue 'dispatch;
	}
	// 8242F1A0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8242F1A4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8242F1A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242F1AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242F1B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242F1B4: 4BFF8525  bl 0x824276d8
	ctx.lr = 0x8242F1B8;
	sub_824276D8(ctx, base);
	// 8242F1B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242F1BC: 48105F48  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F1C0 size=168
    let mut pc: u32 = 0x8242F1C0;
    'dispatch: loop {
        match pc {
            0x8242F1C0 => {
    //   block [0x8242F1C0..0x8242F268)
	// 8242F1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242F1C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242F1CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F1D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242F1D4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F1D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242F1DC: 419A0078  beq cr6, 0x8242f254
	if ctx.cr[6].eq {
	pc = 0x8242F254; continue 'dispatch;
	}
	// 8242F1E0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242F1E4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8242F1E8: 409A006C  bne cr6, 0x8242f254
	if !ctx.cr[6].eq {
	pc = 0x8242F254; continue 'dispatch;
	}
	// 8242F1EC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242F1F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F1F4: 40990060  ble cr6, 0x8242f254
	if !ctx.cr[6].gt {
	pc = 0x8242F254; continue 'dispatch;
	}
	// 8242F1F8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F1FC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F200: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242F204: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242F208: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242F20C: 409A0008  bne cr6, 0x8242f214
	if !ctx.cr[6].eq {
	pc = 0x8242F214; continue 'dispatch;
	}
	// 8242F210: 4BFFFE59  bl 0x8242f068
	ctx.lr = 0x8242F214;
	sub_8242F068(ctx, base);
	// 8242F214: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F218: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F21C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242F220: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242F224: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242F228: 409A000C  bne cr6, 0x8242f234
	if !ctx.cr[6].eq {
	pc = 0x8242F234; continue 'dispatch;
	}
	// 8242F22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242F230: 4BFFFED9  bl 0x8242f108
	ctx.lr = 0x8242F234;
	sub_8242F108(ctx, base);
	// 8242F234: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F238: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F23C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242F240: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242F244: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F248: 409A000C  bne cr6, 0x8242f254
	if !ctx.cr[6].eq {
	pc = 0x8242F254; continue 'dispatch;
	}
	// 8242F24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242F250: 4BFFFD09  bl 0x8242ef58
	ctx.lr = 0x8242F254;
	sub_8242EF58(ctx, base);
	// 8242F254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242F258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242F25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242F260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242F264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8242F268 size=648
    let mut pc: u32 = 0x8242F268;
    'dispatch: loop {
        match pc {
            0x8242F268 => {
    //   block [0x8242F268..0x8242F4F0)
	// 8242F268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F26C: 48105E39  bl 0x825350a4
	ctx.lr = 0x8242F270;
	sub_82535080(ctx, base);
	// 8242F270: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F274: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242F278: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8242F27C: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8242F280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242F284: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8242F288: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8242F28C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8242F290: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8242F294: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 8242F298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242F29C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8242F2A0: 3B8B3F24  addi r28, r11, 0x3f24
	ctx.r[28].s64 = ctx.r[11].s64 + 16164;
	// 8242F2A4: 40990048  ble cr6, 0x8242f2ec
	if !ctx.cr[6].gt {
	pc = 0x8242F2EC; continue 'dispatch;
	}
	// 8242F2A8: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F2AC: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8242F2B0: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 8242F2B4: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 8242F2B8: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F2BC: 88A90000  lbz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F2C0: 7CE53851  subf. r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8242F2C4: 40820014  bne 0x8242f2d8
	if !ctx.cr[0].eq {
	pc = 0x8242F2D8; continue 'dispatch;
	}
	// 8242F2C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242F2CC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8242F2D0: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242F2D4: 409AFFE4  bne cr6, 0x8242f2b8
	if !ctx.cr[6].eq {
	pc = 0x8242F2B8; continue 'dispatch;
	}
	// 8242F2D8: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F2DC: 41820010  beq 0x8242f2ec
	if ctx.cr[0].eq {
	pc = 0x8242F2EC; continue 'dispatch;
	}
	// 8242F2E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242F2E4: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8242F2E8: 4198FFC4  blt cr6, 0x8242f2ac
	if ctx.cr[6].lt {
	pc = 0x8242F2AC; continue 'dispatch;
	}
	// 8242F2EC: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8242F2F0: 419A01F4  beq cr6, 0x8242f4e4
	if ctx.cr[6].eq {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F2F4: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8242F2F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242F2FC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 8242F300: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8242F304: 4810584D  bl 0x82534b50
	ctx.lr = 0x8242F308;
	sub_82534B50(ctx, base);
	// 8242F308: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242F30C: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 8242F310: 556BC23E  srwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F314: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F318: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 8242F31C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8242F320: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242F324: 419901C0  bgt cr6, 0x8242f4e4
	if ctx.cr[6].gt {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F328: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8242F32C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8242F330: 40990048  ble cr6, 0x8242f378
	if !ctx.cr[6].gt {
	pc = 0x8242F378; continue 'dispatch;
	}
	// 8242F334: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F338: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 8242F33C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8242F340: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8242F344: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F348: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F34C: 7CE53851  subf. r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8242F350: 40820014  bne 0x8242f364
	if !ctx.cr[0].eq {
	pc = 0x8242F364; continue 'dispatch;
	}
	// 8242F354: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242F358: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242F35C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F360: 409AFFE4  bne cr6, 0x8242f344
	if !ctx.cr[6].eq {
	pc = 0x8242F344; continue 'dispatch;
	}
	// 8242F364: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F368: 41820010  beq 0x8242f378
	if ctx.cr[0].eq {
	pc = 0x8242F378; continue 'dispatch;
	}
	// 8242F36C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8242F370: 7F08E800  cmpw cr6, r8, r29
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8242F374: 4198FFC4  blt cr6, 0x8242f338
	if ctx.cr[6].lt {
	pc = 0x8242F338; continue 'dispatch;
	}
	// 8242F378: 7F08E800  cmpw cr6, r8, r29
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8242F37C: 419A0168  beq cr6, 0x8242f4e4
	if ctx.cr[6].eq {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F380: A141005C  lhz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8242F384: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 8242F388: A081005E  lhz r4, 0x5e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 8242F38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8242F390: 554AC23E  srwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F394: 5484C23E  srwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shr(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8242F398: 5549063E  clrlwi r9, r10, 0x18
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8242F39C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242F3A0: 886B0007  lbz r3, 7(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 8242F3A4: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 8242F3A8: 8BEB0006  lbz r31, 6(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8242F3AC: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8242F3B0: 8B8B0005  lbz r28, 5(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 8242F3B4: 5147843E  rlwimi r7, r10, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 8242F3B8: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F3BC: 5146801E  rlwimi r6, r10, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 8242F3C0: 39480008  addi r10, r8, 8
	ctx.r[10].s64 = ctx.r[8].s64 + 8;
	// 8242F3C4: 5463403E  rotlwi r3, r3, 8
	ctx.r[3].u64 = ((ctx.r[3].u32).rotate_left(8)) as u64;
	// 8242F3C8: 54E8C43E  rlwinm r8, r7, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 8242F3CC: 7C63FB78  or r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[31].u64;
	// 8242F3D0: 54C7401E  rlwinm r7, r6, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 8242F3D4: A0C10052  lhz r6, 0x52(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8242F3D8: B1590000  sth r10, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8242F3DC: 546A402E  slwi r10, r3, 8
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F3E0: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 8242F3E4: 98B80000  stb r5, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 8242F3E8: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 8242F3EC: 54C6C23E  srwi r6, r6, 8
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8242F3F0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F3F4: 7D3D0774  extsb r29, r9
	ctx.r[29].s64 = ctx.r[9].s8 as i64;
	// 8242F3F8: 91170000  stw r8, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8242F3FC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8242F400: 8101011C  lwz r8, 0x11c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 8242F404: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8242F408: 98DB0000  stb r6, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 8242F40C: 7D4BEBD6  divw r10, r11, r29
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[29].s32;
	// 8242F410: 989E0000  stb r4, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8242F414: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8242F418: 81210114  lwz r9, 0x114(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 8242F41C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8242F420: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8242F424: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F428: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F42C: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8242F430: 409A0014  bne cr6, 0x8242f444
	if !ctx.cr[6].eq {
	pc = 0x8242F444; continue 'dispatch;
	}
	// 8242F434: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8242F438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8242F43C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8242F440: 48000064  b 0x8242f4a4
	pc = 0x8242F4A4; continue 'dispatch;
	// 8242F444: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 8242F448: 409A0010  bne cr6, 0x8242f458
	if !ctx.cr[6].eq {
	pc = 0x8242F458; continue 'dispatch;
	}
	// 8242F44C: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8242F450: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8242F454: 48000050  b 0x8242f4a4
	pc = 0x8242F4A4; continue 'dispatch;
	// 8242F458: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8242F45C: 409A0048  bne cr6, 0x8242f4a4
	if !ctx.cr[6].eq {
	pc = 0x8242F4A4; continue 'dispatch;
	}
	// 8242F460: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F464: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8242F468: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8242F46C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F470: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8242F474: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F478: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8242F47C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8242F480: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8242F484: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8242F488: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F48C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F490: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8242F494: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242F498: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8242F49C: 98DE0000  stb r6, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 8242F4A0: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 8242F4A4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F4A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242F4AC: 419A0038  beq cr6, 0x8242f4e4
	if ctx.cr[6].eq {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F4B0: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F4B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242F4B8: 419A002C  beq cr6, 0x8242f4e4
	if ctx.cr[6].eq {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F4BC: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F4C0: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F4C4: 40810020  ble 0x8242f4e4
	if !ctx.cr[0].gt {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F4C8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242F4CC: 41990018  bgt cr6, 0x8242f4e4
	if ctx.cr[6].gt {
	pc = 0x8242F4E4; continue 'dispatch;
	}
	// 8242F4D0: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F4D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8242F4D8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8242F4DC: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 8242F4E0: 48000008  b 0x8242f4e8
	pc = 0x8242F4E8; continue 'dispatch;
	// 8242F4E4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242F4E8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8242F4EC: 48105C08  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F4F0 size=216
    let mut pc: u32 = 0x8242F4F0;
    'dispatch: loop {
        match pc {
            0x8242F4F0 => {
    //   block [0x8242F4F0..0x8242F5C8)
	// 8242F4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F4F4: 48105BC1  bl 0x825350b4
	ctx.lr = 0x8242F4F8;
	sub_82535080(ctx, base);
	// 8242F4F8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242F500: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8242F504: 393F0018  addi r9, r31, 0x18
	ctx.r[9].s64 = ctx.r[31].s64 + 24;
	// 8242F508: 397F009C  addi r11, r31, 0x9c
	ctx.r[11].s64 = ctx.r[31].s64 + 156;
	// 8242F50C: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 8242F510: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 8242F514: 3BBF000F  addi r29, r31, 0xf
	ctx.r[29].s64 = ctx.r[31].s64 + 15;
	// 8242F518: B37F0002  sth r27, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[27].u16 ) };
	// 8242F51C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8242F520: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8242F524: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8242F528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8242F52C: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 8242F530: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8242F534: 38FF000D  addi r7, r31, 0xd
	ctx.r[7].s64 = ctx.r[31].s64 + 13;
	// 8242F538: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 8242F53C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8242F540: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8242F544: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8242F548: 4BFFFD21  bl 0x8242f268
	ctx.lr = 0x8242F54C;
	sub_8242F268(ctx, base);
	// 8242F54C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F550: 41800070  blt 0x8242f5c0
	if ctx.cr[0].lt {
	pc = 0x8242F5C0; continue 'dispatch;
	}
	// 8242F554: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242F55C: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F560: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F564: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F568: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8242F56C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8242F570: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8242F574: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8242F578: A8610070  lha r3, 0x70(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 8242F57C: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 8242F580: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 8242F584: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 8242F588: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8242F58C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8242F590: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8242F594: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8242F598: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8242F59C: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8242F5A0: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8242F5A4: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 8242F5A8: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8242F5AC: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 8242F5B0: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 8242F5B4: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8242F5B8: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8242F5BC: B37F0098  sth r27, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[27].u16 ) };
	// 8242F5C0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8242F5C4: 48105B40  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F5C8 size=380
    let mut pc: u32 = 0x8242F5C8;
    'dispatch: loop {
        match pc {
            0x8242F5C8 => {
    //   block [0x8242F5C8..0x8242F744)
	// 8242F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F5CC: 48105AF1  bl 0x825350bc
	ctx.lr = 0x8242F5D0;
	sub_82535080(ctx, base);
	// 8242F5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F5D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242F5D8: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8242F5DC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F5E0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F5E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242F5E8: 409A0128  bne cr6, 0x8242f710
	if !ctx.cr[6].eq {
	pc = 0x8242F710; continue 'dispatch;
	}
	// 8242F5EC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242F5F0: 48000AD9  bl 0x824300c8
	ctx.lr = 0x8242F5F4;
	sub_824300C8(ctx, base);
	// 8242F5F4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F5F8: 40820118  bne 0x8242f710
	if !ctx.cr[0].eq {
	pc = 0x8242F710; continue 'dispatch;
	}
	// 8242F5FC: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8242F600: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8242F604: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 8242F608: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8242F60C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 8242F610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242F614: 4E800421  bctrl
	ctx.lr = 0x8242F618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242F618: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F61C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242F620: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242F624: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242F628: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F62C: 40990008  ble cr6, 0x8242f634
	if !ctx.cr[6].gt {
	pc = 0x8242F634; continue 'dispatch;
	}
	// 8242F630: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8242F634: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F638: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F63C: 40990008  ble cr6, 0x8242f644
	if !ctx.cr[6].gt {
	pc = 0x8242F644; continue 'dispatch;
	}
	// 8242F640: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8242F644: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F648: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8242F64C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242F650: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8242F654: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8242F658: 409A0064  bne cr6, 0x8242f6bc
	if !ctx.cr[6].eq {
	pc = 0x8242F6BC; continue 'dispatch;
	}
	// 8242F65C: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242F660: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F664: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8242F668: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F66C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8242F670: 40990080  ble cr6, 0x8242f6f0
	if !ctx.cr[6].gt {
	pc = 0x8242F6F0; continue 'dispatch;
	}
	// 8242F674: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8242F678: 7CE93850  subf r7, r9, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 8242F67C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8242F680: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F684: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8242F688: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F68C: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8242F690: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 8242F694: 7CC74B2E  sthx r6, r7, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 8242F698: A0CA0002  lhz r6, 2(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242F69C: 88AA0002  lbz r5, 2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242F6A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8242F6A4: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8242F6A8: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 8242F6AC: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 8242F6B0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8242F6B4: 4082FFCC  bne 0x8242f680
	if !ctx.cr[0].eq {
	pc = 0x8242F680; continue 'dispatch;
	}
	// 8242F6B8: 48000038  b 0x8242f6f0
	pc = 0x8242F6F0; continue 'dispatch;
	// 8242F6BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F6C0: 40990030  ble cr6, 0x8242f6f0
	if !ctx.cr[6].gt {
	pc = 0x8242F6F0; continue 'dispatch;
	}
	// 8242F6C4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8242F6C8: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 8242F6CC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8242F6D0: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F6D4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242F6D8: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F6DC: 54E7403E  rotlwi r7, r7, 8
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 8242F6E0: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8242F6E4: 7CE8532E  sthx r7, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 8242F6E8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8242F6EC: 4082FFE4  bne 0x8242f6d0
	if !ctx.cr[0].eq {
	pc = 0x8242F6D0; continue 'dispatch;
	}
	// 8242F6F0: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F6F4: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8242F6F8: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8242F6FC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F700: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242F704: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8242F708: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F70C: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8242F710: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F714: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242F718: 409A0024  bne cr6, 0x8242f73c
	if !ctx.cr[6].eq {
	pc = 0x8242F73C; continue 'dispatch;
	}
	// 8242F71C: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242F720: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242F724: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8242F728: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8242F72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242F730: 4E800421  bctrl
	ctx.lr = 0x8242F734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242F734: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242F738: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242F73C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242F740: 481059CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F748 size=376
    let mut pc: u32 = 0x8242F748;
    'dispatch: loop {
        match pc {
            0x8242F748 => {
    //   block [0x8242F748..0x8242F8C0)
	// 8242F748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F74C: 48105971  bl 0x825350bc
	ctx.lr = 0x8242F750;
	sub_82535080(ctx, base);
	// 8242F750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F754: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242F758: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8242F75C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F760: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F764: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242F768: 409A0124  bne cr6, 0x8242f88c
	if !ctx.cr[6].eq {
	pc = 0x8242F88C; continue 'dispatch;
	}
	// 8242F76C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242F770: 48000959  bl 0x824300c8
	ctx.lr = 0x8242F774;
	sub_824300C8(ctx, base);
	// 8242F774: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F778: 40820114  bne 0x8242f88c
	if !ctx.cr[0].eq {
	pc = 0x8242F88C; continue 'dispatch;
	}
	// 8242F77C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8242F780: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8242F784: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 8242F788: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8242F78C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 8242F790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242F794: 4E800421  bctrl
	ctx.lr = 0x8242F798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242F798: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F79C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242F7A0: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242F7A4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8242F7A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F7AC: 40990008  ble cr6, 0x8242f7b4
	if !ctx.cr[6].gt {
	pc = 0x8242F7B4; continue 'dispatch;
	}
	// 8242F7B0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8242F7B4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F7B8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F7BC: 40990008  ble cr6, 0x8242f7c4
	if !ctx.cr[6].gt {
	pc = 0x8242F7C4; continue 'dispatch;
	}
	// 8242F7C0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8242F7C4: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242F7C8: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8242F7CC: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F7D0: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8242F7D4: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8242F7D8: 409A0064  bne cr6, 0x8242f83c
	if !ctx.cr[6].eq {
	pc = 0x8242F83C; continue 'dispatch;
	}
	// 8242F7DC: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242F7E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242F7E4: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8242F7E8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242F7EC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8242F7F0: 40990080  ble cr6, 0x8242f870
	if !ctx.cr[6].gt {
	pc = 0x8242F870; continue 'dispatch;
	}
	// 8242F7F4: 7CDD4850  subf r6, r29, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 8242F7F8: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8242F7FC: 7CFD4050  subf r7, r29, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[29].s64;
	// 8242F800: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242F804: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8242F808: 6129FF80  ori r9, r9, 0xff80
	ctx.r[9].u64 = ctx.r[9].u64 | 65408;
	// 8242F80C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F810: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8242F814: 7CA54A14  add r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 8242F818: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8242F81C: 7CA75B2E  sthx r5, r7, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 8242F820: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242F824: 7CA54A14  add r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 8242F828: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8242F82C: 7CA65B2E  sthx r5, r6, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 8242F830: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8242F834: 4082FFD8  bne 0x8242f80c
	if !ctx.cr[0].eq {
	pc = 0x8242F80C; continue 'dispatch;
	}
	// 8242F838: 48000038  b 0x8242f870
	pc = 0x8242F870; continue 'dispatch;
	// 8242F83C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242F840: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242F844: 4099002C  ble cr6, 0x8242f870
	if !ctx.cr[6].gt {
	pc = 0x8242F870; continue 'dispatch;
	}
	// 8242F848: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8242F84C: 6129FF80  ori r9, r9, 0xff80
	ctx.r[9].u64 = ctx.r[9].u64 | 65408;
	// 8242F850: 7CEBE8AE  lbzx r7, r11, r29
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8242F854: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242F858: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 8242F85C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242F860: 54E7402E  slwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8242F864: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8242F868: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8242F86C: 4198FFE4  blt cr6, 0x8242f850
	if ctx.cr[6].lt {
	pc = 0x8242F850; continue 'dispatch;
	}
	// 8242F870: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F874: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8242F878: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8242F87C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242F880: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8242F884: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8242F888: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8242F88C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F890: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242F894: 409A0024  bne cr6, 0x8242f8b8
	if !ctx.cr[6].eq {
	pc = 0x8242F8B8; continue 'dispatch;
	}
	// 8242F898: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242F89C: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242F8A0: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8242F8A4: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8242F8A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242F8AC: 4E800421  bctrl
	ctx.lr = 0x8242F8B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242F8B0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242F8B4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242F8B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242F8BC: 48105850  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242F8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242F8C0 size=380
    let mut pc: u32 = 0x8242F8C0;
    'dispatch: loop {
        match pc {
            0x8242F8C0 => {
    //   block [0x8242F8C0..0x8242FA3C)
	// 8242F8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242F8C4: 481057F9  bl 0x825350bc
	ctx.lr = 0x8242F8C8;
	sub_82535080(ctx, base);
	// 8242F8C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242F8CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8242F8D0: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8242F8D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F8D8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F8DC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242F8E0: 409A0128  bne cr6, 0x8242fa08
	if !ctx.cr[6].eq {
	pc = 0x8242FA08; continue 'dispatch;
	}
	// 8242F8E4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242F8E8: 480007E1  bl 0x824300c8
	ctx.lr = 0x8242F8EC;
	sub_824300C8(ctx, base);
	// 8242F8EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242F8F0: 40820118  bne 0x8242fa08
	if !ctx.cr[0].eq {
	pc = 0x8242FA08; continue 'dispatch;
	}
	// 8242F8F4: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8242F8F8: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8242F8FC: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 8242F900: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8242F904: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 8242F908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242F90C: 4E800421  bctrl
	ctx.lr = 0x8242F910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242F910: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242F914: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242F918: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242F91C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242F920: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F924: 40990008  ble cr6, 0x8242f92c
	if !ctx.cr[6].gt {
	pc = 0x8242F92C; continue 'dispatch;
	}
	// 8242F928: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8242F92C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242F930: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242F934: 40990008  ble cr6, 0x8242f93c
	if !ctx.cr[6].gt {
	pc = 0x8242F93C; continue 'dispatch;
	}
	// 8242F938: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8242F93C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F940: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8242F944: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8242F948: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8242F94C: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8242F950: 409A0064  bne cr6, 0x8242f9b4
	if !ctx.cr[6].eq {
	pc = 0x8242F9B4; continue 'dispatch;
	}
	// 8242F954: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242F958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F95C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8242F960: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242F964: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8242F968: 40990080  ble cr6, 0x8242f9e8
	if !ctx.cr[6].gt {
	pc = 0x8242F9E8; continue 'dispatch;
	}
	// 8242F96C: 395D0003  addi r10, r29, 3
	ctx.r[10].s64 = ctx.r[29].s64 + 3;
	// 8242F970: 7CE93850  subf r7, r9, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 8242F974: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8242F978: 88CAFFFF  lbz r6, -1(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8242F97C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8242F980: 88AAFFFD  lbz r5, -3(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-3 as u32) ) } as u64;
	// 8242F984: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8242F988: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 8242F98C: 7CC74B2E  sthx r6, r7, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 8242F990: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F994: 88AAFFFE  lbz r5, -2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8242F998: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8242F99C: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8242F9A0: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 8242F9A4: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 8242F9A8: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8242F9AC: 4082FFCC  bne 0x8242f978
	if !ctx.cr[0].eq {
	pc = 0x8242F978; continue 'dispatch;
	}
	// 8242F9B0: 48000038  b 0x8242f9e8
	pc = 0x8242F9E8; continue 'dispatch;
	// 8242F9B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242F9B8: 40990030  ble cr6, 0x8242f9e8
	if !ctx.cr[6].gt {
	pc = 0x8242F9E8; continue 'dispatch;
	}
	// 8242F9BC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8242F9C0: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 8242F9C4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8242F9C8: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242F9CC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242F9D0: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242F9D4: 54E7403E  rotlwi r7, r7, 8
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 8242F9D8: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8242F9DC: 7CE8532E  sthx r7, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 8242F9E0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8242F9E4: 4082FFE4  bne 0x8242f9c8
	if !ctx.cr[0].eq {
	pc = 0x8242F9C8; continue 'dispatch;
	}
	// 8242F9E8: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8242F9EC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8242F9F0: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8242F9F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8242F9F8: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8242F9FC: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8242FA00: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242FA04: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8242FA08: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242FA0C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242FA10: 409A0024  bne cr6, 0x8242fa34
	if !ctx.cr[6].eq {
	pc = 0x8242FA34; continue 'dispatch;
	}
	// 8242FA14: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8242FA18: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8242FA1C: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8242FA20: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8242FA24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242FA28: 4E800421  bctrl
	ctx.lr = 0x8242FA2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242FA2C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242FA30: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242FA34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242FA38: 481056D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FA40 size=116
    let mut pc: u32 = 0x8242FA40;
    'dispatch: loop {
        match pc {
            0x8242FA40 => {
    //   block [0x8242FA40..0x8242FAB4)
	// 8242FA40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242FA44: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8242FA48: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8242FA4C: 394AC79C  addi r10, r10, -0x3864
	ctx.r[10].s64 = ctx.r[10].s64 + -14436;
	// 8242FA50: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FA54: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FA58: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8242FA5C: 40820014  bne 0x8242fa70
	if !ctx.cr[0].eq {
	pc = 0x8242FA70; continue 'dispatch;
	}
	// 8242FA60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FA64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242FA68: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242FA6C: 409AFFE4  bne cr6, 0x8242fa50
	if !ctx.cr[6].eq {
	pc = 0x8242FA50; continue 'dispatch;
	}
	// 8242FA70: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FA74: 40820040  bne 0x8242fab4
	if !ctx.cr[0].eq {
		sub_8242FAB4(ctx, base);
		return;
	}
	// 8242FA78: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8242FA7C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8242FA80: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8242FA84: 394AC794  addi r10, r10, -0x386c
	ctx.r[10].s64 = ctx.r[10].s64 + -14444;
	// 8242FA88: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FA8C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FA90: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8242FA94: 40820014  bne 0x8242faa8
	if !ctx.cr[0].eq {
	pc = 0x8242FAA8; continue 'dispatch;
	}
	// 8242FA98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FA9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242FAA0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242FAA4: 409AFFE4  bne cr6, 0x8242fa88
	if !ctx.cr[6].eq {
	pc = 0x8242FA88; continue 'dispatch;
	}
	// 8242FAA8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FAAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242FAB0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FAB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FAB4 size=8
    let mut pc: u32 = 0x8242FAB4;
    'dispatch: loop {
        match pc {
            0x8242FAB4 => {
    //   block [0x8242FAB4..0x8242FABC)
	// 8242FAB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FAC0 size=16
    let mut pc: u32 = 0x8242FAC0;
    'dispatch: loop {
        match pc {
            0x8242FAC0 => {
    //   block [0x8242FAC0..0x8242FAD0)
	// 8242FAC0: A963009C  lha r11, 0x9c(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as i16) as i64;
	// 8242FAC4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8242FAC8: 409A0008  bne cr6, 0x8242fad0
	if !ctx.cr[6].eq {
		sub_8242FAD0(ctx, base);
		return;
	}
	// 8242FACC: 4BFFFDF4  b 0x8242f8c0
	sub_8242F8C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FAD0 size=12
    let mut pc: u32 = 0x8242FAD0;
    'dispatch: loop {
        match pc {
            0x8242FAD0 => {
    //   block [0x8242FAD0..0x8242FADC)
	// 8242FAD0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242FAD4: 409A0008  bne cr6, 0x8242fadc
	if !ctx.cr[6].eq {
		sub_8242FADC(ctx, base);
		return;
	}
	// 8242FAD8: 4BFFFC70  b 0x8242f748
	sub_8242F748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FADC size=8
    let mut pc: u32 = 0x8242FADC;
    'dispatch: loop {
        match pc {
            0x8242FADC => {
    //   block [0x8242FADC..0x8242FAE4)
	// 8242FADC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242FAE0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FAE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FAE4 size=8
    let mut pc: u32 = 0x8242FAE4;
    'dispatch: loop {
        match pc {
            0x8242FAE4 => {
    //   block [0x8242FAE4..0x8242FAEC)
	// 8242FAE4: 4BFFFAE4  b 0x8242f5c8
	sub_8242F5C8(ctx, base);
	return;
	// 8242FAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


