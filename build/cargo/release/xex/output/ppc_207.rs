pub fn sub_82FFDB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDB40 size=88
    let mut pc: u32 = 0x82FFDB40;
    'dispatch: loop {
        match pc {
            0x82FFDB40 => {
    //   block [0x82FFDB40..0x82FFDB98)
	// 82FFDB40: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDB44: 548B1978  rlwinm r11, r4, 3, 5, 0x1c
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x1FFFFFFFu64;
	// 82FFDB48: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDB4C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFDB50: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFDB54: 40980064  bge cr6, 0x82ffdbb8
	if !ctx.cr[6].lt {
		sub_82FFDBB8(ctx, base);
		return;
	}
	// 82FFDB58: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDB5C: 5489467E  rlwinm r9, r4, 8, 0x19, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82FFDB60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFDB64: 409A0054  bne cr6, 0x82ffdbb8
	if !ctx.cr[6].eq {
		sub_82FFDBB8(ctx, base);
		return;
	}
	// 82FFDB68: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDB6C: 54880000  rlwinm r8, r4, 0, 0, 0
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFDB70: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFDB74: 409A0044  bne cr6, 0x82ffdbb8
	if !ctx.cr[6].eq {
		sub_82FFDBB8(ctx, base);
		return;
	}
	// 82FFDB78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFDB7C: 554A067E  clrlwi r10, r10, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82FFDB80: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82FFDB84: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDB88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFDB8C: 409A000C  bne cr6, 0x82ffdb98
	if !ctx.cr[6].eq {
		sub_82FFDB98(ctx, base);
		return;
	}
	// 82FFDB90: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFDB94: 48000010  b 0x82ffdba4
	sub_82FFDB98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDB98 size=32
    let mut pc: u32 = 0x82FFDB98;
    'dispatch: loop {
        match pc {
            0x82FFDB98 => {
    //   block [0x82FFDB98..0x82FFDBB8)
	// 82FFDB98: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FFDB9C: 7D291E70  srawi r9, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 82FFDBA0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFDBA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFDBA8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFDBAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDBB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFDBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDBB8 size=12
    let mut pc: u32 = 0x82FFDBB8;
    'dispatch: loop {
        match pc {
            0x82FFDBB8 => {
    //   block [0x82FFDBB8..0x82FFDBC4)
	// 82FFDBB8: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 82FFDBBC: 6063100F  ori r3, r3, 0x100f
	ctx.r[3].u64 = ctx.r[3].u64 | 4111;
	// 82FFDBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDBC8 size=72
    let mut pc: u32 = 0x82FFDBC8;
    'dispatch: loop {
        match pc {
            0x82FFDBC8 => {
    //   block [0x82FFDBC8..0x82FFDC10)
	// 82FFDBC8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDBCC: 548A0000  rlwinm r10, r4, 0, 0, 0
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFDBD0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDBD4: 409A003C  bne cr6, 0x82ffdc10
	if !ctx.cr[6].eq {
		sub_82FFDC10(ctx, base);
		return;
	}
	// 82FFDBD8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDBDC: 548B1978  rlwinm r11, r4, 3, 5, 0x1c
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x1FFFFFFFu64;
	// 82FFDBE0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFDBE4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFDBE8: 41980028  blt cr6, 0x82ffdc10
	if ctx.cr[6].lt {
		sub_82FFDC10(ctx, base);
		return;
	}
	// 82FFDBEC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDBF0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFDBF4: 4098001C  bge cr6, 0x82ffdc10
	if !ctx.cr[6].lt {
		sub_82FFDC10(ctx, base);
		return;
	}
	// 82FFDBF8: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDBFC: 5489467E  rlwinm r9, r4, 8, 0x19, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82FFDC00: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFDC04: 409A000C  bne cr6, 0x82ffdc10
	if !ctx.cr[6].eq {
		sub_82FFDC10(ctx, base);
		return;
	}
	// 82FFDC08: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDC10 size=8
    let mut pc: u32 = 0x82FFDC10;
    'dispatch: loop {
        match pc {
            0x82FFDC10 => {
    //   block [0x82FFDC10..0x82FFDC18)
	// 82FFDC10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDC18 size=172
    let mut pc: u32 = 0x82FFDC18;
    'dispatch: loop {
        match pc {
            0x82FFDC18 => {
    //   block [0x82FFDC18..0x82FFDCC4)
	// 82FFDC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDC24: 548B0000  rlwinm r11, r4, 0, 0, 0
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFDC28: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FFDC2C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFDC30: 419A0080  beq cr6, 0x82ffdcb0
	if ctx.cr[6].eq {
	pc = 0x82FFDCB0; continue 'dispatch;
	}
	// 82FFDC34: 548B467E  rlwinm r11, r4, 8, 0x19, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82FFDC38: 216B007F  subfic r11, r11, 0x7f
	ctx.xer.ca = ctx.r[11].u32 <= 127 as u32;
	ctx.r[11].s64 = (127 as i64) - ctx.r[11].s64;
	// 82FFDC3C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82FFDC40: 41990070  bgt cr6, 0x82ffdcb0
	if ctx.cr[6].gt {
	pc = 0x82FFDCB0; continue 'dispatch;
	}
	// 82FFDC44: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFDC48: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDC4C: 3D0000FF  lis r8, 0xff
	ctx.r[8].s64 = 16711680;
	// 82FFDC50: 7D2A2A14  add r9, r10, r5
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82FFDC54: 6108FFFE  ori r8, r8, 0xfffe
	ctx.r[8].u64 = ctx.r[8].u64 | 65534;
	// 82FFDC58: 548A023E  clrlwi r10, r4, 8
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82FFDC5C: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82FFDC60: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDC64: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FFDC68: 40980048  bge cr6, 0x82ffdcb0
	if !ctx.cr[6].lt {
	pc = 0x82FFDCB0; continue 'dispatch;
	}
	// 82FFDC6C: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFDC70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFDC74: 419A0018  beq cr6, 0x82ffdc8c
	if ctx.cr[6].eq {
	pc = 0x82FFDC8C; continue 'dispatch;
	}
	// 82FFDC78: 3929FFFE  addi r9, r9, -2
	ctx.r[9].s64 = ctx.r[9].s64 + -2;
	// 82FFDC7C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFDC80: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDC84: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82FFDC88: 4082FFF0  bne 0x82ffdc78
	if !ctx.cr[0].eq {
	pc = 0x82FFDC78; continue 'dispatch;
	}
	// 82FFDC8C: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDC90: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82FFDC94: 7D245B78  or r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82FFDC98: 4BFFFF31  bl 0x82ffdbc8
	ctx.lr = 0x82FFDC9C;
	sub_82FFDBC8(ctx, base);
	// 82FFDC9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDCA0: 41820010  beq 0x82ffdcb0
	if ctx.cr[0].eq {
	pc = 0x82FFDCB0; continue 'dispatch;
	}
	// 82FFDCA4: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FFDCA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFDCAC: 409A0008  bne cr6, 0x82ffdcb4
	if !ctx.cr[6].eq {
	pc = 0x82FFDCB4; continue 'dispatch;
	}
	// 82FFDCB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDCC8 size=48
    let mut pc: u32 = 0x82FFDCC8;
    'dispatch: loop {
        match pc {
            0x82FFDCC8 => {
    //   block [0x82FFDCC8..0x82FFDCF8)
	// 82FFDCC8: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82FFDCCC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82FFDCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFDCD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDCD8: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFDCDC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FFDCE0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FFDCE4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FFDCE8: 419A0008  beq cr6, 0x82ffdcf0
	if ctx.cr[6].eq {
	pc = 0x82FFDCF0; continue 'dispatch;
	}
	// 82FFDCEC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FFDCF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FFDCF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDCF8 size=72
    let mut pc: u32 = 0x82FFDCF8;
    'dispatch: loop {
        match pc {
            0x82FFDCF8 => {
    //   block [0x82FFDCF8..0x82FFDD40)
	// 82FFDCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDD00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFDD04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDD08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDD0C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFDD10: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDD14: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDD18: 419A0014  beq cr6, 0x82ffdd2c
	if ctx.cr[6].eq {
	pc = 0x82FFDD2C; continue 'dispatch;
	}
	// 82FFDD1C: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82FFDD20: 4BFFE511  bl 0x82ffc230
	ctx.lr = 0x82FFDD24;
	sub_82FFC230(ctx, base);
	// 82FFDD24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFDD28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDD2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDD38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFDD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDD40 size=140
    let mut pc: u32 = 0x82FFDD40;
    'dispatch: loop {
        match pc {
            0x82FFDD40 => {
    //   block [0x82FFDD40..0x82FFDDCC)
	// 82FFDD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDD44: 4BCAB6C9  bl 0x82ca940c
	ctx.lr = 0x82FFDD48;
	sub_82CA93D0(ctx, base);
	// 82FFDD48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDD4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFDD50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDD54: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFDD58: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82FFDD5C: 4BFFE4BD  bl 0x82ffc218
	ctx.lr = 0x82FFDD60;
	sub_82FFC218(ctx, base);
	// 82FFDD60: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFDD64: 4082000C  bne 0x82ffdd70
	if !ctx.cr[0].eq {
	pc = 0x82FFDD70; continue 'dispatch;
	}
	// 82FFDD68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDD6C: 48000058  b 0x82ffddc4
	pc = 0x82FFDDC4; continue 'dispatch;
	// 82FFDD70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDD74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFDD78: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDD7C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFDD80: 4BCAB701  bl 0x82ca9480
	ctx.lr = 0x82FFDD84;
	sub_82CA9480(ctx, base);
	// 82FFDD84: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDD88: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFDD8C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDD90: 419A000C  beq cr6, 0x82ffdd9c
	if ctx.cr[6].eq {
	pc = 0x82FFDD9C; continue 'dispatch;
	}
	// 82FFDD94: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82FFDD98: 4BFFE499  bl 0x82ffc230
	ctx.lr = 0x82FFDD9C;
	sub_82FFC230(ctx, base);
	// 82FFDD9C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDDA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFDDA4: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDDA8: 7D4AE850  subf r10, r10, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[10].s64;
	// 82FFDDAC: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FFDDB0: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFDDB4: 4BCABBFD  bl 0x82ca99b0
	ctx.lr = 0x82FFDDB8;
	sub_82CA99B0(ctx, base);
	// 82FFDDB8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFDDBC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FFDDC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFDDC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFDDC8: 4BCAB694  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDDD0 size=24
    let mut pc: u32 = 0x82FFDDD0;
    'dispatch: loop {
        match pc {
            0x82FFDDD0 => {
    //   block [0x82FFDDD0..0x82FFDDE8)
	// 82FFDDD0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDDD4: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82FFDDD8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDDDC: 4199000C  bgt cr6, 0x82ffdde8
	if ctx.cr[6].gt {
		sub_82FFDDE8(ctx, base);
		return;
	}
	// 82FFDDE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFDDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDDE8 size=4
    let mut pc: u32 = 0x82FFDDE8;
    'dispatch: loop {
        match pc {
            0x82FFDDE8 => {
    //   block [0x82FFDDE8..0x82FFDDEC)
	// 82FFDDE8: 4BFFFF58  b 0x82ffdd40
	sub_82FFDD40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDDF0 size=152
    let mut pc: u32 = 0x82FFDDF0;
    'dispatch: loop {
        match pc {
            0x82FFDDF0 => {
    //   block [0x82FFDDF0..0x82FFDE88)
	// 82FFDDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDDF4: 4BCAB619  bl 0x82ca940c
	ctx.lr = 0x82FFDDF8;
	sub_82CA93D0(ctx, base);
	// 82FFDDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDDFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFDE00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFDE04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FFDE08: 39450001  addi r10, r5, 1
	ctx.r[10].s64 = ctx.r[5].s64 + 1;
	// 82FFDE0C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDE10: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDE14: 41980048  blt cr6, 0x82ffde5c
	if ctx.cr[6].lt {
	pc = 0x82FFDE5C; continue 'dispatch;
	}
	// 82FFDE18: 57EBF0BE  srwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDE1C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FFDE20: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 82FFDE24: 40980008  bge cr6, 0x82ffde2c
	if !ctx.cr[6].lt {
	pc = 0x82FFDE2C; continue 'dispatch;
	}
	// 82FFDE28: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FFDE2C: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFDE30: 4198000C  blt cr6, 0x82ffde3c
	if ctx.cr[6].lt {
	pc = 0x82FFDE3C; continue 'dispatch;
	}
	// 82FFDE34: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFDE38: 40990008  ble cr6, 0x82ffde40
	if !ctx.cr[6].gt {
	pc = 0x82FFDE40; continue 'dispatch;
	}
	// 82FFDE3C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FFDE40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFDE44: 4BFFFEFD  bl 0x82ffdd40
	ctx.lr = 0x82FFDE48;
	sub_82FFDD40(ctx, base);
	// 82FFDE48: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFDE4C: 40820010  bne 0x82ffde5c
	if !ctx.cr[0].eq {
	pc = 0x82FFDE5C; continue 'dispatch;
	}
	// 82FFDE50: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 82FFDE54: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 82FFDE58: 48000028  b 0x82ffde80
	pc = 0x82FFDE80; continue 'dispatch;
	// 82FFDE5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDE60: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFDE64: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDE68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDE6C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFDE70: 556BC00E  slwi r11, r11, 0x18
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(24);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDE74: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82FFDE78: 7D6BFB78  or r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82FFDE7C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDE80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFDE84: 4BCAB5D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDE88 size=40
    let mut pc: u32 = 0x82FFDE88;
    'dispatch: loop {
        match pc {
            0x82FFDE88 => {
    //   block [0x82FFDE88..0x82FFDEB0)
	// 82FFDE88: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDE8C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDE90: 40980020  bge cr6, 0x82ffdeb0
	if !ctx.cr[6].lt {
		sub_82FFDEB0(ctx, base);
		return;
	}
	// 82FFDE94: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDE98: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDE9C: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FFDEA0: 2B09007F  cmplwi cr6, r9, 0x7f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 127 as u32, &mut ctx.xer);
	// 82FFDEA4: 4099000C  ble cr6, 0x82ffdeb0
	if !ctx.cr[6].gt {
		sub_82FFDEB0(ctx, base);
		return;
	}
	// 82FFDEA8: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDEAC: 48000008  b 0x82ffdeb4
	sub_82FFDEB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDEB0 size=28
    let mut pc: u32 = 0x82FFDEB0;
    'dispatch: loop {
        match pc {
            0x82FFDEB0 => {
    //   block [0x82FFDEB0..0x82FFDECC)
	// 82FFDEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFDEB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFDEB8: 419A0014  beq cr6, 0x82ffdecc
	if ctx.cr[6].eq {
		sub_82FFDECC(ctx, base);
		return;
	}
	// 82FFDEBC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDEC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFDEC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFDEC8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDECC size=8
    let mut pc: u32 = 0x82FFDECC;
    'dispatch: loop {
        match pc {
            0x82FFDECC => {
    //   block [0x82FFDECC..0x82FFDED4)
	// 82FFDECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDED8 size=100
    let mut pc: u32 = 0x82FFDED8;
    'dispatch: loop {
        match pc {
            0x82FFDED8 => {
    //   block [0x82FFDED8..0x82FFDF3C)
	// 82FFDED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFDEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFDEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDEEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFDEF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDEF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFDEF8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFDEFC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFDF00: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFDF04: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82FFDF08: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFDF0C: 4BFFE2ED  bl 0x82ffc1f8
	ctx.lr = 0x82FFDF10;
	sub_82FFC1F8(ctx, base);
	// 82FFDF10: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FFDF14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFDF18: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82FFDF1C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FFDF20: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82FFDF24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFDF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDF30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFDF34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFDF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDF40 size=8
    let mut pc: u32 = 0x82FFDF40;
    'dispatch: loop {
        match pc {
            0x82FFDF40 => {
    //   block [0x82FFDF40..0x82FFDF48)
	// 82FFDF40: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FFDF44: 4BB98F34  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDF48 size=160
    let mut pc: u32 = 0x82FFDF48;
    'dispatch: loop {
        match pc {
            0x82FFDF48 => {
    //   block [0x82FFDF48..0x82FFDFE8)
	// 82FFDF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFDF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFDF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDF5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDF60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFDF64: 7D7E2A14  add r11, r30, r5
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[5].u64;
	// 82FFDF68: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFDF6C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FFDF70: 654B4000  oris r11, r10, 0x4000
	ctx.r[11].u64 = ctx.r[10].u64 | 1073741824;
	// 82FFDF74: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFDF78: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFDF7C: 41820018  beq 0x82ffdf94
	if ctx.cr[0].eq {
	pc = 0x82FFDF94; continue 'dispatch;
	}
	// 82FFDF80: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFDF84: 48004E85  bl 0x83002e08
	ctx.lr = 0x82FFDF88;
	sub_83002E08(ctx, base);
	// 82FFDF88: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFDF8C: 41800044  blt 0x82ffdfd0
	if ctx.cr[0].lt {
	pc = 0x82FFDFD0; continue 'dispatch;
	}
	// 82FFDF90: 48000030  b 0x82ffdfc0
	pc = 0x82FFDFC0; continue 'dispatch;
	// 82FFDF94: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82FFDF98: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFDF9C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFDFA0: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 82FFDFA4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82FFDFA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82FFDFAC: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82FFDFB0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FFDFB4: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82FFDFB8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82FFDFBC: 4082FFE8  bne 0x82ffdfa4
	if !ctx.cr[0].eq {
	pc = 0x82FFDFA4; continue 'dispatch;
	}
	// 82FFDFC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FFDFC4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82FFDFC8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFDFCC: 48004D5D  bl 0x83002d28
	ctx.lr = 0x82FFDFD0;
	sub_83002D28(ctx, base);
	// 82FFDFD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFDFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDFDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFDFE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFDFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDFE8 size=96
    let mut pc: u32 = 0x82FFDFE8;
    'dispatch: loop {
        match pc {
            0x82FFDFE8 => {
    //   block [0x82FFDFE8..0x82FFE048)
	// 82FFDFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDFF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFDFF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDFFC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE000: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE004: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFE008: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE00C: 41820028  beq 0x82ffe034
	if ctx.cr[0].eq {
	pc = 0x82FFE034; continue 'dispatch;
	}
	// 82FFE010: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFE014: 48004DF5  bl 0x83002e08
	ctx.lr = 0x82FFE018;
	sub_83002E08(ctx, base);
	// 82FFE018: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFE01C: 41800018  blt 0x82ffe034
	if ctx.cr[0].lt {
	pc = 0x82FFE034; continue 'dispatch;
	}
	// 82FFE020: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE024: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE028: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82FFE02C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE030: 4BFF67E9  bl 0x82ff4818
	ctx.lr = 0x82FFE034;
	sub_82FF4818(ctx, base);
	// 82FFE034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE048 size=196
    let mut pc: u32 = 0x82FFE048;
    'dispatch: loop {
        match pc {
            0x82FFE048 => {
    //   block [0x82FFE048..0x82FFE10C)
	// 82FFE048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE05C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE060: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82FFE064: 482BB901  bl 0x832b9964
	ctx.lr = 0x82FFE068;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82FFE068: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE06C: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82FFE070: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFE074: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE078: 40820014  bne 0x82ffe08c
	if !ctx.cr[0].eq {
	pc = 0x82FFE08C; continue 'dispatch;
	}
	// 82FFE07C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE080: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82FFE084: 482BB8D1  bl 0x832b9954
	ctx.lr = 0x82FFE088;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82FFE088: 48000068  b 0x82ffe0f0
	pc = 0x82FFE0F0; continue 'dispatch;
	// 82FFE08C: 4B268025  bl 0x822660b0
	ctx.lr = 0x82FFE090;
	sub_822660B0(ctx, base);
	// 82FFE090: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FFE094: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FFE098: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82FFE09C: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82FFE0A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FFE0A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFE0A8: 41980030  blt cr6, 0x82ffe0d8
	if ctx.cr[6].lt {
	pc = 0x82FFE0D8; continue 'dispatch;
	}
	// 82FFE0AC: 7CA55850  subf r5, r5, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82FFE0B0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82FFE0B4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFE0B8: 48004C71  bl 0x83002d28
	ctx.lr = 0x82FFE0BC;
	sub_83002D28(ctx, base);
	// 82FFE0BC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE0C0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82FFE0C4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE0C8: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82FFE0CC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE0D0: 482BB885  bl 0x832b9954
	ctx.lr = 0x82FFE0D4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82FFE0D4: 48000024  b 0x82ffe0f8
	pc = 0x82FFE0F8; continue 'dispatch;
	// 82FFE0D8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE0DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE0E0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE0E4: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE0E8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE0EC: 4BFFAA75  bl 0x82ff8b60
	ctx.lr = 0x82FFE0F0;
	sub_82FF8B60(ctx, base);
	// 82FFE0F0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE0F4: 4BFF6725  bl 0x82ff4818
	ctx.lr = 0x82FFE0F8;
	sub_82FF4818(ctx, base);
	// 82FFE0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE110 size=140
    let mut pc: u32 = 0x82FFE110;
    'dispatch: loop {
        match pc {
            0x82FFE110 => {
    //   block [0x82FFE110..0x82FFE19C)
	// 82FFE110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE114: 4BCAB2F9  bl 0x82ca940c
	ctx.lr = 0x82FFE118;
	sub_82CA93D0(ctx, base);
	// 82FFE118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE11C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE120: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FFE124: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE128: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82FFE12C: 3BA00009  li r29, 9
	ctx.r[29].s64 = 9;
	// 82FFE130: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82FFE134: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82FFE138: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82FFE13C: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FFE140: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE144: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82FFE148: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FFE14C: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FFE150: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82FFE154: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82FFE158: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82FFE15C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FFE160: E97E0020  ld r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 82FFE164: F97F0024  std r11, 0x24(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u64 ) };
	// 82FFE168: 4BCAB319  bl 0x82ca9480
	ctx.lr = 0x82FFE16C;
	sub_82CA9480(ctx, base);
	// 82FFE16C: 816100CC  lwz r11, 0xcc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FFE170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFE174: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE178: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82FFE17C: 419A000C  beq cr6, 0x82ffe188
	if ctx.cr[6].eq {
	pc = 0x82FFE188; continue 'dispatch;
	}
	// 82FFE180: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFE184: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82FFE188: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE18C: 556B0104  rlwinm r11, r11, 0, 4, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE190: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE194: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE198: 4BCAB2C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1A0 size=32
    let mut pc: u32 = 0x82FFE1A0;
    'dispatch: loop {
        match pc {
            0x82FFE1A0 => {
    //   block [0x82FFE1A0..0x82FFE1C0)
	// 82FFE1A0: 81430038  lwz r10, 0x38(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFE1A4: 554B0001  rlwinm. r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE1A8: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82FFE1AC: 7F245840  cmpld cr6, r4, r11
	ctx.cr[6].compare_u64(ctx.r[4].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82FFE1B0: 41820024  beq 0x82ffe1d4
	if ctx.cr[0].eq {
		sub_82FFE1D4(ctx, base);
		return;
	}
	// 82FFE1B4: 4198003C  blt cr6, 0x82ffe1f0
	if ctx.cr[6].lt {
		sub_82FFE1F0(ctx, base);
		return;
	}
	// 82FFE1B8: 554B0085  rlwinm. r11, r10, 0, 2, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE1BC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1C0 size=12
    let mut pc: u32 = 0x82FFE1C0;
    'dispatch: loop {
        match pc {
            0x82FFE1C0 => {
    //   block [0x82FFE1C0..0x82FFE1CC)
	// 82FFE1C0: E9630008  ld r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82FFE1C4: 7F245840  cmpld cr6, r4, r11
	ctx.cr[6].compare_u64(ctx.r[4].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82FFE1C8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1CC size=8
    let mut pc: u32 = 0x82FFE1CC;
    'dispatch: loop {
        match pc {
            0x82FFE1CC => {
    //   block [0x82FFE1CC..0x82FFE1D4)
	// 82FFE1CC: 554B00C2  rlwinm r11, r10, 0, 3, 1
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE1D0: 48000058  b 0x82ffe228
	sub_82FFE21C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1D4 size=12
    let mut pc: u32 = 0x82FFE1D4;
    'dispatch: loop {
        match pc {
            0x82FFE1D4 => {
    //   block [0x82FFE1D4..0x82FFE1E0)
	// 82FFE1D4: 4199001C  bgt cr6, 0x82ffe1f0
	if ctx.cr[6].gt {
		sub_82FFE1F0(ctx, base);
		return;
	}
	// 82FFE1D8: 554B0085  rlwinm. r11, r10, 0, 2, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE1DC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1E0 size=12
    let mut pc: u32 = 0x82FFE1E0;
    'dispatch: loop {
        match pc {
            0x82FFE1E0 => {
    //   block [0x82FFE1E0..0x82FFE1EC)
	// 82FFE1E0: E9630008  ld r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82FFE1E4: 7F245840  cmpld cr6, r4, r11
	ctx.cr[6].compare_u64(ctx.r[4].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82FFE1E8: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1EC size=4
    let mut pc: u32 = 0x82FFE1EC;
    'dispatch: loop {
        match pc {
            0x82FFE1EC => {
    //   block [0x82FFE1EC..0x82FFE1F0)
	// 82FFE1EC: 4BFFFFE0  b 0x82ffe1cc
	sub_82FFE1CC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE1F0 size=28
    let mut pc: u32 = 0x82FFE1F0;
    'dispatch: loop {
        match pc {
            0x82FFE1F0 => {
    //   block [0x82FFE1F0..0x82FFE20C)
	// 82FFE1F0: 554B0085  rlwinm. r11, r10, 0, 2, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE1F4: 4082003C  bne 0x82ffe230
	if !ctx.cr[0].eq {
		sub_82FFE230(ctx, base);
		return;
	}
	// 82FFE1F8: 554B0043  rlwinm. r11, r10, 0, 1, 1
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE1FC: 41820020  beq 0x82ffe21c
	if ctx.cr[0].eq {
		sub_82FFE21C(ctx, base);
		return;
	}
	// 82FFE200: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE204: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FFE208: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE20C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE20C size=16
    let mut pc: u32 = 0x82FFE20C;
    'dispatch: loop {
        match pc {
            0x82FFE20C => {
    //   block [0x82FFE20C..0x82FFE21C)
	// 82FFE20C: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE210: 7D292850  subf r9, r9, r5
	ctx.r[9].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82FFE214: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE218: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE21C size=20
    let mut pc: u32 = 0x82FFE21C;
    'dispatch: loop {
        match pc {
            0x82FFE21C => {
    //   block [0x82FFE21C..0x82FFE230)
	// 82FFE21C: 654B7000  oris r11, r10, 0x7000
	ctx.r[11].u64 = ctx.r[10].u64 | 1879048192;
	// 82FFE220: F8830020  std r4, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u64 ) };
	// 82FFE224: 90A30030  stw r5, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82FFE228: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE230 size=12
    let mut pc: u32 = 0x82FFE230;
    'dispatch: loop {
        match pc {
            0x82FFE230 => {
    //   block [0x82FFE230..0x82FFE23C)
	// 82FFE230: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE234: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE238: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE23C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE23C size=4
    let mut pc: u32 = 0x82FFE23C;
    'dispatch: loop {
        match pc {
            0x82FFE23C => {
    //   block [0x82FFE23C..0x82FFE240)
	// 82FFE23C: 4BFFFFC4  b 0x82ffe200
	sub_82FFE1F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE240 size=104
    let mut pc: u32 = 0x82FFE240;
    'dispatch: loop {
        match pc {
            0x82FFE240 => {
    //   block [0x82FFE240..0x82FFE2A8)
	// 82FFE240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE244: 4BCAB1C5  bl 0x82ca9408
	ctx.lr = 0x82FFE248;
	sub_82CA93D0(ctx, base);
	// 82FFE248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE24C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FFE250: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82FFE254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE258: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FFE25C: 4BCAB225  bl 0x82ca9480
	ctx.lr = 0x82FFE260;
	sub_82CA9480(ctx, base);
	// 82FFE260: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFE268: 409A000C  bne cr6, 0x82ffe274
	if !ctx.cr[6].eq {
	pc = 0x82FFE274; continue 'dispatch;
	}
	// 82FFE26C: 39601388  li r11, 0x1388
	ctx.r[11].s64 = 5000;
	// 82FFE270: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFE274: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFE278: FBBF0028  std r29, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u64 ) };
	// 82FFE27C: FBDF0020  std r30, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u64 ) };
	// 82FFE280: 4B267E31  bl 0x822660b0
	ctx.lr = 0x82FFE284;
	sub_822660B0(ctx, base);
	// 82FFE284: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FFE288: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82FFE28C: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FFE290: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82FFE294: 419A000C  beq cr6, 0x82ffe2a0
	if ctx.cr[6].eq {
	pc = 0x82FFE2A0; continue 'dispatch;
	}
	// 82FFE298: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82FFE29C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FFE2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFE2A4: 4BCAB1B4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE2A8 size=52
    let mut pc: u32 = 0x82FFE2A8;
    'dispatch: loop {
        match pc {
            0x82FFE2A8 => {
    //   block [0x82FFE2A8..0x82FFE2DC)
	// 82FFE2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE2B4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82FFE2B8: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82FFE2BC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FFE2C0: 4BFFFEE1  bl 0x82ffe1a0
	ctx.lr = 0x82FFE2C4;
	sub_82FFE1A0(ctx, base);
	// 82FFE2C4: F8E80028  std r7, 0x28(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[7].u64 ) };
	// 82FFE2C8: 90C80034  stw r6, 0x34(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
	// 82FFE2CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE2E0 size=56
    let mut pc: u32 = 0x82FFE2E0;
    'dispatch: loop {
        match pc {
            0x82FFE2E0 => {
    //   block [0x82FFE2E0..0x82FFE318)
	// 82FFE2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE2EC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82FFE2F0: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82FFE2F4: E9680028  ld r11, 0x28(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(40 as u32) ) };
	// 82FFE2F8: 7C845A14  add r4, r4, r11
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82FFE2FC: 4BFFFEA5  bl 0x82ffe1a0
	ctx.lr = 0x82FFE300;
	sub_82FFE1A0(ctx, base);
	// 82FFE300: F8880028  std r4, 0x28(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[4].u64 ) };
	// 82FFE304: 90E80034  stw r7, 0x34(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 82FFE308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE318 size=60
    let mut pc: u32 = 0x82FFE318;
    'dispatch: loop {
        match pc {
            0x82FFE318 => {
    //   block [0x82FFE318..0x82FFE354)
	// 82FFE318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE324: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82FFE328: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82FFE32C: E9680028  ld r11, 0x28(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(40 as u32) ) };
	// 82FFE330: 7CC45850  subf r6, r4, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82FFE334: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FFE338: 4BFFFE69  bl 0x82ffe1a0
	ctx.lr = 0x82FFE33C;
	sub_82FFE1A0(ctx, base);
	// 82FFE33C: F8C80028  std r6, 0x28(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82FFE340: 90E80034  stw r7, 0x34(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 82FFE344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE358 size=132
    let mut pc: u32 = 0x82FFE358;
    'dispatch: loop {
        match pc {
            0x82FFE358 => {
    //   block [0x82FFE358..0x82FFE3DC)
	// 82FFE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE35C: 4BCAB0B1  bl 0x82ca940c
	ctx.lr = 0x82FFE360;
	sub_82CA93D0(ctx, base);
	// 82FFE360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE368: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFE36C: E97F0028  ld r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82FFE370: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FFE374: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE378: 7C645A14  add r3, r4, r11
	ctx.r[3].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82FFE37C: 7FAAF050  subf r29, r10, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 82FFE380: F87F0028  std r3, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u64 ) };
	// 82FFE384: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFE388: 4099004C  ble cr6, 0x82ffe3d4
	if !ctx.cr[6].gt {
	pc = 0x82FFE3D4; continue 'dispatch;
	}
	// 82FFE38C: 48008245  bl 0x830065d0
	ctx.lr = 0x82FFE390;
	sub_830065D0(ctx, base);
	// 82FFE390: 7BAB0020  clrldi r11, r29, 0x20
	ctx.r[11].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82FFE394: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82FFE398: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82FFE39C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FFE3A0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82FFE3A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE3A8: FDA10024  fdiv f13, f1, f0
	ctx.f[13].f64 = ctx.f[1].f64 / ctx.f[0].f64;
	// 82FFE3AC: C80AB220  lfd f0, -0x4de0(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-19936 as u32) ) };
	// 82FFE3B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FFE3B4: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82FFE3B8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FFE3BC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82FFE3C0: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FFE3C4: 4BFFFDDD  bl 0x82ffe1a0
	ctx.lr = 0x82FFE3C8;
	sub_82FFE1A0(ctx, base);
	// 82FFE3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFE3CC: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82FFE3D0: F97F0028  std r11, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 82FFE3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFE3D8: 4BCAB084  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE3E0 size=8
    let mut pc: u32 = 0x82FFE3E0;
    'dispatch: loop {
        match pc {
            0x82FFE3E0 => {
    //   block [0x82FFE3E0..0x82FFE3E8)
	// 82FFE3E0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFE3E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE3E8 size=44
    let mut pc: u32 = 0x82FFE3E8;
    'dispatch: loop {
        match pc {
            0x82FFE3E8 => {
    //   block [0x82FFE3E8..0x82FFE414)
	// 82FFE3E8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82FFE3EC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE3F0: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FFE3F4: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFE3F8: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FFE3FC: 7D48482E  lwzx r10, r8, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FFE400: 7D29382E  lwzx r9, r9, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FFE404: E8CA0008  ld r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82FFE408: E8A90008  ld r5, 8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82FFE40C: 7F253040  cmpld cr6, r5, r6
	ctx.cr[6].compare_u64(ctx.r[5].u64, ctx.r[6].u64, &mut ctx.xer);
	// 82FFE410: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE414 size=40
    let mut pc: u32 = 0x82FFE414;
    'dispatch: loop {
        match pc {
            0x82FFE414 => {
    //   block [0x82FFE414..0x82FFE43C)
	// 82FFE414: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFE41C: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FFE420: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FFE424: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE428: 7D47592E  stwx r10, r7, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82FFE42C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE430: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FFE434: 409AFFB4  bne cr6, 0x82ffe3e8
	if !ctx.cr[6].eq {
		sub_82FFE3E8(ctx, base);
		return;
	}
	// 82FFE438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE440 size=100
    let mut pc: u32 = 0x82FFE440;
    'dispatch: loop {
        match pc {
            0x82FFE440 => {
    //   block [0x82FFE440..0x82FFE4A4)
	// 82FFE440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE44C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE458: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE45C: 4B267C55  bl 0x822660b0
	ctx.lr = 0x82FFE460;
	sub_822660B0(ctx, base);
	// 82FFE460: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82FFE464: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FFE468: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FFE46C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFE470: 40980008  bge cr6, 0x82ffe478
	if !ctx.cr[6].lt {
	pc = 0x82FFE478; continue 'dispatch;
	}
	// 82FFE474: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82FFE478: 4B267C39  bl 0x822660b0
	ctx.lr = 0x82FFE47C;
	sub_822660B0(ctx, base);
	// 82FFE47C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FFE480: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE488: 4BFFFAC1  bl 0x82ffdf48
	ctx.lr = 0x82FFE48C;
	sub_82FFDF48(ctx, base);
	// 82FFE48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE4A8 size=192
    let mut pc: u32 = 0x82FFE4A8;
    'dispatch: loop {
        match pc {
            0x82FFE4A8 => {
    //   block [0x82FFE4A8..0x82FFE568)
	// 82FFE4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE4AC: 4BCAAF61  bl 0x82ca940c
	ctx.lr = 0x82FFE4B0;
	sub_82CA93D0(ctx, base);
	// 82FFE4B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE4B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFE4BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFE4C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE4C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE4C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE4CC: 409A005C  bne cr6, 0x82ffe528
	if !ctx.cr[6].eq {
	pc = 0x82FFE528; continue 'dispatch;
	}
	// 82FFE4D0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FFE4D4: 40990010  ble cr6, 0x82ffe4e4
	if !ctx.cr[6].gt {
	pc = 0x82FFE4E4; continue 'dispatch;
	}
	// 82FFE4D8: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFE4DC: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FFE4E0: 48000008  b 0x82ffe4e8
	pc = 0x82FFE4E8; continue 'dispatch;
	// 82FFE4E4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FFE4E8: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82FFE4EC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82FFE4F0: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFE4F4: 4199000C  bgt cr6, 0x82ffe500
	if ctx.cr[6].gt {
	pc = 0x82FFE500; continue 'dispatch;
	}
	// 82FFE4F8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE4FC: 40980018  bge cr6, 0x82ffe514
	if !ctx.cr[6].lt {
	pc = 0x82FFE514; continue 'dispatch;
	}
	// 82FFE500: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FFE504: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFE508: 4198000C  blt cr6, 0x82ffe514
	if ctx.cr[6].lt {
	pc = 0x82FFE514; continue 'dispatch;
	}
	// 82FFE50C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE510: 48000050  b 0x82ffe560
	pc = 0x82FFE560; continue 'dispatch;
	// 82FFE514: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82FFE518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE51C: 4BFF84C5  bl 0x82ff69e0
	ctx.lr = 0x82FFE520;
	sub_82FF69E0(ctx, base);
	// 82FFE520: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFE524: 4182FFE8  beq 0x82ffe50c
	if ctx.cr[0].eq {
	pc = 0x82FFE50C; continue 'dispatch;
	}
	// 82FFE528: FBBE0008  std r29, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82FFE52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE530: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE534: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE538: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE53C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFE540: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE544: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FFE548: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE54C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFE550: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFE554: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE558: 4BFFFE89  bl 0x82ffe3e0
	ctx.lr = 0x82FFE55C;
	sub_82FFE3E0(ctx, base);
	// 82FFE55C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFE560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE564: 4BCAAEF8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE568 size=208
    let mut pc: u32 = 0x82FFE568;
    'dispatch: loop {
        match pc {
            0x82FFE568 => {
    //   block [0x82FFE568..0x82FFE638)
	// 82FFE568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE56C: 4BCAAEA1  bl 0x82ca940c
	ctx.lr = 0x82FFE570;
	sub_82CA93D0(ctx, base);
	// 82FFE570: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE574: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFE578: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE57C: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82FFE580: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82FFE584: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE588: 419A0010  beq cr6, 0x82ffe598
	if ctx.cr[6].eq {
	pc = 0x82FFE598; continue 'dispatch;
	}
	// 82FFE58C: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 82FFE590: 60631009  ori r3, r3, 0x1009
	ctx.r[3].u64 = ctx.r[3].u64 | 4105;
	// 82FFE594: 4800009C  b 0x82ffe630
	pc = 0x82FFE630; continue 'dispatch;
	// 82FFE598: E96300E0  ld r11, 0xe0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(224 as u32) ) };
	// 82FFE59C: 81030090  lwz r8, 0x90(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFE5A0: 7F2B4040  cmpld cr6, r11, r8
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[8].u64, &mut ctx.xer);
	// 82FFE5A4: 40980014  bge cr6, 0x82ffe5b8
	if !ctx.cr[6].lt {
	pc = 0x82FFE5B8; continue 'dispatch;
	}
	// 82FFE5A8: E9630120  ld r11, 0x120(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) };
	// 82FFE5AC: 81030094  lwz r8, 0x94(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFE5B0: 7F2B4040  cmpld cr6, r11, r8
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[8].u64, &mut ctx.xer);
	// 82FFE5B4: 41980010  blt cr6, 0x82ffe5c4
	if ctx.cr[6].lt {
	pc = 0x82FFE5C4; continue 'dispatch;
	}
	// 82FFE5B8: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 82FFE5BC: 60631005  ori r3, r3, 0x1005
	ctx.r[3].u64 = ctx.r[3].u64 | 4101;
	// 82FFE5C0: 48000070  b 0x82ffe630
	pc = 0x82FFE630; continue 'dispatch;
	// 82FFE5C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFE5C8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FFE5CC: 409A0014  bne cr6, 0x82ffe5e0
	if !ctx.cr[6].eq {
	pc = 0x82FFE5E0; continue 'dispatch;
	}
	// 82FFE5D0: 39610068  addi r11, r1, 0x68
	ctx.r[11].s64 = ctx.r[1].s64 + 104;
	// 82FFE5D4: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 82FFE5D8: FBCB0000  std r30, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82FFE5DC: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FFE5E0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE5E4: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82FFE5E8: 83A30010  lwz r29, 0x10(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE5EC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FFE5F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FFE5F4: 38EB000A  addi r7, r11, 0xa
	ctx.r[7].s64 = ctx.r[11].s64 + 10;
	// 82FFE5F8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82FFE5FC: 20A7001F  subfic r5, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[5].s64 = (31 as i64) - ctx.r[7].s64;
	// 82FFE600: E97D0228  ld r11, 0x228(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(552 as u32) ) };
	// 82FFE604: 78A5D906  sldi r5, r5, 0x3b
	ctx.r[5].u64 = ctx.r[5].u64.wrapping_shl(59);
	ctx.r[5].u32 = ctx.r[5].u64 as u32;
	// 82FFE608: 80FD0064  lwz r7, 0x64(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FFE60C: 79640140  clrldi r4, r11, 5
	ctx.r[4].u64 = ctx.r[11].u64 & 0x07FFFFFFFFFFFFFFu64;
	// 82FFE610: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFE614: 7C842B78  or r4, r4, r5
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[5].u64;
	// 82FFE618: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFE61C: F97D0228  std r11, 0x228(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(552 as u32), ctx.r[11].u64 ) };
	// 82FFE620: 48003E09  bl 0x83002428
	ctx.lr = 0x82FFE624;
	sub_83002428(ctx, base);
	// 82FFE624: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFE628: 41800008  blt 0x82ffe630
	if ctx.cr[0].lt {
	pc = 0x82FFE630; continue 'dispatch;
	}
	// 82FFE62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE630: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFE634: 4BCAAE28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE638 size=236
    let mut pc: u32 = 0x82FFE638;
    'dispatch: loop {
        match pc {
            0x82FFE638 => {
    //   block [0x82FFE638..0x82FFE724)
	// 82FFE638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE64C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE650: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFE654: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE658: 815F00A4  lwz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFE65C: 556B0104  rlwinm r11, r11, 0, 4, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE660: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82FFE664: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFE668: 419A00A4  beq cr6, 0x82ffe70c
	if ctx.cr[6].eq {
	pc = 0x82FFE70C; continue 'dispatch;
	}
	// 82FFE66C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFE674: 419A0010  beq cr6, 0x82ffe684
	if ctx.cr[6].eq {
	pc = 0x82FFE684; continue 'dispatch;
	}
	// 82FFE678: 480034E1  bl 0x83001b58
	ctx.lr = 0x82FFE67C;
	sub_83001B58(ctx, base);
	// 82FFE67C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFE680: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFE684: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FFE688: 397F0040  addi r11, r31, 0x40
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	// 82FFE68C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE690: 409A0034  bne cr6, 0x82ffe6c4
	if !ctx.cr[6].eq {
	pc = 0x82FFE6C4; continue 'dispatch;
	}
	// 82FFE694: 817F00F0  lwz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 82FFE698: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE69C: 40820028  bne 0x82ffe6c4
	if !ctx.cr[0].eq {
	pc = 0x82FFE6C4; continue 'dispatch;
	}
	// 82FFE6A0: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82FFE6A4: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE6A8: 4082001C  bne 0x82ffe6c4
	if !ctx.cr[0].eq {
	pc = 0x82FFE6C4; continue 'dispatch;
	}
	// 82FFE6AC: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE6B0: 556B0109  rlwinm. r11, r11, 0, 4, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE6B4: 41820058  beq 0x82ffe70c
	if ctx.cr[0].eq {
	pc = 0x82FFE70C; continue 'dispatch;
	}
	// 82FFE6B8: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFE6BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFE6C0: 409A004C  bne cr6, 0x82ffe70c
	if !ctx.cr[6].eq {
	pc = 0x82FFE70C; continue 'dispatch;
	}
	// 82FFE6C4: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE6C8: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82FFE6CC: 80FF0018  lwz r7, 0x18(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE6D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE6D4: 392B0200  addi r9, r11, 0x200
	ctx.r[9].s64 = ctx.r[11].s64 + 512;
	// 82FFE6D8: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FFE6DC: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE6E0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE6E4: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FFE6E8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FFE6EC: 812B0204  lwz r9, 0x204(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 82FFE6F0: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82FFE6F4: 812B0204  lwz r9, 0x204(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 82FFE6F8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFE6FC: 914B0204  stw r10, 0x204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), ctx.r[10].u32 ) };
	// 82FFE700: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE704: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 82FFE708: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFE70C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE718: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE71C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FFE728 size=184
    let mut pc: u32 = 0x82FFE728;
    'dispatch: loop {
        match pc {
            0x82FFE728 => {
    //   block [0x82FFE728..0x82FFE7E0)
	// 82FFE728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE73C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE740: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82FFE744: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82FFE748: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82FFE74C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FFE750: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82FFE754: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFE758: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFE75C: 409A0008  bne cr6, 0x82ffe764
	if !ctx.cr[6].eq {
	pc = 0x82FFE764; continue 'dispatch;
	}
	// 82FFE760: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FFE764: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82FFE768: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FFE76C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFE770: 419A0050  beq cr6, 0x82ffe7c0
	if ctx.cr[6].eq {
	pc = 0x82FFE7C0; continue 'dispatch;
	}
	// 82FFE774: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FFE778: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFE77C: 41990044  bgt cr6, 0x82ffe7c0
	if ctx.cr[6].gt {
	pc = 0x82FFE7C0; continue 'dispatch;
	}
	// 82FFE780: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFE784: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82FFE788: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82FFE78C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FFE790: 7D685810  subfc r11, r8, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[8].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82FFE794: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FFE798: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE79C: 41820024  beq 0x82ffe7c0
	if ctx.cr[0].eq {
	pc = 0x82FFE7C0; continue 'dispatch;
	}
	// 82FFE7A0: 3C80807A  lis r4, -0x7f86
	ctx.r[4].s64 = -2139488256;
	// 82FFE7A4: 60841016  ori r4, r4, 0x1016
	ctx.r[4].u64 = ctx.r[4].u64 | 4118;
	// 82FFE7A8: 480031D9  bl 0x83001980
	ctx.lr = 0x82FFE7AC;
	sub_83001980(ctx, base);
	// 82FFE7AC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFE7B0: B3DF0004  sth r30, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 82FFE7B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFE7B8: B3DF0006  sth r30, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 82FFE7BC: 4800000C  b 0x82ffe7c8
	pc = 0x82FFE7C8; continue 'dispatch;
	// 82FFE7C0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82FFE7C4: 480035B5  bl 0x83001d78
	ctx.lr = 0x82FFE7C8;
	sub_83001D78(ctx, base);
	// 82FFE7C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE7D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE7D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE7E0 size=88
    let mut pc: u32 = 0x82FFE7E0;
    'dispatch: loop {
        match pc {
            0x82FFE7E0 => {
    //   block [0x82FFE7E0..0x82FFE838)
	// 82FFE7E0: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE7E4: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82FFE7E8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FFE7EC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE7F0: 419A0024  beq cr6, 0x82ffe814
	if ctx.cr[6].eq {
	pc = 0x82FFE814; continue 'dispatch;
	}
	// 82FFE7F4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE7F8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE7FC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FFE800: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE804: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE808: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FFE80C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE810: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFE814: 816A00B0  lwz r11, 0xb0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFE818: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFE81C: 812A00B4  lwz r9, 0xb4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FFE820: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFE824: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82FFE828: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFE82C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82FFE830: 916A00B4  stw r11, 0xb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82FFE834: 48003534  b 0x83001d68
	sub_83001D68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE838 size=80
    let mut pc: u32 = 0x82FFE838;
    'dispatch: loop {
        match pc {
            0x82FFE838 => {
    //   block [0x82FFE838..0x82FFE888)
	// 82FFE838: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE83C: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82FFE840: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE844: 419A0024  beq cr6, 0x82ffe868
	if ctx.cr[6].eq {
	pc = 0x82FFE868; continue 'dispatch;
	}
	// 82FFE848: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE84C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE850: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFE854: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE858: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE85C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFE860: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE864: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFE868: 39430038  addi r10, r3, 0x38
	ctx.r[10].s64 = ctx.r[3].s64 + 56;
	// 82FFE86C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFE870: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFE874: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFE878: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFE87C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE880: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82FFE884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE888 size=12
    let mut pc: u32 = 0x82FFE888;
    'dispatch: loop {
        match pc {
            0x82FFE888 => {
    //   block [0x82FFE888..0x82FFE894)
	// 82FFE888: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE88C: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE890: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE894 size=76
    let mut pc: u32 = 0x82FFE894;
    'dispatch: loop {
        match pc {
            0x82FFE894 => {
    //   block [0x82FFE894..0x82FFE8E0)
	// 82FFE894: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE898: 39430014  addi r10, r3, 0x14
	ctx.r[10].s64 = ctx.r[3].s64 + 20;
	// 82FFE89C: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE8A0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE8A4: 392B0200  addi r9, r11, 0x200
	ctx.r[9].s64 = ctx.r[11].s64 + 512;
	// 82FFE8A8: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FFE8AC: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE8B0: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFE8B4: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FFE8B8: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FFE8BC: 812B0204  lwz r9, 0x204(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 82FFE8C0: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82FFE8C4: 812B0204  lwz r9, 0x204(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 82FFE8C8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFE8CC: 914B0204  stw r10, 0x204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), ctx.r[10].u32 ) };
	// 82FFE8D0: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE8D4: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 82FFE8D8: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFE8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE8E0 size=84
    let mut pc: u32 = 0x82FFE8E0;
    'dispatch: loop {
        match pc {
            0x82FFE8E0 => {
    //   block [0x82FFE8E0..0x82FFE934)
	// 82FFE8E0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82FFE8E4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82FFE8E8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE8EC: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFE8F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE8F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE8F8: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFE8FC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE900: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE904: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFE908: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FFE90C: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFE910: 816300A0  lwz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FFE914: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFE918: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE91C: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FFE920: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFE924: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE928: 4182000C  beq 0x82ffe934
	if ctx.cr[0].eq {
		sub_82FFE934(ctx, base);
		return;
	}
	// 82FFE92C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFE930: 4800000C  b 0x82ffe93c
	sub_82FFE934(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE934 size=16
    let mut pc: u32 = 0x82FFE934;
    'dispatch: loop {
        match pc {
            0x82FFE934 => {
    //   block [0x82FFE934..0x82FFE944)
	// 82FFE934: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE938: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE93C: 9164001C  stw r11, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FFE940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE948 size=24
    let mut pc: u32 = 0x82FFE948;
    'dispatch: loop {
        match pc {
            0x82FFE948 => {
    //   block [0x82FFE948..0x82FFE960)
	// 82FFE948: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFE94C: 814300A8  lwz r10, 0xa8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FFE950: 656B2800  oris r11, r11, 0x2800
	ctx.r[11].u64 = ctx.r[11].u64 | 671088640;
	// 82FFE954: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFE958: 916300A4  stw r11, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82FFE95C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE960 size=8
    let mut pc: u32 = 0x82FFE960;
    'dispatch: loop {
        match pc {
            0x82FFE960 => {
    //   block [0x82FFE960..0x82FFE968)
	// 82FFE960: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FFE964: 4BFFFF24  b 0x82ffe888
	sub_82FFE888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE968 size=4
    let mut pc: u32 = 0x82FFE968;
    'dispatch: loop {
        match pc {
            0x82FFE968 => {
    //   block [0x82FFE968..0x82FFE96C)
	// 82FFE968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE970 size=24
    let mut pc: u32 = 0x82FFE970;
    'dispatch: loop {
        match pc {
            0x82FFE970 => {
    //   block [0x82FFE970..0x82FFE988)
	// 82FFE970: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFE974: 814300A8  lwz r10, 0xa8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FFE978: 656B0800  oris r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 134217728;
	// 82FFE97C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFE980: 916300A4  stw r11, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82FFE984: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE988 size=8
    let mut pc: u32 = 0x82FFE988;
    'dispatch: loop {
        match pc {
            0x82FFE988 => {
    //   block [0x82FFE988..0x82FFE990)
	// 82FFE988: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FFE98C: 4BFFFEFC  b 0x82ffe888
	sub_82FFE888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE990 size=4
    let mut pc: u32 = 0x82FFE990;
    'dispatch: loop {
        match pc {
            0x82FFE990 => {
    //   block [0x82FFE990..0x82FFE994)
	// 82FFE990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE998 size=16
    let mut pc: u32 = 0x82FFE998;
    'dispatch: loop {
        match pc {
            0x82FFE998 => {
    //   block [0x82FFE998..0x82FFE9A8)
	// 82FFE998: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFE99C: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82FFE9A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE9A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE9A8 size=24
    let mut pc: u32 = 0x82FFE9A8;
    'dispatch: loop {
        match pc {
            0x82FFE9A8 => {
    //   block [0x82FFE9A8..0x82FFE9C0)
	// 82FFE9A8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE9AC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE9B0: 409A0008  bne cr6, 0x82ffe9b8
	if !ctx.cr[6].eq {
	pc = 0x82FFE9B8; continue 'dispatch;
	}
	// 82FFE9B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFE9B8: 386AFFFC  addi r3, r10, -4
	ctx.r[3].s64 = ctx.r[10].s64 + -4;
	// 82FFE9BC: 48003984  b 0x83002340
	sub_83002340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE9C0 size=4
    let mut pc: u32 = 0x82FFE9C0;
    'dispatch: loop {
        match pc {
            0x82FFE9C0 => {
    //   block [0x82FFE9C0..0x82FFE9C4)
	// 82FFE9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE9C8 size=40
    let mut pc: u32 = 0x82FFE9C8;
    'dispatch: loop {
        match pc {
            0x82FFE9C8 => {
    //   block [0x82FFE9C8..0x82FFE9F0)
	// 82FFE9C8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FFE9CC: 38630218  addi r3, r3, 0x218
	ctx.r[3].s64 = ctx.r[3].s64 + 536;
	// 82FFE9D0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82FFE9D4: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFE9D8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FFE9DC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE9E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFE9E4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFE9E8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE9EC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE9F0 size=36
    let mut pc: u32 = 0x82FFE9F0;
    'dispatch: loop {
        match pc {
            0x82FFE9F0 => {
    //   block [0x82FFE9F0..0x82FFEA14)
	// 82FFE9F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE9F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFE9F8: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFE9FC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FFEA00: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82FFEA04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEA08: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FFEA0C: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FFEA10: 4BFF8040  b 0x82ff6a50
	sub_82FF6A50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEA14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFEA14 size=4
    let mut pc: u32 = 0x82FFEA14;
    'dispatch: loop {
        match pc {
            0x82FFEA14 => {
    //   block [0x82FFEA14..0x82FFEA18)
	// 82FFEA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEA18 size=116
    let mut pc: u32 = 0x82FFEA18;
    'dispatch: loop {
        match pc {
            0x82FFEA18 => {
    //   block [0x82FFEA18..0x82FFEA8C)
	// 82FFEA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEA20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFEA24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFEA28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEA2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEA30: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82FFEA34: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82FFEA38: 396BB270  addi r11, r11, -0x4d90
	ctx.r[11].s64 = ctx.r[11].s64 + -19856;
	// 82FFEA3C: 394AB260  addi r10, r10, -0x4da0
	ctx.r[10].s64 = ctx.r[10].s64 + -19872;
	// 82FFEA40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFEA44: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 82FFEA48: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FFEA4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEA50: 4BFFF4F1  bl 0x82ffdf40
	ctx.lr = 0x82FFEA54;
	sub_82FFDF40(ctx, base);
	// 82FFEA54: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82FFEA58: 4BB98421  bl 0x82b96e78
	ctx.lr = 0x82FFEA5C;
	sub_82B96E78(ctx, base);
	// 82FFEA5C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82FFEA60: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82FFEA64: 396BAB50  addi r11, r11, -0x54b0
	ctx.r[11].s64 = ctx.r[11].s64 + -21680;
	// 82FFEA68: 394AB144  addi r10, r10, -0x4ebc
	ctx.r[10].s64 = ctx.r[10].s64 + -20156;
	// 82FFEA6C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FFEA70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFEA74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEA80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFEA84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFEA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEA90 size=172
    let mut pc: u32 = 0x82FFEA90;
    'dispatch: loop {
        match pc {
            0x82FFEA90 => {
    //   block [0x82FFEA90..0x82FFEB3C)
	// 82FFEA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEA94: 4BCAA979  bl 0x82ca940c
	ctx.lr = 0x82FFEA98;
	sub_82CA93D0(ctx, base);
	// 82FFEA98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEA9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFEAA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFEAA4: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFEAA8: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFEAAC: 40820088  bne 0x82ffeb34
	if !ctx.cr[0].eq {
	pc = 0x82FFEB34; continue 'dispatch;
	}
	// 82FFEAB0: 656B0400  oris r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 67108864;
	// 82FFEAB4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82FFEAB8: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFEABC: 387E0050  addi r3, r30, 0x50
	ctx.r[3].s64 = ctx.r[30].s64 + 80;
	// 82FFEAC0: 915E00A4  stw r10, 0xa4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 82FFEAC4: 4BFFF525  bl 0x82ffdfe8
	ctx.lr = 0x82FFEAC8;
	sub_82FFDFE8(ctx, base);
	// 82FFEAC8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFEACC: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 82FFEAD0: 48000024  b 0x82ffeaf4
	pc = 0x82FFEAF4; continue 'dispatch;
	// 82FFEAD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEAD8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFEADC: 409A0008  bne cr6, 0x82ffeae4
	if !ctx.cr[6].eq {
	pc = 0x82FFEAE4; continue 'dispatch;
	}
	// 82FFEAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFEAE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFEAE8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82FFEAEC: 48002E95  bl 0x83001980
	ctx.lr = 0x82FFEAF0;
	sub_83001980(ctx, base);
	// 82FFEAF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEAF4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFEAF8: 409AFFDC  bne cr6, 0x82ffead4
	if !ctx.cr[6].eq {
	pc = 0x82FFEAD4; continue 'dispatch;
	}
	// 82FFEAFC: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFEB00: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FFEB04: 419A0010  beq cr6, 0x82ffeb14
	if ctx.cr[6].eq {
	pc = 0x82FFEB14; continue 'dispatch;
	}
	// 82FFEB08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFEB0C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEB10: 4BFFFEB9  bl 0x82ffe9c8
	ctx.lr = 0x82FFEB14;
	sub_82FFE9C8(ctx, base);
	// 82FFEB14: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFEB18: 815E00B0  lwz r10, 0xb0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFEB1C: 656B0800  oris r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 134217728;
	// 82FFEB20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFEB24: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFEB28: 409A000C  bne cr6, 0x82ffeb34
	if !ctx.cr[6].eq {
	pc = 0x82FFEB34; continue 'dispatch;
	}
	// 82FFEB2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEB30: 4BFFFD59  bl 0x82ffe888
	ctx.lr = 0x82FFEB34;
	sub_82FFE888(ctx, base);
	// 82FFEB34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEB38: 4BCAA924  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEB40 size=376
    let mut pc: u32 = 0x82FFEB40;
    'dispatch: loop {
        match pc {
            0x82FFEB40 => {
    //   block [0x82FFEB40..0x82FFECB8)
	// 82FFEB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEB44: 4BCAA8BD  bl 0x82ca9400
	ctx.lr = 0x82FFEB48;
	sub_82CA93D0(ctx, base);
	// 82FFEB48: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEB4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFEB50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEB54: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FFEB58: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FFEB5C: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 82FFEB60: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82FFEB64: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82FFEB68: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FFEB6C: 48000030  b 0x82ffeb9c
	pc = 0x82FFEB9C; continue 'dispatch;
	// 82FFEB70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEB74: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FFEB78: 409A0008  bne cr6, 0x82ffeb80
	if !ctx.cr[6].eq {
	pc = 0x82FFEB80; continue 'dispatch;
	}
	// 82FFEB7C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FFEB80: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82FFEB84: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFEB88: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FFEB8C: 48003D75  bl 0x83002900
	ctx.lr = 0x82FFEB90;
	sub_83002900(ctx, base);
	// 82FFEB90: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFEB94: 40820114  bne 0x82ffeca8
	if !ctx.cr[0].eq {
	pc = 0x82FFECA8; continue 'dispatch;
	}
	// 82FFEB98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEB9C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FFEBA0: 409AFFD0  bne cr6, 0x82ffeb70
	if !ctx.cr[6].eq {
	pc = 0x82FFEB70; continue 'dispatch;
	}
	// 82FFEBA4: 817F00F0  lwz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 82FFEBA8: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEBAC: 41820060  beq 0x82ffec0c
	if ctx.cr[0].eq {
	pc = 0x82FFEC0C; continue 'dispatch;
	}
	// 82FFEBB0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEBB4: 811F009C  lwz r8, 0x9c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FFEBB8: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FFEBBC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FFEBC0: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82FFEBC4: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFEBC8: 409A0008  bne cr6, 0x82ffebd0
	if !ctx.cr[6].eq {
	pc = 0x82FFEBD0; continue 'dispatch;
	}
	// 82FFEBCC: 813F0098  lwz r9, 0x98(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FFEBD0: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82FFEBD4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEBD8: 3C801000  lis r4, 0x1000
	ctx.r[4].s64 = 268435456;
	// 82FFEBDC: 80C5000C  lwz r6, 0xc(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEBE0: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82FFEBE4: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEBE8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82FFEBEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFEBF0: 686B0001  xori r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u64 ^ 1;
	// 82FFEBF4: 80A50010  lwz r5, 0x10(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEBF8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82FFEBFC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FFEC00: 4BFFF511  bl 0x82ffe110
	ctx.lr = 0x82FFEC04;
	sub_82FFE110(ctx, base);
	// 82FFEC04: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82FFEC08: 480000A0  b 0x82ffeca8
	pc = 0x82FFECA8; continue 'dispatch;
	// 82FFEC0C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82FFEC10: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEC14: 41820060  beq 0x82ffec74
	if ctx.cr[0].eq {
	pc = 0x82FFEC74; continue 'dispatch;
	}
	// 82FFEC18: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEC1C: 811F009C  lwz r8, 0x9c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FFEC20: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FFEC24: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FFEC28: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82FFEC2C: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFEC30: 409A0008  bne cr6, 0x82ffec38
	if !ctx.cr[6].eq {
	pc = 0x82FFEC38; continue 'dispatch;
	}
	// 82FFEC34: 813F0098  lwz r9, 0x98(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FFEC38: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82FFEC3C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEC40: 3C801000  lis r4, 0x1000
	ctx.r[4].s64 = 268435456;
	// 82FFEC44: 80C5000C  lwz r6, 0xc(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEC48: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82FFEC4C: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEC50: 608B0001  ori r11, r4, 1
	ctx.r[11].u64 = ctx.r[4].u64 | 1;
	// 82FFEC54: 80A50010  lwz r5, 0x10(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEC58: 68640001  xori r4, r3, 1
	ctx.r[4].u64 = ctx.r[3].u64 ^ 1;
	// 82FFEC5C: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 82FFEC60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FFEC64: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82FFEC68: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFEC6C: 4BFFF4A5  bl 0x82ffe110
	ctx.lr = 0x82FFEC70;
	sub_82FFE110(ctx, base);
	// 82FFEC70: 4BFFFF94  b 0x82ffec04
	pc = 0x82FFEC04; continue 'dispatch;
	// 82FFEC74: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFEC78: 556A0109  rlwinm. r10, r11, 0, 4, 4
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFEC7C: 41820030  beq 0x82ffecac
	if ctx.cr[0].eq {
	pc = 0x82FFECAC; continue 'dispatch;
	}
	// 82FFEC80: 815F00B0  lwz r10, 0xb0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFEC84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFEC88: 409A0024  bne cr6, 0x82ffecac
	if !ctx.cr[6].eq {
	pc = 0x82FFECAC; continue 'dispatch;
	}
	// 82FFEC8C: 556B0146  rlwinm r11, r11, 0, 5, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFEC90: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFEC94: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFEC98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEC9C: 4BFFFC45  bl 0x82ffe8e0
	ctx.lr = 0x82FFECA0;
	sub_82FFE8E0(ctx, base);
	// 82FFECA0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82FFECA4: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82FFECA8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82FFECAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFECB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFECB4: 4BCAA79C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFECB8 size=456
    let mut pc: u32 = 0x82FFECB8;
    'dispatch: loop {
        match pc {
            0x82FFECB8 => {
    //   block [0x82FFECB8..0x82FFEE80)
	// 82FFECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFECBC: 4BCAA741  bl 0x82ca93fc
	ctx.lr = 0x82FFECC0;
	sub_82CA93D0(ctx, base);
	// 82FFECC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFECC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFECC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFECCC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FFECD0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FFECD4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFECD8: 815E003C  lwz r10, 0x3c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFECDC: 812B0234  lwz r9, 0x234(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(564 as u32) ) } as u64;
	// 82FFECE0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82FFECE4: 914B0234  stw r10, 0x234(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(564 as u32), ctx.r[10].u32 ) };
	// 82FFECE8: 419A0010  beq cr6, 0x82ffecf8
	if ctx.cr[6].eq {
	pc = 0x82FFECF8; continue 'dispatch;
	}
	// 82FFECEC: 814B0238  lwz r10, 0x238(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(568 as u32) ) } as u64;
	// 82FFECF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFECF4: 914B0238  stw r10, 0x238(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(568 as u32), ctx.r[10].u32 ) };
	// 82FFECF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFECFC: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFED00: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82FFED04: 4BFFF5DD  bl 0x82ffe2e0
	ctx.lr = 0x82FFED08;
	sub_82FFE2E0(ctx, base);
	// 82FFED08: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 82FFED0C: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFED10: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFED14: 4BFFF5CD  bl 0x82ffe2e0
	ctx.lr = 0x82FFED18;
	sub_82FFE2E0(ctx, base);
	// 82FFED18: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFED1C: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FFED20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFED24: 813F00F0  lwz r9, 0xf0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 82FFED28: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82FFED2C: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFED30: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FFED34: 552A00C7  rlwinm. r10, r9, 0, 3, 3
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFED38: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82FFED3C: 40820010  bne 0x82ffed4c
	if !ctx.cr[0].eq {
	pc = 0x82FFED4C; continue 'dispatch;
	}
	// 82FFED40: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82FFED44: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFED48: 4182000C  beq 0x82ffed54
	if ctx.cr[0].eq {
	pc = 0x82FFED54; continue 'dispatch;
	}
	// 82FFED4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFED50: 4BFFFB39  bl 0x82ffe888
	ctx.lr = 0x82FFED54;
	sub_82FFE888(ctx, base);
	// 82FFED54: 3B7F0030  addi r27, r31, 0x30
	ctx.r[27].s64 = ctx.r[31].s64 + 48;
	// 82FFED58: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82FFED5C: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82FFED60: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FFED64: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFED68: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FFED6C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82FFED70: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82FFED74: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FFED78: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFED7C: 41820048  beq 0x82ffedc4
	if ctx.cr[0].eq {
	pc = 0x82FFEDC4; continue 'dispatch;
	}
	// 82FFED80: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFED84: 3B9F0048  addi r28, r31, 0x48
	ctx.r[28].s64 = ctx.r[31].s64 + 72;
	// 82FFED88: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFED8C: 409A001C  bne cr6, 0x82ffeda8
	if !ctx.cr[6].eq {
	pc = 0x82FFEDA8; continue 'dispatch;
	}
	// 82FFED90: 833F008C  lwz r25, 0x8c(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFED94: 4B26731D  bl 0x822660b0
	ctx.lr = 0x82FFED98;
	sub_822660B0(ctx, base);
	// 82FFED98: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FFED9C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FFEDA0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFEDA4: 4BFFF1A5  bl 0x82ffdf48
	ctx.lr = 0x82FFEDA8;
	sub_82FFDF48(ctx, base);
	// 82FFEDA8: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FFEDAC: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 82FFEDB0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEDB4: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FFEDB8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEDBC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFEDC0: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFEDC4: 83BD0004  lwz r29, 4(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEDC8: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FFEDCC: 419A000C  beq cr6, 0x82ffedd8
	if ctx.cr[6].eq {
	pc = 0x82FFEDD8; continue 'dispatch;
	}
	// 82FFEDD0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEDD4: 40820020  bne 0x82ffedf4
	if !ctx.cr[0].eq {
	pc = 0x82FFEDF4; continue 'dispatch;
	}
	// 82FFEDD8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEDDC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82FFEDE0: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82FFEDE4: E89E0018  ld r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	// 82FFEDE8: 386B0218  addi r3, r11, 0x218
	ctx.r[3].s64 = ctx.r[11].s64 + 536;
	// 82FFEDEC: 4BFFF6BD  bl 0x82ffe4a8
	ctx.lr = 0x82FFEDF0;
	sub_82FFE4A8(ctx, base);
	// 82FFEDF0: 48000088  b 0x82ffee78
	pc = 0x82FFEE78; continue 'dispatch;
	// 82FFEDF4: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FFEDF8: 419A0080  beq cr6, 0x82ffee78
	if ctx.cr[6].eq {
	pc = 0x82FFEE78; continue 'dispatch;
	}
	// 82FFEDFC: E97E0018  ld r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	// 82FFEE00: 797C2EE0  rldicl r28, r11, 5, 0x3b
	ctx.r[28].u64 = ctx.r[11].u64 & 0x07FFFFFFFFFFFFFFu64;
	// 82FFEE04: 48000018  b 0x82ffee1c
	pc = 0x82FFEE1C; continue 'dispatch;
	// 82FFEE08: 83BD0004  lwz r29, 4(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEE0C: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FFEE10: 419A0028  beq cr6, 0x82ffee38
	if ctx.cr[6].eq {
	pc = 0x82FFEE38; continue 'dispatch;
	}
	// 82FFEE14: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEE18: 41820020  beq 0x82ffee38
	if ctx.cr[0].eq {
	pc = 0x82FFEE38; continue 'dispatch;
	}
	// 82FFEE1C: 3BDDFFFC  addi r30, r29, -4
	ctx.r[30].s64 = ctx.r[29].s64 + -4;
	// 82FFEE20: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFEE24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE28: 48002B21  bl 0x83001948
	ctx.lr = 0x82FFEE2C;
	sub_83001948(ctx, base);
	// 82FFEE2C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFEE30: 4082FFD8  bne 0x82ffee08
	if !ctx.cr[0].eq {
	pc = 0x82FFEE08; continue 'dispatch;
	}
	// 82FFEE34: 48000044  b 0x82ffee78
	pc = 0x82FFEE78; continue 'dispatch;
	// 82FFEE38: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEE3C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82FFEE40: E95E0018  ld r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	// 82FFEE44: E91F0028  ld r8, 0x28(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82FFEE48: 38690218  addi r3, r9, 0x218
	ctx.r[3].s64 = ctx.r[9].s64 + 536;
	// 82FFEE4C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFEE50: 7F285040  cmpld cr6, r8, r10
	ctx.cr[6].compare_u64(ctx.r[8].u64, ctx.r[10].u64, &mut ctx.xer);
	// 82FFEE54: 41980008  blt cr6, 0x82ffee5c
	if ctx.cr[6].lt {
	pc = 0x82FFEE5C; continue 'dispatch;
	}
	// 82FFEE58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFEE5C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEE60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFEE64: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82FFEE68: 419A000C  beq cr6, 0x82ffee74
	if ctx.cr[6].eq {
	pc = 0x82FFEE74; continue 'dispatch;
	}
	// 82FFEE6C: 4BFF7BE5  bl 0x82ff6a50
	ctx.lr = 0x82FFEE70;
	sub_82FF6A50(ctx, base);
	// 82FFEE70: 48000008  b 0x82ffee78
	pc = 0x82FFEE78; continue 'dispatch;
	// 82FFEE74: 4BFFF56D  bl 0x82ffe3e0
	ctx.lr = 0x82FFEE78;
	sub_82FFE3E0(ctx, base);
	// 82FFEE78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFEE7C: 4BCAA5D0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEE80 size=500
    let mut pc: u32 = 0x82FFEE80;
    'dispatch: loop {
        match pc {
            0x82FFEE80 => {
    //   block [0x82FFEE80..0x82FFF074)
	// 82FFEE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEE84: 4BCAA581  bl 0x82ca9404
	ctx.lr = 0x82FFEE88;
	sub_82CA93D0(ctx, base);
	// 82FFEE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEE8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFEE90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFEE94: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FFEE98: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFEE9C: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFEEA0: 812B0234  lwz r9, 0x234(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(564 as u32) ) } as u64;
	// 82FFEEA4: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82FFEEA8: 914B0234  stw r10, 0x234(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(564 as u32), ctx.r[10].u32 ) };
	// 82FFEEAC: 419A0010  beq cr6, 0x82ffeebc
	if ctx.cr[6].eq {
	pc = 0x82FFEEBC; continue 'dispatch;
	}
	// 82FFEEB0: 814B0238  lwz r10, 0x238(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(568 as u32) ) } as u64;
	// 82FFEEB4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82FFEEB8: 914B0238  stw r10, 0x238(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(568 as u32), ctx.r[10].u32 ) };
	// 82FFEEBC: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFEEC0: 3B9D0030  addi r28, r29, 0x30
	ctx.r[28].s64 = ctx.r[29].s64 + 48;
	// 82FFEEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFEEC8: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFEECC: 419A0008  beq cr6, 0x82ffeed4
	if ctx.cr[6].eq {
	pc = 0x82FFEED4; continue 'dispatch;
	}
	// 82FFEED0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FFEED4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82FFEED8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEEDC: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFEEE0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82FFEEE4: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FFEEE8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFEEEC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FFEEF0: 555BDFFE  rlwinm r27, r10, 0x1b, 0x1f, 0x1f
	ctx.r[27].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFEEF4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFEEF8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEEFC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFEF00: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFEF04: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFEF08: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEF0C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FFEF10: 419A00B4  beq cr6, 0x82ffefc4
	if ctx.cr[6].eq {
	pc = 0x82FFEFC4; continue 'dispatch;
	}
	// 82FFEF14: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFEF18: 395D0048  addi r10, r29, 0x48
	ctx.r[10].s64 = ctx.r[29].s64 + 72;
	// 82FFEF1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFEF20: 409A0008  bne cr6, 0x82ffef28
	if !ctx.cr[6].eq {
	pc = 0x82FFEF28; continue 'dispatch;
	}
	// 82FFEF24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFEF28: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFEF2C: 409A0078  bne cr6, 0x82ffefa4
	if !ctx.cr[6].eq {
	pc = 0x82FFEFA4; continue 'dispatch;
	}
	// 82FFEF30: 817D00AC  lwz r11, 0xac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFEF34: 556B014B  rlwinm. r11, r11, 0, 5, 5
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEF38: 4082006C  bne 0x82ffefa4
	if !ctx.cr[0].eq {
	pc = 0x82FFEFA4; continue 'dispatch;
	}
	// 82FFEF3C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEF40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFEF44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFEF48: 419A0008  beq cr6, 0x82ffef50
	if ctx.cr[6].eq {
	pc = 0x82FFEF50; continue 'dispatch;
	}
	// 82FFEF4C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82FFEF50: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEF54: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFEF58: 409A0008  bne cr6, 0x82ffef60
	if !ctx.cr[6].eq {
	pc = 0x82FFEF60; continue 'dispatch;
	}
	// 82FFEF5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFEF60: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFEF64: 409A0010  bne cr6, 0x82ffef74
	if !ctx.cr[6].eq {
	pc = 0x82FFEF74; continue 'dispatch;
	}
	// 82FFEF68: 387D0050  addi r3, r29, 0x50
	ctx.r[3].s64 = ctx.r[29].s64 + 80;
	// 82FFEF6C: 4BFFF07D  bl 0x82ffdfe8
	ctx.lr = 0x82FFEF70;
	sub_82FFDFE8(ctx, base);
	// 82FFEF70: 48000034  b 0x82ffefa4
	pc = 0x82FFEFA4; continue 'dispatch;
	// 82FFEF74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEF78: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFEF7C: 409A0008  bne cr6, 0x82ffef84
	if !ctx.cr[6].eq {
	pc = 0x82FFEF84; continue 'dispatch;
	}
	// 82FFEF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFEF84: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFEF88: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFEF8C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFEF90: 419A0014  beq cr6, 0x82ffefa4
	if ctx.cr[6].eq {
	pc = 0x82FFEFA4; continue 'dispatch;
	}
	// 82FFEF94: 815D008C  lwz r10, 0x8c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFEF98: 387D0050  addi r3, r29, 0x50
	ctx.r[3].s64 = ctx.r[29].s64 + 80;
	// 82FFEF9C: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FFEFA0: 4BFFF4A1  bl 0x82ffe440
	ctx.lr = 0x82FFEFA4;
	sub_82FFE440(ctx, base);
	// 82FFEFA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEFA8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEFAC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFEFB0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFEFB4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEFB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFEFBC: 93DE0000  stw r30, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFEFC0: 93DE0004  stw r30, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FFEFC4: 4B2670ED  bl 0x822660b0
	ctx.lr = 0x82FFEFC8;
	sub_822660B0(ctx, base);
	// 82FFEFC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFEFCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFEFD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FFEFD4: 387D00B8  addi r3, r29, 0xb8
	ctx.r[3].s64 = ctx.r[29].s64 + 184;
	// 82FFEFD8: 4BFFF341  bl 0x82ffe318
	ctx.lr = 0x82FFEFDC;
	sub_82FFE318(ctx, base);
	// 82FFEFDC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FFEFE0: 387D00F8  addi r3, r29, 0xf8
	ctx.r[3].s64 = ctx.r[29].s64 + 248;
	// 82FFEFE4: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFEFE8: 4BFFF331  bl 0x82ffe318
	ctx.lr = 0x82FFEFEC;
	sub_82FFE318(ctx, base);
	// 82FFEFEC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FFEFF0: 419A007C  beq cr6, 0x82fff06c
	if ctx.cr[6].eq {
	pc = 0x82FFF06C; continue 'dispatch;
	}
	// 82FFEFF4: 817D00AC  lwz r11, 0xac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFEFF8: 556B014B  rlwinm. r11, r11, 0, 5, 5
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEFFC: 40820070  bne 0x82fff06c
	if !ctx.cr[0].eq {
	pc = 0x82FFF06C; continue 'dispatch;
	}
	// 82FFF000: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF004: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFF008: 409A0014  bne cr6, 0x82fff01c
	if !ctx.cr[6].eq {
	pc = 0x82FFF01C; continue 'dispatch;
	}
	// 82FFF00C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFF010: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF014: 4BFFF9B5  bl 0x82ffe9c8
	ctx.lr = 0x82FFF018;
	sub_82FFE9C8(ctx, base);
	// 82FFF018: 48000054  b 0x82fff06c
	pc = 0x82FFF06C; continue 'dispatch;
	// 82FFF01C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF020: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFF024: 409A0008  bne cr6, 0x82fff02c
	if !ctx.cr[6].eq {
	pc = 0x82FFF02C; continue 'dispatch;
	}
	// 82FFF028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFF02C: 813D0010  lwz r9, 0x10(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF030: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82FFF034: E94A0014  ld r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	// 82FFF038: E91D0028  ld r8, 0x28(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) };
	// 82FFF03C: 38690218  addi r3, r9, 0x218
	ctx.r[3].s64 = ctx.r[9].s64 + 536;
	// 82FFF040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFF044: 7F285040  cmpld cr6, r8, r10
	ctx.cr[6].compare_u64(ctx.r[8].u64, ctx.r[10].u64, &mut ctx.xer);
	// 82FFF048: 41980008  blt cr6, 0x82fff050
	if ctx.cr[6].lt {
	pc = 0x82FFF050; continue 'dispatch;
	}
	// 82FFF04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFF050: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF054: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFF058: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82FFF05C: 419A000C  beq cr6, 0x82fff068
	if ctx.cr[6].eq {
	pc = 0x82FFF068; continue 'dispatch;
	}
	// 82FFF060: 4BFF79F1  bl 0x82ff6a50
	ctx.lr = 0x82FFF064;
	sub_82FF6A50(ctx, base);
	// 82FFF064: 48000008  b 0x82fff06c
	pc = 0x82FFF06C; continue 'dispatch;
	// 82FFF068: 4BFFF379  bl 0x82ffe3e0
	ctx.lr = 0x82FFF06C;
	sub_82FFE3E0(ctx, base);
	// 82FFF06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF070: 4BCAA3E4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF078 size=80
    let mut pc: u32 = 0x82FFF078;
    'dispatch: loop {
        match pc {
            0x82FFF078 => {
    //   block [0x82FFF078..0x82FFF0C8)
	// 82FFF078: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF07C: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82FFF080: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF084: 419A0024  beq cr6, 0x82fff0a8
	if ctx.cr[6].eq {
	pc = 0x82FFF0A8; continue 'dispatch;
	}
	// 82FFF088: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF08C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF090: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFF094: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF098: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF09C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFF0A0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF0A4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFF0A8: 39430040  addi r10, r3, 0x40
	ctx.r[10].s64 = ctx.r[3].s64 + 64;
	// 82FFF0AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFF0B0: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF0B4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFF0B8: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF0BC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF0C0: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82FFF0C4: 4BFFF7C4  b 0x82ffe888
	sub_82FFE888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF0C8 size=20
    let mut pc: u32 = 0x82FFF0C8;
    'dispatch: loop {
        match pc {
            0x82FFF0C8 => {
    //   block [0x82FFF0C8..0x82FFF0DC)
	// 82FFF0C8: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82FFF0CC: 39280048  addi r9, r8, 0x48
	ctx.r[9].s64 = ctx.r[8].s64 + 72;
	// 82FFF0D0: 81680048  lwz r11, 0x48(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFF0D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFF0D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF0DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF0DC size=164
    let mut pc: u32 = 0x82FFF0DC;
    'dispatch: loop {
        match pc {
            0x82FFF0DC => {
    //   block [0x82FFF0DC..0x82FFF180)
	// 82FFF0DC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF0E0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFF0E4: 409A0008  bne cr6, 0x82fff0ec
	if !ctx.cr[6].eq {
	pc = 0x82FFF0EC; continue 'dispatch;
	}
	// 82FFF0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF0EC: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 82FFF0F0: 8148008C  lwz r10, 0x8c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFF0F4: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82FFF0F8: 7D4A2050  subf r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82FFF0FC: 80CB0020  lwz r6, 0x20(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFF100: 7D465050  subf r10, r6, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[6].s64;
	// 82FFF104: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FFF108: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82FFF10C: 4098006C  bge cr6, 0x82fff178
	if !ctx.cr[6].lt {
	pc = 0x82FFF178; continue 'dispatch;
	}
	// 82FFF110: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF114: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFF118: 419A0024  beq cr6, 0x82fff13c
	if ctx.cr[6].eq {
	pc = 0x82FFF13C; continue 'dispatch;
	}
	// 82FFF11C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF120: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF124: 90C70004  stw r6, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FFF128: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF12C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF130: 90C70000  stw r6, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FFF134: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFF138: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFF13C: A14B0048  lhz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFF140: 554A0529  rlwinm. r10, r10, 0, 0x14, 0x14
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF144: 40820024  bne 0x82fff168
	if !ctx.cr[0].eq {
	pc = 0x82FFF168; continue 'dispatch;
	}
	// 82FFF148: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF14C: 80EA0010  lwz r7, 0x10(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF150: 81470238  lwz r10, 0x238(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(568 as u32) ) } as u64;
	// 82FFF154: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFF158: 91470238  stw r10, 0x238(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(568 as u32), ctx.r[10].u32 ) };
	// 82FFF15C: A14B0048  lhz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFF160: 614A0800  ori r10, r10, 0x800
	ctx.r[10].u64 = ctx.r[10].u64 | 2048;
	// 82FFF164: B14B0048  sth r10, 0x48(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u16 ) };
	// 82FFF168: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF16C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF170: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFF174: 409AFF68  bne cr6, 0x82fff0dc
	if !ctx.cr[6].eq {
	pc = 0x82FFF0DC; continue 'dispatch;
	}
	// 82FFF178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFF17C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF180 size=20
    let mut pc: u32 = 0x82FFF180;
    'dispatch: loop {
        match pc {
            0x82FFF180 => {
    //   block [0x82FFF180..0x82FFF194)
	// 82FFF180: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFF184: 38680050  addi r3, r8, 0x50
	ctx.r[3].s64 = ctx.r[8].s64 + 80;
	// 82FFF188: 8148008C  lwz r10, 0x8c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFF18C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFF190: 4BFFF2B0  b 0x82ffe440
	sub_82FFE440(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF194 size=4
    let mut pc: u32 = 0x82FFF194;
    'dispatch: loop {
        match pc {
            0x82FFF194 => {
    //   block [0x82FFF194..0x82FFF198)
	// 82FFF194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF198 size=308
    let mut pc: u32 = 0x82FFF198;
    'dispatch: loop {
        match pc {
            0x82FFF198 => {
    //   block [0x82FFF198..0x82FFF2CC)
	// 82FFF198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF19C: 4BCAA25D  bl 0x82ca93f8
	ctx.lr = 0x82FFF1A0;
	sub_82CA93D0(ctx, base);
	// 82FFF1A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF1A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF1A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FFF1AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82FFF1B0: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82FFF1B4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82FFF1B8: 396BA368  addi r11, r11, -0x5c98
	ctx.r[11].s64 = ctx.r[11].s64 + -23704;
	// 82FFF1BC: 394AB270  addi r10, r10, -0x4d90
	ctx.r[10].s64 = ctx.r[10].s64 + -19856;
	// 82FFF1C0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FFF1C4: 3929B260  addi r9, r9, -0x4da0
	ctx.r[9].s64 = ctx.r[9].s64 + -19872;
	// 82FFF1C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFF1CC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFF1D0: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 82FFF1D4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FFF1D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFF1DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF1E0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FFF1E4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FFF1E8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FFF1EC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82FFF1F0: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82FFF1F4: 4BFF91CD  bl 0x82ff83c0
	ctx.lr = 0x82FFF1F8;
	sub_82FF83C0(ctx, base);
	// 82FFF1F8: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82FFF1FC: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82FFF200: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 82FFF204: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 82FFF208: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82FFF20C: 931F00A0  stw r24, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[24].u32 ) };
	// 82FFF210: 393F0038  addi r9, r31, 0x38
	ctx.r[9].s64 = ctx.r[31].s64 + 56;
	// 82FFF214: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFF218: 391F0040  addi r8, r31, 0x40
	ctx.r[8].s64 = ctx.r[31].s64 + 64;
	// 82FFF21C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FFF220: 397F0048  addi r11, r31, 0x48
	ctx.r[11].s64 = ctx.r[31].s64 + 72;
	// 82FFF224: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82FFF228: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FFF22C: 939F0098  stw r28, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[28].u32 ) };
	// 82FFF230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF234: 937F009C  stw r27, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[27].u32 ) };
	// 82FFF238: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82FFF23C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82FFF240: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82FFF244: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82FFF248: 911F0040  stw r8, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82FFF24C: 911F0044  stw r8, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 82FFF250: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82FFF254: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82FFF258: 4BFFEC81  bl 0x82ffded8
	ctx.lr = 0x82FFF25C;
	sub_82FFDED8(ctx, base);
	// 82FFF25C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82FFF260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF264: 3BCAB228  addi r30, r10, -0x4dd8
	ctx.r[30].s64 = ctx.r[10].s64 + -19928;
	// 82FFF268: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82FFF26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FFF270: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFF274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF278: 81590044  lwz r10, 0x44(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF27C: 915F008C  stw r10, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 82FFF280: 81590048  lwz r10, 0x48(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FFF284: 915F0090  stw r10, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82FFF288: 8159004C  lwz r10, 0x4c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FFF28C: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82FFF290: B17F00A8  sth r11, 0xa8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u16 ) };
	// 82FFF294: B17F00AA  sth r11, 0xaa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(170 as u32), ctx.r[11].u16 ) };
	// 82FFF298: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FFF29C: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82FFF2A0: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82FFF2A4: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82FFF2A8: 4BFFEF99  bl 0x82ffe240
	ctx.lr = 0x82FFF2AC;
	sub_82FFE240(ctx, base);
	// 82FFF2AC: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 82FFF2B0: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 82FFF2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FFF2B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF2BC: 4BFFEF85  bl 0x82ffe240
	ctx.lr = 0x82FFF2C0;
	sub_82FFE240(ctx, base);
	// 82FFF2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF2C4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFF2C8: 4BCAA180  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF2D0 size=76
    let mut pc: u32 = 0x82FFF2D0;
    'dispatch: loop {
        match pc {
            0x82FFF2D0 => {
    //   block [0x82FFF2D0..0x82FFF31C)
	// 82FFF2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF2D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFF2DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF2E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF2E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF2E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFF2EC: 4BFFF72D  bl 0x82ffea18
	ctx.lr = 0x82FFF2F0;
	sub_82FFEA18(ctx, base);
	// 82FFF2F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF2F4: 4182000C  beq 0x82fff300
	if ctx.cr[0].eq {
	pc = 0x82FFF300; continue 'dispatch;
	}
	// 82FFF2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF2FC: 4B8464B5  bl 0x828457b0
	ctx.lr = 0x82FFF300;
	sub_828457B0(ctx, base);
	// 82FFF300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFF308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFF314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFF318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF320 size=216
    let mut pc: u32 = 0x82FFF320;
    'dispatch: loop {
        match pc {
            0x82FFF320 => {
    //   block [0x82FFF320..0x82FFF3F8)
	// 82FFF320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF324: 4BCAA0D5  bl 0x82ca93f8
	ctx.lr = 0x82FFF328;
	sub_82CA93D0(ctx, base);
	// 82FFF328: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF32C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF330: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFF334: 38800138  li r4, 0x138
	ctx.r[4].s64 = 312;
	// 82FFF338: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 82FFF33C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FFF340: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FFF344: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82FFF348: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82FFF34C: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 82FFF350: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82FFF354: 4BFFCEC5  bl 0x82ffc218
	ctx.lr = 0x82FFF358;
	sub_82FFC218(ctx, base);
	// 82FFF358: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF35C: 40820010  bne 0x82fff36c
	if !ctx.cr[0].eq {
	pc = 0x82FFF36C; continue 'dispatch;
	}
	// 82FFF360: 3FE08007  lis r31, -0x7ff9
	ctx.r[31].s64 = -2147024896;
	// 82FFF364: 63FF000E  ori r31, r31, 0xe
	ctx.r[31].u64 = ctx.r[31].u64 | 14;
	// 82FFF368: 48000084  b 0x82fff3ec
	pc = 0x82FFF3EC; continue 'dispatch;
	// 82FFF36C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82FFF370: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82FFF374: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FFF378: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFF37C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFF380: 4BFFFE19  bl 0x82fff198
	ctx.lr = 0x82FFF384;
	sub_82FFF198(ctx, base);
	// 82FFF384: 811F0230  lwz r8, 0x230(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(560 as u32) ) } as u64;
	// 82FFF388: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFF38C: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 82FFF390: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FFF394: 917F0230  stw r11, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[11].u32 ) };
	// 82FFF398: 409A0048  bne cr6, 0x82fff3e0
	if !ctx.cr[6].eq {
	pc = 0x82FFF3E0; continue 'dispatch;
	}
	// 82FFF39C: E97F0228  ld r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) };
	// 82FFF3A0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82FFF3A4: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82FFF3A8: 81410104  lwz r10, 0x104(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82FFF3AC: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FFF3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82FFF3B4: 79640140  clrldi r4, r11, 5
	ctx.r[4].u64 = ctx.r[11].u64 & 0x07FFFFFFFFFFFFFFu64;
	// 82FFF3B8: F8DF0228  std r6, 0x228(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[6].u64 ) };
	// 82FFF3BC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82FFF3C0: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82FFF3C4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FFF3C8: 48003221  bl 0x830025e8
	ctx.lr = 0x82FFF3CC;
	sub_830025E8(ctx, base);
	// 82FFF3CC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFF3D0: 40800010  bge 0x82fff3e0
	if !ctx.cr[0].lt {
	pc = 0x82FFF3E0; continue 'dispatch;
	}
	// 82FFF3D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFF3D8: 4BFF5441  bl 0x82ff4818
	ctx.lr = 0x82FFF3DC;
	sub_82FF4818(ctx, base);
	// 82FFF3DC: 48000010  b 0x82fff3ec
	pc = 0x82FFF3EC; continue 'dispatch;
	// 82FFF3E0: 8161010C  lwz r11, 0x10c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(268 as u32) ) } as u64;
	// 82FFF3E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FFF3E8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FFF3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF3F0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FFF3F4: 4BCAA054  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF3F8 size=284
    let mut pc: u32 = 0x82FFF3F8;
    'dispatch: loop {
        match pc {
            0x82FFF3F8 => {
    //   block [0x82FFF3F8..0x82FFF514)
	// 82FFF3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF3FC: 4BCAA00D  bl 0x82ca9408
	ctx.lr = 0x82FFF400;
	sub_82CA93D0(ctx, base);
	// 82FFF400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF404: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFF408: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FFF40C: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82FFF410: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFF414: 48000038  b 0x82fff44c
	pc = 0x82FFF44C; continue 'dispatch;
	// 82FFF418: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF41C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF420: 409A0008  bne cr6, 0x82fff428
	if !ctx.cr[6].eq {
	pc = 0x82FFF428; continue 'dispatch;
	}
	// 82FFF424: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FFF428: A14B0044  lhz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF42C: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 82FFF430: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF434: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFF438: 5545AFFE  rlwinm r5, r10, 0x15, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x000007FFu64;
	// 82FFF43C: 4BFFFA45  bl 0x82ffee80
	ctx.lr = 0x82FFF440;
	sub_82FFEE80(ctx, base);
	// 82FFF440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF444: 48002925  bl 0x83001d68
	ctx.lr = 0x82FFF448;
	sub_83001D68(ctx, base);
	// 82FFF448: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF44C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF450: 409AFFC8  bne cr6, 0x82fff418
	if !ctx.cr[6].eq {
	pc = 0x82FFF418; continue 'dispatch;
	}
	// 82FFF454: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFF458: 3BFD0038  addi r31, r29, 0x38
	ctx.r[31].s64 = ctx.r[29].s64 + 56;
	// 82FFF45C: 48000044  b 0x82fff4a0
	pc = 0x82FFF4A0; continue 'dispatch;
	// 82FFF460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF464: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF468: 409A000C  bne cr6, 0x82fff474
	if !ctx.cr[6].eq {
	pc = 0x82FFF474; continue 'dispatch;
	}
	// 82FFF46C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FFF470: 48000024  b 0x82fff494
	pc = 0x82FFF494; continue 'dispatch;
	// 82FFF474: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF478: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF47C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFF480: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF484: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF488: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFF48C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF490: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFF494: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82FFF498: 480028D1  bl 0x83001d68
	ctx.lr = 0x82FFF49C;
	sub_83001D68(ctx, base);
	// 82FFF49C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF4A0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF4A4: 409AFFBC  bne cr6, 0x82fff460
	if !ctx.cr[6].eq {
	pc = 0x82FFF460; continue 'dispatch;
	}
	// 82FFF4A8: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FFF4AC: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
	// 82FFF4B0: 48000044  b 0x82fff4f4
	pc = 0x82FFF4F4; continue 'dispatch;
	// 82FFF4B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF4B8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF4BC: 409A000C  bne cr6, 0x82fff4c8
	if !ctx.cr[6].eq {
	pc = 0x82FFF4C8; continue 'dispatch;
	}
	// 82FFF4C0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FFF4C4: 48000024  b 0x82fff4e8
	pc = 0x82FFF4E8; continue 'dispatch;
	// 82FFF4C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF4CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF4D0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFF4D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF4D8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF4DC: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFF4E0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF4E4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFF4E8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82FFF4EC: 4800287D  bl 0x83001d68
	ctx.lr = 0x82FFF4F0;
	sub_83001D68(ctx, base);
	// 82FFF4F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF4F4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FFF4F8: 409AFFBC  bne cr6, 0x82fff4b4
	if !ctx.cr[6].eq {
	pc = 0x82FFF4B4; continue 'dispatch;
	}
	// 82FFF4FC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82FFF500: 939D00B0  stw r28, 0xb0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82FFF504: 939D00B4  stw r28, 0xb4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 82FFF508: 917D00A4  stw r11, 0xa4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82FFF50C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF510: 4BCA9F48  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF518 size=68
    let mut pc: u32 = 0x82FFF518;
    'dispatch: loop {
        match pc {
            0x82FFF518 => {
    //   block [0x82FFF518..0x82FFF55C)
	// 82FFF518: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFF51C: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF520: 419A001C  beq cr6, 0x82fff53c
	if ctx.cr[6].eq {
	pc = 0x82FFF53C; continue 'dispatch;
	}
	// 82FFF524: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF528: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF52C: 40820010  bne 0x82fff53c
	if !ctx.cr[0].eq {
	pc = 0x82FFF53C; continue 'dispatch;
	}
	// 82FFF530: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF534: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF538: 409AFFEC  bne cr6, 0x82fff524
	if !ctx.cr[6].eq {
	pc = 0x82FFF524; continue 'dispatch;
	}
	// 82FFF53C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF540: 40980014  bge cr6, 0x82fff554
	if !ctx.cr[6].lt {
	pc = 0x82FFF554; continue 'dispatch;
	}
	// 82FFF544: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FFF548: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF54C: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FFF550: 4099000C  ble cr6, 0x82fff55c
	if !ctx.cr[6].gt {
		sub_82FFF55C(ctx, base);
		return;
	}
	// 82FFF554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF55C size=12
    let mut pc: u32 = 0x82FFF55C;
    'dispatch: loop {
        match pc {
            0x82FFF55C => {
    //   block [0x82FFF55C..0x82FFF568)
	// 82FFF55C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF560: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFF564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF568 size=348
    let mut pc: u32 = 0x82FFF568;
    'dispatch: loop {
        match pc {
            0x82FFF568 => {
    //   block [0x82FFF568..0x82FFF6C4)
	// 82FFF568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF56C: 4BCA9E9D  bl 0x82ca9408
	ctx.lr = 0x82FFF570;
	sub_82CA93D0(ctx, base);
	// 82FFF570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF578: 7C870734  extsh r7, r4
	ctx.r[7].s64 = ctx.r[4].s16 as i64;
	// 82FFF57C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF580: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82FFF584: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82FFF588: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82FFF58C: 4098012C  bge cr6, 0x82fff6b8
	if !ctx.cr[6].lt {
	pc = 0x82FFF6B8; continue 'dispatch;
	}
	// 82FFF590: 2F0BFF00  cmpwi cr6, r11, -0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, -256, &mut ctx.xer);
	// 82FFF594: 41980124  blt cr6, 0x82fff6b8
	if ctx.cr[6].lt {
	pc = 0x82FFF6B8; continue 'dispatch;
	}
	// 82FFF598: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF59C: 4098000C  bge cr6, 0x82fff5a8
	if !ctx.cr[6].lt {
	pc = 0x82FFF5A8; continue 'dispatch;
	}
	// 82FFF5A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFF5A4: 48000118  b 0x82fff6bc
	pc = 0x82FFF6BC; continue 'dispatch;
	// 82FFF5A8: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF5AC: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FFF5B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF5B4: 7D6A4050  subf r11, r10, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82FFF5B8: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82FFF5BC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFF5C0: 556AE8FE  srwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFF5C4: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82FFF5C8: 419A004C  beq cr6, 0x82fff614
	if ctx.cr[6].eq {
	pc = 0x82FFF614; continue 'dispatch;
	}
	// 82FFF5CC: 7CCAFA14  add r6, r10, r31
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82FFF5D0: 7F895830  slw r9, r28, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[28].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF5D4: 89660006  lbz r11, 6(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FFF5D8: 7D6B4839  and. r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF5DC: 4182000C  beq 0x82fff5e8
	if ctx.cr[0].eq {
	pc = 0x82FFF5E8; continue 'dispatch;
	}
	// 82FFF5E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFF5E4: 480000D8  b 0x82fff6bc
	pc = 0x82FFF6BC; continue 'dispatch;
	// 82FFF5E8: 397F0006  addi r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 + 6;
	// 82FFF5EC: 7CCA58AE  lbzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFF5F0: 7CC94B78  or r9, r6, r9
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	// 82FFF5F4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82FFF5F8: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FFF5FC: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82FFF600: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF604: 418000AC  blt 0x82fff6b0
	if ctx.cr[0].lt {
	pc = 0x82FFF6B0; continue 'dispatch;
	}
	// 82FFF608: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 82FFF60C: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82FFF610: 480000A0  b 0x82fff6b0
	pc = 0x82FFF6B0; continue 'dispatch;
	// 82FFF614: 3BDF0006  addi r30, r31, 6
	ctx.r[30].s64 = ctx.r[31].s64 + 6;
	// 82FFF618: 7F8B5830  slw r11, r28, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[28].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF61C: 7D2AF0AE  lbzx r9, r10, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FFF620: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82FFF624: 7D6AF1AE  stbx r11, r10, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u8) };
	// 82FFF628: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF62C: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FFF630: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF634: 409A000C  bne cr6, 0x82fff640
	if !ctx.cr[6].eq {
	pc = 0x82FFF640; continue 'dispatch;
	}
	// 82FFF638: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF63C: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82FFF640: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF644: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF648: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFF64C: 556A063F  clrlwi. r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF650: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FFF654: 40820030  bne 0x82fff684
	if !ctx.cr[0].eq {
	pc = 0x82FFF684; continue 'dispatch;
	}
	// 82FFF658: 3BBE0020  addi r29, r30, 0x20
	ctx.r[29].s64 = ctx.r[30].s64 + 32;
	// 82FFF65C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FFF660: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFF664: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF668: 4BCA9E19  bl 0x82ca9480
	ctx.lr = 0x82FFF66C;
	sub_82CA9480(ctx, base);
	// 82FFF66C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FFF670: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFF674: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF678: 4BCAA339  bl 0x82ca99b0
	ctx.lr = 0x82FFF67C;
	sub_82CA99B0(ctx, base);
	// 82FFF67C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF680: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFF684: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF688: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF68C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FFF690: 556A043E  clrlwi r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFF694: 554BE8FE  srwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFF698: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFF69C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FFF6A0: 7F8A5030  slw r10, r28, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[28].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF6A4: 896B0006  lbz r11, 6(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FFF6A8: 7D4B5839  and. r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF6AC: 4082FF94  bne 0x82fff640
	if !ctx.cr[0].eq {
	pc = 0x82FFF640; continue 'dispatch;
	}
	// 82FFF6B0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82FFF6B4: 48000008  b 0x82fff6bc
	pc = 0x82FFF6BC; continue 'dispatch;
	// 82FFF6B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF6C0: 4BCA9D98  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF6C8 size=204
    let mut pc: u32 = 0x82FFF6C8;
    'dispatch: loop {
        match pc {
            0x82FFF6C8 => {
    //   block [0x82FFF6C8..0x82FFF794)
	// 82FFF6C8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FFF6CC: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF6D0: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82FFF6D4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FFF6D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFF6DC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82FFF6E0: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FFF6E8: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82FFF6EC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82FFF6F0: A0E30004  lhz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF6F4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF6F8: 7CE75050  subf r7, r7, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82FFF6FC: 54FF043E  clrlwi r31, r7, 0x10
	ctx.r[31].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82FFF700: 57E7E8FE  srwi r7, r31, 3
	ctx.r[7].u32 = ctx.r[31].u32.wrapping_shr(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FFF704: 57FF077E  clrlwi r31, r31, 0x1d
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0x00000007u64;
	// 82FFF708: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82FFF70C: 7CDFF830  slw r31, r6, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[6].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF710: 88E70006  lbz r7, 6(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FFF714: 7FE73839  and. r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 & ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FFF718: 41820014  beq 0x82fff72c
	if ctx.cr[0].eq {
	pc = 0x82FFF72C; continue 'dispatch;
	}
	// 82FFF71C: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82FFF720: 7CC75830  slw r7, r6, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF724: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82FFF728: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82FFF72C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFF730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFF734: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF738: 419A0014  beq cr6, 0x82fff74c
	if ctx.cr[6].eq {
	pc = 0x82FFF74C; continue 'dispatch;
	}
	// 82FFF73C: A0E30002  lhz r7, 2(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FFF740: 555F043E  clrlwi r31, r10, 0x10
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF744: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82FFF748: 409A002C  bne cr6, 0x82fff774
	if !ctx.cr[6].eq {
	pc = 0x82FFF774; continue 'dispatch;
	}
	// 82FFF74C: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FFF750: 419A002C  beq cr6, 0x82fff77c
	if ctx.cr[6].eq {
	pc = 0x82FFF77C; continue 'dispatch;
	}
	// 82FFF754: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FFF758: 5547043E  clrlwi r7, r10, 0x10
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF75C: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF760: 419A001C  beq cr6, 0x82fff77c
	if ctx.cr[6].eq {
	pc = 0x82FFF77C; continue 'dispatch;
	}
	// 82FFF764: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82FFF768: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FFF76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FFF770: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82FFF774: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFF778: 4BFFFF78  b 0x82fff6f0
	pc = 0x82FFF6F0; continue 'dispatch;
	// 82FFF77C: 7D644850  subf r11, r4, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 82FFF780: 50C83830  rlwimi r8, r6, 7, 0, 0x18
	ctx.r[8].u64 = (((ctx.r[6].u32).rotate_left(7) as u64) & 0x00000000FFFFFF80) | (ctx.r[8].u64 & 0xFFFFFFFF0000007F);
	// 82FFF784: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82FFF788: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82FFF78C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82FFF790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF798 size=56
    let mut pc: u32 = 0x82FFF798;
    'dispatch: loop {
        match pc {
            0x82FFF798 => {
    //   block [0x82FFF798..0x82FFF7D0)
	// 82FFF798: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FFF79C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FFF7A0: 1D460007  mulli r10, r6, 7
	ctx.r[10].s64 = ctx.r[6].s64 * 7;
	// 82FFF7A4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FFF7A8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FFF7AC: 7D053214  add r8, r5, r6
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82FFF7B0: B1230002  sth r9, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82FFF7B4: 556A043E  clrlwi r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFF7B8: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82FFF7BC: B1430000  sth r10, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FFF7C0: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82FFF7C4: 8968FFFF  lbz r11, -1(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82FFF7C8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFF7CC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF7D0 size=36
    let mut pc: u32 = 0x82FFF7D0;
    'dispatch: loop {
        match pc {
            0x82FFF7D0 => {
    //   block [0x82FFF7D0..0x82FFF7F4)
	// 82FFF7D0: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF7D4: 556BFE7E  rlwinm r11, r11, 0x1f, 0x19, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82FFF7D8: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 82FFF7DC: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFF7E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82FFF7E4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFF7E8: 4182FFE8  beq 0x82fff7d0
	if ctx.cr[0].eq {
	pc = 0x82FFF7D0; continue 'dispatch;
	}
	// 82FFF7EC: B1430000  sth r10, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FFF7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF7F8 size=32
    let mut pc: u32 = 0x82FFF7F8;
    'dispatch: loop {
        match pc {
            0x82FFF7F8 => {
    //   block [0x82FFF7F8..0x82FFF818)
	// 82FFF7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF7FC: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82FFF800: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FFF804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFF808: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82FFF80C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFF810: 38630006  addi r3, r3, 6
	ctx.r[3].s64 = ctx.r[3].s64 + 6;
	// 82FFF814: 4BCAA19C  b 0x82ca99b0
	sub_82CA99B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF818 size=84
    let mut pc: u32 = 0x82FFF818;
    'dispatch: loop {
        match pc {
            0x82FFF818 => {
    //   block [0x82FFF818..0x82FFF86C)
	// 82FFF818: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFF81C: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF820: 419A001C  beq cr6, 0x82fff83c
	if ctx.cr[6].eq {
	pc = 0x82FFF83C; continue 'dispatch;
	}
	// 82FFF824: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF828: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF82C: 40820010  bne 0x82fff83c
	if !ctx.cr[0].eq {
	pc = 0x82FFF83C; continue 'dispatch;
	}
	// 82FFF830: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF834: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF838: 409AFFEC  bne cr6, 0x82fff824
	if !ctx.cr[6].eq {
	pc = 0x82FFF824; continue 'dispatch;
	}
	// 82FFF83C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFF840: 4098002C  bge cr6, 0x82fff86c
	if !ctx.cr[6].lt {
		sub_82FFF86C(ctx, base);
		return;
	}
	// 82FFF844: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF848: 554A067F  clrlwi. r10, r10, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF84C: 41820020  beq 0x82fff86c
	if ctx.cr[0].eq {
		sub_82FFF86C(ctx, base);
		return;
	}
	// 82FFF850: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FFF854: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF858: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FFF85C: 41990010  bgt cr6, 0x82fff86c
	if ctx.cr[6].gt {
		sub_82FFF86C(ctx, base);
		return;
	}
	// 82FFF860: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF864: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFF868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF86C size=8
    let mut pc: u32 = 0x82FFF86C;
    'dispatch: loop {
        match pc {
            0x82FFF86C => {
    //   block [0x82FFF86C..0x82FFF874)
	// 82FFF86C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF878 size=52
    let mut pc: u32 = 0x82FFF878;
    'dispatch: loop {
        match pc {
            0x82FFF878 => {
    //   block [0x82FFF878..0x82FFF8AC)
	// 82FFF878: 1D460007  mulli r10, r6, 7
	ctx.r[10].s64 = ctx.r[6].s64 * 7;
	// 82FFF87C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FFF880: 5489043E  clrlwi r9, r4, 0x10
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FFF884: 7D653214  add r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82FFF888: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82FFF88C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FFF890: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FFF894: B123000A  sth r9, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[9].u16 ) };
	// 82FFF898: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFF89C: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82FFF8A0: 896BFFFF  lbz r11, -1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82FFF8A4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF8A8: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF8AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF8AC size=48
    let mut pc: u32 = 0x82FFF8AC;
    'dispatch: loop {
        match pc {
            0x82FFF8AC => {
    //   block [0x82FFF8AC..0x82FFF8DC)
	// 82FFF8AC: A163000A  lhz r11, 0xa(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FFF8B0: 89430008  lbz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF8B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFF8B8: 554A0E3C  rlwinm r10, r10, 1, 0x18, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82FFF8BC: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82FFF8C0: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FFF8C4: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82FFF8C8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF8CC: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF8D0: 7D4B5839  and. r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF8D4: 4182FFD8  beq 0x82fff8ac
	if ctx.cr[0].eq {
	pc = 0x82FFF8AC; continue 'dispatch;
	}
	// 82FFF8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FFF8E0 size=208
    let mut pc: u32 = 0x82FFF8E0;
    'dispatch: loop {
        match pc {
            0x82FFF8E0 => {
    //   block [0x82FFF8E0..0x82FFF9B0)
	// 82FFF8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF8E4: 4BCA9B25  bl 0x82ca9408
	ctx.lr = 0x82FFF8E8;
	sub_82CA93D0(ctx, base);
	// 82FFF8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF8EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFF8F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFF8F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFF8F8: 57BFE8FF  rlwinm. r31, r29, 0x1d, 3, 0x1f
	ctx.r[31].u64 = ctx.r[29].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFF8FC: 4182002C  beq 0x82fff928
	if ctx.cr[0].eq {
	pc = 0x82FFF928; continue 'dispatch;
	}
	// 82FFF900: 20BF0020  subfic r5, r31, 0x20
	ctx.xer.ca = ctx.r[31].u32 <= 32 as u32;
	ctx.r[5].s64 = (32 as i64) - ctx.r[31].s64;
	// 82FFF904: 7C9FF214  add r4, r31, r30
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82FFF908: 4BCB1229  bl 0x82cb0b30
	ctx.lr = 0x82FFF90C;
	sub_82CB0B30(ctx, base);
	// 82FFF90C: 215C0000  subfic r10, r28, 0
	ctx.xer.ca = ctx.r[28].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[28].s64;
	// 82FFF910: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82FFF914: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FFF918: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFF91C: 5544063E  clrlwi r4, r10, 0x18
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FFF920: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82FFF924: 4BCAA08D  bl 0x82ca99b0
	ctx.lr = 0x82FFF928;
	sub_82CA99B0(ctx, base);
	// 82FFF928: 57A9077F  clrlwi. r9, r29, 0x1d
	ctx.r[9].u64 = ctx.r[29].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFF92C: 4182007C  beq 0x82fff9a8
	if ctx.cr[0].eq {
	pc = 0x82FFF9A8; continue 'dispatch;
	}
	// 82FFF930: 38DE001F  addi r6, r30, 0x1f
	ctx.r[6].s64 = ctx.r[30].s64 + 31;
	// 82FFF934: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FFF938: 7F1E3040  cmplw cr6, r30, r6
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82FFF93C: 4098003C  bge cr6, 0x82fff978
	if !ctx.cr[6].lt {
	pc = 0x82FFF978; continue 'dispatch;
	}
	// 82FFF940: 21090008  subfic r8, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[9].s64;
	// 82FFF944: 5527063E  clrlwi r7, r9, 0x18
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82FFF948: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82FFF94C: 54E4063E  clrlwi r4, r7, 0x18
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82FFF950: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF954: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FFF958: 7C634030  slw r3, r3, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[3].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF95C: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FFF960: 7CA52430  srw r5, r5, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[5].u32) >> ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF964: 7C652B78  or r5, r3, r5
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[5].u64;
	// 82FFF968: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82FFF96C: 98AB0000  stb r5, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 82FFF970: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82FFF974: 4198FFD4  blt cr6, 0x82fff948
	if ctx.cr[6].lt {
	pc = 0x82FFF948; continue 'dispatch;
	}
	// 82FFF978: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF97C: 5528063E  clrlwi r8, r9, 0x18
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82FFF980: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FFF984: 7D4A4430  srw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF988: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82FFF98C: 419A001C  beq cr6, 0x82fff9a8
	if ctx.cr[6].eq {
	pc = 0x82FFF9A8; continue 'dispatch;
	}
	// 82FFF990: 21290008  subfic r9, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[9].s64;
	// 82FFF994: 390000FF  li r8, 0xff
	ctx.r[8].s64 = 255;
	// 82FFF998: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FFF99C: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF9A0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FFF9A4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82FFF9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF9AC: 4BCA9AAC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF9B0 size=60
    let mut pc: u32 = 0x82FFF9B0;
    'dispatch: loop {
        match pc {
            0x82FFF9B0 => {
    //   block [0x82FFF9B0..0x82FFF9EC)
	// 82FFF9B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFF9B4: 7C890734  extsh r9, r4
	ctx.r[9].s64 = ctx.r[4].s16 as i64;
	// 82FFF9B8: A14B0024  lhz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFF9BC: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82FFF9C0: 7D4A0735  extsh. r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF9C4: 40810038  ble 0x82fff9fc
	if !ctx.cr[0].gt {
		sub_82FFF9EC(ctx, base);
		return;
	}
	// 82FFF9C8: A14B0026  lhz r10, 0x26(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(38 as u32) ) } as u64;
	// 82FFF9CC: 55480021  rlwinm. r8, r10, 0, 0, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FFF9D0: 4082001C  bne 0x82fff9ec
	if !ctx.cr[0].eq {
		sub_82FFF9EC(ctx, base);
		return;
	}
	// 82FFF9D4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82FFF9D8: B08B0020  sth r4, 0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[4].u16 ) };
	// 82FFF9DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFF9E0: 61298000  ori r9, r9, 0x8000
	ctx.r[9].u64 = ctx.r[9].u64 | 32768;
	// 82FFF9E4: B12B0026  sth r9, 0x26(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(38 as u32), ctx.r[9].u16 ) };
	// 82FFF9E8: 4800003C  b 0x82fffa24
	sub_82FFFA04(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF9EC size=24
    let mut pc: u32 = 0x82FFF9EC;
    'dispatch: loop {
        match pc {
            0x82FFF9EC => {
    //   block [0x82FFF9EC..0x82FFFA04)
	// 82FFF9EC: A14B0020  lhz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFF9F0: 7D0A4850  subf r8, r10, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82FFF9F4: 7D080735  extsh. r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FFF9F8: 4080000C  bge 0x82fffa04
	if !ctx.cr[0].lt {
		sub_82FFFA04(ctx, base);
		return;
	}
	// 82FFF9FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFFA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFA04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFFA04 size=96
    let mut pc: u32 = 0x82FFFA04;
    'dispatch: loop {
        match pc {
            0x82FFFA04 => {
    //   block [0x82FFFA04..0x82FFFA64)
	// 82FFFA04: A10B0022  lhz r8, 0x22(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFA08: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFA0C: 5487043E  clrlwi r7, r4, 0x10
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FFFA10: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82FFFA14: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82FFFA18: 7D290735  extsh. r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFFA1C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFA20: 40810008  ble 0x82fffa28
	if !ctx.cr[0].gt {
	pc = 0x82FFFA28; continue 'dispatch;
	}
	// 82FFFA24: B08B0022  sth r4, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[4].u16 ) };
	// 82FFFA28: 5547ECFE  rlwinm r7, r10, 0x1d, 0x13, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFA2C: 5549ECFE  rlwinm r9, r10, 0x1d, 0x13, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFA30: 5546077E  clrlwi r6, r10, 0x1d
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFA34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FFFA38: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFA3C: 7CE758AE  lbzx r7, r7, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFFA40: 7D063030  slw r6, r8, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFA44: 7D0A5030  slw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFA48: 7CA958AE  lbzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFFA4C: 7CC83838  and r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 & ctx.r[7].u64;
	// 82FFFA50: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 82FFFA54: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82FFFA58: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82FFFA5C: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82FFFA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFA68 size=332
    let mut pc: u32 = 0x82FFFA68;
    'dispatch: loop {
        match pc {
            0x82FFFA68 => {
    //   block [0x82FFFA68..0x82FFFBB4)
	// 82FFFA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFA70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFA74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFA78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFA7C: A17F0026  lhz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 82FFFA80: 556B0021  rlwinm. r11, r11, 0, 0, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFFA84: 4182011C  beq 0x82fffba0
	if ctx.cr[0].eq {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFA88: A15F0020  lhz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFFA8C: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 82FFFA90: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FFFA94: 7D290735  extsh. r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFFA98: 41800108  blt 0x82fffba0
	if ctx.cr[0].lt {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFA9C: A11F0022  lhz r8, 0x22(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFAA0: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82FFFAA4: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFFAA8: 418100F8  bgt 0x82fffba0
	if ctx.cr[0].gt {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFAAC: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFAB0: 5489043E  clrlwi r9, r4, 0x10
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FFFAB4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82FFFAB8: 556A043F  clrlwi. r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFFABC: 40820070  bne 0x82fffb2c
	if !ctx.cr[0].eq {
	pc = 0x82FFFB2C; continue 'dispatch;
	}
	// 82FFFAC0: 550B043E  clrlwi r11, r8, 0x10
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82FFFAC4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFFAC8: 409A0020  bne cr6, 0x82fffae8
	if !ctx.cr[6].eq {
	pc = 0x82FFFAE8; continue 'dispatch;
	}
	// 82FFFACC: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FFFAD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFFAD4: 4BCA9EDD  bl 0x82ca99b0
	ctx.lr = 0x82FFFAD8;
	sub_82CA99B0(ctx, base);
	// 82FFFAD8: A17F0026  lhz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 82FFFADC: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82FFFAE0: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 82FFFAE4: 480000BC  b 0x82fffba0
	pc = 0x82FFFBA0; continue 'dispatch;
	// 82FFFAE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FFFAEC: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFFAF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFFAF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFFAF8: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFAFC: B17F0020  sth r11, 0x20(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82FFFB00: 5549E8FE  srwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFFB04: 554B077E  clrlwi r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFB08: 7D0B5830  slw r11, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFB0C: 7D29F8AE  lbzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FFFB10: 7D6B4839  and. r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFFB14: 4182FFD8  beq 0x82fffaec
	if ctx.cr[0].eq {
	pc = 0x82FFFAEC; continue 'dispatch;
	}
	// 82FFFB18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFB1C: 5544043E  clrlwi r4, r10, 0x10
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFB24: 4BFFFDBD  bl 0x82fff8e0
	ctx.lr = 0x82FFFB28;
	sub_82FFF8E0(ctx, base);
	// 82FFFB28: 48000078  b 0x82fffba0
	pc = 0x82FFFBA0; continue 'dispatch;
	// 82FFFB2C: 554BE8FE  srwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFFB30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FFFB34: 5547077E  clrlwi r7, r10, 0x1d
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFB38: 7D073830  slw r7, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFB3C: 7CCBF8AE  lbzx r6, r11, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FFFB40: 7CC73878  andc r7, r6, r7
	ctx.r[7].u64 = ctx.r[6].u64 & !ctx.r[7].u64;
	// 82FFFB44: 7CEBF9AE  stbx r7, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u8) };
	// 82FFFB48: A17F0022  lhz r11, 0x22(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFB4C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFFB50: 409A0050  bne cr6, 0x82fffba0
	if !ctx.cr[6].eq {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFB54: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFB58: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82FFFB5C: A0FF0022  lhz r7, 0x22(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFB60: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FFFB64: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FFFB68: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82FFFB6C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FFFB70: B0FF0022  sth r7, 0x22(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(34 as u32), ctx.r[7].u16 ) };
	// 82FFFB74: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82FFFB78: 5546E8FE  srwi r6, r10, 3
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82FFFB7C: 5547077E  clrlwi r7, r10, 0x1d
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82FFFB80: 7D073830  slw r7, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFB84: 7CC6F8AE  lbzx r6, r6, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FFFB88: 7CE73039  and. r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FFFB8C: 41820014  beq 0x82fffba0
	if ctx.cr[0].eq {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFB90: A0FF0022  lhz r7, 0x22(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFB94: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82FFFB98: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82FFFB9C: 419AFFC0  beq cr6, 0x82fffb5c
	if ctx.cr[6].eq {
	pc = 0x82FFFB5C; continue 'dispatch;
	}
	// 82FFFBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFFBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFBAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFBB8 size=220
    let mut pc: u32 = 0x82FFFBB8;
    'dispatch: loop {
        match pc {
            0x82FFFBB8 => {
    //   block [0x82FFFBB8..0x82FFFC94)
	// 82FFFBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFBBC: 4BCA9851  bl 0x82ca940c
	ctx.lr = 0x82FFFBC0;
	sub_82CA93D0(ctx, base);
	// 82FFFBC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFBC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFFBC8: 54AA043E  clrlwi r10, r5, 0x10
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 82FFFBCC: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82FFFBD0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFFBD4: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82FFFBD8: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFFBDC: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82FFFBE0: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82FFFBE4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FFFBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFFBEC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FFFBF0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FFFBF4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFFBF8: 5566ECFE  rlwinm r6, r11, 0x1d, 0x13, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82FFFBFC: 5565077E  clrlwi r5, r11, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82FFFC00: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFFC04: 7CE52830  slw r5, r7, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFFC08: 7CC6F0AE  lbzx r6, r6, r30
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FFFC0C: 7CA63039  and. r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FFFC10: 41820010  beq 0x82fffc20
	if ctx.cr[0].eq {
	pc = 0x82FFFC20; continue 'dispatch;
	}
	// 82FFFC14: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FFFC18: 5506063E  clrlwi r6, r8, 0x18
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82FFFC1C: 7D4A3378  or r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 82FFFC20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFFC24: 419A0034  beq cr6, 0x82fffc58
	if ctx.cr[6].eq {
	pc = 0x82FFFC58; continue 'dispatch;
	}
	// 82FFFC28: 5508FE7F  rlwinm. r8, r8, 0x1f, 0x19, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FFFC2C: 40820014  bne 0x82fffc40
	if !ctx.cr[0].eq {
	pc = 0x82FFFC40; continue 'dispatch;
	}
	// 82FFFC30: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82FFFC34: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FFFC38: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82FFFC3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFFC40: 38DD0025  addi r6, r29, 0x25
	ctx.r[6].s64 = ctx.r[29].s64 + 37;
	// 82FFFC44: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FFFC48: 7F1F3040  cmplw cr6, r31, r6
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82FFFC4C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFFC50: 4198FFA8  blt cr6, 0x82fffbf8
	if ctx.cr[6].lt {
	pc = 0x82FFFBF8; continue 'dispatch;
	}
	// 82FFFC54: 48000010  b 0x82fffc64
	pc = 0x82FFFC64; continue 'dispatch;
	// 82FFFC58: 50EA3830  rlwimi r10, r7, 7, 0, 0x18
	ctx.r[10].u64 = (((ctx.r[7].u32).rotate_left(7) as u64) & 0x00000000FFFFFF80) | (ctx.r[10].u64 & 0xFFFFFFFF0000007F);
	// 82FFFC5C: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82FFFC60: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FFFC64: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FFFC68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFFC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFFC70: 4BCA9D41  bl 0x82ca99b0
	ctx.lr = 0x82FFFC74;
	sub_82CA99B0(ctx, base);
	// 82FFFC74: A17E0026  lhz r11, 0x26(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(38 as u32) ) } as u64;
	// 82FFFC78: A15E0022  lhz r10, 0x22(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(34 as u32) ) } as u64;
	// 82FFFC7C: 7C7DF850  subf r3, r29, r31
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82FFFC80: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82FFFC84: B17E0026  sth r11, 0x26(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 82FFFC88: B15E0024  sth r10, 0x24(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[10].u16 ) };
	// 82FFFC8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFC90: 4BCA97CC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFC98 size=84
    let mut pc: u32 = 0x82FFFC98;
    'dispatch: loop {
        match pc {
            0x82FFFC98 => {
    //   block [0x82FFFC98..0x82FFFCEC)
	// 82FFFC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFCA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFFCA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFCA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFCAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFFCB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FFFCB4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FFFCB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFFCBC: B3FE0026  sth r31, 0x26(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(38 as u32), ctx.r[31].u16 ) };
	// 82FFFCC0: 4BCA9CF1  bl 0x82ca99b0
	ctx.lr = 0x82FFFCC4;
	sub_82CA99B0(ctx, base);
	// 82FFFCC4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FFFCC8: B3FE0020  sth r31, 0x20(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[31].u16 ) };
	// 82FFFCCC: B3FE0022  sth r31, 0x22(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(34 as u32), ctx.r[31].u16 ) };
	// 82FFFCD0: B17E0024  sth r11, 0x24(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 82FFFCD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFCE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFCE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFCF0 size=156
    let mut pc: u32 = 0x82FFFCF0;
    'dispatch: loop {
        match pc {
            0x82FFFCF0 => {
    //   block [0x82FFFCF0..0x82FFFD8C)
	// 82FFFCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFCF4: 4BCA9711  bl 0x82ca9404
	ctx.lr = 0x82FFFCF8;
	sub_82CA93D0(ctx, base);
	// 82FFFCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFCFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFD00: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82FFFD04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FFFD08: 392BB278  addi r9, r11, -0x4d88
	ctx.r[9].s64 = ctx.r[11].s64 + -19848;
	// 82FFFD0C: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82FFFD10: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFFD14: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82FFFD18: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FFFD1C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82FFFD20: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FFFD24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFFD28: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FFFD2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FFFD30: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FFFD34: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FFFD38: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FFFD3C: 480013B5  bl 0x830010f0
	ctx.lr = 0x82FFFD40;
	sub_830010F0(ctx, base);
	// 82FFFD40: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82FFFD44: 480013AD  bl 0x830010f0
	ctx.lr = 0x82FFFD48;
	sub_830010F0(ctx, base);
	// 82FFFD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFD4C: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FFFD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFD54: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FFFD58: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FFFD5C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82FFFD60: F97F0048  std r11, 0x48(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 82FFFD64: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82FFFD68: B17F0052  sth r11, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 82FFFD6C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FFFD70: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FFFD74: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FFFD78: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FFFD7C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FFFD80: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FFFD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFFD88: 4BCA96CC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFD90 size=264
    let mut pc: u32 = 0x82FFFD90;
    'dispatch: loop {
        match pc {
            0x82FFFD90 => {
    //   block [0x82FFFD90..0x82FFFE98)
	// 82FFFD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFD94: 4BCA9671  bl 0x82ca9404
	ctx.lr = 0x82FFFD98;
	sub_82CA93D0(ctx, base);
	// 82FFFD98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFD9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFFDA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FFFDA4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FFFDA8: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FFFDAC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FFFDB0: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFDB4: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFFDB8: 41820010  beq 0x82fffdc8
	if ctx.cr[0].eq {
	pc = 0x82FFFDC8; continue 'dispatch;
	}
	// 82FFFDBC: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82FFFDC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFFDC4: 557C043E  clrlwi r28, r11, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFFDC8: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FFFDCC: 5789043E  clrlwi r9, r28, 0x10
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82FFFDD0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFFDD4: 409A00B8  bne cr6, 0x82fffe8c
	if !ctx.cr[6].eq {
	pc = 0x82FFFE8C; continue 'dispatch;
	}
	// 82FFFDD8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FFFDDC: 419A004C  beq cr6, 0x82fffe28
	if ctx.cr[6].eq {
	pc = 0x82FFFE28; continue 'dispatch;
	}
	// 82FFFDE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FFFDE4: 419A00A4  beq cr6, 0x82fffe88
	if ctx.cr[6].eq {
	pc = 0x82FFFE88; continue 'dispatch;
	}
	// 82FFFDE8: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FFFDEC: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 82FFFDF0: 409A0098  bne cr6, 0x82fffe88
	if !ctx.cr[6].eq {
	pc = 0x82FFFE88; continue 'dispatch;
	}
	// 82FFFDF4: A17E0008  lhz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFDF8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FFFDFC: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FFFE00: 41990088  bgt cr6, 0x82fffe88
	if ctx.cr[6].gt {
	pc = 0x82FFFE88; continue 'dispatch;
	}
	// 82FFFE04: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FFFE08: 7CAB2851  subf. r5, r11, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FFFE0C: 40820010  bne 0x82fffe1c
	if !ctx.cr[0].eq {
	pc = 0x82FFFE1C; continue 'dispatch;
	}
	// 82FFFE10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE14: 419A0078  beq cr6, 0x82fffe8c
	if ctx.cr[6].eq {
	pc = 0x82FFFE8C; continue 'dispatch;
	}
	// 82FFFE18: 48000048  b 0x82fffe60
	pc = 0x82FFFE60; continue 'dispatch;
	// 82FFFE1C: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFFE20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE24: 48000038  b 0x82fffe5c
	pc = 0x82FFFE5C; continue 'dispatch;
	// 82FFFE28: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FFFE2C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82FFFE30: 409A0030  bne cr6, 0x82fffe60
	if !ctx.cr[6].eq {
	pc = 0x82FFFE60; continue 'dispatch;
	}
	// 82FFFE34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFE38: A0BE0008  lhz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFE3C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFFE40: 48001AD9  bl 0x83001918
	ctx.lr = 0x82FFFE44;
	sub_83001918(ctx, base);
	// 82FFFE44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFFE48: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFFE4C: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82FFFE50: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFFE58: 38ABFFFC  addi r5, r11, -4
	ctx.r[5].s64 = ctx.r[11].s64 + -4;
	// 82FFFE5C: 4082002C  bne 0x82fffe88
	if !ctx.cr[0].eq {
	pc = 0x82FFFE88; continue 'dispatch;
	}
	// 82FFFE60: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFFE64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE68: 41820010  beq 0x82fffe78
	if ctx.cr[0].eq {
	pc = 0x82FFFE78; continue 'dispatch;
	}
	// 82FFFE6C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82FFFE70: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FFFE74: 48000008  b 0x82fffe7c
	pc = 0x82FFFE7C; continue 'dispatch;
	// 82FFFE78: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFFE7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE80: 409AFF30  bne cr6, 0x82fffdb0
	if !ctx.cr[6].eq {
	pc = 0x82FFFDB0; continue 'dispatch;
	}
	// 82FFFE84: 48000008  b 0x82fffe8c
	pc = 0x82FFFE8C; continue 'dispatch;
	// 82FFFE88: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFFE8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FFFE90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFFE94: 4BCA95C0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFE98 size=104
    let mut pc: u32 = 0x82FFFE98;
    'dispatch: loop {
        match pc {
            0x82FFFE98 => {
    //   block [0x82FFFE98..0x82FFFF00)
	// 82FFFE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFEA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFEA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFEA8: 8964000D  lbz r11, 0xd(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FFFEAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFEB0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82FFFEB4: 409A0034  bne cr6, 0x82fffee8
	if !ctx.cr[6].eq {
	pc = 0x82FFFEE8; continue 'dispatch;
	}
	// 82FFFEB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFEBC: A0A40008  lhz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFEC0: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFFEC4: 48001A25  bl 0x830018e8
	ctx.lr = 0x82FFFEC8;
	sub_830018E8(ctx, base);
	// 82FFFEC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFFECC: A15F0052  lhz r10, 0x52(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(82 as u32) ) } as u64;
	// 82FFFED0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFFED4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FFFED8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFFEDC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFFEE0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FFFEE4: 48000008  b 0x82fffeec
	pc = 0x82FFFEEC; continue 'dispatch;
	// 82FFFEE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFFEEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFEF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFF00 size=112
    let mut pc: u32 = 0x82FFFF00;
    'dispatch: loop {
        match pc {
            0x82FFFF00 => {
    //   block [0x82FFFF00..0x82FFFF70)
	// 82FFFF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFFF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFF14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFF18: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82FFFF1C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF20: 480014C9  bl 0x830013e8
	ctx.lr = 0x82FFFF24;
	sub_830013E8(ctx, base);
	// 82FFFF24: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82FFFF28: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF2C: 480014BD  bl 0x830013e8
	ctx.lr = 0x82FFFF30;
	sub_830013E8(ctx, base);
	// 82FFFF30: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FFFF34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFFF38: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFFF3C: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82FFFF40: 419A0018  beq cr6, 0x82ffff58
	if ctx.cr[6].eq {
	pc = 0x82FFFF58; continue 'dispatch;
	}
	// 82FFFF44: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82FFFF48: 4BFFC2E9  bl 0x82ffc230
	ctx.lr = 0x82FFFF4C;
	sub_82FFC230(ctx, base);
	// 82FFFF4C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82FFFF50: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82FFFF54: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82FFFF58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFF64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFFF70 size=84
    let mut pc: u32 = 0x82FFFF70;
    'dispatch: loop {
        match pc {
            0x82FFFF70 => {
    //   block [0x82FFFF70..0x82FFFFC4)
	// 82FFFF70: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82FFFF74: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82FFFF78: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFFF7C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82FFFF80: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFFF84: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FFFF88: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF8C: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFF90: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FFFF94: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF98: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF9C: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FFFFA0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFFA4: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FFFFA8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFFAC: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82FFFFB0: 409A0008  bne cr6, 0x82ffffb8
	if !ctx.cr[6].eq {
	pc = 0x82FFFFB8; continue 'dispatch;
	}
	// 82FFFFB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFFB8: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FFFFBC: 90C4001C  stw r6, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82FFFFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFFFC8 size=96
    let mut pc: u32 = 0x82FFFFC8;
    'dispatch: loop {
        match pc {
            0x82FFFFC8 => {
    //   block [0x82FFFFC8..0x83000028)
	// 82FFFFC8: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 82FFFFCC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82FFFFD0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFFFD4: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFFFD8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFFDC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFFE0: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFFFE4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFFE8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFFEC: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFFFF0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFFF4: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFFFF8: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFFFFC: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83000000: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000004: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83000008: 90A4001C  stw r5, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8300000C: 90C40020  stw r6, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 83000010: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000014: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000018: 41820010  beq 0x83000028
	if ctx.cr[0].eq {
		sub_83000028(ctx, base);
		return;
	}
	// 8300001C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000020: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000024: 48000008  b 0x8300002c
	sub_83000028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000028 size=12
    let mut pc: u32 = 0x83000028;
    'dispatch: loop {
        match pc {
            0x83000028 => {
    //   block [0x83000028..0x83000034)
	// 83000028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300002C: 91640024  stw r11, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83000030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000038 size=100
    let mut pc: u32 = 0x83000038;
    'dispatch: loop {
        match pc {
            0x83000038 => {
    //   block [0x83000038..0x8300009C)
	// 83000038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300003C: 4BCA93C9  bl 0x82ca9404
	ctx.lr = 0x83000040;
	sub_82CA93D0(ctx, base);
	// 83000040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300004C: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 83000050: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83000054: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83000058: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8300005C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83000060: 4BFFC1B9  bl 0x82ffc218
	ctx.lr = 0x83000064;
	sub_82FFC218(ctx, base);
	// 83000064: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000068: 40820010  bne 0x83000078
	if !ctx.cr[0].eq {
	pc = 0x83000078; continue 'dispatch;
	}
	// 8300006C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83000070: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83000074: 48000020  b 0x83000094
	pc = 0x83000094; continue 'dispatch;
	// 83000078: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8300007C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83000080: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83000084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83000088: 4BFFFC69  bl 0x82fffcf0
	ctx.lr = 0x8300008C;
	sub_82FFFCF0(ctx, base);
	// 8300008C: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83000090: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83000094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000098: 4BCA93BC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830000A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830000A0 size=320
    let mut pc: u32 = 0x830000A0;
    'dispatch: loop {
        match pc {
            0x830000A0 => {
    //   block [0x830000A0..0x830001E0)
	// 830000A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830000A4: 4BCA9359  bl 0x82ca93fc
	ctx.lr = 0x830000A8;
	sub_82CA93D0(ctx, base);
	// 830000A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830000AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830000B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830000B4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830000B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830000BC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830000C0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 830000C4: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 830000C8: 48001029  bl 0x830010f0
	ctx.lr = 0x830000CC;
	sub_830010F0(ctx, base);
	// 830000CC: 7D7BE850  subf r11, r27, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[27].s64;
	// 830000D0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830000D4: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 830000D8: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 830000DC: 3BDF003C  addi r30, r31, 0x3c
	ctx.r[30].s64 = ctx.r[31].s64 + 60;
	// 830000E0: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 830000E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830000E8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 830000EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830000F0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830000F4: 5524023E  clrlwi r4, r9, 8
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 830000F8: E95B0330  ld r10, 0x330(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(816 as u32) ) };
	// 830000FC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83000100: F97F0048  std r11, 0x48(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 83000104: 48001375  bl 0x83001478
	ctx.lr = 0x83000108;
	sub_83001478(ctx, base);
	// 83000108: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8300010C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000110: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83000114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000118: 48001059  bl 0x83001170
	ctx.lr = 0x8300011C;
	sub_83001170(ctx, base);
	// 8300011C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83000120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000124: 419A006C  beq cr6, 0x83000190
	if ctx.cr[6].eq {
	pc = 0x83000190; continue 'dispatch;
	}
	// 83000128: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8300012C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000130: 48000FD9  bl 0x83001108
	ctx.lr = 0x83000134;
	sub_83001108(ctx, base);
	// 83000134: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83000138: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300013C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83000140: 897B0015  lbz r11, 0x15(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(21 as u32) ) } as u64;
	// 83000144: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83000148: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8300014C: 7C8BD850  subf r4, r11, r27
	ctx.r[4].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 83000150: 48001021  bl 0x83001170
	ctx.lr = 0x83000154;
	sub_83001170(ctx, base);
	// 83000154: 897B0011  lbz r11, 0x11(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(17 as u32) ) } as u64;
	// 83000158: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8300015C: 409A0014  bne cr6, 0x83000170
	if !ctx.cr[6].eq {
	pc = 0x83000170; continue 'dispatch;
	}
	// 83000160: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000164: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 83000168: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8300016C: 48000018  b 0x83000184
	pc = 0x83000184; continue 'dispatch;
	// 83000170: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83000174: 409A0010  bne cr6, 0x83000184
	if !ctx.cr[6].eq {
	pc = 0x83000184; continue 'dispatch;
	}
	// 83000178: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8300017C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000180: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83000184: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83000188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300018C: 409AFFA0  bne cr6, 0x8300012c
	if !ctx.cr[6].eq {
	pc = 0x8300012C; continue 'dispatch;
	}
	// 83000190: 80DF0064  lwz r6, 0x64(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83000194: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 83000198: 419A0024  beq cr6, 0x830001bc
	if ctx.cr[6].eq {
	pc = 0x830001BC; continue 'dispatch;
	}
	// 8300019C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830001A0: A0FF0050  lhz r7, 0x50(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830001A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830001A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830001AC: 4BFFFBE5  bl 0x82fffd90
	ctx.lr = 0x830001B0;
	sub_82FFFD90(ctx, base);
	// 830001B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830001B4: 40820008  bne 0x830001bc
	if !ctx.cr[0].eq {
	pc = 0x830001BC; continue 'dispatch;
	}
	// 830001B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830001BC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830001C0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 830001C4: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 830001C8: E89F0048  ld r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 830001CC: 386B0250  addi r3, r11, 0x250
	ctx.r[3].s64 = ctx.r[11].s64 + 592;
	// 830001D0: 4BFFE2D9  bl 0x82ffe4a8
	ctx.lr = 0x830001D4;
	sub_82FFE4A8(ctx, base);
	// 830001D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830001D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830001DC: 4BCA9270  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830001E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830001E0 size=264
    let mut pc: u32 = 0x830001E0;
    'dispatch: loop {
        match pc {
            0x830001E0 => {
    //   block [0x830001E0..0x830002E8)
	// 830001E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830001E4: 4BCA9225  bl 0x82ca9408
	ctx.lr = 0x830001E8;
	sub_82CA93D0(ctx, base);
	// 830001E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830001EC: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 830001F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830001F4: 3943003C  addi r10, r3, 0x3c
	ctx.r[10].s64 = ctx.r[3].s64 + 60;
	// 830001F8: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 830001FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83000200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000204: 419A0030  beq cr6, 0x83000234
	if ctx.cr[6].eq {
	pc = 0x83000234; continue 'dispatch;
	}
	// 83000208: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8300020C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83000210: 1D290018  mulli r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 * 24;
	// 83000214: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83000218: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8300021C: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 83000220: E9290330  ld r9, 0x330(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(816 as u32) ) };
	// 83000224: 7D6B43D6  divw r11, r11, r8
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 83000228: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300022C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83000230: 7D7F4B78  or r31, r11, r9
	ctx.r[31].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 83000234: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83000238: 3B830030  addi r28, r3, 0x30
	ctx.r[28].s64 = ctx.r[3].s64 + 48;
	// 8300023C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000240: 419A009C  beq cr6, 0x830002dc
	if ctx.cr[6].eq {
	pc = 0x830002DC; continue 'dispatch;
	}
	// 83000244: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000248: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8300024C: 554A0673  rlwinm. r10, r10, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83000250: 41820014  beq 0x83000264
	if ctx.cr[0].eq {
	pc = 0x83000264; continue 'dispatch;
	}
	// 83000254: A1430050  lhz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 83000258: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300025C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83000260: 48000008  b 0x83000268
	pc = 0x83000268; continue 'dispatch;
	// 83000264: A1430050  lhz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 83000268: A124000A  lhz r9, 0xa(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 8300026C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83000270: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000274: 409A0068  bne cr6, 0x830002dc
	if !ctx.cr[6].eq {
	pc = 0x830002DC; continue 'dispatch;
	}
	// 83000278: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8300027C: 2B3F0000  cmpldi cr6, r31, 0
	ctx.cr[6].compare_u64(ctx.r[31].u64, 0, &mut ctx.xer);
	// 83000280: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83000284: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 83000288: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8300028C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000290: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 83000294: E94A0330  ld r10, 0x330(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(816 as u32) ) };
	// 83000298: 7D6B43D6  divw r11, r11, r8
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8300029C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 830002A0: 7D7E5378  or r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830002A4: 419A0018  beq cr6, 0x830002bc
	if ctx.cr[6].eq {
	pc = 0x830002BC; continue 'dispatch;
	}
	// 830002A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830002AC: 7D5FF050  subf r10, r31, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 830002B0: 796BFFE6  rldicr r11, r11, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[11].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 830002B4: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 830002B8: 41980024  blt cr6, 0x830002dc
	if ctx.cr[6].lt {
	pc = 0x830002DC; continue 'dispatch;
	}
	// 830002BC: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 830002C0: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830002C4: 41820010  beq 0x830002d4
	if ctx.cr[0].eq {
	pc = 0x830002D4; continue 'dispatch;
	}
	// 830002C8: 4BFFFBD1  bl 0x82fffe98
	ctx.lr = 0x830002CC;
	sub_82FFFE98(ctx, base);
	// 830002CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830002D0: 4082000C  bne 0x830002dc
	if !ctx.cr[0].eq {
	pc = 0x830002DC; continue 'dispatch;
	}
	// 830002D4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 830002D8: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830002DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830002E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830002E4: 4BCA9174  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830002E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830002E8 size=92
    let mut pc: u32 = 0x830002E8;
    'dispatch: loop {
        match pc {
            0x830002E8 => {
    //   block [0x830002E8..0x83000344)
	// 830002E8: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 830002EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830002F0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830002F4: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830002F8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830002FC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000300: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000304: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000308: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300030C: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83000310: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83000314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000318: 409A0008  bne cr6, 0x83000320
	if !ctx.cr[6].eq {
	pc = 0x83000320; continue 'dispatch;
	}
	// 8300031C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000320: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000324: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 83000328: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300032C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000330: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83000334: 90A4001C  stw r5, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83000338: 90C40020  stw r6, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8300033C: 90E40024  stw r7, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 83000340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000348 size=212
    let mut pc: u32 = 0x83000348;
    'dispatch: loop {
        match pc {
            0x83000348 => {
    //   block [0x83000348..0x8300041C)
	// 83000348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000354: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300035C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000360: 556A00C7  rlwinm. r10, r11, 0, 3, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83000364: 408200A4  bne 0x83000408
	if !ctx.cr[0].eq {
	pc = 0x83000408; continue 'dispatch;
	}
	// 83000368: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300036C: 4082009C  bne 0x83000408
	if !ctx.cr[0].eq {
	pc = 0x83000408; continue 'dispatch;
	}
	// 83000370: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83000374: 4BFFFE6D  bl 0x830001e0
	ctx.lr = 0x83000378;
	sub_830001E0(ctx, base);
	// 83000378: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300037C: 2B240000  cmpldi cr6, r4, 0
	ctx.cr[6].compare_u64(ctx.r[4].u64, 0, &mut ctx.xer);
	// 83000380: 409A0014  bne cr6, 0x83000394
	if !ctx.cr[6].eq {
	pc = 0x83000394; continue 'dispatch;
	}
	// 83000384: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000388: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300038C: 4182007C  beq 0x83000408
	if ctx.cr[0].eq {
	pc = 0x83000408; continue 'dispatch;
	}
	// 83000390: E89F0048  ld r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 83000394: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000398: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 8300039C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 830003A0: 419A0058  beq cr6, 0x830003f8
	if ctx.cr[6].eq {
	pc = 0x830003F8; continue 'dispatch;
	}
	// 830003A4: E97F0028  ld r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 830003A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830003AC: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 830003B0: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 830003B4: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 830003B8: 41980050  blt cr6, 0x83000408
	if ctx.cr[6].lt {
	pc = 0x83000408; continue 'dispatch;
	}
	// 830003BC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830003C0: E9450008  ld r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 830003C4: 386B0250  addi r3, r11, 0x250
	ctx.r[3].s64 = ctx.r[11].s64 + 592;
	// 830003C8: 7F2A2040  cmpld cr6, r10, r4
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[4].u64, &mut ctx.xer);
	// 830003CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830003D0: 41980008  blt cr6, 0x830003d8
	if ctx.cr[6].lt {
	pc = 0x830003D8; continue 'dispatch;
	}
	// 830003D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830003D8: F8850008  std r4, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 830003DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830003E0: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830003E4: 419A000C  beq cr6, 0x830003f0
	if ctx.cr[6].eq {
	pc = 0x830003F0; continue 'dispatch;
	}
	// 830003E8: 4BFF6669  bl 0x82ff6a50
	ctx.lr = 0x830003EC;
	sub_82FF6A50(ctx, base);
	// 830003EC: 4800001C  b 0x83000408
	pc = 0x83000408; continue 'dispatch;
	// 830003F0: 4BFFDFF1  bl 0x82ffe3e0
	ctx.lr = 0x830003F4;
	sub_82FFE3E0(ctx, base);
	// 830003F4: 48000014  b 0x83000408
	pc = 0x83000408; continue 'dispatch;
	// 830003F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830003FC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83000400: 386B0250  addi r3, r11, 0x250
	ctx.r[3].s64 = ctx.r[11].s64 + 592;
	// 83000404: 4BFFE0A5  bl 0x82ffe4a8
	ctx.lr = 0x83000408;
	sub_82FFE4A8(ctx, base);
	// 83000408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000420 size=740
    let mut pc: u32 = 0x83000420;
    'dispatch: loop {
        match pc {
            0x83000420 => {
    //   block [0x83000420..0x83000704)
	// 83000420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000424: 4BCA8FD5  bl 0x82ca93f8
	ctx.lr = 0x83000428;
	sub_82CA93D0(ctx, base);
	// 83000428: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300042C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83000430: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83000434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000438: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8300043C: 4BFFFDA5  bl 0x830001e0
	ctx.lr = 0x83000440;
	sub_830001E0(ctx, base);
	// 83000440: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83000444: 480001CC  b 0x83000610
	pc = 0x83000610; continue 'dispatch;
	// 83000448: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 8300044C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83000450: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83000454: 48000CB5  bl 0x83001108
	ctx.lr = 0x83000458;
	sub_83001108(ctx, base);
	// 83000458: 907A0004  stw r3, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8300045C: 89430010  lbz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000460: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 83000464: 89630015  lbz r11, 0x15(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 83000468: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300046C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 83000470: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 83000474: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83000478: 7F6B1850  subf r27, r11, r3
	ctx.r[27].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8300047C: 41820040  beq 0x830004bc
	if ctx.cr[0].eq {
	pc = 0x830004BC; continue 'dispatch;
	}
	// 83000480: 7D7B1850  subf r11, r27, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 83000484: E95B0330  ld r10, 0x330(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(816 as u32) ) };
	// 83000488: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8300048C: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 83000490: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 83000494: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 83000498: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8300049C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830004A0: F97F0048  std r11, 0x48(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 830004A4: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830004A8: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830004AC: 41820010  beq 0x830004bc
	if ctx.cr[0].eq {
	pc = 0x830004BC; continue 'dispatch;
	}
	// 830004B0: A17F0050  lhz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830004B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830004B8: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 830004BC: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830004C0: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830004C4: 4182001C  beq 0x830004e0
	if ctx.cr[0].eq {
	pc = 0x830004E0; continue 'dispatch;
	}
	// 830004C8: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830004CC: 639C0001  ori r28, r28, 1
	ctx.r[28].u64 = ctx.r[28].u64 | 1;
	// 830004D0: 40820010  bne 0x830004e0
	if !ctx.cr[0].eq {
	pc = 0x830004E0; continue 'dispatch;
	}
	// 830004D4: A17F0052  lhz r11, 0x52(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(82 as u32) ) } as u64;
	// 830004D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830004DC: B17F0052  sth r11, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 830004E0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830004E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830004E8: 419A0008  beq cr6, 0x830004f0
	if ctx.cr[6].eq {
	pc = 0x830004F0; continue 'dispatch;
	}
	// 830004EC: 639C0004  ori r28, r28, 4
	ctx.r[28].u64 = ctx.r[28].u64 | 4;
	// 830004F0: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 830004F4: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 830004F8: 409A0070  bne cr6, 0x83000568
	if !ctx.cr[6].eq {
	pc = 0x83000568; continue 'dispatch;
	}
	// 830004FC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83000500: A3BE0008  lhz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000504: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000508: 419A0168  beq cr6, 0x83000670
	if ctx.cr[6].eq {
	pc = 0x83000670; continue 'dispatch;
	}
	// 8300050C: 895E000C  lbz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000510: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83000514: 4182015C  beq 0x83000670
	if ctx.cr[0].eq {
	pc = 0x83000670; continue 'dispatch;
	}
	// 83000518: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300051C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83000520: 419A001C  beq cr6, 0x8300053c
	if ctx.cr[6].eq {
	pc = 0x8300053C; continue 'dispatch;
	}
	// 83000524: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83000528: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300052C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000530: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83000534: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83000538: 4BCA8F49  bl 0x82ca9480
	ctx.lr = 0x8300053C;
	sub_82CA9480(ctx, base);
	// 8300053C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83000540: 7D7D5851  subf. r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000544: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83000548: 408200A0  bne 0x830005e8
	if !ctx.cr[0].eq {
	pc = 0x830005E8; continue 'dispatch;
	}
	// 8300054C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83000550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000554: 419A0094  beq cr6, 0x830005e8
	if ctx.cr[6].eq {
	pc = 0x830005E8; continue 'dispatch;
	}
	// 83000558: 80DF005C  lwz r6, 0x5c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8300055C: 63870008  ori r7, r28, 8
	ctx.r[7].u64 = ctx.r[28].u64 | 8;
	// 83000560: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83000564: 48000128  b 0x8300068c
	pc = 0x8300068C; continue 'dispatch;
	// 83000568: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8300056C: 419A0130  beq cr6, 0x8300069c
	if ctx.cr[6].eq {
	pc = 0x8300069C; continue 'dispatch;
	}
	// 83000570: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83000574: 419A0148  beq cr6, 0x830006bc
	if ctx.cr[6].eq {
	pc = 0x830006BC; continue 'dispatch;
	}
	// 83000578: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8300057C: 409A017C  bne cr6, 0x830006f8
	if !ctx.cr[6].eq {
	pc = 0x830006F8; continue 'dispatch;
	}
	// 83000580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000584: A0BE0008  lhz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000588: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300058C: 4800138D  bl 0x83001918
	ctx.lr = 0x83000590;
	sub_83001918(ctx, base);
	// 83000590: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83000594: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83000598: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8300059C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830005A0: A13E0008  lhz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830005A4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830005A8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 830005AC: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830005B0: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 830005B4: 909F005C  stw r4, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 830005B8: 909F0060  stw r4, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 830005BC: 4BFFBC5D  bl 0x82ffc218
	ctx.lr = 0x830005C0;
	sub_82FFC218(ctx, base);
	// 830005C0: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 830005C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830005C8: 41820014  beq 0x830005dc
	if ctx.cr[0].eq {
	pc = 0x830005DC; continue 'dispatch;
	}
	// 830005CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830005D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830005D4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 830005D8: 4BCA8EA9  bl 0x82ca9480
	ctx.lr = 0x830005DC;
	sub_82CA9480(ctx, base);
	// 830005DC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830005E0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830005E4: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830005E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830005EC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830005F0: 4BFFC7D9  bl 0x82ffcdc8
	ctx.lr = 0x830005F4;
	sub_82FFCDC8(ctx, base);
	// 830005F4: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830005F8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830005FC: 556B0104  rlwinm r11, r11, 0, 4, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83000600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000604: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83000608: 933A0004  stw r25, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8300060C: 4BFFFBD5  bl 0x830001e0
	ctx.lr = 0x83000610;
	sub_830001E0(ctx, base);
	// 83000610: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000614: 2B230000  cmpldi cr6, r3, 0
	ctx.cr[6].compare_u64(ctx.r[3].u64, 0, &mut ctx.xer);
	// 83000618: 409AFE30  bne cr6, 0x83000448
	if !ctx.cr[6].eq {
	pc = 0x83000448; continue 'dispatch;
	}
	// 8300061C: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000620: 40820010  bne 0x83000630
	if !ctx.cr[0].eq {
	pc = 0x83000630; continue 'dispatch;
	}
	// 83000624: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83000628: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300062C: 4BCA8E1C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 83000630: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83000634: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000638: 48000DB1  bl 0x830013e8
	ctx.lr = 0x8300063C;
	sub_830013E8(ctx, base);
	// 8300063C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83000640: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000644: 48000DA5  bl 0x830013e8
	ctx.lr = 0x83000648;
	sub_830013E8(ctx, base);
	// 83000648: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300064C: 933F0064  stw r25, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 83000650: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83000654: 656B5000  oris r11, r11, 0x5000
	ctx.r[11].u64 = ctx.r[11].u64 | 1342177280;
	// 83000658: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300065C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83000660: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83000664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000668: 4BFFF961  bl 0x82ffffc8
	ctx.lr = 0x8300066C;
	sub_82FFFFC8(ctx, base);
	// 8300066C: 48000084  b 0x830006f0
	pc = 0x830006F0; continue 'dispatch;
	// 83000670: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83000674: 419A000C  beq cr6, 0x83000680
	if ctx.cr[6].eq {
	pc = 0x83000680; continue 'dispatch;
	}
	// 83000678: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300067C: 48000008  b 0x83000684
	pc = 0x83000684; continue 'dispatch;
	// 83000680: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83000684: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83000688: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300068C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83000690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000694: 4BFFFC55  bl 0x830002e8
	ctx.lr = 0x83000698;
	sub_830002E8(ctx, base);
	// 83000698: 48000058  b 0x830006f0
	pc = 0x830006F0; continue 'dispatch;
	// 8300069C: A17E0008  lhz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830006A0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830006A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830006A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830006AC: 38CBFFF7  addi r6, r11, -9
	ctx.r[6].s64 = ctx.r[11].s64 + -9;
	// 830006B0: 38AA0009  addi r5, r10, 9
	ctx.r[5].s64 = ctx.r[10].s64 + 9;
	// 830006B4: 4BFFF8BD  bl 0x82ffff70
	ctx.lr = 0x830006B8;
	sub_82FFFF70(ctx, base);
	// 830006B8: 48000038  b 0x830006f0
	pc = 0x830006F0; continue 'dispatch;
	// 830006BC: A17E0008  lhz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830006C0: 34CBFFFA  addic. r6, r11, -6
	ctx.xer.ca = (ctx.r[11].u32 > (!(-6 as u32)));
	ctx.r[6].s64 = ctx.r[11].s64 + -6;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830006C4: 41820010  beq 0x830006d4
	if ctx.cr[0].eq {
	pc = 0x830006D4; continue 'dispatch;
	}
	// 830006C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830006CC: 38AB0006  addi r5, r11, 6
	ctx.r[5].s64 = ctx.r[11].s64 + 6;
	// 830006D0: 48000008  b 0x830006d8
	pc = 0x830006D8; continue 'dispatch;
	// 830006D4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830006D8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830006DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830006E0: 4BFFF8E9  bl 0x82ffffc8
	ctx.lr = 0x830006E4;
	sub_82FFFFC8(ctx, base);
	// 830006E4: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830006E8: 656B4000  oris r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 1073741824;
	// 830006EC: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830006F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830006F4: 4BFFFF34  b 0x83000628
	pc = 0x83000628; continue 'dispatch;
	// 830006F8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830006FC: 556B0104  rlwinm r11, r11, 0, 4, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83000700: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000708 size=140
    let mut pc: u32 = 0x83000708;
    'dispatch: loop {
        match pc {
            0x83000708 => {
    //   block [0x83000708..0x83000794)
	// 83000708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300070C: 4BCA8CFD  bl 0x82ca9408
	ctx.lr = 0x83000710;
	sub_82CA93D0(ctx, base);
	// 83000710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000718: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8300071C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83000720: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83000724: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83000728: 556B0104  rlwinm r11, r11, 0, 4, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8300072C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83000730: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000738: 419A0020  beq cr6, 0x83000758
	if ctx.cr[6].eq {
	pc = 0x83000758; continue 'dispatch;
	}
	// 8300073C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83000740: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000744: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83000748: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8300074C: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000750: 4BFFC679  bl 0x82ffcdc8
	ctx.lr = 0x83000754;
	sub_82FFCDC8(ctx, base);
	// 83000754: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83000758: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300075C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83000760: 409A0024  bne cr6, 0x83000784
	if !ctx.cr[6].eq {
	pc = 0x83000784; continue 'dispatch;
	}
	// 83000764: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 83000768: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300076C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83000770: 409A0014  bne cr6, 0x83000784
	if !ctx.cr[6].eq {
	pc = 0x83000784; continue 'dispatch;
	}
	// 83000774: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83000778: 4BFFBAB9  bl 0x82ffc230
	ctx.lr = 0x8300077C;
	sub_82FFC230(ctx, base);
	// 8300077C: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83000780: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 83000784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000788: 4BFFFBC1  bl 0x83000348
	ctx.lr = 0x8300078C;
	sub_83000348(ctx, base);
	// 8300078C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000790: 4BCA8CC8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000798 size=16
    let mut pc: u32 = 0x83000798;
    'dispatch: loop {
        match pc {
            0x83000798 => {
    //   block [0x83000798..0x830007A8)
	// 83000798: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300079C: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 830007A0: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830007A4: 4BFFFBA4  b 0x83000348
	sub_83000348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830007A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830007A8 size=368
    let mut pc: u32 = 0x830007A8;
    'dispatch: loop {
        match pc {
            0x830007A8 => {
    //   block [0x830007A8..0x83000918)
	// 830007A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830007AC: 4BCA8C5D  bl 0x82ca9408
	ctx.lr = 0x830007B0;
	sub_82CA93D0(ctx, base);
	// 830007B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830007B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830007B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830007BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830007C0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830007C4: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830007C8: 41820014  beq 0x830007dc
	if ctx.cr[0].eq {
	pc = 0x830007DC; continue 'dispatch;
	}
	// 830007CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830007D0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830007D4: 4BFFC5F5  bl 0x82ffcdc8
	ctx.lr = 0x830007D8;
	sub_82FFCDC8(ctx, base);
	// 830007D8: 48000058  b 0x83000830
	pc = 0x83000830; continue 'dispatch;
	// 830007DC: 897C000D  lbz r11, 0xd(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(13 as u32) ) } as u64;
	// 830007E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830007E4: 419A0128  beq cr6, 0x8300090c
	if ctx.cr[6].eq {
	pc = 0x8300090C; continue 'dispatch;
	}
	// 830007E8: 7D64E850  subf r11, r4, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[4].s64;
	// 830007EC: E9440330  ld r10, 0x330(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(816 as u32) ) };
	// 830007F0: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 830007F4: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 830007F8: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830007FC: 7D284BD6  divw r9, r8, r9
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 83000800: 556806B5  rlwinm. r8, r11, 0, 0x1a, 0x1a
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83000804: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 83000808: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 8300080C: 40820030  bne 0x8300083c
	if !ctx.cr[0].eq {
	pc = 0x8300083C; continue 'dispatch;
	}
	// 83000810: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83000814: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000818: 48000959  bl 0x83001170
	ctx.lr = 0x8300081C;
	sub_83001170(ctx, base);
	// 8300081C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83000820: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000824: 409A000C  bne cr6, 0x83000830
	if !ctx.cr[6].eq {
	pc = 0x83000830; continue 'dispatch;
	}
	// 83000828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300082C: 4BFFFB1D  bl 0x83000348
	ctx.lr = 0x83000830;
	sub_83000348(ctx, base);
	// 83000830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83000834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000838: 4BCA8C20  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8300083C: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000840: 4082001C  bne 0x8300085c
	if !ctx.cr[0].eq {
	pc = 0x8300085C; continue 'dispatch;
	}
	// 83000844: E97F0048  ld r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 83000848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8300084C: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 83000850: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83000854: 7F2B4840  cmpld cr6, r11, r9
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[9].u64, &mut ctx.xer);
	// 83000858: 4098FF74  bge cr6, 0x830007cc
	if !ctx.cr[6].lt {
	pc = 0x830007CC; continue 'dispatch;
	}
	// 8300085C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 83000860: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000868: 48000909  bl 0x83001170
	ctx.lr = 0x8300086C;
	sub_83001170(ctx, base);
	// 8300086C: 897C000D  lbz r11, 0xd(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(13 as u32) ) } as u64;
	// 83000870: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83000874: 409A0014  bne cr6, 0x83000888
	if !ctx.cr[6].eq {
	pc = 0x83000888; continue 'dispatch;
	}
	// 83000878: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300087C: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 83000880: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83000884: 48000018  b 0x8300089c
	pc = 0x8300089C; continue 'dispatch;
	// 83000888: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8300088C: 409A0010  bne cr6, 0x8300089c
	if !ctx.cr[6].eq {
	pc = 0x8300089C; continue 'dispatch;
	}
	// 83000890: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83000894: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000898: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8300089C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830008A0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830008A4: 409AFF8C  bne cr6, 0x83000830
	if !ctx.cr[6].eq {
	pc = 0x83000830; continue 'dispatch;
	}
	// 830008A8: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830008AC: A0FF0050  lhz r7, 0x50(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830008B0: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830008B4: 41820014  beq 0x830008c8
	if ctx.cr[0].eq {
	pc = 0x830008C8; continue 'dispatch;
	}
	// 830008B8: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830008BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830008C0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830008C4: 48000008  b 0x830008cc
	pc = 0x830008CC; continue 'dispatch;
	// 830008C8: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830008CC: A15C000A  lhz r10, 0xa(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 830008D0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830008D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830008D8: 409AFF58  bne cr6, 0x83000830
	if !ctx.cr[6].eq {
	pc = 0x83000830; continue 'dispatch;
	}
	// 830008DC: 80DF0064  lwz r6, 0x64(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830008E0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 830008E4: 409A0010  bne cr6, 0x830008f4
	if !ctx.cr[6].eq {
	pc = 0x830008F4; continue 'dispatch;
	}
	// 830008E8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830008EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830008F0: 4099FF38  ble cr6, 0x83000828
	if !ctx.cr[6].gt {
	pc = 0x83000828; continue 'dispatch;
	}
	// 830008F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830008F8: 80BF0060  lwz r5, 0x60(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830008FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000900: 4BFFF491  bl 0x82fffd90
	ctx.lr = 0x83000904;
	sub_82FFFD90(ctx, base);
	// 83000904: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83000908: 4082FF20  bne 0x83000828
	if !ctx.cr[0].eq {
	pc = 0x83000828; continue 'dispatch;
	}
	// 8300090C: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 83000910: 6063100D  ori r3, r3, 0x100d
	ctx.r[3].u64 = ctx.r[3].u64 | 4109;
	// 83000914: 4BFFFF20  b 0x83000834
	pc = 0x83000834; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000918 size=16
    let mut pc: u32 = 0x83000918;
    'dispatch: loop {
        match pc {
            0x83000918 => {
    //   block [0x83000918..0x83000928)
	// 83000918: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300091C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83000920: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83000924: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000928 size=8
    let mut pc: u32 = 0x83000928;
    'dispatch: loop {
        match pc {
            0x83000928 => {
    //   block [0x83000928..0x83000930)
	// 83000928: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8300092C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000930 size=16
    let mut pc: u32 = 0x83000930;
    'dispatch: loop {
        match pc {
            0x83000930 => {
    //   block [0x83000930..0x83000940)
	// 83000930: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000934: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83000938: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8300093C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000940 size=8
    let mut pc: u32 = 0x83000940;
    'dispatch: loop {
        match pc {
            0x83000940 => {
    //   block [0x83000940..0x83000948)
	// 83000940: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 83000944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000948 size=16
    let mut pc: u32 = 0x83000948;
    'dispatch: loop {
        match pc {
            0x83000948 => {
    //   block [0x83000948..0x83000958)
	// 83000948: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300094C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 83000950: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83000954: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000958 size=8
    let mut pc: u32 = 0x83000958;
    'dispatch: loop {
        match pc {
            0x83000958 => {
    //   block [0x83000958..0x83000960)
	// 83000958: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 8300095C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000960 size=116
    let mut pc: u32 = 0x83000960;
    'dispatch: loop {
        match pc {
            0x83000960 => {
    //   block [0x83000960..0x830009D4)
	// 83000960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300096C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000974: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83000978: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8300097C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000980: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000984: 41990010  bgt cr6, 0x83000994
	if ctx.cr[6].gt {
	pc = 0x83000994; continue 'dispatch;
	}
	// 83000988: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300098C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000990: 40980008  bge cr6, 0x83000998
	if !ctx.cr[6].lt {
	pc = 0x83000998; continue 'dispatch;
	}
	// 83000994: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83000998: 7BE40020  clrldi r4, r31, 0x20
	ctx.r[4].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 8300099C: 387E0050  addi r3, r30, 0x50
	ctx.r[3].s64 = ctx.r[30].s64 + 80;
	// 830009A0: 4BFFD909  bl 0x82ffe2a8
	ctx.lr = 0x830009A4;
	sub_82FFE2A8(ctx, base);
	// 830009A4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830009A8: 57EAF0BE  srwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830009AC: 915E001C  stw r10, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 830009B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830009B4: 40980008  bge cr6, 0x830009bc
	if !ctx.cr[6].lt {
	pc = 0x830009BC; continue 'dispatch;
	}
	// 830009B8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830009BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830009C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830009C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830009C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830009CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830009D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830009D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830009D8 size=216
    let mut pc: u32 = 0x830009D8;
    'dispatch: loop {
        match pc {
            0x830009D8 => {
    //   block [0x830009D8..0x83000AB0)
	// 830009D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830009DC: 4BCA8A2D  bl 0x82ca9408
	ctx.lr = 0x830009E0;
	sub_82CA93D0(ctx, base);
	// 830009E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830009E4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830009E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830009EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830009F0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830009F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830009F8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830009FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000A00: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000A04: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83000A08: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000A0C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000A10: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83000A14: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83000A18: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000A1C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83000A20: 83A4000C  lwz r29, 0xc(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000A24: 57ABF0BE  srwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83000A28: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83000A2C: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83000A30: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000A34: 4098000C  bge cr6, 0x83000a40
	if !ctx.cr[6].lt {
	pc = 0x83000A40; continue 'dispatch;
	}
	// 83000A38: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83000A3C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83000A40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83000A44: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83000A48: 4B265669  bl 0x822660b0
	ctx.lr = 0x83000A4C;
	sub_822660B0(ctx, base);
	// 83000A4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000A50: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83000A54: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 83000A58: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83000A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83000A60: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 83000A64: 3B8AB280  addi r28, r10, -0x4d80
	ctx.r[28].s64 = ctx.r[10].s64 + -19840;
	// 83000A68: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83000A6C: 7BA50020  clrldi r5, r29, 0x20
	ctx.r[5].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 83000A70: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83000A74: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83000A78: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83000A7C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83000A80: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83000A84: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83000A88: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 83000A8C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83000A90: 4BFFD7B1  bl 0x82ffe240
	ctx.lr = 0x83000A94;
	sub_82FFE240(ctx, base);
	// 83000A94: 389C001C  addi r4, r28, 0x1c
	ctx.r[4].s64 = ctx.r[28].s64 + 28;
	// 83000A98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83000A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000AA0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83000AA4: 4BFFD79D  bl 0x82ffe240
	ctx.lr = 0x83000AA8;
	sub_82FFE240(ctx, base);
	// 83000AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000AAC: 4BCA89AC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000AB0 size=76
    let mut pc: u32 = 0x83000AB0;
    'dispatch: loop {
        match pc {
            0x83000AB0 => {
    //   block [0x83000AB0..0x83000AFC)
	// 83000AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000AC4: E97F0078  ld r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	// 83000AC8: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 83000ACC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83000AD0: 40980018  bge cr6, 0x83000ae8
	if !ctx.cr[6].lt {
	pc = 0x83000AE8; continue 'dispatch;
	}
	// 83000AD4: 4B2655DD  bl 0x822660b0
	ctx.lr = 0x83000AD8;
	sub_822660B0(ctx, base);
	// 83000AD8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83000ADC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83000AE0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000AE4: 4BFFD7C5  bl 0x82ffe2a8
	ctx.lr = 0x83000AE8;
	sub_82FFE2A8(ctx, base);
	// 83000AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000AF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000B00 size=76
    let mut pc: u32 = 0x83000B00;
    'dispatch: loop {
        match pc {
            0x83000B00 => {
    //   block [0x83000B00..0x83000B4C)
	// 83000B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000B0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000B14: E97F0078  ld r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	// 83000B18: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 83000B1C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83000B20: 40990018  ble cr6, 0x83000b38
	if !ctx.cr[6].gt {
	pc = 0x83000B38; continue 'dispatch;
	}
	// 83000B24: 4B26558D  bl 0x822660b0
	ctx.lr = 0x83000B28;
	sub_822660B0(ctx, base);
	// 83000B28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83000B2C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83000B30: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000B34: 4BFFD775  bl 0x82ffe2a8
	ctx.lr = 0x83000B38;
	sub_82FFE2A8(ctx, base);
	// 83000B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000B44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000B50 size=164
    let mut pc: u32 = 0x83000B50;
    'dispatch: loop {
        match pc {
            0x83000B50 => {
    //   block [0x83000B50..0x83000BF4)
	// 83000B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000B64: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000B6C: 419A0034  beq cr6, 0x83000ba0
	if ctx.cr[6].eq {
	pc = 0x83000BA0; continue 'dispatch;
	}
	// 83000B70: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000B74: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 83000B78: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000B7C: 7D4A2850  subf r10, r10, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 83000B80: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83000B84: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 83000B88: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000B8C: 4099000C  ble cr6, 0x83000b98
	if !ctx.cr[6].gt {
	pc = 0x83000B98; continue 'dispatch;
	}
	// 83000B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83000B94: 48000008  b 0x83000b9c
	pc = 0x83000B9C; continue 'dispatch;
	// 83000B98: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000B9C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83000BA0: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000BA4: 54891838  slwi r9, r4, 3
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83000BA8: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83000BAC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83000BB0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83000BB4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83000BB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83000BBC: 90BF0024  stw r5, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 83000BC0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83000BC4: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 83000BC8: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83000BCC: 78840020  clrldi r4, r4, 0x20
	ctx.r[4].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 83000BD0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83000BD4: 4BFFD785  bl 0x82ffe358
	ctx.lr = 0x83000BD8;
	sub_82FFE358(ctx, base);
	// 83000BD8: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 83000BDC: 556327FE  rlwinm r3, r11, 4, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 83000BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000BF8 size=604
    let mut pc: u32 = 0x83000BF8;
    'dispatch: loop {
        match pc {
            0x83000BF8 => {
    //   block [0x83000BF8..0x83000E54)
	// 83000BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000C00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83000C04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000C10: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 83000C14: E93F0078  ld r9, 0x78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	// 83000C18: 419A0020  beq cr6, 0x83000c38
	if ctx.cr[6].eq {
	pc = 0x83000C38; continue 'dispatch;
	}
	// 83000C1C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000C20: 40980018  bge cr6, 0x83000c38
	if !ctx.cr[6].lt {
	pc = 0x83000C38; continue 'dispatch;
	}
	// 83000C24: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83000C28: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000C2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000C30: 7C845050  subf r4, r4, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 83000C34: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83000C38: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000C3C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000C40: 41990010  bgt cr6, 0x83000c50
	if ctx.cr[6].gt {
	pc = 0x83000C50; continue 'dispatch;
	}
	// 83000C44: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000C48: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000C4C: 40980008  bge cr6, 0x83000c54
	if !ctx.cr[6].lt {
	pc = 0x83000C54; continue 'dispatch;
	}
	// 83000C50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83000C54: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000C58: 7D644850  subf r11, r4, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 83000C5C: 41980008  blt cr6, 0x83000c64
	if ctx.cr[6].lt {
	pc = 0x83000C64; continue 'dispatch;
	}
	// 83000C60: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 83000C64: 5568F0BE  srwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83000C68: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83000C6C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83000C70: 5565F0BE  srwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83000C74: 7D054050  subf r8, r5, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 83000C78: 7FC85A14  add r30, r8, r11
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83000C7C: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000C80: 40980008  bge cr6, 0x83000c88
	if !ctx.cr[6].lt {
	pc = 0x83000C88; continue 'dispatch;
	}
	// 83000C84: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 83000C88: 548BE8FE  srwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83000C8C: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000C90: 5525E8FE  srwi r5, r9, 3
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shr(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83000C94: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83000C98: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 83000C9C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83000CA0: 7C8B4A14  add r4, r11, r9
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83000CA4: 41980184  blt cr6, 0x83000e28
	if ctx.cr[6].lt {
	pc = 0x83000E28; continue 'dispatch;
	}
	// 83000CA8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000CAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83000CB0: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83000CB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83000CB8: 5525502A  slwi r5, r9, 0xa
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(10);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83000CBC: 419A000C  beq cr6, 0x83000cc8
	if ctx.cr[6].eq {
	pc = 0x83000CC8; continue 'dispatch;
	}
	// 83000CC0: 7CA55396  divwu r5, r5, r10
	ctx.r[5].u32 = ctx.r[5].u32 / ctx.r[10].u32;
	// 83000CC4: 0CCA0000  twi 6, r10, 0
	// 83000CC8: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83000CCC: 7D4A3051  subf. r10, r10, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83000CD0: 41820020  beq 0x83000cf0
	if ctx.cr[0].eq {
	pc = 0x83000CF0; continue 'dispatch;
	}
	// 83000CD4: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83000CD8: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 83000CDC: 1D081F40  mulli r8, r8, 0x1f40
	ctx.r[8].s64 = ctx.r[8].s64 * 8000;
	// 83000CE0: 08CA0000  tdi 6, r10, 0
	// tdi: trap doubleword immediate — TODO: implement trap semantics
	// 83000CE4: 7D485392  divdu r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 / ctx.r[10].u64;
	// 83000CE8: 5548003E  slwi r8, r10, 0
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83000CEC: 4800000C  b 0x83000cf8
	pc = 0x83000CF8; continue 'dispatch;
	// 83000CF0: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83000CF4: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83000CF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83000CFC: 409A00B4  bne cr6, 0x83000db0
	if !ctx.cr[6].eq {
	pc = 0x83000DB0; continue 'dispatch;
	}
	// 83000D00: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83000D04: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000D08: 4199001C  bgt cr6, 0x83000d24
	if ctx.cr[6].gt {
	pc = 0x83000D24; continue 'dispatch;
	}
	// 83000D0C: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000D10: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000D14: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000D18: 419800C8  blt cr6, 0x83000de0
	if ctx.cr[6].lt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000D1C: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000D20: 480000BC  b 0x83000ddc
	pc = 0x83000DDC; continue 'dispatch;
	// 83000D24: 57C9E8FE  srwi r9, r30, 3
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shr(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83000D28: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83000D2C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000D30: 41990018  bgt cr6, 0x83000d48
	if ctx.cr[6].gt {
	pc = 0x83000D48; continue 'dispatch;
	}
	// 83000D34: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000D38: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000D3C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000D40: 409800A0  bge cr6, 0x83000de0
	if !ctx.cr[6].lt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000D44: 4BFFFFD8  b 0x83000d1c
	pc = 0x83000D1C; continue 'dispatch;
	// 83000D48: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83000D4C: 7D0AF214  add r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 83000D50: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000D54: 4099002C  ble cr6, 0x83000d80
	if !ctx.cr[6].gt {
	pc = 0x83000D80; continue 'dispatch;
	}
	// 83000D58: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83000D5C: 40990018  ble cr6, 0x83000d74
	if !ctx.cr[6].gt {
	pc = 0x83000D74; continue 'dispatch;
	}
	// 83000D60: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83000D64: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000D68: 4099000C  ble cr6, 0x83000d74
	if !ctx.cr[6].gt {
	pc = 0x83000D74; continue 'dispatch;
	}
	// 83000D6C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83000D70: 48000070  b 0x83000de0
	pc = 0x83000DE0; continue 'dispatch;
	// 83000D74: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83000D78: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83000D7C: 48000064  b 0x83000de0
	pc = 0x83000DE0; continue 'dispatch;
	// 83000D80: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83000D84: 40990018  ble cr6, 0x83000d9c
	if !ctx.cr[6].gt {
	pc = 0x83000D9C; continue 'dispatch;
	}
	// 83000D88: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83000D8C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000D90: 4099000C  ble cr6, 0x83000d9c
	if !ctx.cr[6].gt {
	pc = 0x83000D9C; continue 'dispatch;
	}
	// 83000D94: 556AE8FE  srwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000D98: 48000010  b 0x83000da8
	pc = 0x83000DA8; continue 'dispatch;
	// 83000D9C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000DA0: 41980040  blt cr6, 0x83000de0
	if ctx.cr[6].lt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000DA4: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000DA8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000DAC: 48000034  b 0x83000de0
	pc = 0x83000DE0; continue 'dispatch;
	// 83000DB0: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83000DB4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000DB8: 41990028  bgt cr6, 0x83000de0
	if ctx.cr[6].gt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000DBC: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83000DC0: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000DC4: 4199001C  bgt cr6, 0x83000de0
	if ctx.cr[6].gt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000DC8: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000DCC: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000DD0: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000DD4: 4198000C  blt cr6, 0x83000de0
	if ctx.cr[6].lt {
	pc = 0x83000DE0; continue 'dispatch;
	}
	// 83000DD8: 556AE8FE  srwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000DDC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83000DE0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000DE4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000DE8: 41990010  bgt cr6, 0x83000df8
	if ctx.cr[6].gt {
	pc = 0x83000DF8; continue 'dispatch;
	}
	// 83000DEC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000DF0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000DF4: 40980008  bge cr6, 0x83000dfc
	if !ctx.cr[6].lt {
	pc = 0x83000DFC; continue 'dispatch;
	}
	// 83000DF8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83000DFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83000E00: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 83000E04: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000E08: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 83000E0C: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 83000E10: 909F0034  stw r4, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 83000E14: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83000E18: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 83000E1C: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83000E20: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 83000E24: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 83000E28: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83000E2C: 78840020  clrldi r4, r4, 0x20
	ctx.r[4].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 83000E30: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83000E34: 4BFFD475  bl 0x82ffe2a8
	ctx.lr = 0x83000E38;
	sub_82FFE2A8(ctx, base);
	// 83000E38: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83000E3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000E48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83000E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000E58 size=100
    let mut pc: u32 = 0x83000E58;
    'dispatch: loop {
        match pc {
            0x83000E58 => {
    //   block [0x83000E58..0x83000EBC)
	// 83000E58: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83000E5C: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83000E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000E64: 5548502A  slwi r8, r10, 0xa
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83000E68: 419A000C  beq cr6, 0x83000e74
	if ctx.cr[6].eq {
	pc = 0x83000E74; continue 'dispatch;
	}
	// 83000E6C: 7D085B96  divwu r8, r8, r11
	ctx.r[8].u32 = ctx.r[8].u32 / ctx.r[11].u32;
	// 83000E70: 0CCB0000  twi 6, r11, 0
	// 83000E74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83000E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000E7C: 91430040  stw r10, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83000E80: 554A502A  slwi r10, r10, 0xa
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(10);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000E84: 419A000C  beq cr6, 0x83000e90
	if ctx.cr[6].eq {
	pc = 0x83000E90; continue 'dispatch;
	}
	// 83000E88: 7D4A5B96  divwu r10, r10, r11
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 83000E8C: 0CCB0000  twi 6, r11, 0
	// 83000E90: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83000E94: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000E98: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000E9C: 4099002C  ble cr6, 0x83000ec8
	if !ctx.cr[6].gt {
		sub_83000EC8(ctx, base);
		return;
	}
	// 83000EA0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83000EA4: 41990024  bgt cr6, 0x83000ec8
	if ctx.cr[6].gt {
		sub_83000EC8(ctx, base);
		return;
	}
	// 83000EA8: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83000EAC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000EB0: 4099000C  ble cr6, 0x83000ebc
	if !ctx.cr[6].gt {
		sub_83000EBC(ctx, base);
		return;
	}
	// 83000EB4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83000EB8: 48000024  b 0x83000edc
	sub_83000EC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000EBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000EBC size=12
    let mut pc: u32 = 0x83000EBC;
    'dispatch: loop {
        match pc {
            0x83000EBC => {
    //   block [0x83000EBC..0x83000EC8)
	// 83000EBC: 556AE8FE  srwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83000EC0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000EC4: 48000018  b 0x83000edc
	sub_83000EC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83000EC8 size=44
    let mut pc: u32 = 0x83000EC8;
    'dispatch: loop {
        match pc {
            0x83000EC8 => {
    //   block [0x83000EC8..0x83000EF4)
	// 83000EC8: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83000ECC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000ED0: 4099000C  ble cr6, 0x83000edc
	if !ctx.cr[6].gt {
	pc = 0x83000EDC; continue 'dispatch;
	}
	// 83000ED4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83000ED8: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83000EDC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000EE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000EE4: 40980008  bge cr6, 0x83000eec
	if !ctx.cr[6].lt {
	pc = 0x83000EEC; continue 'dispatch;
	}
	// 83000EE8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83000EEC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000EF8 size=152
    let mut pc: u32 = 0x83000EF8;
    'dispatch: loop {
        match pc {
            0x83000EF8 => {
    //   block [0x83000EF8..0x83000F90)
	// 83000EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000F04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000F0C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83000F10: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83000F14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000F18: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83000F1C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83000F20: 4198005C  blt cr6, 0x83000f7c
	if ctx.cr[6].lt {
	pc = 0x83000F7C; continue 'dispatch;
	}
	// 83000F24: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83000F28: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000F2C: 41980050  blt cr6, 0x83000f7c
	if ctx.cr[6].lt {
	pc = 0x83000F7C; continue 'dispatch;
	}
	// 83000F30: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83000F34: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000F38: 41980044  blt cr6, 0x83000f7c
	if ctx.cr[6].lt {
	pc = 0x83000F7C; continue 'dispatch;
	}
	// 83000F3C: E97F0078  ld r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	// 83000F40: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000F44: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83000F48: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83000F4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000F50: 40990008  ble cr6, 0x83000f58
	if !ctx.cr[6].gt {
	pc = 0x83000F58; continue 'dispatch;
	}
	// 83000F54: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83000F58: 79640020  clrldi r4, r11, 0x20
	ctx.r[4].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83000F5C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83000F60: 4BFFD349  bl 0x82ffe2a8
	ctx.lr = 0x83000F64;
	sub_82FFE2A8(ctx, base);
	// 83000F64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83000F68: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83000F6C: 813F0044  lwz r9, 0x44(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83000F70: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83000F74: 7D695050  subf r11, r9, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83000F78: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83000F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000F88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000F90 size=144
    let mut pc: u32 = 0x83000F90;
    'dispatch: loop {
        match pc {
            0x83000F90 => {
    //   block [0x83000F90..0x83001020)
	// 83000F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000F9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000FA4: 4B26510D  bl 0x822660b0
	ctx.lr = 0x83000FA8;
	sub_822660B0(ctx, base);
	// 83000FA8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000FB0: 419A0034  beq cr6, 0x83000fe4
	if ctx.cr[6].eq {
	pc = 0x83000FE4; continue 'dispatch;
	}
	// 83000FB4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000FB8: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 83000FBC: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000FC0: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 83000FC4: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83000FC8: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 83000FCC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000FD0: 4099000C  ble cr6, 0x83000fdc
	if !ctx.cr[6].gt {
	pc = 0x83000FDC; continue 'dispatch;
	}
	// 83000FD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83000FD8: 48000008  b 0x83000fe0
	pc = 0x83000FE0; continue 'dispatch;
	// 83000FDC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83000FE0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83000FE4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000FE8: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83000FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83000FF0: 409A000C  bne cr6, 0x83000ffc
	if !ctx.cr[6].eq {
	pc = 0x83000FFC; continue 'dispatch;
	}
	// 83000FF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83000FF8: 48000014  b 0x8300100c
	pc = 0x8300100C; continue 'dispatch;
	// 83000FFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001000: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 83001004: 7C6B5396  divwu r3, r11, r10
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 83001008: 0CCA0000  twi 6, r10, 0
	// 8300100C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83001010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83001014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83001018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001020 size=208
    let mut pc: u32 = 0x83001020;
    'dispatch: loop {
        match pc {
            0x83001020 => {
    //   block [0x83001020..0x830010F0)
	// 83001020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001024: 4BCA83D5  bl 0x82ca93f8
	ctx.lr = 0x83001028;
	sub_82CA93D0(ctx, base);
	// 83001028: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300102C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001030: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83001034: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83001038: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8300103C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 83001040: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 83001044: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83001048: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 8300104C: 4BFFFF45  bl 0x83000f90
	ctx.lr = 0x83001050;
	sub_83000F90(ctx, base);
	// 83001050: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83001054: E95F0078  ld r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	// 83001058: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300105C: 7D2A1A14  add r9, r10, r3
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83001060: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83001064: 40990010  ble cr6, 0x83001074
	if !ctx.cr[6].gt {
	pc = 0x83001074; continue 'dispatch;
	}
	// 83001068: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300106C: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83001070: 48000008  b 0x83001078
	pc = 0x83001078; continue 'dispatch;
	// 83001074: 7CE9F214  add r7, r9, r30
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 83001078: 391DFFFF  addi r8, r29, -1
	ctx.r[8].s64 = ctx.r[29].s64 + -1;
	// 8300107C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001080: 554BF0BE  srwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83001084: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 83001088: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300108C: 40990008  ble cr6, 0x83001094
	if !ctx.cr[6].gt {
	pc = 0x83001094; continue 'dispatch;
	}
	// 83001090: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83001094: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 83001098: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300109C: 7C6BDA14  add r3, r11, r27
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830010A0: 419A0048  beq cr6, 0x830010e8
	if ctx.cr[6].eq {
	pc = 0x830010E8; continue 'dispatch;
	}
	// 830010A4: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830010A8: 41980040  blt cr6, 0x830010e8
	if ctx.cr[6].lt {
	pc = 0x830010E8; continue 'dispatch;
	}
	// 830010AC: 7F1AC840  cmplw cr6, r26, r25
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[25].u32, &mut ctx.xer);
	// 830010B0: 7D5AC850  subf r10, r26, r25
	ctx.r[10].s64 = ctx.r[25].s64 - ctx.r[26].s64;
	// 830010B4: 41980008  blt cr6, 0x830010bc
	if ctx.cr[6].lt {
	pc = 0x830010BC; continue 'dispatch;
	}
	// 830010B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830010BC: 7F1AC040  cmplw cr6, r26, r24
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[24].u32, &mut ctx.xer);
	// 830010C0: 7D7AC050  subf r11, r26, r24
	ctx.r[11].s64 = ctx.r[24].s64 - ctx.r[26].s64;
	// 830010C4: 41980008  blt cr6, 0x830010cc
	if ctx.cr[6].lt {
	pc = 0x830010CC; continue 'dispatch;
	}
	// 830010C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830010CC: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830010D0: 4098000C  bge cr6, 0x830010dc
	if !ctx.cr[6].lt {
	pc = 0x830010DC; continue 'dispatch;
	}
	// 830010D4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 830010D8: 48000010  b 0x830010e8
	pc = 0x830010E8; continue 'dispatch;
	// 830010DC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830010E0: 40990008  ble cr6, 0x830010e8
	if !ctx.cr[6].gt {
	pc = 0x830010E8; continue 'dispatch;
	}
	// 830010E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830010E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830010EC: 4BCA835C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830010F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830010F0 size=20
    let mut pc: u32 = 0x830010F0;
    'dispatch: loop {
        match pc {
            0x830010F0 => {
    //   block [0x830010F0..0x83001104)
	// 830010F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830010F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830010F8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830010FC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83001100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001108 size=72
    let mut pc: u32 = 0x83001108;
    'dispatch: loop {
        match pc {
            0x83001108 => {
    //   block [0x83001108..0x83001150)
	// 83001108: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300110C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83001110: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001114: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83001118: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300111C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001120: 41820030  beq 0x83001150
	if ctx.cr[0].eq {
		sub_83001150(ctx, base);
		return;
	}
	// 83001124: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001128: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8300112C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83001130: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83001134: 409A0008  bne cr6, 0x8300113c
	if !ctx.cr[6].eq {
	pc = 0x8300113C; continue 'dispatch;
	}
	// 83001138: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300113C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001140: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001144: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83001148: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300114C: 48000018  b 0x83001164
	sub_83001150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001150 size=32
    let mut pc: u32 = 0x83001150;
    'dispatch: loop {
        match pc {
            0x83001150 => {
    //   block [0x83001150..0x83001170)
	// 83001150: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001154: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83001158: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300115C: 409A0008  bne cr6, 0x83001164
	if !ctx.cr[6].eq {
	pc = 0x83001164; continue 'dispatch;
	}
	// 83001160: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83001164: 99230014  stb r9, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 83001168: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8300116C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001170 size=28
    let mut pc: u32 = 0x83001170;
    'dispatch: loop {
        match pc {
            0x83001170 => {
    //   block [0x83001170..0x8300118C)
	// 83001170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83001178: 409A0014  bne cr6, 0x8300118c
	if !ctx.cr[6].eq {
		sub_8300118C(ctx, base);
		return;
	}
	// 8300117C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83001180: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83001184: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 83001188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300118C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300118C size=72
    let mut pc: u32 = 0x8300118C;
    'dispatch: loop {
        match pc {
            0x8300118C => {
    //   block [0x8300118C..0x830011D4)
	// 8300118C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001190: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83001194: 419A0040  beq cr6, 0x830011d4
	if ctx.cr[6].eq {
		sub_830011D4(ctx, base);
		return;
	}
	// 83001198: 892A0015  lbz r9, 0x15(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 8300119C: 1D290018  mulli r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 * 24;
	// 830011A0: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 830011A4: 3929FFD0  addi r9, r9, -0x30
	ctx.r[9].s64 = ctx.r[9].s64 + -48;
	// 830011A8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830011AC: 409A0028  bne cr6, 0x830011d4
	if !ctx.cr[6].eq {
		sub_830011D4(ctx, base);
		return;
	}
	// 830011B0: 892A0015  lbz r9, 0x15(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 830011B4: 89050015  lbz r8, 0x15(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(21 as u32) ) } as u64;
	// 830011B8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830011BC: 40990018  ble cr6, 0x830011d4
	if !ctx.cr[6].gt {
		sub_830011D4(ctx, base);
		return;
	}
	// 830011C0: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 830011C4: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 830011C8: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 830011CC: 996A0014  stb r11, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830011D0: 4BFFFFB4  b 0x83001184
	sub_83001170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830011D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830011D4 size=56
    let mut pc: u32 = 0x830011D4;
    'dispatch: loop {
        match pc {
            0x830011D4 => {
    //   block [0x830011D4..0x8300120C)
	// 830011D4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 830011D8: A1240340  lhz r9, 0x340(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(832 as u32) ) } as u64;
	// 830011DC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830011E0: 890A0015  lbz r8, 0x15(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 830011E4: 1D080018  mulli r8, r8, 0x18
	ctx.r[8].s64 = ctx.r[8].s64 * 24;
	// 830011E8: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 830011EC: A1080310  lhz r8, 0x310(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(784 as u32) ) } as u64;
	// 830011F0: 7D094050  subf r8, r9, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 830011F4: 5508043E  clrlwi r8, r8, 0x10
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 830011F8: 2B088000  cmplwi cr6, r8, 0x8000
	ctx.cr[6].compare_u32(ctx.r[8].u32, 32768 as u32, &mut ctx.xer);
	// 830011FC: 41980010  blt cr6, 0x8300120c
	if ctx.cr[6].lt {
		sub_8300120C(ctx, base);
		return;
	}
	// 83001200: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83001204: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83001208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300120C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300120C size=52
    let mut pc: u32 = 0x8300120C;
    'dispatch: loop {
        match pc {
            0x8300120C => {
    //   block [0x8300120C..0x83001240)
	// 8300120C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83001210: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83001214: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83001218: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8300121C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83001220: A0EA0340  lhz r7, 0x340(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(832 as u32) ) } as u64;
	// 83001224: 7CE93850  subf r7, r9, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 83001228: 54E7043E  clrlwi r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 8300122C: 2B078000  cmplwi cr6, r7, 0x8000
	ctx.cr[6].compare_u32(ctx.r[7].u32, 32768 as u32, &mut ctx.xer);
	// 83001230: 41980010  blt cr6, 0x83001240
	if ctx.cr[6].lt {
		sub_83001240(ctx, base);
		return;
	}
	// 83001234: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83001238: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300123C: 4BFFFFD4  b 0x83001210
	pc = 0x83001210; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001240 size=132
    let mut pc: u32 = 0x83001240;
    'dispatch: loop {
        match pc {
            0x83001240 => {
    //   block [0x83001240..0x830012C4)
	// 83001240: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83001244: 409A00B8  bne cr6, 0x830012fc
	if !ctx.cr[6].eq {
		sub_830012FC(ctx, base);
		return;
	}
	// 83001248: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8300124C: 89250015  lbz r9, 0x15(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(21 as u32) ) } as u64;
	// 83001250: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83001254: 40980034  bge cr6, 0x83001288
	if !ctx.cr[6].lt {
	pc = 0x83001288; continue 'dispatch;
	}
	// 83001258: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300125C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001260: 41820068  beq 0x830012c8
	if ctx.cr[0].eq {
		sub_830012C8(ctx, base);
		return;
	}
	// 83001264: 1D2A0018  mulli r9, r10, 0x18
	ctx.r[9].s64 = ctx.r[10].s64 * 24;
	// 83001268: 89050015  lbz r8, 0x15(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(21 as u32) ) } as u64;
	// 8300126C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83001270: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83001274: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83001278: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8300127C: 4198FFDC  blt cr6, 0x83001258
	if ctx.cr[6].lt {
	pc = 0x83001258; continue 'dispatch;
	}
	// 83001280: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83001284: 409A0058  bne cr6, 0x830012dc
	if !ctx.cr[6].eq {
		sub_830012DC(ctx, base);
		return;
	}
	// 83001288: 7D455850  subf r10, r5, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 8300128C: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 83001290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83001294: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 83001298: 99450014  stb r10, 0x14(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8300129C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830012A0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830012A4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830012A8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830012AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830012B0: 409A0008  bne cr6, 0x830012b8
	if !ctx.cr[6].eq {
	pc = 0x830012B8; continue 'dispatch;
	}
	// 830012B4: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 830012B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830012BC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830012C0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830012C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830012C4 size=4
    let mut pc: u32 = 0x830012C4;
    'dispatch: loop {
        match pc {
            0x830012C4 => {
    //   block [0x830012C4..0x830012C8)
	// 830012C4: 4BFFFF40  b 0x83001204
	sub_830011D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830012C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830012C8 size=20
    let mut pc: u32 = 0x830012C8;
    'dispatch: loop {
        match pc {
            0x830012C8 => {
    //   block [0x830012C8..0x830012DC)
	// 830012C8: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 830012CC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 830012D0: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 830012D4: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 830012D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830012DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830012DC size=32
    let mut pc: u32 = 0x830012DC;
    'dispatch: loop {
        match pc {
            0x830012DC => {
    //   block [0x830012DC..0x830012FC)
	// 830012DC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 830012E0: 7D0A2850  subf r8, r10, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 830012E4: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 830012E8: 7D084BD6  divw r8, r8, r9
	ctx.r[8].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 830012EC: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 830012F0: 990A0014  stb r8, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 830012F4: 99650014  stb r11, 0x14(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830012F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830012FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830012FC size=12
    let mut pc: u32 = 0x830012FC;
    'dispatch: loop {
        match pc {
            0x830012FC => {
    //   block [0x830012FC..0x83001308)
	// 830012FC: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83001300: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83001304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001308 size=224
    let mut pc: u32 = 0x83001308;
    'dispatch: loop {
        match pc {
            0x83001308 => {
    //   block [0x83001308..0x830013E8)
	// 83001308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300130C: 4BCA8101  bl 0x82ca940c
	ctx.lr = 0x83001310;
	sub_82CA93D0(ctx, base);
	// 83001310: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83001314: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83001318: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8300131C: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 83001320: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83001324: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 83001328: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8300132C: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 83001330: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83001334: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 83001338: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300133C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83001340: 419A002C  beq cr6, 0x8300136c
	if ctx.cr[6].eq {
	pc = 0x8300136C; continue 'dispatch;
	}
	// 83001344: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001348: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300134C: 409A000C  bne cr6, 0x83001358
	if !ctx.cr[6].eq {
	pc = 0x83001358; continue 'dispatch;
	}
	// 83001350: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83001354: 48000010  b 0x83001364
	pc = 0x83001364; continue 'dispatch;
	// 83001358: 7D5E5850  subf r10, r30, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8300135C: 7D4AEBD6  divw r10, r10, r29
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[29].s32;
	// 83001360: 995E0014  stb r10, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 83001364: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83001368: 48000060  b 0x830013c8
	pc = 0x830013C8; continue 'dispatch;
	// 8300136C: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001370: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83001374: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001378: 4182002C  beq 0x830013a4
	if ctx.cr[0].eq {
	pc = 0x830013A4; continue 'dispatch;
	}
	// 8300137C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83001384: 409A000C  bne cr6, 0x83001390
	if !ctx.cr[6].eq {
	pc = 0x83001390; continue 'dispatch;
	}
	// 83001388: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300138C: 48000010  b 0x8300139c
	pc = 0x8300139C; continue 'dispatch;
	// 83001390: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 83001394: 7D4AEBD6  divw r10, r10, r29
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[29].s32;
	// 83001398: 995F0014  stb r10, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8300139C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 830013A0: 48000028  b 0x830013c8
	pc = 0x830013C8; continue 'dispatch;
	// 830013A4: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830013A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830013AC: 409A000C  bne cr6, 0x830013b8
	if !ctx.cr[6].eq {
	pc = 0x830013B8; continue 'dispatch;
	}
	// 830013B0: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830013B4: 48000010  b 0x830013c4
	pc = 0x830013C4; continue 'dispatch;
	// 830013B8: 7D455850  subf r10, r5, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 830013BC: 7D4AEBD6  divw r10, r10, r29
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[29].s32;
	// 830013C0: 99450014  stb r10, 0x14(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 830013C4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 830013C8: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830013CC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830013D0: 41820014  beq 0x830013e4
	if ctx.cr[0].eq {
	pc = 0x830013E4; continue 'dispatch;
	}
	// 830013D4: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 830013D8: 992B0014  stb r9, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 830013DC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830013E0: 4BFFFF58  b 0x83001338
	pc = 0x83001338; continue 'dispatch;
	// 830013E4: 4BCA8078  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830013E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830013E8 size=140
    let mut pc: u32 = 0x830013E8;
    'dispatch: loop {
        match pc {
            0x830013E8 => {
    //   block [0x830013E8..0x83001474)
	// 830013E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830013EC: 4BCA8011  bl 0x82ca93fc
	ctx.lr = 0x830013F0;
	sub_82CA93D0(ctx, base);
	// 830013F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830013F4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830013F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830013FC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83001400: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 83001404: 48000050  b 0x83001454
	pc = 0x83001454; continue 'dispatch;
	// 83001408: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300140C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83001410: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83001414: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001418: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8300141C: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 83001420: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83001424: 8B2B0014  lbz r25, 0x14(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001428: 7F8A5850  subf r28, r10, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8300142C: 48000010  b 0x8300143c
	pc = 0x8300143C; continue 'dispatch;
	// 83001430: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 83001434: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83001438: 8B3E0014  lbz r25, 0x14(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300143C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83001440: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001444: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83001448: 4BFFB981  bl 0x82ffcdc8
	ctx.lr = 0x8300144C;
	sub_82FFCDC8(ctx, base);
	// 8300144C: 7F2BCB79  or. r11, r25, r25
	ctx.r[11].u64 = ctx.r[25].u64 | ctx.r[25].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001450: 4082FFE0  bne 0x83001430
	if !ctx.cr[0].eq {
	pc = 0x83001430; continue 'dispatch;
	}
	// 83001454: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300145C: 409AFFAC  bne cr6, 0x83001408
	if !ctx.cr[6].eq {
	pc = 0x83001408; continue 'dispatch;
	}
	// 83001460: 935D0004  stw r26, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83001464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001468: 935D0008  stw r26, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 8300146C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83001470: 4BCA7FDC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001478 size=240
    let mut pc: u32 = 0x83001478;
    'dispatch: loop {
        match pc {
            0x83001478 => {
    //   block [0x83001478..0x83001568)
	// 83001478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300147C: 4BCA7F81  bl 0x82ca93fc
	ctx.lr = 0x83001480;
	sub_82CA93D0(ctx, base);
	// 83001480: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001488: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8300148C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83001490: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83001494: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 83001498: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300149C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 830014A0: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 830014A4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830014A8: 419A00A4  beq cr6, 0x8300154c
	if ctx.cr[6].eq {
	pc = 0x8300154C; continue 'dispatch;
	}
	// 830014AC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 830014B0: 83C50000  lwz r30, 0(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830014B4: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 830014B8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830014BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830014C0: 4BFFFE49  bl 0x83001308
	ctx.lr = 0x830014C4;
	sub_83001308(ctx, base);
	// 830014C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830014C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830014CC: 419A0024  beq cr6, 0x830014f0
	if ctx.cr[6].eq {
	pc = 0x830014F0; continue 'dispatch;
	}
	// 830014D0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830014D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830014D8: 419A000C  beq cr6, 0x830014e4
	if ctx.cr[6].eq {
	pc = 0x830014E4; continue 'dispatch;
	}
	// 830014DC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830014E0: 48000008  b 0x830014e8
	pc = 0x830014E8; continue 'dispatch;
	// 830014E4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830014E8: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830014EC: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 830014F0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830014F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830014F8: 419A0024  beq cr6, 0x8300151c
	if ctx.cr[6].eq {
	pc = 0x8300151C; continue 'dispatch;
	}
	// 830014FC: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001500: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83001504: 419A000C  beq cr6, 0x83001510
	if ctx.cr[6].eq {
	pc = 0x83001510; continue 'dispatch;
	}
	// 83001508: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300150C: 48000008  b 0x83001514
	pc = 0x83001514; continue 'dispatch;
	// 83001510: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83001514: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83001518: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8300151C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83001520: 7F23CA14  add r25, r3, r25
	ctx.r[25].u64 = ctx.r[3].u64 + ctx.r[25].u64;
	// 83001524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83001528: 409A000C  bne cr6, 0x83001534
	if !ctx.cr[6].eq {
	pc = 0x83001534; continue 'dispatch;
	}
	// 8300152C: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83001530: 48000010  b 0x83001540
	pc = 0x83001540; continue 'dispatch;
	// 83001534: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83001538: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 8300153C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83001540: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83001544: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83001548: 409AFF64  bne cr6, 0x830014ac
	if !ctx.cr[6].eq {
	pc = 0x830014AC; continue 'dispatch;
	}
	// 8300154C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83001554: 409A0008  bne cr6, 0x8300155c
	if !ctx.cr[6].eq {
	pc = 0x8300155C; continue 'dispatch;
	}
	// 83001558: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 8300155C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83001560: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83001564: 4BCA7EE8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001568 size=288
    let mut pc: u32 = 0x83001568;
    'dispatch: loop {
        match pc {
            0x83001568 => {
    //   block [0x83001568..0x83001688)
	// 83001568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300156C: 4BCA7E9D  bl 0x82ca9408
	ctx.lr = 0x83001570;
	sub_82CA93D0(ctx, base);
	// 83001570: 555E0037  rlwinm. r30, r10, 0, 0, 0x1b
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83001574: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 83001578: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8300157C: 4082000C  bne 0x83001588
	if !ctx.cr[0].eq {
	pc = 0x83001588; continue 'dispatch;
	}
	// 83001580: 555F063E  clrlwi r31, r10, 0x18
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83001584: 48000054  b 0x830015d8
	pc = 0x830015D8; continue 'dispatch;
	// 83001588: 2B0A3FFF  cmplwi cr6, r10, 0x3fff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16383 as u32, &mut ctx.xer);
	// 8300158C: 4099002C  ble cr6, 0x830015b8
	if !ctx.cr[6].gt {
	pc = 0x830015B8; continue 'dispatch;
	}
	// 83001590: 3BC000C0  li r30, 0xc0
	ctx.r[30].s64 = 192;
	// 83001594: 555D863E  rlwinm r29, r10, 0x10, 0x18, 0x1f
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83001598: 515E46BE  rlwimi r30, r10, 8, 0x1a, 0x1f
	ctx.r[30].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000003F) | (ctx.r[30].u64 & 0xFFFFFFFFFFFFFFC0);
	// 8300159C: 555CC63E  rlwinm r28, r10, 0x18, 0x18, 0x1f
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 830015A0: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 830015A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830015A8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 830015AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830015B0: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 830015B4: 48000018  b 0x830015cc
	pc = 0x830015CC; continue 'dispatch;
	// 830015B8: 2B0A007F  cmplwi cr6, r10, 0x7f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 127 as u32, &mut ctx.xer);
	// 830015BC: 40990014  ble cr6, 0x830015d0
	if !ctx.cr[6].gt {
	pc = 0x830015D0; continue 'dispatch;
	}
	// 830015C0: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 830015C4: 515EC67E  rlwimi r30, r10, 0x18, 0x19, 0x1f
	ctx.r[30].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x000000000000007F) | (ctx.r[30].u64 & 0xFFFFFFFFFFFFFF80);
	// 830015C8: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 830015CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830015D0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830015D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830015D8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 830015DC: 419A000C  beq cr6, 0x830015e8
	if ctx.cr[6].eq {
	pc = 0x830015E8; continue 'dispatch;
	}
	// 830015E0: 57EA063E  clrlwi r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 830015E4: 615F0040  ori r31, r10, 0x40
	ctx.r[31].u64 = ctx.r[10].u64 | 64;
	// 830015E8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830015EC: 419A002C  beq cr6, 0x83001618
	if ctx.cr[6].eq {
	pc = 0x83001618; continue 'dispatch;
	}
	// 830015F0: 57EA063E  clrlwi r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 830015F4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 830015F8: 615F0020  ori r31, r10, 0x20
	ctx.r[31].u64 = ctx.r[10].u64 | 32;
	// 830015FC: 409A001C  bne cr6, 0x83001618
	if !ctx.cr[6].eq {
	pc = 0x83001618; continue 'dispatch;
	}
	// 83001600: A1410056  lhz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83001604: 5547C23E  srwi r7, r10, 8
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83001608: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 8300160C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001610: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83001614: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001618: 552A07FF  clrlwi. r10, r9, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300161C: 40820018  bne 0x83001634
	if !ctx.cr[0].eq {
	pc = 0x83001634; continue 'dispatch;
	}
	// 83001620: 2B0901FE  cmplwi cr6, r9, 0x1fe
	ctx.cr[6].compare_u32(ctx.r[9].u32, 510 as u32, &mut ctx.xer);
	// 83001624: 41990010  bgt cr6, 0x83001634
	if ctx.cr[6].gt {
	pc = 0x83001634; continue 'dispatch;
	}
	// 83001628: 552AFE3E  rlwinm r10, r9, 0x1f, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 8300162C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83001630: 4800002C  b 0x8300165c
	pc = 0x8300165C; continue 'dispatch;
	// 83001634: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83001638: 419A000C  beq cr6, 0x83001644
	if ctx.cr[6].eq {
	pc = 0x83001644; continue 'dispatch;
	}
	// 8300163C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83001640: 4BFFFFEC  b 0x8300162c
	pc = 0x8300162C; continue 'dispatch;
	// 83001644: 552AC63E  rlwinm r10, r9, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 83001648: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8300164C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001650: 57EA063E  clrlwi r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 83001654: 615F0010  ori r31, r10, 0x10
	ctx.r[31].u64 = ctx.r[10].u64 | 16;
	// 83001658: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8300165C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001660: 2F040080  cmpwi cr6, r4, 0x80
	ctx.cr[6].compare_i32(ctx.r[4].s32, 128, &mut ctx.xer);
	// 83001664: 409A0010  bne cr6, 0x83001674
	if !ctx.cr[6].eq {
	pc = 0x83001674; continue 'dispatch;
	}
	// 83001668: 57EA063E  clrlwi r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8300166C: 615F0080  ori r31, r10, 0x80
	ctx.r[31].u64 = ctx.r[10].u64 | 128;
	// 83001670: 4800000C  b 0x8300167c
	pc = 0x8300167C; continue 'dispatch;
	// 83001674: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83001678: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300167C: 9BE30000  stb r31, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83001680: 7C635850  subf r3, r3, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83001684: 4BCA7DD4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001688 size=60
    let mut pc: u32 = 0x83001688;
    'dispatch: loop {
        match pc {
            0x83001688 => {
    //   block [0x83001688..0x830016C4)
	// 83001688: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300168C: 556A0032  rlwinm r10, r11, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83001690: 2B0A00C0  cmplwi cr6, r10, 0xc0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 192 as u32, &mut ctx.xer);
	// 83001694: 409A0030  bne cr6, 0x830016c4
	if !ctx.cr[6].eq {
		sub_830016C4(ctx, base);
		return;
	}
	// 83001698: 89430001  lbz r10, 1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8300169C: 556B44AE  rlwinm r11, r11, 8, 0x12, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 830016A0: 89230002  lbz r9, 2(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 830016A4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830016A8: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 830016AC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830016B0: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830016B4: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 830016B8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830016BC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830016C0: 48000024  b 0x830016e4
	sub_830016E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830016C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830016C4 size=28
    let mut pc: u32 = 0x830016C4;
    'dispatch: loop {
        match pc {
            0x830016C4 => {
    //   block [0x830016C4..0x830016E0)
	// 830016C4: 556A0630  rlwinm r10, r11, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830016C8: 2B0A0080  cmplwi cr6, r10, 0x80
	ctx.cr[6].compare_u32(ctx.r[10].u32, 128 as u32, &mut ctx.xer);
	// 830016CC: 409A0014  bne cr6, 0x830016e0
	if !ctx.cr[6].eq {
		sub_830016E0(ctx, base);
		return;
	}
	// 830016D0: 89430001  lbz r10, 1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 830016D4: 556B44AE  rlwinm r11, r11, 8, 0x12, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 830016D8: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 830016DC: 4BFFFFE0  b 0x830016bc
	sub_83001688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830016E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830016E0 size=12
    let mut pc: u32 = 0x830016E0;
    'dispatch: loop {
        match pc {
            0x830016E0 => {
    //   block [0x830016E0..0x830016EC)
	// 830016E0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 830016E4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830016E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830016F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830016F0 size=448
    let mut pc: u32 = 0x830016F0;
    'dispatch: loop {
        match pc {
            0x830016F0 => {
    //   block [0x830016F0..0x830018B0)
	// 830016F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830016F4: 4BCA7D05  bl 0x82ca93f8
	ctx.lr = 0x830016F8;
	sub_82CA93D0(ctx, base);
	// 830016F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830016FC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83001700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001704: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83001708: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8300170C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83001710: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001714: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83001718: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 8300171C: 933F0004  stw r25, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 83001720: 3B860001  addi r28, r6, 1
	ctx.r[28].s64 = ctx.r[6].s64 + 1;
	// 83001724: B33F0008  sth r25, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u16 ) };
	// 83001728: 387A0001  addi r3, r26, 1
	ctx.r[3].s64 = ctx.r[26].s64 + 1;
	// 8300172C: B33F000A  sth r25, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[25].u16 ) };
	// 83001730: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 83001734: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83001738: 556B073F  clrlwi. r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300173C: 4182000C  beq 0x83001748
	if ctx.cr[0].eq {
	pc = 0x83001748; continue 'dispatch;
	}
	// 83001740: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83001744: 4800000C  b 0x83001750
	pc = 0x83001750; continue 'dispatch;
	// 83001748: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300174C: 4BFFFF3D  bl 0x83001688
	ctx.lr = 0x83001750;
	sub_83001688(ctx, base);
	// 83001750: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001754: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83001758: 419A00CC  beq cr6, 0x83001824
	if ctx.cr[6].eq {
	pc = 0x83001824; continue 'dispatch;
	}
	// 8300175C: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83001760: 409800C4  bge cr6, 0x83001824
	if !ctx.cr[6].lt {
	pc = 0x83001824; continue 'dispatch;
	}
	// 83001764: 891F000C  lbz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001768: 550B06B5  rlwinm. r11, r8, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300176C: 41820078  beq 0x830017e4
	if ctx.cr[0].eq {
	pc = 0x830017E4; continue 'dispatch;
	}
	// 83001770: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83001774: 419A0044  beq cr6, 0x830017b8
	if ctx.cr[6].eq {
	pc = 0x830017B8; continue 'dispatch;
	}
	// 83001778: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300177C: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 83001780: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 83001784: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 83001788: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300178C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001790: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83001794: 409A001C  bne cr6, 0x830017b0
	if !ctx.cr[6].eq {
	pc = 0x830017B0; continue 'dispatch;
	}
	// 83001798: 88EB000D  lbz r7, 0xd(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 8300179C: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 830017A0: 419A0084  beq cr6, 0x83001824
	if ctx.cr[6].eq {
	pc = 0x83001824; continue 'dispatch;
	}
	// 830017A4: 88EB000C  lbz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830017A8: 54E706B5  rlwinm. r7, r7, 0, 0x1a, 0x1a
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 830017AC: 40820088  bne 0x83001834
	if !ctx.cr[0].eq {
	pc = 0x83001834; continue 'dispatch;
	}
	// 830017B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830017B4: 409AFFCC  bne cr6, 0x83001780
	if !ctx.cr[6].eq {
	pc = 0x83001780; continue 'dispatch;
	}
	// 830017B8: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830017BC: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 830017C0: A13F000A  lhz r9, 0xa(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 830017C4: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 830017C8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 830017CC: B15F000A  sth r10, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 830017D0: 89430001  lbz r10, 1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 830017D4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830017D8: A17F000A  lhz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 830017DC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830017E0: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 830017E4: 550B06F7  rlwinm. r11, r8, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830017E8: 41820064  beq 0x8300184c
	if ctx.cr[0].eq {
	pc = 0x8300184C; continue 'dispatch;
	}
	// 830017EC: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830017F0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 830017F4: A13F0008  lhz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830017F8: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 830017FC: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83001800: B15F0008  sth r10, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 83001804: 89430001  lbz r10, 1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83001808: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8300180C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001810: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83001814: B17F0008  sth r11, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 83001818: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300181C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001820: 4082003C  bne 0x8300185c
	if !ctx.cr[0].eq {
	pc = 0x8300185C; continue 'dispatch;
	}
	// 83001824: 9B3F000D  stb r25, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[25].u8 ) };
	// 83001828: 7C7A1850  subf r3, r26, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	// 8300182C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83001830: 4BCA7C18  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 83001834: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83001838: 550A0673  rlwinm. r10, r8, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300183C: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 83001840: 4182FFA4  beq 0x830017e4
	if ctx.cr[0].eq {
	pc = 0x830017E4; continue 'dispatch;
	}
	// 83001844: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001848: 4BFFFF98  b 0x830017e0
	pc = 0x830017E0; continue 'dispatch;
	// 8300184C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001850: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83001854: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 83001858: B17F0008  sth r11, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 8300185C: 550B0631  rlwinm. r11, r8, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001860: 41820010  beq 0x83001870
	if ctx.cr[0].eq {
	pc = 0x83001870; continue 'dispatch;
	}
	// 83001864: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 83001868: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 8300186C: 4800001C  b 0x83001888
	pc = 0x83001888; continue 'dispatch;
	// 83001870: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001874: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83001878: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 8300187C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83001880: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83001884: 4098FFA0  bge cr6, 0x83001824
	if !ctx.cr[6].lt {
	pc = 0x83001824; continue 'dispatch;
	}
	// 83001888: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300188C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001890: 40820018  bne 0x830018a8
	if !ctx.cr[0].eq {
	pc = 0x830018A8; continue 'dispatch;
	}
	// 83001894: 7D7A1850  subf r11, r26, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	// 83001898: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8300189C: 4098FF88  bge cr6, 0x83001824
	if !ctx.cr[6].lt {
	pc = 0x83001824; continue 'dispatch;
	}
	// 830018A0: 7D6BC050  subf r11, r11, r24
	ctx.r[11].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 830018A4: B17F0008  sth r11, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 830018A8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830018AC: 4BFFFF7C  b 0x83001828
	pc = 0x83001828; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830018B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830018B0 size=24
    let mut pc: u32 = 0x830018B0;
    'dispatch: loop {
        match pc {
            0x830018B0 => {
    //   block [0x830018B0..0x830018C8)
	// 830018B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830018B4: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 830018B8: 90AB0005  stw r5, 5(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[5].u32 ) };
	// 830018BC: 90CB0001  stw r6, 1(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[6].u32 ) };
	// 830018C0: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 830018C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830018C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830018C8 size=16
    let mut pc: u32 = 0x830018C8;
    'dispatch: loop {
        match pc {
            0x830018C8 => {
    //   block [0x830018C8..0x830018D8)
	// 830018C8: 2B050009  cmplwi cr6, r5, 9
	ctx.cr[6].compare_u32(ctx.r[5].u32, 9 as u32, &mut ctx.xer);
	// 830018CC: 4098000C  bge cr6, 0x830018d8
	if !ctx.cr[6].lt {
		sub_830018D8(ctx, base);
		return;
	}
	// 830018D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830018D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830018D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830018D8 size=12
    let mut pc: u32 = 0x830018D8;
    'dispatch: loop {
        match pc {
            0x830018D8 => {
    //   block [0x830018D8..0x830018E4)
	// 830018D8: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830018DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830018E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830018E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830018E8 size=16
    let mut pc: u32 = 0x830018E8;
    'dispatch: loop {
        match pc {
            0x830018E8 => {
    //   block [0x830018E8..0x830018F8)
	// 830018E8: 2B050006  cmplwi cr6, r5, 6
	ctx.cr[6].compare_u32(ctx.r[5].u32, 6 as u32, &mut ctx.xer);
	// 830018EC: 4098000C  bge cr6, 0x830018f8
	if !ctx.cr[6].lt {
		sub_830018F8(ctx, base);
		return;
	}
	// 830018F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830018F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830018F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830018F8 size=12
    let mut pc: u32 = 0x830018F8;
    'dispatch: loop {
        match pc {
            0x830018F8 => {
    //   block [0x830018F8..0x83001904)
	// 830018F8: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830018FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83001900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001908 size=12
    let mut pc: u32 = 0x83001908;
    'dispatch: loop {
        match pc {
            0x83001908 => {
    //   block [0x83001908..0x83001914)
	// 83001908: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8300190C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83001910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001918 size=16
    let mut pc: u32 = 0x83001918;
    'dispatch: loop {
        match pc {
            0x83001918 => {
    //   block [0x83001918..0x83001928)
	// 83001918: 2B050004  cmplwi cr6, r5, 4
	ctx.cr[6].compare_u32(ctx.r[5].u32, 4 as u32, &mut ctx.xer);
	// 8300191C: 4098000C  bge cr6, 0x83001928
	if !ctx.cr[6].lt {
		sub_83001928(ctx, base);
		return;
	}
	// 83001920: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83001924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83001928 size=28
    let mut pc: u32 = 0x83001928;
    'dispatch: loop {
        match pc {
            0x83001928 => {
    //   block [0x83001928..0x83001944)
	// 83001928: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 8300192C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83001930: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001934: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83001938: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8300193C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83001940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001948 size=32
    let mut pc: u32 = 0x83001948;
    'dispatch: loop {
        match pc {
            0x83001948 => {
    //   block [0x83001948..0x83001968)
	// 83001948: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8300194C: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 83001950: E9490018  ld r10, 0x18(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	// 83001954: 79482EE0  rldicl r8, r10, 5, 0x3b
	ctx.r[8].u64 = ctx.r[10].u64 & 0x07FFFFFFFFFFFFFFu64;
	// 83001958: 7F285840  cmpld cr6, r8, r11
	ctx.cr[6].compare_u64(ctx.r[8].u64, ctx.r[11].u64, &mut ctx.xer);
	// 8300195C: 4199000C  bgt cr6, 0x83001968
	if ctx.cr[6].gt {
		sub_83001968(ctx, base);
		return;
	}
	// 83001960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83001964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001968 size=20
    let mut pc: u32 = 0x83001968;
    'dispatch: loop {
        match pc {
            0x83001968 => {
    //   block [0x83001968..0x8300197C)
	// 83001968: 794A0140  clrldi r10, r10, 5
	ctx.r[10].u64 = ctx.r[10].u64 & 0x07FFFFFFFFFFFFFFu64;
	// 8300196C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83001970: 796AD80E  rldimi r10, r11, 0x3b, 0
	ctx.r[10].u64 = ((ctx.r[11].u64).rotate_left(59) & 0xF800000000000000) | (ctx.r[10].u64 & 0x07FFFFFFFFFFFFFF);
	// 83001974: F9490018  std r10, 0x18(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 83001978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001980 size=132
    let mut pc: u32 = 0x83001980;
    'dispatch: loop {
        match pc {
            0x83001980 => {
    //   block [0x83001980..0x83001A04)
	// 83001980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83001988: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300198C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83001990: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001994: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001998: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300199C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830019A0: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830019A4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830019A8: 5565AFFE  rlwinm r5, r11, 0x15, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 830019AC: 4BFFD4D5  bl 0x82ffee80
	ctx.lr = 0x830019B0;
	sub_82FFEE80(ctx, base);
	// 830019B0: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 830019B4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830019B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830019BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830019C0: 4082000C  bne 0x830019cc
	if !ctx.cr[0].eq {
	pc = 0x830019CC; continue 'dispatch;
	}
	// 830019C4: 4BFFCE1D  bl 0x82ffe7e0
	ctx.lr = 0x830019C8;
	sub_82FFE7E0(ctx, base);
	// 830019C8: 48000024  b 0x830019ec
	pc = 0x830019EC; continue 'dispatch;
	// 830019CC: A15F0048  lhz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830019D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830019D4: 997F004B  stb r11, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 830019D8: 614B8000  ori r11, r10, 0x8000
	ctx.r[11].u64 = ctx.r[10].u64 | 32768;
	// 830019DC: 993F004A  stb r9, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[9].u8 ) };
	// 830019E0: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 830019E4: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 830019E8: 4BFFD691  bl 0x82fff078
	ctx.lr = 0x830019EC;
	sub_82FFF078(ctx, base);
	// 830019EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830019F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830019F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830019F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830019FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83001A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001A08 size=132
    let mut pc: u32 = 0x83001A08;
    'dispatch: loop {
        match pc {
            0x83001A08 => {
    //   block [0x83001A08..0x83001A8C)
	// 83001A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83001A10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83001A14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001A18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001A1C: A15F0048  lhz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001A20: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001A24: 614A2000  ori r10, r10, 0x2000
	ctx.r[10].u64 = ctx.r[10].u64 | 8192;
	// 83001A28: 55690673  rlwinm. r9, r11, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83001A2C: B15F0048  sth r10, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u16 ) };
	// 83001A30: 4082000C  bne 0x83001a3c
	if !ctx.cr[0].eq {
	pc = 0x83001A3C; continue 'dispatch;
	}
	// 83001A34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83001A38: 48000040  b 0x83001a78
	pc = 0x83001A78; continue 'dispatch;
	// 83001A3C: 893F004B  lbz r9, 0x4b(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83001A40: 716B00BF  andi. r11, r11, 0xbf
	ctx.r[11].u64 = ctx.r[11].u64 & 191;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001A44: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 83001A48: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001A4C: 554A0421  rlwinm. r10, r10, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001A50: 993F004B  stb r9, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[9].u8 ) };
	// 83001A54: 4082001C  bne 0x83001a70
	if !ctx.cr[0].eq {
	pc = 0x83001A70; continue 'dispatch;
	}
	// 83001A58: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001A5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83001A60: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001A64: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 83001A68: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001A6C: 4BFFD60D  bl 0x82fff078
	ctx.lr = 0x83001A70;
	sub_82FFF078(ctx, base);
	// 83001A70: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001A74: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83001A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83001A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83001A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83001A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83001A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001A90 size=48
    let mut pc: u32 = 0x83001A90;
    'dispatch: loop {
        match pc {
            0x83001A90 => {
    //   block [0x83001A90..0x83001AC0)
	// 83001A90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001A94: A1640048  lhz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001A98: 8944004B  lbz r10, 0x4b(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(75 as u32) ) } as u64;
	// 83001A9C: 8924004A  lbz r9, 0x4a(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001AA0: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 83001AA4: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 83001AA8: 712900DF  andi. r9, r9, 0xdf
	ctx.r[9].u64 = ctx.r[9].u64 & 223;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83001AAC: B1640048  sth r11, 0x48(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001AB0: 556B0421  rlwinm. r11, r11, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001AB4: 9944004B  stb r10, 0x4b(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 83001AB8: 9924004A  stb r9, 0x4a(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(74 as u32), ctx.r[9].u8 ) };
	// 83001ABC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001AC0 size=20
    let mut pc: u32 = 0x83001AC0;
    'dispatch: loop {
        match pc {
            0x83001AC0 => {
    //   block [0x83001AC0..0x83001AD4)
	// 83001AC0: A1640048  lhz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001AC4: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001AC8: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 83001ACC: B1640048  sth r11, 0x48(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001AD0: 4BFFD5A8  b 0x82fff078
	sub_82FFF078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001AD4 size=4
    let mut pc: u32 = 0x83001AD4;
    'dispatch: loop {
        match pc {
            0x83001AD4 => {
    //   block [0x83001AD4..0x83001AD8)
	// 83001AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001AD8 size=44
    let mut pc: u32 = 0x83001AD8;
    'dispatch: loop {
        match pc {
            0x83001AD8 => {
    //   block [0x83001AD8..0x83001B04)
	// 83001AD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83001AE0: 8904004A  lbz r8, 0x4a(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001AE4: 8944004B  lbz r10, 0x4b(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(75 as u32) ) } as u64;
	// 83001AE8: A1640048  lhz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001AEC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 83001AF0: 90A4004C  stw r5, 0x4c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 83001AF4: 55680021  rlwinm. r8, r11, 0, 0, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83001AF8: 9924004A  stb r9, 0x4a(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(74 as u32), ctx.r[9].u8 ) };
	// 83001AFC: 9944004B  stb r10, 0x4b(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 83001B00: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B04 size=16
    let mut pc: u32 = 0x83001B04;
    'dispatch: loop {
        match pc {
            0x83001B04 => {
    //   block [0x83001B04..0x83001B14)
	// 83001B04: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 83001B08: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001B0C: B1640048  sth r11, 0x48(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001B10: 4BFFD568  b 0x82fff078
	sub_82FFF078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B14 size=4
    let mut pc: u32 = 0x83001B14;
    'dispatch: loop {
        match pc {
            0x83001B14 => {
    //   block [0x83001B14..0x83001B18)
	// 83001B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B18 size=44
    let mut pc: u32 = 0x83001B18;
    'dispatch: loop {
        match pc {
            0x83001B18 => {
    //   block [0x83001B18..0x83001B44)
	// 83001B18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001B1C: 8944004B  lbz r10, 0x4b(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(75 as u32) ) } as u64;
	// 83001B20: 8924004A  lbz r9, 0x4a(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001B24: A1640048  lhz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001B28: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 83001B2C: 712900DF  andi. r9, r9, 0xdf
	ctx.r[9].u64 = ctx.r[9].u64 & 223;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83001B30: 90A4004C  stw r5, 0x4c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 83001B34: 55680021  rlwinm. r8, r11, 0, 0, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83001B38: 9944004B  stb r10, 0x4b(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 83001B3C: 9924004A  stb r9, 0x4a(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(74 as u32), ctx.r[9].u8 ) };
	// 83001B40: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B44 size=16
    let mut pc: u32 = 0x83001B44;
    'dispatch: loop {
        match pc {
            0x83001B44 => {
    //   block [0x83001B44..0x83001B54)
	// 83001B44: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 83001B48: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001B4C: B1640048  sth r11, 0x48(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001B50: 4BFFD528  b 0x82fff078
	sub_82FFF078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B54 size=4
    let mut pc: u32 = 0x83001B54;
    'dispatch: loop {
        match pc {
            0x83001B54 => {
    //   block [0x83001B54..0x83001B58)
	// 83001B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B58 size=16
    let mut pc: u32 = 0x83001B58;
    'dispatch: loop {
        match pc {
            0x83001B58 => {
    //   block [0x83001B58..0x83001B68)
	// 83001B58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001B5C: 8964004B  lbz r11, 0x4b(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(75 as u32) ) } as u64;
	// 83001B60: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001B64: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B68 size=32
    let mut pc: u32 = 0x83001B68;
    'dispatch: loop {
        match pc {
            0x83001B68 => {
    //   block [0x83001B68..0x83001B88)
	// 83001B68: A1640048  lhz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001B6C: 8944004A  lbz r10, 0x4a(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001B70: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 83001B74: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001B78: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001B7C: B1640048  sth r11, 0x48(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001B80: 40820008  bne 0x83001b88
	if !ctx.cr[0].eq {
		sub_83001B88(ctx, base);
		return;
	}
	// 83001B84: 4BFFCC5C  b 0x82ffe7e0
	sub_82FFE7E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B88 size=8
    let mut pc: u32 = 0x83001B88;
    'dispatch: loop {
        match pc {
            0x83001B88 => {
    //   block [0x83001B88..0x83001B90)
	// 83001B88: 4BFFCCB0  b 0x82ffe838
	sub_82FFE838(ctx, base);
	return;
	// 83001B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001B90 size=20
    let mut pc: u32 = 0x83001B90;
    'dispatch: loop {
        match pc {
            0x83001B90 => {
    //   block [0x83001B90..0x83001BA4)
	// 83001B90: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83001B94: 2B053FFF  cmplwi cr6, r5, 0x3fff
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16383 as u32, &mut ctx.xer);
	// 83001B98: 4099000C  ble cr6, 0x83001ba4
	if !ctx.cr[6].gt {
		sub_83001BA4(ctx, base);
		return;
	}
	// 83001B9C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83001BA0: 48000020  b 0x83001bc0
	sub_83001BB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001BA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001BA4 size=16
    let mut pc: u32 = 0x83001BA4;
    'dispatch: loop {
        match pc {
            0x83001BA4 => {
    //   block [0x83001BA4..0x83001BB4)
	// 83001BA4: 2B05007F  cmplwi cr6, r5, 0x7f
	ctx.cr[6].compare_u32(ctx.r[5].u32, 127 as u32, &mut ctx.xer);
	// 83001BA8: 4099000C  ble cr6, 0x83001bb4
	if !ctx.cr[6].gt {
		sub_83001BB4(ctx, base);
		return;
	}
	// 83001BAC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83001BB0: 48000010  b 0x83001bc0
	sub_83001BB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001BB4 size=72
    let mut pc: u32 = 0x83001BB4;
    'dispatch: loop {
        match pc {
            0x83001BB4 => {
    //   block [0x83001BB4..0x83001BFC)
	// 83001BB4: 2B05000F  cmplwi cr6, r5, 0xf
	ctx.cr[6].compare_u32(ctx.r[5].u32, 15 as u32, &mut ctx.xer);
	// 83001BB8: 40990008  ble cr6, 0x83001bc0
	if !ctx.cr[6].gt {
	pc = 0x83001BC0; continue 'dispatch;
	}
	// 83001BBC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83001BC0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83001BC4: 419A0008  beq cr6, 0x83001bcc
	if ctx.cr[6].eq {
	pc = 0x83001BCC; continue 'dispatch;
	}
	// 83001BC8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83001BCC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001BD0: 4082000C  bne 0x83001bdc
	if !ctx.cr[0].eq {
	pc = 0x83001BDC; continue 'dispatch;
	}
	// 83001BD4: 2B0401FE  cmplwi cr6, r4, 0x1fe
	ctx.cr[6].compare_u32(ctx.r[4].u32, 510 as u32, &mut ctx.xer);
	// 83001BD8: 40990010  ble cr6, 0x83001be8
	if !ctx.cr[6].gt {
	pc = 0x83001BE8; continue 'dispatch;
	}
	// 83001BDC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83001BE0: 409A0008  bne cr6, 0x83001be8
	if !ctx.cr[6].eq {
	pc = 0x83001BE8; continue 'dispatch;
	}
	// 83001BE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001BE8: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 83001BEC: 419A0008  beq cr6, 0x83001bf4
	if ctx.cr[6].eq {
	pc = 0x83001BF4; continue 'dispatch;
	}
	// 83001BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001BF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83001BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001C00 size=20
    let mut pc: u32 = 0x83001C00;
    'dispatch: loop {
        match pc {
            0x83001C00 => {
    //   block [0x83001C00..0x83001C14)
	// 83001C00: 3967FFFE  addi r11, r7, -2
	ctx.r[11].s64 = ctx.r[7].s64 + -2;
	// 83001C04: 2B043FFF  cmplwi cr6, r4, 0x3fff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16383 as u32, &mut ctx.xer);
	// 83001C08: 4099000C  ble cr6, 0x83001c14
	if !ctx.cr[6].gt {
		sub_83001C14(ctx, base);
		return;
	}
	// 83001C0C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83001C10: 48000020  b 0x83001c30
	sub_83001C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001C14 size=16
    let mut pc: u32 = 0x83001C14;
    'dispatch: loop {
        match pc {
            0x83001C14 => {
    //   block [0x83001C14..0x83001C24)
	// 83001C14: 2B04007F  cmplwi cr6, r4, 0x7f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 127 as u32, &mut ctx.xer);
	// 83001C18: 4099000C  ble cr6, 0x83001c24
	if !ctx.cr[6].gt {
		sub_83001C24(ctx, base);
		return;
	}
	// 83001C1C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 83001C20: 48000010  b 0x83001c30
	sub_83001C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001C24 size=84
    let mut pc: u32 = 0x83001C24;
    'dispatch: loop {
        match pc {
            0x83001C24 => {
    //   block [0x83001C24..0x83001C78)
	// 83001C24: 2B04000F  cmplwi cr6, r4, 0xf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 15 as u32, &mut ctx.xer);
	// 83001C28: 40990008  ble cr6, 0x83001c30
	if !ctx.cr[6].gt {
	pc = 0x83001C30; continue 'dispatch;
	}
	// 83001C2C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83001C30: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83001C34: 419A0008  beq cr6, 0x83001c3c
	if ctx.cr[6].eq {
	pc = 0x83001C3C; continue 'dispatch;
	}
	// 83001C38: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 83001C3C: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 83001C40: 419A0008  beq cr6, 0x83001c48
	if ctx.cr[6].eq {
	pc = 0x83001C48; continue 'dispatch;
	}
	// 83001C44: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83001C48: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001C4C: 4082000C  bne 0x83001c58
	if !ctx.cr[0].eq {
	pc = 0x83001C58; continue 'dispatch;
	}
	// 83001C50: 2B0B01FE  cmplwi cr6, r11, 0x1fe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 510 as u32, &mut ctx.xer);
	// 83001C54: 40990010  ble cr6, 0x83001c64
	if !ctx.cr[6].gt {
	pc = 0x83001C64; continue 'dispatch;
	}
	// 83001C58: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83001C5C: 409A0008  bne cr6, 0x83001c64
	if !ctx.cr[6].eq {
	pc = 0x83001C64; continue 'dispatch;
	}
	// 83001C60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83001C64: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83001C68: 40990008  ble cr6, 0x83001c70
	if !ctx.cr[6].gt {
	pc = 0x83001C70; continue 'dispatch;
	}
	// 83001C6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83001C70: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83001C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001C78 size=236
    let mut pc: u32 = 0x83001C78;
    'dispatch: loop {
        match pc {
            0x83001C78 => {
    //   block [0x83001C78..0x83001D64)
	// 83001C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001C7C: 4BCA7789  bl 0x82ca9404
	ctx.lr = 0x83001C80;
	sub_82CA93D0(ctx, base);
	// 83001C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001C88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 83001C8C: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 83001C90: 392AB2B8  addi r9, r10, -0x4d48
	ctx.r[9].s64 = ctx.r[10].s64 + -19784;
	// 83001C94: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 83001C98: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 83001C9C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83001CA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83001CA4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83001CA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83001CAC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83001CB0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83001CB4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83001CB8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83001CBC: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 83001CC0: F8BF0018  std r5, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[5].u64 ) };
	// 83001CC4: 4B2643ED  bl 0x822660b0
	ctx.lr = 0x83001CC8;
	sub_822660B0(ctx, base);
	// 83001CC8: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83001CCC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83001CD0: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 83001CD4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83001CD8: 4BCA77A9  bl 0x82ca9480
	ctx.lr = 0x83001CDC;
	sub_82CA9480(ctx, base);
	// 83001CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83001CE0: 57CA0675  rlwinm. r10, r30, 0, 0x19, 0x1a
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001CE4: 937F0030  stw r27, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[27].u32 ) };
	// 83001CE8: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 83001CEC: 93BF0044  stw r29, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83001CF0: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83001CF4: 997F004B  stb r11, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 83001CF8: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001CFC: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001D00: 4082000C  bne 0x83001d0c
	if !ctx.cr[0].eq {
	pc = 0x83001D0C; continue 'dispatch;
	}
	// 83001D04: 39600800  li r11, 0x800
	ctx.r[11].s64 = 2048;
	// 83001D08: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001D0C: 57CB05EF  rlwinm. r11, r30, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001D10: 4182000C  beq 0x83001d1c
	if ctx.cr[0].eq {
	pc = 0x83001D1C; continue 'dispatch;
	}
	// 83001D14: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 83001D18: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001D1C: 57CB05AD  rlwinm. r11, r30, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001D20: 41820010  beq 0x83001d30
	if ctx.cr[0].eq {
	pc = 0x83001D30; continue 'dispatch;
	}
	// 83001D24: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001D28: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 83001D2C: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001D30: 57CB056B  rlwinm. r11, r30, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001D34: 41820010  beq 0x83001d44
	if ctx.cr[0].eq {
	pc = 0x83001D44; continue 'dispatch;
	}
	// 83001D38: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001D3C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 83001D40: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001D44: 57CB0631  rlwinm. r11, r30, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001D48: 41820010  beq 0x83001d58
	if ctx.cr[0].eq {
	pc = 0x83001D58; continue 'dispatch;
	}
	// 83001D4C: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83001D50: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 83001D54: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83001D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83001D60: 4BCA76F4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001D68 size=12
    let mut pc: u32 = 0x83001D68;
    'dispatch: loop {
        match pc {
            0x83001D68 => {
    //   block [0x83001D68..0x83001D74)
	// 83001D68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001D6C: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 83001D70: 4BFFA4C0  b 0x82ffc230
	sub_82FFC230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83001D78 size=664
    let mut pc: u32 = 0x83001D78;
    'dispatch: loop {
        match pc {
            0x83001D78 => {
    //   block [0x83001D78..0x83002010)
	// 83001D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001D7C: 4BCA7661  bl 0x82ca93dc
	ctx.lr = 0x83001D80;
	sub_82CA93D0(ctx, base);
	// 83001D80: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001D84: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83001D88: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 83001D8C: 7CF43B78  mr r20, r7
	ctx.r[20].u64 = ctx.r[7].u64;
	// 83001D90: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 83001D94: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 83001D98: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83001D9C: 41820014  beq 0x83001db0
	if ctx.cr[0].eq {
	pc = 0x83001DB0; continue 'dispatch;
	}
	// 83001DA0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83001DA4: 409A01F8  bne cr6, 0x83001f9c
	if !ctx.cr[6].eq {
	pc = 0x83001F9C; continue 'dispatch;
	}
	// 83001DA8: 7E719B78  mr r17, r19
	ctx.r[17].u64 = ctx.r[19].u64;
	// 83001DAC: 48000008  b 0x83001db4
	pc = 0x83001DB4; continue 'dispatch;
	// 83001DB0: 3A200001  li r17, 1
	ctx.r[17].s64 = 1;
	// 83001DB4: 825B0014  lwz r18, 0x14(r27)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001DB8: 5579FFFF  rlwinm. r25, r11, 0x1f, 0x1f, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83001DBC: 81720098  lwz r11, 0x98(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(152 as u32) ) } as u64;
	// 83001DC0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83001DC4: 831B0044  lwz r24, 0x44(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 83001DC8: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	// 83001DCC: 5577023E  clrlwi r23, r11, 8
	ctx.r[23].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 83001DD0: 4082000C  bne 0x83001ddc
	if !ctx.cr[0].eq {
	pc = 0x83001DDC; continue 'dispatch;
	}
	// 83001DD4: 2F180003  cmpwi cr6, r24, 3
	ctx.cr[6].compare_i32(ctx.r[24].s32, 3, &mut ctx.xer);
	// 83001DD8: 409A0080  bne cr6, 0x83001e58
	if !ctx.cr[6].eq {
	pc = 0x83001E58; continue 'dispatch;
	}
	// 83001DDC: 54CB1838  slwi r11, r6, 3
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83001DE0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83001DE4: 7D4BA050  subf r10, r11, r20
	ctx.r[10].s64 = ctx.r[20].s64 - ctx.r[11].s64;
	// 83001DE8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 83001DEC: 4198005C  blt cr6, 0x83001e48
	if ctx.cr[6].lt {
	pc = 0x83001E48; continue 'dispatch;
	}
	// 83001DF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83001DF4: 48000054  b 0x83001e48
	pc = 0x83001E48; continue 'dispatch;
	// 83001DF8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001DFC: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 83001E00: 7F0B9040  cmplw cr6, r11, r18
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[18].u32, &mut ctx.xer);
	// 83001E04: 409A003C  bne cr6, 0x83001e40
	if !ctx.cr[6].eq {
	pc = 0x83001E40; continue 'dispatch;
	}
	// 83001E08: 81690034  lwz r11, 0x34(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 83001E0C: 556707BD  rlwinm. r7, r11, 0, 0x1e, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83001E10: 40820010  bne 0x83001e20
	if !ctx.cr[0].eq {
	pc = 0x83001E20; continue 'dispatch;
	}
	// 83001E14: A0E90048  lhz r7, 0x48(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001E18: 54E705AD  rlwinm. r7, r7, 0, 0x16, 0x16
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83001E1C: 41820024  beq 0x83001e40
	if ctx.cr[0].eq {
	pc = 0x83001E40; continue 'dispatch;
	}
	// 83001E20: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001E24: 7E799B78  mr r25, r19
	ctx.r[25].u64 = ctx.r[19].u64;
	// 83001E28: 7E7A9B78  mr r26, r19
	ctx.r[26].u64 = ctx.r[19].u64;
	// 83001E2C: 40820028  bne 0x83001e54
	if !ctx.cr[0].eq {
	pc = 0x83001E54; continue 'dispatch;
	}
	// 83001E30: A1690048  lhz r11, 0x48(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001E34: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001E38: 4082001C  bne 0x83001e54
	if !ctx.cr[0].eq {
	pc = 0x83001E54; continue 'dispatch;
	}
	// 83001E3C: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 83001E40: 550B1838  slwi r11, r8, 3
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83001E44: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83001E48: 7F0AA040  cmplw cr6, r10, r20
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[20].u32, &mut ctx.xer);
	// 83001E4C: 409AFFAC  bne cr6, 0x83001df8
	if !ctx.cr[6].eq {
	pc = 0x83001DF8; continue 'dispatch;
	}
	// 83001E50: 48000008  b 0x83001e58
	pc = 0x83001E58; continue 'dispatch;
	// 83001E54: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	// 83001E58: 817B003C  lwz r11, 0x3c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 83001E5C: 2F180003  cmpwi cr6, r24, 3
	ctx.cr[6].compare_i32(ctx.r[24].s32, 3, &mut ctx.xer);
	// 83001E60: 83FB0040  lwz r31, 0x40(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 83001E64: 7FDF5850  subf r30, r31, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 83001E68: 409A0108  bne cr6, 0x83001f70
	if !ctx.cr[6].eq {
	pc = 0x83001F70; continue 'dispatch;
	}
	// 83001E6C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83001E70: 41980010  blt cr6, 0x83001e80
	if ctx.cr[6].lt {
	pc = 0x83001E80; continue 'dispatch;
	}
	// 83001E74: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 83001E78: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 83001E7C: 41990008  bgt cr6, 0x83001e84
	if ctx.cr[6].gt {
	pc = 0x83001E84; continue 'dispatch;
	}
	// 83001E80: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83001E84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83001E88: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83001E8C: 419A0008  beq cr6, 0x83001e94
	if ctx.cr[6].eq {
	pc = 0x83001E94; continue 'dispatch;
	}
	// 83001E90: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83001E94: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 83001E98: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83001E9C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83001EA0: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 83001EA4: 4BFFFD5D  bl 0x83001c00
	ctx.lr = 0x83001EA8;
	sub_83001C00(ctx, base);
	// 83001EA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83001EAC: 419A0038  beq cr6, 0x83001ee4
	if ctx.cr[6].eq {
	pc = 0x83001EE4; continue 'dispatch;
	}
	// 83001EB0: 7D7E1810  subfc r11, r30, r3
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[30].u32;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 83001EB4: 3B000080  li r24, 0x80
	ctx.r[24].s64 = 128;
	// 83001EB8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83001EBC: 557D07FF  clrlwi. r29, r11, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83001EC0: 4082001C  bne 0x83001edc
	if !ctx.cr[0].eq {
	pc = 0x83001EDC; continue 'dispatch;
	}
	// 83001EC4: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83001EC8: 40980014  bge cr6, 0x83001edc
	if !ctx.cr[6].lt {
	pc = 0x83001EDC; continue 'dispatch;
	}
	// 83001ECC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83001ED0: 419A000C  beq cr6, 0x83001edc
	if ctx.cr[6].eq {
	pc = 0x83001EDC; continue 'dispatch;
	}
	// 83001ED4: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 83001ED8: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 83001EDC: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 83001EE0: 48000054  b 0x83001f34
	pc = 0x83001F34; continue 'dispatch;
	// 83001EE4: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 83001EE8: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83001EEC: 41990028  bgt cr6, 0x83001f14
	if ctx.cr[6].gt {
	pc = 0x83001F14; continue 'dispatch;
	}
	// 83001EF0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 83001EF4: 3B000080  li r24, 0x80
	ctx.r[24].s64 = 128;
	// 83001EF8: 915B0040  stw r10, 0x40(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83001EFC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83001F00: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 83001F04: 40980074  bge cr6, 0x83001f78
	if !ctx.cr[6].lt {
	pc = 0x83001F78; continue 'dispatch;
	}
	// 83001F08: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83001F0C: 419A006C  beq cr6, 0x83001f78
	if ctx.cr[6].eq {
	pc = 0x83001F78; continue 'dispatch;
	}
	// 83001F10: 48000064  b 0x83001f74
	pc = 0x83001F74; continue 'dispatch;
	// 83001F14: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 83001F18: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83001F1C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83001F20: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83001F24: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83001F28: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 83001F2C: 4BFFFCD5  bl 0x83001c00
	ctx.lr = 0x83001F30;
	sub_83001C00(ctx, base);
	// 83001F30: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83001F34: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83001F38: 419A0040  beq cr6, 0x83001f78
	if ctx.cr[6].eq {
	pc = 0x83001F78; continue 'dispatch;
	}
	// 83001F3C: 2B1E0988  cmplwi cr6, r30, 0x988
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2440 as u32, &mut ctx.xer);
	// 83001F40: 40980018  bge cr6, 0x83001f58
	if !ctx.cr[6].lt {
	pc = 0x83001F58; continue 'dispatch;
	}
	// 83001F44: 7D63F050  subf r11, r3, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 83001F48: 2B0B04C4  cmplwi cr6, r11, 0x4c4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1220 as u32, &mut ctx.xer);
	// 83001F4C: 40990014  ble cr6, 0x83001f60
	if !ctx.cr[6].gt {
	pc = 0x83001F60; continue 'dispatch;
	}
	// 83001F50: 57DEF87E  srwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83001F54: 48000024  b 0x83001f78
	pc = 0x83001F78; continue 'dispatch;
	// 83001F58: 2B030100  cmplwi cr6, r3, 0x100
	ctx.cr[6].compare_u32(ctx.r[3].u32, 256 as u32, &mut ctx.xer);
	// 83001F5C: 4099000C  ble cr6, 0x83001f68
	if !ctx.cr[6].gt {
	pc = 0x83001F68; continue 'dispatch;
	}
	// 83001F60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001F64: 48000014  b 0x83001f78
	pc = 0x83001F78; continue 'dispatch;
	// 83001F68: 3BC004C4  li r30, 0x4c4
	ctx.r[30].s64 = 1220;
	// 83001F6C: 4800000C  b 0x83001f78
	pc = 0x83001F78; continue 'dispatch;
	// 83001F70: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 83001F74: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 83001F78: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83001F7C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83001F80: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83001F84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83001F88: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83001F8C: 4BFFFC05  bl 0x83001b90
	ctx.lr = 0x83001F90;
	sub_83001B90(ctx, base);
	// 83001F90: 7FE3F214  add r31, r3, r30
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83001F94: 7F1FB040  cmplw cr6, r31, r22
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[22].u32, &mut ctx.xer);
	// 83001F98: 40990018  ble cr6, 0x83001fb0
	if !ctx.cr[6].gt {
	pc = 0x83001FB0; continue 'dispatch;
	}
	// 83001F9C: 92740000  stw r19, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 83001FA0: 7E719B78  mr r17, r19
	ctx.r[17].u64 = ctx.r[19].u64;
	// 83001FA4: B2740004  sth r19, 4(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[19].u16 ) };
	// 83001FA8: B2740006  sth r19, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[19].u16 ) };
	// 83001FAC: 48000058  b 0x83002004
	pc = 0x83002004; continue 'dispatch;
	// 83001FB0: 817B0040  lwz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 83001FB4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83001FB8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83001FBC: 917B0040  stw r11, 0x40(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83001FC0: 419A0014  beq cr6, 0x83001fd4
	if ctx.cr[6].eq {
	pc = 0x83001FD4; continue 'dispatch;
	}
	// 83001FC4: A17B0048  lhz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001FC8: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 83001FCC: B17B0048  sth r11, 0x48(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001FD0: 48000018  b 0x83001fe8
	pc = 0x83001FE8; continue 'dispatch;
	// 83001FD4: A17B0048  lhz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001FD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83001FDC: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 83001FE0: 5565AFFE  rlwinm r5, r11, 0x15, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 83001FE4: 4BFFCE9D  bl 0x82ffee80
	ctx.lr = 0x83001FE8;
	sub_82FFEE80(ctx, base);
	// 83001FE8: A17B0048  lhz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001FEC: 53350FBC  rlwimi r21, r25, 1, 0x1e, 0x1e
	ctx.r[21].u64 = (((ctx.r[25].u32).rotate_left(1) as u64) & 0x0000000000000002) | (ctx.r[21].u64 & 0xFFFFFFFFFFFFFFFD);
	// 83001FF0: 52AB3DF0  rlwimi r11, r21, 7, 0x17, 0x18
	ctx.r[11].u64 = (((ctx.r[21].u32).rotate_left(7) as u64) & 0x0000000000000180) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFE7F);
	// 83001FF4: B17B0048  sth r11, 0x48(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83001FF8: 93740000  stw r27, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83001FFC: B3F40004  sth r31, 4(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 83002000: B3D40006  sth r30, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 83002004: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 83002008: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8300200C: 4BCA7420  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002010 size=816
    let mut pc: u32 = 0x83002010;
    'dispatch: loop {
        match pc {
            0x83002010 => {
    //   block [0x83002010..0x83002340)
	// 83002010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002014: 4BCA73C9  bl 0x82ca93dc
	ctx.lr = 0x83002018;
	sub_82CA93D0(ctx, base);
	// 83002018: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300201C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002020: A38A0000  lhz r28, 0(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002024: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002028: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 8300202C: 7D114378  mr r17, r8
	ctx.r[17].u64 = ctx.r[8].u64;
	// 83002030: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 83002034: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83002038: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8300203C: 83BF0034  lwz r29, 0x34(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002040: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83002044: 82DF0044  lwz r22, 0x44(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83002048: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8300204C: 7E7C5851  subf. r19, r28, r11
	ctx.r[19].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 83002050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83002054: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83002058: 41820018  beq 0x83002070
	if ctx.cr[0].eq {
	pc = 0x83002070; continue 'dispatch;
	}
	// 8300205C: A13F0048  lhz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002060: 3AC00080  li r22, 0x80
	ctx.r[22].s64 = 128;
	// 83002064: 552905AD  rlwinm. r9, r9, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83002068: 41820008  beq 0x83002070
	if ctx.cr[0].eq {
	pc = 0x83002070; continue 'dispatch;
	}
	// 8300206C: 63BD0003  ori r29, r29, 3
	ctx.r[29].u64 = ctx.r[29].u64 | 3;
	// 83002070: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83002074: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83002078: 40980010  bge cr6, 0x83002088
	if !ctx.cr[6].lt {
	pc = 0x83002088; continue 'dispatch;
	}
	// 8300207C: 57AB07B6  rlwinm r11, r29, 0, 0x1e, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 83002080: 556B0628  rlwinm r11, r11, 0, 0x18, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83002084: 617D0003  ori r29, r11, 3
	ctx.r[29].u64 = ctx.r[11].u64 | 3;
	// 83002088: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300208C: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 83002090: 41820048  beq 0x830020d8
	if ctx.cr[0].eq {
	pc = 0x830020D8; continue 'dispatch;
	}
	// 83002094: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002098: 92460000  stw r18, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 8300209C: 7E5A9378  mr r26, r18
	ctx.r[26].u64 = ctx.r[18].u64;
	// 830020A0: 40820014  bne 0x830020b4
	if !ctx.cr[0].eq {
	pc = 0x830020B4; continue 'dispatch;
	}
	// 830020A4: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830020A8: A16900A8  lhz r11, 0xa8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(168 as u32) ) } as u64;
	// 830020AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830020B0: B16900A8  sth r11, 0xa8(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(168 as u32), ctx.r[11].u16 ) };
	// 830020B4: 57AB077B  rlwinm. r11, r29, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830020B8: 4182000C  beq 0x830020c4
	if ctx.cr[0].eq {
	pc = 0x830020C4; continue 'dispatch;
	}
	// 830020BC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 830020C0: 4800001C  b 0x830020dc
	pc = 0x830020DC; continue 'dispatch;
	// 830020C4: 57AB0738  rlwinm r11, r29, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 830020C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830020CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830020D0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 830020D4: 48000008  b 0x830020dc
	pc = 0x830020DC; continue 'dispatch;
	// 830020D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830020DC: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830020E0: 41820038  beq 0x83002118
	if ctx.cr[0].eq {
	pc = 0x83002118; continue 'dispatch;
	}
	// 830020E4: 7E579378  mr r23, r18
	ctx.r[23].u64 = ctx.r[18].u64;
	// 830020E8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 830020EC: 419A0014  beq cr6, 0x83002100
	if ctx.cr[6].eq {
	pc = 0x83002100; continue 'dispatch;
	}
	// 830020F0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830020F4: A16A00AA  lhz r11, 0xaa(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(170 as u32) ) } as u64;
	// 830020F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830020FC: B16A00AA  sth r11, 0xaa(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(170 as u32), ctx.r[11].u16 ) };
	// 83002100: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002104: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002108: 5566C7FE  rlwinm r6, r11, 0x18, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8300210C: 556ACFFE  rlwinm r10, r11, 0x19, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 83002110: 68D90001  xori r25, r6, 1
	ctx.r[25].u64 = ctx.r[6].u64 ^ 1;
	// 83002114: A30800AA  lhz r24, 0xaa(r8)
	ctx.r[24].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(170 as u32) ) } as u64;
	// 83002118: 57AB056B  rlwinm. r11, r29, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300211C: 41820008  beq 0x83002124
	if ctx.cr[0].eq {
	pc = 0x83002124; continue 'dispatch;
	}
	// 83002120: 92510000  stw r18, 0(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 83002124: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002128: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8300212C: 40990008  ble cr6, 0x83002134
	if !ctx.cr[6].gt {
	pc = 0x83002134; continue 'dispatch;
	}
	// 83002130: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83002134: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002138: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8300213C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83002140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002144: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002148: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300214C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83002150: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 83002154: 4BFFAE35  bl 0x82ffcf88
	ctx.lr = 0x83002158;
	sub_82FFCF88(ctx, base);
	// 83002158: 57AB056D  rlwinm. r11, r29, 0, 0x15, 0x16
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300215C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83002160: 4182001C  beq 0x8300217c
	if ctx.cr[0].eq {
	pc = 0x8300217C; continue 'dispatch;
	}
	// 83002164: 397B0009  addi r11, r27, 9
	ctx.r[11].s64 = ctx.r[27].s64 + 9;
	// 83002168: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300216C: 7FEBF12E  stwx r31, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u32) };
	// 83002170: A17E014A  lhz r11, 0x14a(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(330 as u32) ) } as u64;
	// 83002174: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83002178: B17E014A  sth r11, 0x14a(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(330 as u32), ctx.r[11].u16 ) };
	// 8300217C: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83002180: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002184: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002188: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8300218C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83002190: B3010056  sth r24, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[24].u16 ) };
	// 83002194: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 83002198: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8300219C: 814A0098  lwz r10, 0x98(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(152 as u32) ) } as u64;
	// 830021A0: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 830021A4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830021A8: A16B004C  lhz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 830021AC: 554A023E  clrlwi r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 830021B0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830021B4: 7FAB1A14  add r29, r11, r3
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 830021B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830021BC: 4BFFF3AD  bl 0x83001568
	ctx.lr = 0x830021C0;
	sub_83001568(ctx, base);
	// 830021C0: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830021C4: 7D63E214  add r11, r3, r28
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 830021C8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830021CC: 7F43EA14  add r26, r3, r29
	ctx.r[26].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 830021D0: 915E0020  stw r10, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 830021D4: B1740000  sth r11, 0(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830021D8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 830021DC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830021E0: 409A0054  bne cr6, 0x83002234
	if !ctx.cr[6].eq {
	pc = 0x83002234; continue 'dispatch;
	}
	// 830021E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830021E8: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830021EC: A16B00A8  lhz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 830021F0: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830021F4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830021F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830021FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83002200: 409A0008  bne cr6, 0x83002208
	if !ctx.cr[6].eq {
	pc = 0x83002208; continue 'dispatch;
	}
	// 83002204: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002208: 395B0009  addi r10, r27, 9
	ctx.r[10].s64 = ctx.r[27].s64 + 9;
	// 8300220C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83002210: 7D6AF12E  stwx r11, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 83002214: A17E014A  lhz r11, 0x14a(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(330 as u32) ) } as u64;
	// 83002218: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300221C: B17E014A  sth r11, 0x14a(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(330 as u32), ctx.r[11].u16 ) };
	// 83002220: 92510000  stw r18, 0(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 83002224: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002228: 814B00AC  lwz r10, 0xac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8300222C: 654A4000  oris r10, r10, 0x4000
	ctx.r[10].u64 = ctx.r[10].u64 | 1073741824;
	// 83002230: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 83002234: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 83002238: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 8300223C: 419A001C  beq cr6, 0x83002258
	if ctx.cr[6].eq {
	pc = 0x83002258; continue 'dispatch;
	}
	// 83002240: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002244: 7F135840  cmplw cr6, r19, r11
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002248: 41980010  blt cr6, 0x83002258
	if ctx.cr[6].lt {
	pc = 0x83002258; continue 'dispatch;
	}
	// 8300224C: 7E6B9851  subf. r19, r11, r19
	ctx.r[19].s64 = ctx.r[19].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 83002250: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 83002254: 4082FFEC  bne 0x83002240
	if !ctx.cr[0].eq {
	pc = 0x83002240; continue 'dispatch;
	}
	// 83002258: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300225C: 419A0040  beq cr6, 0x8300229c
	if ctx.cr[6].eq {
	pc = 0x8300229C; continue 'dispatch;
	}
	// 83002260: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002264: 7FB35850  subf r29, r19, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[19].s64;
	// 83002268: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300226C: 40990008  ble cr6, 0x83002274
	if !ctx.cr[6].gt {
	pc = 0x83002274; continue 'dispatch;
	}
	// 83002270: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83002274: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002278: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300227C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002280: 7C8B9A14  add r4, r11, r19
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 83002284: 4BCA71FD  bl 0x82ca9480
	ctx.lr = 0x83002288;
	sub_82CA9480(ctx, base);
	// 83002288: 7F9DE051  subf. r28, r29, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300228C: 7F5DD214  add r26, r29, r26
	ctx.r[26].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 83002290: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 83002294: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 83002298: 4082FFC8  bne 0x83002260
	if !ctx.cr[0].eq {
	pc = 0x83002260; continue 'dispatch;
	}
	// 8300229C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 830022A0: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830022A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830022A8: 41980090  blt cr6, 0x83002338
	if ctx.cr[6].lt {
	pc = 0x83002338; continue 'dispatch;
	}
	// 830022AC: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 830022B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830022B4: 40820014  bne 0x830022c8
	if !ctx.cr[0].eq {
	pc = 0x830022C8; continue 'dispatch;
	}
	// 830022B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830022BC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830022C0: 4BFFC521  bl 0x82ffe7e0
	ctx.lr = 0x830022C4;
	sub_82FFE7E0(ctx, base);
	// 830022C4: 48000074  b 0x83002338
	pc = 0x83002338; continue 'dispatch;
	// 830022C8: 556A0031  rlwinm. r10, r11, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830022CC: 41820018  beq 0x830022e4
	if ctx.cr[0].eq {
	pc = 0x830022E4; continue 'dispatch;
	}
	// 830022D0: 895F004B  lbz r10, 0x4b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 830022D4: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830022D8: 614A0080  ori r10, r10, 0x80
	ctx.r[10].u64 = ctx.r[10].u64 | 128;
	// 830022DC: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 830022E0: 995F004B  stb r10, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 830022E4: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 830022E8: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830022EC: 41820018  beq 0x83002304
	if ctx.cr[0].eq {
	pc = 0x83002304; continue 'dispatch;
	}
	// 830022F0: 895F004B  lbz r10, 0x4b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 830022F4: 716B00EF  andi. r11, r11, 0xef
	ctx.r[11].u64 = ctx.r[11].u64 & 239;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830022F8: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 830022FC: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83002300: 995F004B  stb r10, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 83002304: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002308: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300230C: 895F004B  lbz r10, 0x4b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83002310: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 83002314: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002318: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300231C: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83002320: 41820014  beq 0x83002334
	if ctx.cr[0].eq {
	pc = 0x83002334; continue 'dispatch;
	}
	// 83002324: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 83002328: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 8300232C: 4BFFCD4D  bl 0x82fff078
	ctx.lr = 0x83002330;
	sub_82FFF078(ctx, base);
	// 83002330: 48000008  b 0x83002338
	pc = 0x83002338; continue 'dispatch;
	// 83002334: 4BFFC505  bl 0x82ffe838
	ctx.lr = 0x83002338;
	sub_82FFE838(ctx, base);
	// 83002338: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8300233C: 4BCA70F0  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002340 size=20
    let mut pc: u32 = 0x83002340;
    'dispatch: loop {
        match pc {
            0x83002340 => {
    //   block [0x83002340..0x83002354)
	// 83002340: A1630048  lhz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002344: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 83002348: 556A0529  rlwinm. r10, r11, 0, 0x14, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300234C: B1630048  sth r11, 0x48(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83002350: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002354 size=36
    let mut pc: u32 = 0x83002354;
    'dispatch: loop {
        match pc {
            0x83002354 => {
    //   block [0x83002354..0x83002378)
	// 83002354: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002358: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300235C: 816A0238  lwz r11, 0x238(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(568 as u32) ) } as u64;
	// 83002360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83002364: 916A0238  stw r11, 0x238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(568 as u32), ctx.r[11].u32 ) };
	// 83002368: A1630048  lhz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8300236C: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 83002370: B1630048  sth r11, 0x48(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 83002374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002378 size=172
    let mut pc: u32 = 0x83002378;
    'dispatch: loop {
        match pc {
            0x83002378 => {
    //   block [0x83002378..0x83002424)
	// 83002378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300237C: 4BCA7085  bl 0x82ca9400
	ctx.lr = 0x83002380;
	sub_82CA93D0(ctx, base);
	// 83002380: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002388: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8300238C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83002390: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83002394: 550B0631  rlwinm. r11, r8, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002398: 41820030  beq 0x830023c8
	if ctx.cr[0].eq {
	pc = 0x830023C8; continue 'dispatch;
	}
	// 8300239C: 57C51838  slwi r5, r30, 3
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 830023A0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 830023A4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830023A8: 4BCA70D9  bl 0x82ca9480
	ctx.lr = 0x830023AC;
	sub_82CA9480(ctx, base);
	// 830023AC: 397E000B  addi r11, r30, 0xb
	ctx.r[11].s64 = ctx.r[30].s64 + 11;
	// 830023B0: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 830023B4: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 830023B8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830023BC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830023C0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830023C4: 4800004C  b 0x83002410
	pc = 0x83002410; continue 'dispatch;
	// 830023C8: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 830023CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830023D0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830023D4: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830023D8: 419A002C  beq cr6, 0x83002404
	if ctx.cr[6].eq {
	pc = 0x83002404; continue 'dispatch;
	}
	// 830023DC: 3BA60004  addi r29, r6, 4
	ctx.r[29].s64 = ctx.r[6].s64 + 4;
	// 830023E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830023E4: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830023E8: 809DFFFC  lwz r4, -4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830023EC: 4BCA7095  bl 0x82ca9480
	ctx.lr = 0x830023F0;
	sub_82CA9480(ctx, base);
	// 830023F0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830023F4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830023F8: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 830023FC: 7F9C5A14  add r28, r28, r11
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 83002400: 4082FFE0  bne 0x830023e0
	if !ctx.cr[0].eq {
	pc = 0x830023E0; continue 'dispatch;
	}
	// 83002404: 7D7BD214  add r11, r27, r26
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 83002408: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8300240C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83002410: 7D7BD214  add r11, r27, r26
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 83002414: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83002418: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8300241C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83002420: 4BCA7030  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002428 size=444
    let mut pc: u32 = 0x83002428;
    'dispatch: loop {
        match pc {
            0x83002428 => {
    //   block [0x83002428..0x830025E4)
	// 83002428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300242C: 4BCA6FC5  bl 0x82ca93f0
	ctx.lr = 0x83002430;
	sub_82CA93D0(ctx, base);
	// 83002430: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002434: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83002438: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8300243C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83002440: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83002444: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 83002448: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8300244C: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 83002450: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83002454: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002458: 419A0020  beq cr6, 0x83002478
	if ctx.cr[6].eq {
	pc = 0x83002478; continue 'dispatch;
	}
	// 8300245C: 39580004  addi r10, r24, 4
	ctx.r[10].s64 = ctx.r[24].s64 + 4;
	// 83002460: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83002464: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002468: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300246C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83002470: 7F69DA14  add r27, r9, r27
	ctx.r[27].u64 = ctx.r[9].u64 + ctx.r[27].u64;
	// 83002474: 4082FFF0  bne 0x83002464
	if !ctx.cr[0].eq {
	pc = 0x83002464; continue 'dispatch;
	}
	// 83002478: 57970631  rlwinm. r23, r28, 0, 0x18, 0x18
	ctx.r[23].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8300247C: 41820010  beq 0x8300248c
	if ctx.cr[0].eq {
	pc = 0x8300248C; continue 'dispatch;
	}
	// 83002480: 397D000A  addi r11, r29, 0xa
	ctx.r[11].s64 = ctx.r[29].s64 + 10;
	// 83002484: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83002488: 48000008  b 0x83002490
	pc = 0x83002490; continue 'dispatch;
	// 8300248C: 397B0058  addi r11, r27, 0x58
	ctx.r[11].s64 = ctx.r[27].s64 + 88;
	// 83002490: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83002494: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83002498: 409A0010  bne cr6, 0x830024a8
	if !ctx.cr[6].eq {
	pc = 0x830024A8; continue 'dispatch;
	}
	// 8300249C: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 830024A0: 60631009  ori r3, r3, 0x1009
	ctx.r[3].u64 = ctx.r[3].u64 | 4105;
	// 830024A4: 48000138  b 0x830025dc
	pc = 0x830025DC; continue 'dispatch;
	// 830024A8: 7F1B3840  cmplw cr6, r27, r7
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[7].u32, &mut ctx.xer);
	// 830024AC: 40990010  ble cr6, 0x830024bc
	if !ctx.cr[6].gt {
	pc = 0x830024BC; continue 'dispatch;
	}
	// 830024B0: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 830024B4: 60631006  ori r3, r3, 0x1006
	ctx.r[3].u64 = ctx.r[3].u64 | 4102;
	// 830024B8: 48000124  b 0x830025dc
	pc = 0x830025DC; continue 'dispatch;
	// 830024BC: 2B1B04C4  cmplwi cr6, r27, 0x4c4
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1220 as u32, &mut ctx.xer);
	// 830024C0: 4099001C  ble cr6, 0x830024dc
	if !ctx.cr[6].gt {
	pc = 0x830024DC; continue 'dispatch;
	}
	// 830024C4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 830024C8: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830024CC: 419A0008  beq cr6, 0x830024d4
	if ctx.cr[6].eq {
	pc = 0x830024D4; continue 'dispatch;
	}
	// 830024D0: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 830024D4: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 830024D8: 48000008  b 0x830024e0
	pc = 0x830024E0; continue 'dispatch;
	// 830024DC: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 830024E0: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 830024E4: 4BFF9D35  bl 0x82ffc218
	ctx.lr = 0x830024E8;
	sub_82FFC218(ctx, base);
	// 830024E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830024EC: 40820010  bne 0x830024fc
	if !ctx.cr[0].eq {
	pc = 0x830024FC; continue 'dispatch;
	}
	// 830024F0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830024F4: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830024F8: 480000E4  b 0x830025dc
	pc = 0x830025DC; continue 'dispatch;
	// 830024FC: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 83002500: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83002504: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83002508: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300250C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83002510: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83002514: 4BFFF765  bl 0x83001c78
	ctx.lr = 0x83002518;
	sub_83001C78(ctx, base);
	// 83002518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300251C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 83002520: 409A002C  bne cr6, 0x8300254c
	if !ctx.cr[6].eq {
	pc = 0x8300254C; continue 'dispatch;
	}
	// 83002524: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83002528: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8300252C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83002530: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83002534: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83002538: 4BFFFE41  bl 0x83002378
	ctx.lr = 0x8300253C;
	sub_83002378(ctx, base);
	// 8300253C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83002540: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83002544: 4BFFF3C5  bl 0x83001908
	ctx.lr = 0x83002548;
	sub_83001908(ctx, base);
	// 83002548: 48000070  b 0x830025b8
	pc = 0x830025B8; continue 'dispatch;
	// 8300254C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83002550: 419A001C  beq cr6, 0x8300256c
	if ctx.cr[6].eq {
	pc = 0x8300256C; continue 'dispatch;
	}
	// 83002554: 57A51838  slwi r5, r29, 3
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83002558: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8300255C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83002560: 4BCA6F21  bl 0x82ca9480
	ctx.lr = 0x83002564;
	sub_82CA9480(ctx, base);
	// 83002564: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83002568: 4800004C  b 0x830025b4
	pc = 0x830025B4; continue 'dispatch;
	// 8300256C: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 83002570: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 83002574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83002578: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300257C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83002580: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83002584: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002588: 419A002C  beq cr6, 0x830025b4
	if ctx.cr[6].eq {
	pc = 0x830025B4; continue 'dispatch;
	}
	// 8300258C: 3BD80004  addi r30, r24, 4
	ctx.r[30].s64 = ctx.r[24].s64 + 4;
	// 83002590: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83002594: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002598: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8300259C: 4BCA6EE5  bl 0x82ca9480
	ctx.lr = 0x830025A0;
	sub_82CA9480(ctx, base);
	// 830025A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830025A4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830025A8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 830025AC: 7F9C5A14  add r28, r28, r11
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 830025B0: 4082FFE0  bne 0x83002590
	if !ctx.cr[0].eq {
	pc = 0x83002590; continue 'dispatch;
	}
	// 830025B4: 937F003C  stw r27, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[27].u32 ) };
	// 830025B8: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830025BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830025C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830025C4: 5566AFFE  rlwinm r6, r11, 0x15, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 830025C8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830025CC: 4BFFC6ED  bl 0x82ffecb8
	ctx.lr = 0x830025D0;
	sub_82FFECB8(ctx, base);
	// 830025D0: 81610104  lwz r11, 0x104(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 830025D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830025D8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830025DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830025E0: 4BCA6E60  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830025E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830025E8 size=320
    let mut pc: u32 = 0x830025E8;
    'dispatch: loop {
        match pc {
            0x830025E8 => {
    //   block [0x830025E8..0x83002728)
	// 830025E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830025EC: 4BCA6E05  bl 0x82ca93f0
	ctx.lr = 0x830025F0;
	sub_82CA93D0(ctx, base);
	// 830025F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830025F4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830025F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830025FC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83002600: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83002604: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83002608: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8300260C: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 83002610: 615D0001  ori r29, r10, 1
	ctx.r[29].u64 = ctx.r[10].u64 | 1;
	// 83002614: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83002618: 409A0008  bne cr6, 0x83002620
	if !ctx.cr[6].eq {
	pc = 0x83002620; continue 'dispatch;
	}
	// 8300261C: 57BD066E  rlwinm r29, r29, 0, 0x19, 0x17
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 83002620: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83002624: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83002628: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 8300262C: 419A0020  beq cr6, 0x8300264c
	if ctx.cr[6].eq {
	pc = 0x8300264C; continue 'dispatch;
	}
	// 83002630: 395B0004  addi r10, r27, 4
	ctx.r[10].s64 = ctx.r[27].s64 + 4;
	// 83002634: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83002638: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300263C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002640: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83002644: 7FE9FA14  add r31, r9, r31
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 83002648: 4082FFF0  bne 0x83002638
	if !ctx.cr[0].eq {
	pc = 0x83002638; continue 'dispatch;
	}
	// 8300264C: 57AA0631  rlwinm. r10, r29, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83002650: 41820010  beq 0x83002660
	if ctx.cr[0].eq {
	pc = 0x83002660; continue 'dispatch;
	}
	// 83002654: 397E000A  addi r11, r30, 0xa
	ctx.r[11].s64 = ctx.r[30].s64 + 10;
	// 83002658: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300265C: 48000008  b 0x83002664
	pc = 0x83002664; continue 'dispatch;
	// 83002660: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 83002664: 388B0009  addi r4, r11, 9
	ctx.r[4].s64 = ctx.r[11].s64 + 9;
	// 83002668: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300266C: 419A0008  beq cr6, 0x83002674
	if ctx.cr[6].eq {
	pc = 0x83002674; continue 'dispatch;
	}
	// 83002670: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 83002674: 2B1F04C4  cmplwi cr6, r31, 0x4c4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1220 as u32, &mut ctx.xer);
	// 83002678: 40990010  ble cr6, 0x83002688
	if !ctx.cr[6].gt {
	pc = 0x83002688; continue 'dispatch;
	}
	// 8300267C: 3C60807A  lis r3, -0x7f86
	ctx.r[3].s64 = -2139488256;
	// 83002680: 60631006  ori r3, r3, 0x1006
	ctx.r[3].u64 = ctx.r[3].u64 | 4102;
	// 83002684: 4800009C  b 0x83002720
	pc = 0x83002720; continue 'dispatch;
	// 83002688: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 8300268C: 4BFF9B8D  bl 0x82ffc218
	ctx.lr = 0x83002690;
	sub_82FFC218(ctx, base);
	// 83002690: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002694: 40820010  bne 0x830026a4
	if !ctx.cr[0].eq {
	pc = 0x830026A4; continue 'dispatch;
	}
	// 83002698: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8300269C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830026A0: 48000080  b 0x83002720
	pc = 0x83002720; continue 'dispatch;
	// 830026A4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830026A8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 830026AC: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 830026B0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830026B4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830026B8: FB4B0000  std r26, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 830026BC: 934B0008  stw r26, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 830026C0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830026C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830026C8: 4BFFF5B1  bl 0x83001c78
	ctx.lr = 0x830026CC;
	sub_83001C78(ctx, base);
	// 830026CC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 830026D0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830026D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830026D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830026DC: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 830026E0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830026E4: 4BFFFC95  bl 0x83002378
	ctx.lr = 0x830026E8;
	sub_83002378(ctx, base);
	// 830026E8: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 830026EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830026F0: 80790050  lwz r3, 0x50(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(80 as u32) ) } as u64;
	// 830026F4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830026F8: 4BFFF1B9  bl 0x830018b0
	ctx.lr = 0x830026FC;
	sub_830018B0(ctx, base);
	// 830026FC: A1790048  lhz r11, 0x48(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002700: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83002704: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83002708: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300270C: 5566AFFE  rlwinm r6, r11, 0x15, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 83002710: 4BFFC5A9  bl 0x82ffecb8
	ctx.lr = 0x83002714;
	sub_82FFECB8(ctx, base);
	// 83002714: 81610114  lwz r11, 0x114(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 83002718: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300271C: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 83002720: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83002724: 4BCA6D1C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002728 size=248
    let mut pc: u32 = 0x83002728;
    'dispatch: loop {
        match pc {
            0x83002728 => {
    //   block [0x83002728..0x83002820)
	// 83002728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300272C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83002730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83002734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83002738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300273C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83002740: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83002744: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 83002748: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8300274C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 83002750: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002754: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83002758: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300275C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002760: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002764: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83002768: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300276C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002770: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002774: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83002778: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300277C: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 83002780: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83002784: 409A0008  bne cr6, 0x8300278c
	if !ctx.cr[6].eq {
	pc = 0x8300278C; continue 'dispatch;
	}
	// 83002788: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8300278C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83002790: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83002794: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002798: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 8300279C: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 830027A0: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 830027A4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830027A8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830027AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830027B0: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 830027B4: 90DF0020  stw r6, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 830027B8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830027BC: 4BCA6CC5  bl 0x82ca9480
	ctx.lr = 0x830027C0;
	sub_82CA9480(ctx, base);
	// 830027C0: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830027C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830027C8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 830027CC: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830027D0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830027D4: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830027D8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830027DC: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 830027E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830027E4: 419A000C  beq cr6, 0x830027f0
	if ctx.cr[6].eq {
	pc = 0x830027F0; continue 'dispatch;
	}
	// 830027E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830027EC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830027F0: A17E0048  lhz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830027F4: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830027F8: 41820010  beq 0x83002808
	if ctx.cr[0].eq {
	pc = 0x83002808; continue 'dispatch;
	}
	// 830027FC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83002800: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 83002804: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83002808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300280C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83002810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83002814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83002818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300281C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002820 size=132
    let mut pc: u32 = 0x83002820;
    'dispatch: loop {
        match pc {
            0x83002820 => {
    //   block [0x83002820..0x830028A4)
	// 83002820: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 83002824: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83002828: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300282C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83002830: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002834: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002838: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300283C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83002840: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002844: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002848: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300284C: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83002850: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002854: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 83002858: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300285C: 409A0008  bne cr6, 0x83002864
	if !ctx.cr[6].eq {
	pc = 0x83002864; continue 'dispatch;
	}
	// 83002860: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 83002864: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83002868: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300286C: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83002870: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83002874: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83002878: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300287C: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83002880: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83002884: 9164001C  stw r11, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83002888: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8300288C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83002890: 419A0014  beq cr6, 0x830028a4
	if ctx.cr[6].eq {
		sub_830028A4(ctx, base);
		return;
	}
	// 83002894: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 83002898: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300289C: 556BE7BC  rlwinm r11, r11, 0x1c, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830028A0: 48000008  b 0x830028a8
	sub_830028A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830028A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830028A4 size=80
    let mut pc: u32 = 0x830028A4;
    'dispatch: loop {
        match pc {
            0x830028A4 => {
    //   block [0x830028A4..0x830028F4)
	// 830028A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830028A8: 91640020  stw r11, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830028AC: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 830028B0: 91640024  stw r11, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830028B4: 81430038  lwz r10, 0x38(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830028B8: 91440028  stw r10, 0x28(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 830028BC: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 830028C0: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 830028C4: 419A0018  beq cr6, 0x830028dc
	if ctx.cr[6].eq {
	pc = 0x830028DC; continue 'dispatch;
	}
	// 830028C8: 81440028  lwz r10, 0x28(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 830028CC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830028D0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 830028D4: 91640024  stw r11, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830028D8: 91440028  stw r10, 0x28(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 830028DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830028E0: 9164002C  stw r11, 0x2c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830028E4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830028E8: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 830028EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830028F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830028F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830028F4 size=12
    let mut pc: u32 = 0x830028F4;
    'dispatch: loop {
        match pc {
            0x830028F4 => {
    //   block [0x830028F4..0x83002900)
	// 830028F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830028F8: 9164002C  stw r11, 0x2c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830028FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002900 size=256
    let mut pc: u32 = 0x83002900;
    'dispatch: loop {
        match pc {
            0x83002900 => {
    //   block [0x83002900..0x83002A00)
	// 83002900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83002908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300290C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002914: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83002918: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300291C: 4182001C  beq 0x83002938
	if ctx.cr[0].eq {
	pc = 0x83002938; continue 'dispatch;
	}
	// 83002920: 4BFFFF01  bl 0x83002820
	ctx.lr = 0x83002924;
	sub_83002820(ctx, base);
	// 83002924: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83002928: 716B00EF  andi. r11, r11, 0xef
	ctx.r[11].u64 = ctx.r[11].u64 & 239;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300292C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83002930: 997F004B  stb r11, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 83002934: 480000B8  b 0x830029ec
	pc = 0x830029EC; continue 'dispatch;
	// 83002938: 556A0031  rlwinm. r10, r11, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300293C: 41820030  beq 0x8300296c
	if ctx.cr[0].eq {
	pc = 0x8300296C; continue 'dispatch;
	}
	// 83002940: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83002948: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300294C: 40820008  bne 0x83002954
	if !ctx.cr[0].eq {
	pc = 0x83002954; continue 'dispatch;
	}
	// 83002950: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83002954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300295C: 4BFFFDCD  bl 0x83002728
	ctx.lr = 0x83002960;
	sub_83002728(ctx, base);
	// 83002960: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83002964: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 83002968: 4BFFFFC4  b 0x8300292c
	pc = 0x8300292C; continue 'dispatch;
	// 8300296C: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83002970: 41820030  beq 0x830029a0
	if ctx.cr[0].eq {
	pc = 0x830029A0; continue 'dispatch;
	}
	// 83002974: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002978: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300297C: 556B04A5  rlwinm. r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002980: 40820008  bne 0x83002988
	if !ctx.cr[0].eq {
	pc = 0x83002988; continue 'dispatch;
	}
	// 83002984: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83002988: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300298C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002990: 4BFFFD99  bl 0x83002728
	ctx.lr = 0x83002994;
	sub_83002728(ctx, base);
	// 83002994: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83002998: 716B00BF  andi. r11, r11, 0xbf
	ctx.r[11].u64 = ctx.r[11].u64 & 191;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300299C: 4BFFFF90  b 0x8300292c
	pc = 0x8300292C; continue 'dispatch;
	// 830029A0: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830029A4: A17F0048  lhz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830029A8: 4182002C  beq 0x830029d4
	if ctx.cr[0].eq {
	pc = 0x830029D4; continue 'dispatch;
	}
	// 830029AC: 556B04E7  rlwinm. r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830029B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830029B4: 40820008  bne 0x830029bc
	if !ctx.cr[0].eq {
	pc = 0x830029BC; continue 'dispatch;
	}
	// 830029B8: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830029BC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 830029C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830029C4: 4BFFFD65  bl 0x83002728
	ctx.lr = 0x830029C8;
	sub_83002728(ctx, base);
	// 830029C8: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 830029CC: 716B00DF  andi. r11, r11, 0xdf
	ctx.r[11].u64 = ctx.r[11].u64 & 223;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830029D0: 4BFFFF5C  b 0x8300292c
	pc = 0x8300292C; continue 'dispatch;
	// 830029D4: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 830029D8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830029DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830029E0: B17F0048  sth r11, 0x48(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u16 ) };
	// 830029E4: 4BFFBDFD  bl 0x82ffe7e0
	ctx.lr = 0x830029E8;
	sub_82FFE7E0(ctx, base);
	// 830029E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830029EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830029F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830029F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830029F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830029FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002A00 size=20
    let mut pc: u32 = 0x83002A00;
    'dispatch: loop {
        match pc {
            0x83002A00 => {
    //   block [0x83002A00..0x83002A14)
	// 83002A00: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83002A04: 3C804E57  lis r4, 0x4e57
	ctx.r[4].s64 = 1314324480;
	// 83002A08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002A0C: 60844D20  ori r4, r4, 0x4d20
	ctx.r[4].u64 = ctx.r[4].u64 | 19744;
	// 83002A10: 482B7804  b 0x832ba214
	sub_832BA214(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002A18 size=8
    let mut pc: u32 = 0x83002A18;
    'dispatch: loop {
        match pc {
            0x83002A18 => {
    //   block [0x83002A18..0x83002A20)
	// 83002A18: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83002A1C: 482B7808  b 0x832ba224
	sub_832BA224(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002A20 size=28
    let mut pc: u32 = 0x83002A20;
    'dispatch: loop {
        match pc {
            0x83002A20 => {
    //   block [0x83002A20..0x83002A3C)
	// 83002A20: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 83002A24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002A28: 394AB2C8  addi r10, r10, -0x4d38
	ctx.r[10].s64 = ctx.r[10].s64 + -19768;
	// 83002A2C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002A30: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83002A34: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83002A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002A40 size=64
    let mut pc: u32 = 0x83002A40;
    'dispatch: loop {
        match pc {
            0x83002A40 => {
    //   block [0x83002A40..0x83002A80)
	// 83002A40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83002A44: 390B97BC  addi r8, r11, -0x6844
	ctx.r[8].s64 = ctx.r[11].s64 + -26692;
	// 83002A48: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83002A4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83002A50: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83002A54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83002A58: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83002A5C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83002A60: 4082FFE8  bne 0x83002a48
	if !ctx.cr[0].eq {
	pc = 0x83002A48; continue 'dispatch;
	}
	// 83002A64: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83002A68: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002A6C: 7D2B5396  divwu r9, r11, r10
	ctx.r[9].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 83002A70: 0CCA0000  twi 6, r10, 0
	// 83002A74: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83002A78: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83002A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002A80 size=356
    let mut pc: u32 = 0x83002A80;
    'dispatch: loop {
        match pc {
            0x83002A80 => {
    //   block [0x83002A80..0x83002BE4)
	// 83002A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002A84: 4BCA6981  bl 0x82ca9404
	ctx.lr = 0x83002A88;
	sub_82CA93D0(ctx, base);
	// 83002A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002A8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002A90: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 83002A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002A98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83002A9C: 637B4005  ori r27, r27, 0x4005
	ctx.r[27].u64 = ctx.r[27].u64 | 16389;
	// 83002AA0: 409A0008  bne cr6, 0x83002aa8
	if !ctx.cr[6].eq {
	pc = 0x83002AA8; continue 'dispatch;
	}
	// 83002AA4: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 83002AA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002AAC: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002AB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002AB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002ABC: 4E800421  bctrl
	ctx.lr = 0x83002AC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002AC0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002AC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002AC8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83002ACC: 41820090  beq 0x83002b5c
	if ctx.cr[0].eq {
	pc = 0x83002B5C; continue 'dispatch;
	}
	// 83002AD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83002AD4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83002AD8: 419A0100  beq cr6, 0x83002bd8
	if ctx.cr[6].eq {
	pc = 0x83002BD8; continue 'dispatch;
	}
	// 83002ADC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002AE0: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 83002AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002AE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002AF0: 4E800421  bctrl
	ctx.lr = 0x83002AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002AF4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002AF8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002AFC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83002B00: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 83002B04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002B08: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002B0C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83002B10: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83002B14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83002B18: 419A0044  beq cr6, 0x83002b5c
	if ctx.cr[6].eq {
	pc = 0x83002B5C; continue 'dispatch;
	}
	// 83002B1C: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83002B20: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 83002B24: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002B28: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002B2C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83002B30: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83002B34: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83002B38: 482B714D  bl 0x832b9c84
	ctx.lr = 0x83002B3C;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83002B3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002B40: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 83002B44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83002B48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83002B4C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83002B50: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83002B54: 4198FF88  blt cr6, 0x83002adc
	if ctx.cr[6].lt {
	pc = 0x83002ADC; continue 'dispatch;
	}
	// 83002B58: 48000080  b 0x83002bd8
	pc = 0x83002BD8; continue 'dispatch;
	// 83002B5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002B60: 3F608007  lis r27, -0x7ff9
	ctx.r[27].s64 = -2147024896;
	// 83002B64: 637B000E  ori r27, r27, 0xe
	ctx.r[27].u64 = ctx.r[27].u64 | 14;
	// 83002B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002B6C: 419A006C  beq cr6, 0x83002bd8
	if ctx.cr[6].eq {
	pc = 0x83002BD8; continue 'dispatch;
	}
	// 83002B70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002B74: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83002B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002B7C: 40990040  ble cr6, 0x83002bbc
	if !ctx.cr[6].gt {
	pc = 0x83002BBC; continue 'dispatch;
	}
	// 83002B80: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83002B84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002B8C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002B90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002B94: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83002B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002B9C: 4E800421  bctrl
	ctx.lr = 0x83002BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002BA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002BA4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83002BA8: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 83002BAC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83002BB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002BB4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002BB8: 4198FFCC  blt cr6, 0x83002b84
	if ctx.cr[6].lt {
	pc = 0x83002B84; continue 'dispatch;
	}
	// 83002BBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002BC4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002BC8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002BD0: 4E800421  bctrl
	ctx.lr = 0x83002BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002BD4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83002BD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83002BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83002BE0: 4BCA6874  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002BE8 size=148
    let mut pc: u32 = 0x83002BE8;
    'dispatch: loop {
        match pc {
            0x83002BE8 => {
    //   block [0x83002BE8..0x83002C7C)
	// 83002BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002BEC: 4BCA681D  bl 0x82ca9408
	ctx.lr = 0x83002BF0;
	sub_82CA93D0(ctx, base);
	// 83002BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002BF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002C00: 419A0074  beq cr6, 0x83002c74
	if ctx.cr[6].eq {
	pc = 0x83002C74; continue 'dispatch;
	}
	// 83002C04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002C08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002C0C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83002C10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002C14: 40990040  ble cr6, 0x83002c54
	if !ctx.cr[6].gt {
	pc = 0x83002C54; continue 'dispatch;
	}
	// 83002C18: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83002C1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002C24: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C2C: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83002C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002C34: 4E800421  bctrl
	ctx.lr = 0x83002C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002C38: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C3C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83002C40: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 83002C44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83002C48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002C4C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002C50: 4198FFCC  blt cr6, 0x83002c1c
	if ctx.cr[6].lt {
	pc = 0x83002C1C; continue 'dispatch;
	}
	// 83002C54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002C5C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002C68: 4E800421  bctrl
	ctx.lr = 0x83002C6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002C6C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83002C70: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83002C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83002C78: 4BCA67E0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002C80 size=152
    let mut pc: u32 = 0x83002C80;
    'dispatch: loop {
        match pc {
            0x83002C80 => {
    //   block [0x83002C80..0x83002D18)
	// 83002C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002C84: 4BCA6779  bl 0x82ca93fc
	ctx.lr = 0x83002C88;
	sub_82CA93D0(ctx, base);
	// 83002C88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002C8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83002C90: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83002C94: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83002C98: 4B263419  bl 0x822660b0
	ctx.lr = 0x83002C9C;
	sub_822660B0(ctx, base);
	// 83002C9C: 7F23DA14  add r25, r3, r27
	ctx.r[25].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	// 83002CA0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002CA4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002CA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83002CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002CB0: 40990030  ble cr6, 0x83002ce0
	if !ctx.cr[6].gt {
	pc = 0x83002CE0; continue 'dispatch;
	}
	// 83002CB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83002CB8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002CBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83002CC0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83002CC4: 480001CD  bl 0x83002e90
	ctx.lr = 0x83002CC8;
	sub_83002E90(ctx, base);
	// 83002CC8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002CCC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83002CD0: 7C7CE378  or r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[28].u64;
	// 83002CD4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83002CD8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002CDC: 4198FFDC  blt cr6, 0x83002cb8
	if ctx.cr[6].lt {
	pc = 0x83002CB8; continue 'dispatch;
	}
	// 83002CE0: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 83002CE4: 419A0010  beq cr6, 0x83002cf4
	if ctx.cr[6].eq {
	pc = 0x83002CF4; continue 'dispatch;
	}
	// 83002CE8: 4B2633C9  bl 0x822660b0
	ctx.lr = 0x83002CEC;
	sub_822660B0(ctx, base);
	// 83002CEC: 7D63C851  subf. r11, r3, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002CF0: 4081001C  ble 0x83002d0c
	if !ctx.cr[0].gt {
	pc = 0x83002D0C; continue 'dispatch;
	}
	// 83002CF4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83002CF8: 419A000C  beq cr6, 0x83002d04
	if ctx.cr[6].eq {
	pc = 0x83002D04; continue 'dispatch;
	}
	// 83002CFC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83002D00: 4BFFFFA0  b 0x83002ca0
	pc = 0x83002CA0; continue 'dispatch;
	// 83002D04: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83002D08: 419AFF98  beq cr6, 0x83002ca0
	if ctx.cr[6].eq {
	pc = 0x83002CA0; continue 'dispatch;
	}
	// 83002D0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83002D10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83002D14: 4BCA6738  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002D18 size=16
    let mut pc: u32 = 0x83002D18;
    'dispatch: loop {
        match pc {
            0x83002D18 => {
    //   block [0x83002D18..0x83002D28)
	// 83002D18: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83002D1C: 396BB2C8  addi r11, r11, -0x4d38
	ctx.r[11].s64 = ctx.r[11].s64 + -19768;
	// 83002D20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002D24: 4BFFFEC4  b 0x83002be8
	sub_83002BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002D28 size=220
    let mut pc: u32 = 0x83002D28;
    'dispatch: loop {
        match pc {
            0x83002D28 => {
    //   block [0x83002D28..0x83002E04)
	// 83002D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002D2C: 4BCA66E1  bl 0x82ca940c
	ctx.lr = 0x83002D30;
	sub_82CA93D0(ctx, base);
	// 83002D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002D34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83002D38: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83002D3C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 83002D40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002D44: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002D48: 409A0010  bne cr6, 0x83002d58
	if !ctx.cr[6].eq {
	pc = 0x83002D58; continue 'dispatch;
	}
	// 83002D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002D50: 4BFFFCF1  bl 0x83002a40
	ctx.lr = 0x83002D54;
	sub_83002A40(ctx, base);
	// 83002D54: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83002D58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002D5C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83002D60: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83002D64: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83002D68: 4B263349  bl 0x822660b0
	ctx.lr = 0x83002D6C;
	sub_822660B0(ctx, base);
	// 83002D6C: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83002D70: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 83002D74: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83002D78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002D7C: 482B6BE9  bl 0x832b9964
	ctx.lr = 0x83002D80;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83002D80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002D84: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83002D88: 419A0030  beq cr6, 0x83002db8
	if ctx.cr[6].eq {
	pc = 0x83002DB8; continue 'dispatch;
	}
	// 83002D8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002D90: 41820028  beq 0x83002db8
	if ctx.cr[0].eq {
	pc = 0x83002DB8; continue 'dispatch;
	}
	// 83002D94: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002D98: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83002D9C: 7D2A4851  subf. r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83002DA0: 41810044  bgt 0x83002de4
	if ctx.cr[0].gt {
	pc = 0x83002DE4; continue 'dispatch;
	}
	// 83002DA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002DA8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83002DAC: 419A000C  beq cr6, 0x83002db8
	if ctx.cr[6].eq {
	pc = 0x83002DB8; continue 'dispatch;
	}
	// 83002DB0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002DB4: 4082FFE4  bne 0x83002d98
	if !ctx.cr[0].eq {
	pc = 0x83002D98; continue 'dispatch;
	}
	// 83002DB8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83002DBC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 83002DC0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002DC4: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83002DC8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002DCC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002DD0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002DD8: 482B6B7D  bl 0x832b9954
	ctx.lr = 0x83002DDC;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83002DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83002DE0: 4BCA667C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 83002DE4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002DE8: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 83002DEC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002DF0: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83002DF4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002DF8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83002DFC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83002E00: 4BFFFFD4  b 0x83002dd4
	pc = 0x83002DD4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002E08 size=136
    let mut pc: u32 = 0x83002E08;
    'dispatch: loop {
        match pc {
            0x83002E08 => {
    //   block [0x83002E08..0x83002E90)
	// 83002E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002E0C: 4BCA6601  bl 0x82ca940c
	ctx.lr = 0x83002E10;
	sub_82CA93D0(ctx, base);
	// 83002E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002E14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002E18: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002E1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83002E20: 419A005C  beq cr6, 0x83002e7c
	if ctx.cr[6].eq {
	pc = 0x83002E7C; continue 'dispatch;
	}
	// 83002E24: 3BAB0010  addi r29, r11, 0x10
	ctx.r[29].s64 = ctx.r[11].s64 + 16;
	// 83002E28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002E2C: 482B6B39  bl 0x832b9964
	ctx.lr = 0x83002E30;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83002E30: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002E34: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 83002E38: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002E3C: 419A002C  beq cr6, 0x83002e68
	if ctx.cr[6].eq {
	pc = 0x83002E68; continue 'dispatch;
	}
	// 83002E40: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002E44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83002E48: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83002E4C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002E50: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002E54: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83002E58: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002E5C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002E60: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83002E64: 4800000C  b 0x83002e70
	pc = 0x83002E70; continue 'dispatch;
	// 83002E68: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 83002E6C: 63DE4005  ori r30, r30, 0x4005
	ctx.r[30].u64 = ctx.r[30].u64 | 16389;
	// 83002E70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002E74: 482B6AE1  bl 0x832b9954
	ctx.lr = 0x83002E78;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83002E78: 4800000C  b 0x83002e84
	pc = 0x83002E84; continue 'dispatch;
	// 83002E7C: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 83002E80: 63DE4005  ori r30, r30, 0x4005
	ctx.r[30].u64 = ctx.r[30].u64 | 16389;
	// 83002E84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83002E8C: 4BCA65D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002E90 size=508
    let mut pc: u32 = 0x83002E90;
    'dispatch: loop {
        match pc {
            0x83002E90 => {
    //   block [0x83002E90..0x8300308C)
	// 83002E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002E94: 4BCA6575  bl 0x82ca9408
	ctx.lr = 0x83002E98;
	sub_82CA93D0(ctx, base);
	// 83002E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002E9C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83002EA0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83002EA4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83002EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002EAC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83002EB0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002EB4: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 83002EB8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83002EBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002EC0: 482B6AA5  bl 0x832b9964
	ctx.lr = 0x83002EC4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83002EC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002EC8: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 83002ECC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83002ED0: 419A0070  beq cr6, 0x83002f40
	if ctx.cr[6].eq {
	pc = 0x83002F40; continue 'dispatch;
	}
	// 83002ED4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002ED8: 41820068  beq 0x83002f40
	if ctx.cr[0].eq {
	pc = 0x83002F40; continue 'dispatch;
	}
	// 83002EDC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002EE0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83002EE4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83002EE8: 419A0008  beq cr6, 0x83002ef0
	if ctx.cr[6].eq {
	pc = 0x83002EF0; continue 'dispatch;
	}
	// 83002EEC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83002EF0: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002EF4: 2B070103  cmplwi cr6, r7, 0x103
	ctx.cr[6].compare_u32(ctx.r[7].u32, 259 as u32, &mut ctx.xer);
	// 83002EF8: 419A003C  beq cr6, 0x83002f34
	if ctx.cr[6].eq {
	pc = 0x83002F34; continue 'dispatch;
	}
	// 83002EFC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002F00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83002F04: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83002F08: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002F0C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F10: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83002F14: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83002F18: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83002F1C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83002F20: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83002F24: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83002F28: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002F2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83002F30: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83002F34: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83002F38: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83002F3C: 409AFFA0  bne cr6, 0x83002edc
	if !ctx.cr[6].eq {
	pc = 0x83002EDC; continue 'dispatch;
	}
	// 83002F40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002F44: 482B6A11  bl 0x832b9954
	ctx.lr = 0x83002F48;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83002F48: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83002F4C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83002F50: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83002F54: 419A0048  beq cr6, 0x83002f9c
	if ctx.cr[6].eq {
	pc = 0x83002F9C; continue 'dispatch;
	}
	// 83002F58: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83002F5C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F60: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83002F64: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F68: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83002F6C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F70: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F74: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83002F78: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002F7C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83002F80: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83002F84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002F8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002F90: 4E800421  bctrl
	ctx.lr = 0x83002F94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002F94: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83002F98: 4BFFFFB0  b 0x83002f48
	pc = 0x83002F48; continue 'dispatch;
	// 83002F9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002FA0: 482B69C5  bl 0x832b9964
	ctx.lr = 0x83002FA4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83002FA4: 4B26310D  bl 0x822660b0
	ctx.lr = 0x83002FA8;
	sub_822660B0(ctx, base);
	// 83002FA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002FAC: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83002FB0: 419A0074  beq cr6, 0x83003024
	if ctx.cr[6].eq {
	pc = 0x83003024; continue 'dispatch;
	}
	// 83002FB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002FB8: 4182006C  beq 0x83003024
	if ctx.cr[0].eq {
	pc = 0x83003024; continue 'dispatch;
	}
	// 83002FBC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83002FC0: 390BFFFC  addi r8, r11, -4
	ctx.r[8].s64 = ctx.r[11].s64 + -4;
	// 83002FC4: 7D435051  subf. r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83002FC8: 4181005C  bgt 0x83003024
	if ctx.cr[0].gt {
	pc = 0x83003024; continue 'dispatch;
	}
	// 83002FCC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002FD0: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 83002FD4: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83002FD8: 419A0008  beq cr6, 0x83002fe0
	if ctx.cr[6].eq {
	pc = 0x83002FE0; continue 'dispatch;
	}
	// 83002FDC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83002FE0: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002FE4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83002FE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83002FEC: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83002FF0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002FF4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002FF8: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83002FFC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83003000: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83003004: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003008: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8300300C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003010: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003014: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83003018: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8300301C: 9388000C  stw r28, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 83003020: 409AFF9C  bne cr6, 0x83002fbc
	if !ctx.cr[6].eq {
	pc = 0x83002FBC; continue 'dispatch;
	}
	// 83003024: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003028: 482B692D  bl 0x832b9954
	ctx.lr = 0x8300302C;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8300302C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83003030: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83003034: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003038: 419A0048  beq cr6, 0x83003080
	if ctx.cr[6].eq {
	pc = 0x83003080; continue 'dispatch;
	}
	// 8300303C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83003040: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003044: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83003048: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300304C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83003050: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003054: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003058: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8300305C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003060: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003064: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83003068: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300306C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003070: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003074: 4E800421  bctrl
	ctx.lr = 0x83003078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003078: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8300307C: 4BFFFFB0  b 0x8300302c
	pc = 0x8300302C; continue 'dispatch;
	// 83003080: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83003088: 4BCA63D0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003090 size=192
    let mut pc: u32 = 0x83003090;
    'dispatch: loop {
        match pc {
            0x83003090 => {
    //   block [0x83003090..0x83003150)
	// 83003090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003098: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300309C: 906100B4  stw r3, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 830030A0: 908100BC  stw r4, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 830030A4: 90A100C4  stw r5, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[5].u32 ) };
	// 830030A8: 90C100CC  stw r6, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[6].u32 ) };
	// 830030AC: 90E100D4  stw r7, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[7].u32 ) };
	// 830030B0: 910100DC  stw r8, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[8].u32 ) };
	// 830030B4: 816100BC  lwz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 830030B8: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 830030BC: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 830030C0: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 830030C4: 816100CC  lwz r11, 0xcc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 830030C8: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 830030CC: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 830030D0: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830030D4: 816100DC  lwz r11, 0xdc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 830030D8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830030DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830030E0: 99610077  stb r11, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[11].u8 ) };
	// 830030E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830030E8: 9961006F  stb r11, 0x6f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(111 as u32), ctx.r[11].u8 ) };
	// 830030EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830030F0: 99610067  stb r11, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[11].u8 ) };
	// 830030F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830030F8: 396B97C8  addi r11, r11, -0x6838
	ctx.r[11].s64 = ctx.r[11].s64 + -26680;
	// 830030FC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83003100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003104: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83003108: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8300310C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 83003110: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83003114: 390B97C0  addi r8, r11, -0x6840
	ctx.r[8].s64 = ctx.r[11].s64 + -26688;
	// 83003118: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 8300311C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83003120: 38CBB3FC  addi r6, r11, -0x4c04
	ctx.r[6].s64 = ctx.r[11].s64 + -19460;
	// 83003124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83003128: 38ABB3D4  addi r5, r11, -0x4c2c
	ctx.r[5].s64 = ctx.r[11].s64 + -19500;
	// 8300312C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003130: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 83003134: 4800011D  bl 0x83003250
	ctx.lr = 0x83003138;
	sub_83003250(ctx, base);
	// 83003138: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 8300313C: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003140: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83003144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003150 size=68
    let mut pc: u32 = 0x83003150;
    'dispatch: loop {
        match pc {
            0x83003150 => {
    //   block [0x83003150..0x83003194)
	// 83003150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300315C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83003160: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83003164: 388BC174  addi r4, r11, -0x3e8c
	ctx.r[4].s64 = ctx.r[11].s64 + -16012;
	// 83003168: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300316C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003170: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003174: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300317C: 4E800421  bctrl
	ctx.lr = 0x83003180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003180: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83003188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300318C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003198 size=176
    let mut pc: u32 = 0x83003198;
    'dispatch: loop {
        match pc {
            0x83003198 => {
    //   block [0x83003198..0x83003248)
	// 83003198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300319C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830031A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830031A4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830031A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830031AC: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 830031B0: 48000231  bl 0x830033e0
	ctx.lr = 0x830031B4;
	sub_830033E0(ctx, base);
	// 830031B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830031B8: 409A0010  bne cr6, 0x830031c8
	if !ctx.cr[6].eq {
	pc = 0x830031C8; continue 'dispatch;
	}
	// 830031BC: 3D608050  lis r11, -0x7fb0
	ctx.r[11].s64 = -2142240768;
	// 830031C0: 6163004D  ori r3, r11, 0x4d
	ctx.r[3].u64 = ctx.r[11].u64 | 77;
	// 830031C4: 48000074  b 0x83003238
	pc = 0x83003238; continue 'dispatch;
	// 830031C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830031CC: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 830031D0: 48000211  bl 0x830033e0
	ctx.lr = 0x830031D4;
	sub_830033E0(ctx, base);
	// 830031D4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830031D8: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830031DC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830031E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830031E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830031E8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830031EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830031F0: 4E800421  bctrl
	ctx.lr = 0x830031F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830031F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830031F8: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 830031FC: 480001E5  bl 0x830033e0
	ctx.lr = 0x83003200;
	sub_830033E0(ctx, base);
	// 83003200: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83003204: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003208: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300320C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003210: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 83003214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003218: 4E800421  bctrl
	ctx.lr = 0x8300321C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300321C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83003220: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003224: 409A0010  bne cr6, 0x83003234
	if !ctx.cr[6].eq {
	pc = 0x83003234; continue 'dispatch;
	}
	// 83003228: 3D608050  lis r11, -0x7fb0
	ctx.r[11].s64 = -2142240768;
	// 8300322C: 61630008  ori r3, r11, 8
	ctx.r[3].u64 = ctx.r[11].u64 | 8;
	// 83003230: 48000008  b 0x83003238
	pc = 0x83003238; continue 'dispatch;
	// 83003234: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83003238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300323C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003248 size=8
    let mut pc: u32 = 0x83003248;
    'dispatch: loop {
        match pc {
            0x83003248 => {
    //   block [0x83003248..0x83003250)
	// 83003248: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 8300324C: 8204C190  lwz r16, -0x3e70(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-15984 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003250 size=360
    let mut pc: u32 = 0x83003250;
    'dispatch: loop {
        match pc {
            0x83003250 => {
    //   block [0x83003250..0x830033B8)
	// 83003250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300325C: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 83003260: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003264: 907F00E4  stw r3, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[3].u32 ) };
	// 83003268: 909F00EC  stw r4, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 8300326C: 90BF00F4  stw r5, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[5].u32 ) };
	// 83003270: 90DF00FC  stw r6, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[6].u32 ) };
	// 83003274: 90FF0104  stw r7, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[7].u32 ) };
	// 83003278: 911F010C  stw r8, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[8].u32 ) };
	// 8300327C: 913F0114  stw r9, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[9].u32 ) };
	// 83003280: 915F011C  stw r10, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[10].u32 ) };
	// 83003284: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83003288: 396BF240  addi r11, r11, -0xdc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3520;
	// 8300328C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003290: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83003294: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003298: 4BFFFF01  bl 0x83003198
	ctx.lr = 0x8300329C;
	sub_83003198(ctx, base);
	// 8300329C: 907F00A8  stw r3, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[3].u32 ) };
	// 830032A0: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830032A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830032A8: 40980014  bge cr6, 0x830032bc
	if !ctx.cr[6].lt {
	pc = 0x830032BC; continue 'dispatch;
	}
	// 830032AC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830032B0: 48006961  bl 0x83009c10
	ctx.lr = 0x830032B4;
	sub_83009C10(ctx, base);
	// 830032B4: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830032B8: 480000EC  b 0x830033a4
	pc = 0x830033A4; continue 'dispatch;
	// 830032BC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830032C0: 48001E31  bl 0x830050f0
	ctx.lr = 0x830032C4;
	sub_830050F0(ctx, base);
	// 830032C4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830032C8: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 830032CC: 48000115  bl 0x830033e0
	ctx.lr = 0x830032D0;
	sub_830033E0(ctx, base);
	// 830032D0: 907F00B8  stw r3, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[3].u32 ) };
	// 830032D4: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 830032D8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830032DC: 897F0147  lbz r11, 0x147(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(327 as u32) ) } as u64;
	// 830032E0: 99610077  stb r11, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[11].u8 ) };
	// 830032E4: 897F013F  lbz r11, 0x13f(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(319 as u32) ) } as u64;
	// 830032E8: 9961006F  stb r11, 0x6f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(111 as u32), ctx.r[11].u8 ) };
	// 830032EC: 897F0137  lbz r11, 0x137(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(311 as u32) ) } as u64;
	// 830032F0: 99610067  stb r11, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[11].u8 ) };
	// 830032F4: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 830032F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830032FC: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 83003300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83003304: 815F011C  lwz r10, 0x11c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 83003308: 813F0114  lwz r9, 0x114(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8300330C: 811F010C  lwz r8, 0x10c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 83003310: 80FF0104  lwz r7, 0x104(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83003314: 80DF00FC  lwz r6, 0xfc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 83003318: 80BF00F4  lwz r5, 0xf4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300331C: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83003320: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 83003324: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 83003328: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300332C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003334: 4E800421  bctrl
	ctx.lr = 0x83003338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003338: 907F00A8  stw r3, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[3].u32 ) };
	// 8300333C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 83003340: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003344: 40980024  bge cr6, 0x83003368
	if !ctx.cr[6].lt {
	pc = 0x83003368; continue 'dispatch;
	}
	// 83003348: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8300334C: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83003350: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83003354: 48001E1D  bl 0x83005170
	ctx.lr = 0x83003358;
	sub_83005170(ctx, base);
	// 83003358: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8300335C: 480068B5  bl 0x83009c10
	ctx.lr = 0x83003360;
	sub_83009C10(ctx, base);
	// 83003360: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83003364: 48000040  b 0x830033a4
	pc = 0x830033A4; continue 'dispatch;
	// 83003368: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8300336C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003370: 419A0018  beq cr6, 0x83003388
	if ctx.cr[6].eq {
	pc = 0x83003388; continue 'dispatch;
	}
	// 83003374: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83003378: 48000069  bl 0x830033e0
	ctx.lr = 0x8300337C;
	sub_830033E0(ctx, base);
	// 8300337C: 4BFFFDD5  bl 0x83003150
	ctx.lr = 0x83003380;
	sub_83003150(ctx, base);
	// 83003380: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 83003384: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83003388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300338C: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83003390: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83003394: 48001DDD  bl 0x83005170
	ctx.lr = 0x83003398;
	sub_83005170(ctx, base);
	// 83003398: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8300339C: 48006875  bl 0x83009c10
	ctx.lr = 0x830033A0;
	sub_83009C10(ctx, base);
	// 830033A0: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830033A4: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830033A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830033AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830033B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830033B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830033B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830033B8 size=40
    let mut pc: u32 = 0x830033B8;
    'dispatch: loop {
        match pc {
            0x830033B8 => {
    //   block [0x830033B8..0x830033E0)
	// 830033B8: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830033BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830033C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830033C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830033C8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830033CC: 48001DA5  bl 0x83005170
	ctx.lr = 0x830033D0;
	sub_83005170(ctx, base);
	// 830033D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830033D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830033D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830033DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830033E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830033E0 size=16
    let mut pc: u32 = 0x830033E0;
    'dispatch: loop {
        match pc {
            0x830033E0 => {
    //   block [0x830033E0..0x830033F0)
	// 830033E0: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 830033E4: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830033E8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830033EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830033F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830033F0 size=144
    let mut pc: u32 = 0x830033F0;
    'dispatch: loop {
        match pc {
            0x830033F0 => {
    //   block [0x830033F0..0x83003480)
	// 830033F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830033F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830033F8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 830033FC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 83003400: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 83003404: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 83003408: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8300340C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003410: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83003414: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83003418: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 8300341C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003420: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83003424: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 83003428: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300342C: 40990014  ble cr6, 0x83003440
	if !ctx.cr[6].gt {
	pc = 0x83003440; continue 'dispatch;
	}
	// 83003430: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 83003434: 616B0057  ori r11, r11, 0x57
	ctx.r[11].u64 = ctx.r[11].u64 | 87;
	// 83003438: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300343C: 48000030  b 0x8300346c
	pc = 0x8300346C; continue 'dispatch;
	// 83003440: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 83003444: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 83003448: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300344C: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83003450: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003454: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003458: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300345C: 4BD14E75  bl 0x82d182d0
	ctx.lr = 0x83003460;
	sub_82D182D0(ctx, base);
	// 83003460: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83003464: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8300346C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83003470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83003474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003480 size=48
    let mut pc: u32 = 0x83003480;
    'dispatch: loop {
        match pc {
            0x83003480 => {
    //   block [0x83003480..0x830034B0)
	// 83003480: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83003484: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83003488: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 8300348C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003490: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 83003494: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 83003498: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300349C: 409A0014  bne cr6, 0x830034b0
	if !ctx.cr[6].eq {
		sub_830034B0(ctx, base);
		return;
	}
	// 830034A0: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 830034A4: 616B0057  ori r11, r11, 0x57
	ctx.r[11].u64 = ctx.r[11].u64 | 87;
	// 830034A8: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 830034AC: 48000088  b 0x83003534
	sub_83003504(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830034B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830034B0 size=84
    let mut pc: u32 = 0x830034B0;
    'dispatch: loop {
        match pc {
            0x830034B0 => {
    //   block [0x830034B0..0x83003504)
	// 830034B0: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 830034B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830034B8: 419A004C  beq cr6, 0x83003504
	if ctx.cr[6].eq {
		sub_83003504(ctx, base);
		return;
	}
	// 830034BC: 81610024  lwz r11, 0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 830034C0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830034C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830034C8: 419A003C  beq cr6, 0x83003504
	if ctx.cr[6].eq {
		sub_83003504(ctx, base);
		return;
	}
	// 830034CC: 81610024  lwz r11, 0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 830034D0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830034D4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830034D8: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830034DC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830034E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830034E4: 91610014  stw r11, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830034E8: 81610024  lwz r11, 0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 830034EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830034F0: 91610024  stw r11, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830034F4: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 830034F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830034FC: 9161001C  stw r11, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83003500: 4BFFFFB0  b 0x830034b0
	pc = 0x830034B0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003504 size=56
    let mut pc: u32 = 0x83003504;
    'dispatch: loop {
        match pc {
            0x83003504 => {
    //   block [0x83003504..0x8300353C)
	// 83003504: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 83003508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300350C: 409A001C  bne cr6, 0x83003528
	if !ctx.cr[6].eq {
	pc = 0x83003528; continue 'dispatch;
	}
	// 83003510: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 83003514: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83003518: 91610014  stw r11, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300351C: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 83003520: 616B007A  ori r11, r11, 0x7a
	ctx.r[11].u64 = ctx.r[11].u64 | 122;
	// 83003524: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 83003528: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300352C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83003530: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83003534: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 83003538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003540 size=88
    let mut pc: u32 = 0x83003540;
    'dispatch: loop {
        match pc {
            0x83003540 => {
    //   block [0x83003540..0x83003598)
	// 83003540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300354C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003550: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83003554: 396BF240  addi r11, r11, -0xdc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3520;
	// 83003558: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300355C: 916100FC  stw r11, 0xfc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 83003560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003564: 4BCC4AFD  bl 0x82cc8060
	ctx.lr = 0x83003568;
	sub_82CC8060(ctx, base);
	// 83003568: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300356C: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 83003570: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83003574: 23EB0000  subfic r31, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[31].s64 = (0 as i64) - ctx.r[11].s64;
	// 83003578: 806100FC  lwz r3, 0xfc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 8300357C: 48006695  bl 0x83009c10
	ctx.lr = 0x83003580;
	sub_83009C10(ctx, base);
	// 83003580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003584: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 83003588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300358C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83003594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003598 size=44
    let mut pc: u32 = 0x83003598;
    'dispatch: loop {
        match pc {
            0x83003598 => {
    //   block [0x83003598..0x830035C4)
	// 83003598: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8300359C: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 830035A0: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 830035A4: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 830035A8: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830035AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830035B0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830035B4: 81410024  lwz r10, 0x24(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 830035B8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830035BC: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830035C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830035C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830035C8 size=24
    let mut pc: u32 = 0x830035C8;
    'dispatch: loop {
        match pc {
            0x830035C8 => {
    //   block [0x830035C8..0x830035E0)
	// 830035C8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 830035CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 830035D0: 396BC404  addi r11, r11, -0x3bfc
	ctx.r[11].s64 = ctx.r[11].s64 + -15356;
	// 830035D4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 830035D8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830035DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830035E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830035E0 size=72
    let mut pc: u32 = 0x830035E0;
    'dispatch: loop {
        match pc {
            0x830035E0 => {
    //   block [0x830035E0..0x83003628)
	// 830035E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830035E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830035E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830035EC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830035F0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830035F4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830035F8: 4BFFFFD1  bl 0x830035c8
	ctx.lr = 0x830035FC;
	sub_830035C8(ctx, base);
	// 830035FC: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003600: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83003604: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003608: 419A000C  beq cr6, 0x83003614
	if ctx.cr[6].eq {
	pc = 0x83003614; continue 'dispatch;
	}
	// 8300360C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003610: 4B8421A1  bl 0x828457b0
	ctx.lr = 0x83003614;
	sub_828457B0(ctx, base);
	// 83003614: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300361C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003628 size=112
    let mut pc: u32 = 0x83003628;
    'dispatch: loop {
        match pc {
            0x83003628 => {
    //   block [0x83003628..0x83003698)
	// 83003628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300362C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003634: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83003638: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300363C: 396BC404  addi r11, r11, -0x3bfc
	ctx.r[11].s64 = ctx.r[11].s64 + -15356;
	// 83003640: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003644: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003648: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300364C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83003650: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83003654: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300365C: 419A0014  beq cr6, 0x83003670
	if ctx.cr[6].eq {
	pc = 0x83003670; continue 'dispatch;
	}
	// 83003660: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003664: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83003668: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300366C: 4800000C  b 0x83003678
	pc = 0x83003678; continue 'dispatch;
	// 83003670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003674: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83003678: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 8300367C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83003680: 482B6615  bl 0x832b9c94
	ctx.lr = 0x83003684;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 83003684: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83003688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300368C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003698 size=40
    let mut pc: u32 = 0x83003698;
    'dispatch: loop {
        match pc {
            0x83003698 => {
    //   block [0x83003698..0x830036C0)
	// 83003698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300369C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830036A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830036A4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830036A8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830036AC: 48000015  bl 0x830036c0
	ctx.lr = 0x830036B0;
	sub_830036C0(ctx, base);
	// 830036B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830036B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830036B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830036BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830036C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830036C0 size=40
    let mut pc: u32 = 0x830036C0;
    'dispatch: loop {
        match pc {
            0x830036C0 => {
    //   block [0x830036C0..0x830036E8)
	// 830036C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830036C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830036C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830036CC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830036D0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830036D4: 48000015  bl 0x830036e8
	ctx.lr = 0x830036D8;
	sub_830036E8(ctx, base);
	// 830036D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830036DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830036E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830036E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830036E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830036E8 size=224
    let mut pc: u32 = 0x830036E8;
    'dispatch: loop {
        match pc {
            0x830036E8 => {
    //   block [0x830036E8..0x830037C8)
	// 830036E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830036EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830036F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830036F4: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 830036F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830036FC: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003700: 48001089  bl 0x83004788
	ctx.lr = 0x83003704;
	sub_83004788(ctx, base);
	// 83003704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003708: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8300370C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83003714: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 83003718: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300371C: 48000CCD  bl 0x830043e8
	ctx.lr = 0x83003720;
	sub_830043E8(ctx, base);
	// 83003720: E8A30000  ld r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 83003724: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83003728: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300372C: E8C10068  ld r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83003730: 48000099  bl 0x830037c8
	ctx.lr = 0x83003734;
	sub_830037C8(ctx, base);
	// 83003734: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003738: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300373C: 4800146D  bl 0x83004ba8
	ctx.lr = 0x83003740;
	sub_83004BA8(ctx, base);
	// 83003740: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003744: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003748: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8300374C: 48000C6D  bl 0x830043b8
	ctx.lr = 0x83003750;
	sub_830043B8(ctx, base);
	// 83003750: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003754: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003758: 48001669  bl 0x83004dc0
	ctx.lr = 0x8300375C;
	sub_83004DC0(ctx, base);
	// 8300375C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003760: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003764: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83003768: 48000C51  bl 0x830043b8
	ctx.lr = 0x8300376C;
	sub_830043B8(ctx, base);
	// 8300376C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003770: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003774: 4BEE4DC5  bl 0x82ee8538
	ctx.lr = 0x83003778;
	sub_82EE8538(ctx, base);
	// 83003778: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300377C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003780: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83003784: 48000C35  bl 0x830043b8
	ctx.lr = 0x83003788;
	sub_830043B8(ctx, base);
	// 83003788: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300378C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003790: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003794: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003798: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8300379C: 480015D5  bl 0x83004d70
	ctx.lr = 0x830037A0;
	sub_83004D70(ctx, base);
	// 830037A0: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 830037A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830037A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830037AC: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 830037B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830037B4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830037B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830037BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830037C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830037C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830037C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830037C8 size=232
    let mut pc: u32 = 0x830037C8;
    'dispatch: loop {
        match pc {
            0x830037C8 => {
    //   block [0x830037C8..0x830038B0)
	// 830037C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830037CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830037D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830037D4: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 830037D8: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 830037DC: F8A100A0  std r5, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[5].u64 ) };
	// 830037E0: F8C100A8  std r6, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[6].u64 ) };
	// 830037E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830037E8: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 830037EC: 48000BFD  bl 0x830043e8
	ctx.lr = 0x830037F0;
	sub_830043E8(ctx, base);
	// 830037F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830037F4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830037F8: 48001469  bl 0x83004c60
	ctx.lr = 0x830037FC;
	sub_83004C60(ctx, base);
	// 830037FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83003800: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003804: 419A0048  beq cr6, 0x8300384c
	if ctx.cr[6].eq {
	pc = 0x8300384C; continue 'dispatch;
	}
	// 83003808: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8300380C: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83003810: 48000F79  bl 0x83004788
	ctx.lr = 0x83003814;
	sub_83004788(ctx, base);
	// 83003814: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003818: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8300381C: 48001445  bl 0x83004c60
	ctx.lr = 0x83003820;
	sub_83004C60(ctx, base);
	// 83003820: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83003824: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003828: 419A0024  beq cr6, 0x8300384c
	if ctx.cr[6].eq {
	pc = 0x8300384C; continue 'dispatch;
	}
	// 8300382C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83003830: 480008C1  bl 0x830040f0
	ctx.lr = 0x83003834;
	sub_830040F0(ctx, base);
	// 83003834: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003838: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8300383C: 48000BAD  bl 0x830043e8
	ctx.lr = 0x83003840;
	sub_830043E8(ctx, base);
	// 83003840: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83003844: 4800005C  b 0x830038a0
	pc = 0x830038A0; continue 'dispatch;
	// 83003848: 48000058  b 0x830038a0
	pc = 0x830038A0; continue 'dispatch;
	// 8300384C: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 83003850: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 83003854: 48000BE5  bl 0x83004438
	ctx.lr = 0x83003858;
	sub_83004438(ctx, base);
	// 83003858: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8300385C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003860: 419A0028  beq cr6, 0x83003888
	if ctx.cr[6].eq {
	pc = 0x83003888; continue 'dispatch;
	}
	// 83003864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83003868: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300386C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 83003870: 48001449  bl 0x83004cb8
	ctx.lr = 0x83003874;
	sub_83004CB8(ctx, base);
	// 83003874: E8A30000  ld r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 83003878: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8300387C: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83003880: 48000039  bl 0x830038b8
	ctx.lr = 0x83003884;
	sub_830038B8(ctx, base);
	// 83003884: 4BFFFFC8  b 0x8300384c
	pc = 0x8300384C; continue 'dispatch;
	// 83003888: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300388C: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 83003890: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83003894: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 83003898: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300389C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 830038A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830038A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830038A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830038AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830038B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830038B0 size=8
    let mut pc: u32 = 0x830038B0;
    'dispatch: loop {
        match pc {
            0x830038B0 => {
    //   block [0x830038B0..0x830038B8)
	// 830038B0: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 830038B4: 8204C410  lwz r16, -0x3bf0(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-15344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830038B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830038B8 size=2060
    let mut pc: u32 = 0x830038B8;
    'dispatch: loop {
        match pc {
            0x830038B8 => {
    //   block [0x830038B8..0x830040C4)
	// 830038B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830038BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830038C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830038C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830038C8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830038CC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830038D0: 907F00E4  stw r3, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[3].u32 ) };
	// 830038D4: 909F00EC  stw r4, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 830038D8: F8BF00F0  std r5, 0xf0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[5].u64 ) };
	// 830038DC: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 830038E0: 396BF240  addi r11, r11, -0xdc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3520;
	// 830038E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830038E8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 830038EC: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 830038F0: 4BFFFAF1  bl 0x830033e0
	ctx.lr = 0x830038F4;
	sub_830033E0(ctx, base);
	// 830038F4: 480014BD  bl 0x83004db0
	ctx.lr = 0x830038F8;
	sub_83004DB0(ctx, base);
	// 830038F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830038FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003900: 419A0038  beq cr6, 0x83003938
	if ctx.cr[6].eq {
	pc = 0x83003938; continue 'dispatch;
	}
	// 83003904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83003908: 388B16C0  addi r4, r11, 0x16c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5824;
	// 8300390C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83003910: 4B2EE631  bl 0x822f1f40
	ctx.lr = 0x83003914;
	sub_822F1F40(ctx, base);
	// 83003914: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 83003918: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300391C: 4B2EE455  bl 0x822f1d70
	ctx.lr = 0x83003920;
	sub_822F1D70(ctx, base);
	// 83003920: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 83003924: 388BB5E0  addi r4, r11, -0x4a20
	ctx.r[4].s64 = ctx.r[11].s64 + -18976;
	// 83003928: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300392C: 4BCA98D5  bl 0x82cad200
	ctx.lr = 0x83003930;
	sub_82CAD200(ctx, base);
	// 83003930: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83003934: 4B16DEDD  bl 0x82171810
	ctx.lr = 0x83003938;
	sub_82171810(ctx, base);
	// 83003938: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8300393C: 4BFFFAA5  bl 0x830033e0
	ctx.lr = 0x83003940;
	sub_830033E0(ctx, base);
	// 83003940: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 83003944: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83003948: 480014F1  bl 0x83004e38
	ctx.lr = 0x8300394C;
	sub_83004E38(ctx, base);
	// 8300394C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003950: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003954: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003958: 48001251  bl 0x83004ba8
	ctx.lr = 0x8300395C;
	sub_83004BA8(ctx, base);
	// 8300395C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003960: 48001451  bl 0x83004db0
	ctx.lr = 0x83003964;
	sub_83004DB0(ctx, base);
	// 83003964: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300396C: 419A0018  beq cr6, 0x83003984
	if ctx.cr[6].eq {
	pc = 0x83003984; continue 'dispatch;
	}
	// 83003970: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003974: 4BEE4BC5  bl 0x82ee8538
	ctx.lr = 0x83003978;
	sub_82EE8538(ctx, base);
	// 83003978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300397C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83003980: 48000050  b 0x830039d0
	pc = 0x830039D0; continue 'dispatch;
	// 83003984: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003988: 4BEE4BB1  bl 0x82ee8538
	ctx.lr = 0x8300398C;
	sub_82EE8538(ctx, base);
	// 8300398C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003990: 48001421  bl 0x83004db0
	ctx.lr = 0x83003994;
	sub_83004DB0(ctx, base);
	// 83003994: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300399C: 419A0018  beq cr6, 0x830039b4
	if ctx.cr[6].eq {
	pc = 0x830039B4; continue 'dispatch;
	}
	// 830039A0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830039A4: 48001205  bl 0x83004ba8
	ctx.lr = 0x830039A8;
	sub_83004BA8(ctx, base);
	// 830039A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830039AC: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830039B0: 48000020  b 0x830039d0
	pc = 0x830039D0; continue 'dispatch;
	// 830039B4: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 830039B8: 4BFFFA29  bl 0x830033e0
	ctx.lr = 0x830039BC;
	sub_830033E0(ctx, base);
	// 830039BC: 907F0080  stw r3, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 830039C0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830039C4: 4BEE4B75  bl 0x82ee8538
	ctx.lr = 0x830039C8;
	sub_82EE8538(ctx, base);
	// 830039C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830039CC: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830039D0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830039D4: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830039D8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830039DC: 409A014C  bne cr6, 0x83003b28
	if !ctx.cr[6].eq {
	pc = 0x83003B28; continue 'dispatch;
	}
	// 830039E0: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830039E4: 480013DD  bl 0x83004dc0
	ctx.lr = 0x830039E8;
	sub_83004DC0(ctx, base);
	// 830039E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830039EC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830039F0: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830039F4: 480013BD  bl 0x83004db0
	ctx.lr = 0x830039F8;
	sub_83004DB0(ctx, base);
	// 830039F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830039FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003A00: 409A0014  bne cr6, 0x83003a14
	if !ctx.cr[6].eq {
	pc = 0x83003A14; continue 'dispatch;
	}
	// 83003A04: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003A08: 480013B9  bl 0x83004dc0
	ctx.lr = 0x83003A0C;
	sub_83004DC0(ctx, base);
	// 83003A0C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003A10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003A14: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003A18: 48000971  bl 0x83004388
	ctx.lr = 0x83003A1C;
	sub_83004388(ctx, base);
	// 83003A1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A20: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003A24: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003A28: 409A0018  bne cr6, 0x83003a40
	if !ctx.cr[6].eq {
	pc = 0x83003A40; continue 'dispatch;
	}
	// 83003A2C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003A30: 48000959  bl 0x83004388
	ctx.lr = 0x83003A34;
	sub_83004388(ctx, base);
	// 83003A34: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003A38: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003A3C: 48000040  b 0x83003a7c
	pc = 0x83003A7C; continue 'dispatch;
	// 83003A40: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003A44: 48001165  bl 0x83004ba8
	ctx.lr = 0x83003A48;
	sub_83004BA8(ctx, base);
	// 83003A48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A4C: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003A50: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003A54: 409A0018  bne cr6, 0x83003a6c
	if !ctx.cr[6].eq {
	pc = 0x83003A6C; continue 'dispatch;
	}
	// 83003A58: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003A5C: 4800114D  bl 0x83004ba8
	ctx.lr = 0x83003A60;
	sub_83004BA8(ctx, base);
	// 83003A60: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003A64: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003A68: 48000014  b 0x83003a7c
	pc = 0x83003A7C; continue 'dispatch;
	// 83003A6C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003A70: 4BEE4AC9  bl 0x82ee8538
	ctx.lr = 0x83003A74;
	sub_82EE8538(ctx, base);
	// 83003A74: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003A78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003A7C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003A80: 480012C1  bl 0x83004d40
	ctx.lr = 0x83003A84;
	sub_83004D40(ctx, base);
	// 83003A84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A88: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003A8C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003A90: 409A0040  bne cr6, 0x83003ad0
	if !ctx.cr[6].eq {
	pc = 0x83003AD0; continue 'dispatch;
	}
	// 83003A94: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003A98: 48001319  bl 0x83004db0
	ctx.lr = 0x83003A9C;
	sub_83004DB0(ctx, base);
	// 83003A9C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003AA4: 419A0010  beq cr6, 0x83003ab4
	if ctx.cr[6].eq {
	pc = 0x83003AB4; continue 'dispatch;
	}
	// 83003AA8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003AAC: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83003AB0: 48000010  b 0x83003ac0
	pc = 0x83003AC0; continue 'dispatch;
	// 83003AB4: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003AB8: 48001329  bl 0x83004de0
	ctx.lr = 0x83003ABC;
	sub_83004DE0(ctx, base);
	// 83003ABC: 907F00B0  stw r3, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 83003AC0: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003AC4: 4800127D  bl 0x83004d40
	ctx.lr = 0x83003AC8;
	sub_83004D40(ctx, base);
	// 83003AC8: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 83003ACC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003AD0: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003AD4: 48000885  bl 0x83004358
	ctx.lr = 0x83003AD8;
	sub_83004358(ctx, base);
	// 83003AD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003ADC: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003AE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003AE4: 409A0040  bne cr6, 0x83003b24
	if !ctx.cr[6].eq {
	pc = 0x83003B24; continue 'dispatch;
	}
	// 83003AE8: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003AEC: 480012C5  bl 0x83004db0
	ctx.lr = 0x83003AF0;
	sub_83004DB0(ctx, base);
	// 83003AF0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003AF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003AF8: 419A0010  beq cr6, 0x83003b08
	if ctx.cr[6].eq {
	pc = 0x83003B08; continue 'dispatch;
	}
	// 83003AFC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003B00: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83003B04: 48000010  b 0x83003b14
	pc = 0x83003B14; continue 'dispatch;
	// 83003B08: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003B0C: 4800096D  bl 0x83004478
	ctx.lr = 0x83003B10;
	sub_83004478(ctx, base);
	// 83003B10: 907F00B4  stw r3, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 83003B14: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003B18: 48000841  bl 0x83004358
	ctx.lr = 0x83003B1C;
	sub_83004358(ctx, base);
	// 83003B1C: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83003B20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003B24: 4800018C  b 0x83003cb0
	pc = 0x83003CB0; continue 'dispatch;
	// 83003B28: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003B2C: 4800107D  bl 0x83004ba8
	ctx.lr = 0x83003B30;
	sub_83004BA8(ctx, base);
	// 83003B30: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B34: 4800128D  bl 0x83004dc0
	ctx.lr = 0x83003B38;
	sub_83004DC0(ctx, base);
	// 83003B38: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003B3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003B40: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003B44: 48001065  bl 0x83004ba8
	ctx.lr = 0x83003B48;
	sub_83004BA8(ctx, base);
	// 83003B48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003B4C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003B50: 48001059  bl 0x83004ba8
	ctx.lr = 0x83003B54;
	sub_83004BA8(ctx, base);
	// 83003B54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B58: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003B5C: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003B60: 4BEE49D9  bl 0x82ee8538
	ctx.lr = 0x83003B64;
	sub_82EE8538(ctx, base);
	// 83003B64: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003B68: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B6C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003B70: 409A0010  bne cr6, 0x83003b80
	if !ctx.cr[6].eq {
	pc = 0x83003B80; continue 'dispatch;
	}
	// 83003B74: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003B78: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83003B7C: 4800007C  b 0x83003bf8
	pc = 0x83003BF8; continue 'dispatch;
	// 83003B80: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003B84: 4800123D  bl 0x83004dc0
	ctx.lr = 0x83003B88;
	sub_83004DC0(ctx, base);
	// 83003B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B8C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83003B90: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003B94: 4800121D  bl 0x83004db0
	ctx.lr = 0x83003B98;
	sub_83004DB0(ctx, base);
	// 83003B98: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003BA0: 409A0014  bne cr6, 0x83003bb4
	if !ctx.cr[6].eq {
	pc = 0x83003BB4; continue 'dispatch;
	}
	// 83003BA4: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003BA8: 48001219  bl 0x83004dc0
	ctx.lr = 0x83003BAC;
	sub_83004DC0(ctx, base);
	// 83003BAC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003BB0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003BB4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003BB8: 48000FF1  bl 0x83004ba8
	ctx.lr = 0x83003BBC;
	sub_83004BA8(ctx, base);
	// 83003BBC: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003BC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003BC4: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003BC8: 4BEE4971  bl 0x82ee8538
	ctx.lr = 0x83003BCC;
	sub_82EE8538(ctx, base);
	// 83003BCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003BD0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003BD4: 4BEE4965  bl 0x82ee8538
	ctx.lr = 0x83003BD8;
	sub_82EE8538(ctx, base);
	// 83003BD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003BDC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003BE0: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003BE4: 4BEE4955  bl 0x82ee8538
	ctx.lr = 0x83003BE8;
	sub_82EE8538(ctx, base);
	// 83003BE8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003BEC: 480011D5  bl 0x83004dc0
	ctx.lr = 0x83003BF0;
	sub_83004DC0(ctx, base);
	// 83003BF0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003BF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003BF8: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003BFC: 4800078D  bl 0x83004388
	ctx.lr = 0x83003C00;
	sub_83004388(ctx, base);
	// 83003C00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C04: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003C0C: 409A0018  bne cr6, 0x83003c24
	if !ctx.cr[6].eq {
	pc = 0x83003C24; continue 'dispatch;
	}
	// 83003C10: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003C14: 48000775  bl 0x83004388
	ctx.lr = 0x83003C18;
	sub_83004388(ctx, base);
	// 83003C18: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003C1C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003C20: 48000058  b 0x83003c78
	pc = 0x83003C78; continue 'dispatch;
	// 83003C24: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C28: 48001199  bl 0x83004dc0
	ctx.lr = 0x83003C2C;
	sub_83004DC0(ctx, base);
	// 83003C2C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C30: 48000F79  bl 0x83004ba8
	ctx.lr = 0x83003C34;
	sub_83004BA8(ctx, base);
	// 83003C34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C38: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C3C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003C40: 409A0020  bne cr6, 0x83003c60
	if !ctx.cr[6].eq {
	pc = 0x83003C60; continue 'dispatch;
	}
	// 83003C44: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C48: 48001179  bl 0x83004dc0
	ctx.lr = 0x83003C4C;
	sub_83004DC0(ctx, base);
	// 83003C4C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C50: 48000F59  bl 0x83004ba8
	ctx.lr = 0x83003C54;
	sub_83004BA8(ctx, base);
	// 83003C54: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003C58: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003C5C: 4800001C  b 0x83003c78
	pc = 0x83003C78; continue 'dispatch;
	// 83003C60: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C64: 4800115D  bl 0x83004dc0
	ctx.lr = 0x83003C68;
	sub_83004DC0(ctx, base);
	// 83003C68: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C6C: 4BEE48CD  bl 0x82ee8538
	ctx.lr = 0x83003C70;
	sub_82EE8538(ctx, base);
	// 83003C70: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003C74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003C78: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C7C: 48001145  bl 0x83004dc0
	ctx.lr = 0x83003C80;
	sub_83004DC0(ctx, base);
	// 83003C80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003C84: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003C88: 48001139  bl 0x83004dc0
	ctx.lr = 0x83003C8C;
	sub_83004DC0(ctx, base);
	// 83003C8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83003C94: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003C98: 48001139  bl 0x83004dd0
	ctx.lr = 0x83003C9C;
	sub_83004DD0(ctx, base);
	// 83003C9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003CA0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003CA4: 4800112D  bl 0x83004dd0
	ctx.lr = 0x83003CA8;
	sub_83004DD0(ctx, base);
	// 83003CA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003CAC: 4B2EDED5  bl 0x822f1b80
	ctx.lr = 0x83003CB0;
	sub_822F1B80(ctx, base);
	// 83003CB0: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83003CB4: 4800111D  bl 0x83004dd0
	ctx.lr = 0x83003CB8;
	sub_83004DD0(ctx, base);
	// 83003CB8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CBC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003CC0: 409A0384  bne cr6, 0x83004044
	if !ctx.cr[6].eq {
	pc = 0x83004044; continue 'dispatch;
	}
	// 83003CC4: 48000014  b 0x83003cd8
	pc = 0x83003CD8; continue 'dispatch;
	// 83003CC8: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003CCC: 480010F5  bl 0x83004dc0
	ctx.lr = 0x83003CD0;
	sub_83004DC0(ctx, base);
	// 83003CD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CD4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83003CD8: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003CDC: 480006AD  bl 0x83004388
	ctx.lr = 0x83003CE0;
	sub_83004388(ctx, base);
	// 83003CE0: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003CE4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CE8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003CEC: 419A0348  beq cr6, 0x83004034
	if ctx.cr[6].eq {
	pc = 0x83004034; continue 'dispatch;
	}
	// 83003CF0: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003CF4: 480010DD  bl 0x83004dd0
	ctx.lr = 0x83003CF8;
	sub_83004DD0(ctx, base);
	// 83003CF8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CFC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003D00: 409A0334  bne cr6, 0x83004034
	if !ctx.cr[6].eq {
	pc = 0x83004034; continue 'dispatch;
	}
	// 83003D04: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D08: 48000EA1  bl 0x83004ba8
	ctx.lr = 0x83003D0C;
	sub_83004BA8(ctx, base);
	// 83003D0C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83003D10: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D14: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83003D18: 409A0190  bne cr6, 0x83003ea8
	if !ctx.cr[6].eq {
	pc = 0x83003EA8; continue 'dispatch;
	}
	// 83003D1C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D20: 4BEE4819  bl 0x82ee8538
	ctx.lr = 0x83003D24;
	sub_82EE8538(ctx, base);
	// 83003D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D28: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003D2C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003D30: 480010A1  bl 0x83004dd0
	ctx.lr = 0x83003D34;
	sub_83004DD0(ctx, base);
	// 83003D34: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003D3C: 409A0040  bne cr6, 0x83003d7c
	if !ctx.cr[6].eq {
	pc = 0x83003D7C; continue 'dispatch;
	}
	// 83003D40: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003D44: 4800108D  bl 0x83004dd0
	ctx.lr = 0x83003D48;
	sub_83004DD0(ctx, base);
	// 83003D48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003D4C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003D50: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D54: 4800107D  bl 0x83004dd0
	ctx.lr = 0x83003D58;
	sub_83004DD0(ctx, base);
	// 83003D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003D5C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003D60: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D64: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003D68: 480008D9  bl 0x83004640
	ctx.lr = 0x83003D6C;
	sub_83004640(ctx, base);
	// 83003D6C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D70: 4BEE47C9  bl 0x82ee8538
	ctx.lr = 0x83003D74;
	sub_82EE8538(ctx, base);
	// 83003D74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D78: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003D7C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003D80: 48001031  bl 0x83004db0
	ctx.lr = 0x83003D84;
	sub_83004DB0(ctx, base);
	// 83003D84: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003D8C: 419A0010  beq cr6, 0x83003d9c
	if ctx.cr[6].eq {
	pc = 0x83003D9C; continue 'dispatch;
	}
	// 83003D90: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003D94: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83003D98: 4800010C  b 0x83003ea4
	pc = 0x83003EA4; continue 'dispatch;
	// 83003D9C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003DA0: 48000E09  bl 0x83004ba8
	ctx.lr = 0x83003DA4;
	sub_83004BA8(ctx, base);
	// 83003DA4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DA8: 48001029  bl 0x83004dd0
	ctx.lr = 0x83003DAC;
	sub_83004DD0(ctx, base);
	// 83003DAC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DB0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003DB4: 409A003C  bne cr6, 0x83003df0
	if !ctx.cr[6].eq {
	pc = 0x83003DF0; continue 'dispatch;
	}
	// 83003DB8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003DBC: 4BEE477D  bl 0x82ee8538
	ctx.lr = 0x83003DC0;
	sub_82EE8538(ctx, base);
	// 83003DC0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DC4: 4800100D  bl 0x83004dd0
	ctx.lr = 0x83003DC8;
	sub_83004DD0(ctx, base);
	// 83003DC8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DCC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003DD0: 409A0020  bne cr6, 0x83003df0
	if !ctx.cr[6].eq {
	pc = 0x83003DF0; continue 'dispatch;
	}
	// 83003DD4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003DD8: 48000FF9  bl 0x83004dd0
	ctx.lr = 0x83003DDC;
	sub_83004DD0(ctx, base);
	// 83003DDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003DE0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003DE4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003DE8: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83003DEC: 480000B8  b 0x83003ea4
	pc = 0x83003EA4; continue 'dispatch;
	// 83003DF0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003DF4: 4BEE4745  bl 0x82ee8538
	ctx.lr = 0x83003DF8;
	sub_82EE8538(ctx, base);
	// 83003DF8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DFC: 48000FD5  bl 0x83004dd0
	ctx.lr = 0x83003E00;
	sub_83004DD0(ctx, base);
	// 83003E00: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E04: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003E08: 409A0048  bne cr6, 0x83003e50
	if !ctx.cr[6].eq {
	pc = 0x83003E50; continue 'dispatch;
	}
	// 83003E0C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003E10: 48000D99  bl 0x83004ba8
	ctx.lr = 0x83003E14;
	sub_83004BA8(ctx, base);
	// 83003E14: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E18: 48000FB9  bl 0x83004dd0
	ctx.lr = 0x83003E1C;
	sub_83004DD0(ctx, base);
	// 83003E1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003E20: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003E24: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003E28: 48000FA9  bl 0x83004dd0
	ctx.lr = 0x83003E2C;
	sub_83004DD0(ctx, base);
	// 83003E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003E30: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003E34: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003E38: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003E3C: 480003D5  bl 0x83004210
	ctx.lr = 0x83003E40;
	sub_83004210(ctx, base);
	// 83003E40: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003E44: 4BEE46F5  bl 0x82ee8538
	ctx.lr = 0x83003E48;
	sub_82EE8538(ctx, base);
	// 83003E48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E4C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003E50: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003E54: 48000F7D  bl 0x83004dd0
	ctx.lr = 0x83003E58;
	sub_83004DD0(ctx, base);
	// 83003E58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003E5C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003E60: 48000F71  bl 0x83004dd0
	ctx.lr = 0x83003E64;
	sub_83004DD0(ctx, base);
	// 83003E64: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E68: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003E6C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003E70: 48000F61  bl 0x83004dd0
	ctx.lr = 0x83003E74;
	sub_83004DD0(ctx, base);
	// 83003E74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003E78: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003E7C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003E80: 4BEE46B9  bl 0x82ee8538
	ctx.lr = 0x83003E84;
	sub_82EE8538(ctx, base);
	// 83003E84: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E88: 48000F49  bl 0x83004dd0
	ctx.lr = 0x83003E8C;
	sub_83004DD0(ctx, base);
	// 83003E8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003E90: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003E94: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003E98: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003E9C: 480007A5  bl 0x83004640
	ctx.lr = 0x83003EA0;
	sub_83004640(ctx, base);
	// 83003EA0: 48000194  b 0x83004034
	pc = 0x83004034; continue 'dispatch;
	// 83003EA4: 4800018C  b 0x83004030
	pc = 0x83004030; continue 'dispatch;
	// 83003EA8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003EAC: 48000CFD  bl 0x83004ba8
	ctx.lr = 0x83003EB0;
	sub_83004BA8(ctx, base);
	// 83003EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003EB4: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003EB8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003EBC: 48000F15  bl 0x83004dd0
	ctx.lr = 0x83003EC0;
	sub_83004DD0(ctx, base);
	// 83003EC0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003EC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003EC8: 409A0040  bne cr6, 0x83003f08
	if !ctx.cr[6].eq {
	pc = 0x83003F08; continue 'dispatch;
	}
	// 83003ECC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003ED0: 48000F01  bl 0x83004dd0
	ctx.lr = 0x83003ED4;
	sub_83004DD0(ctx, base);
	// 83003ED4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003ED8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003EDC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003EE0: 48000EF1  bl 0x83004dd0
	ctx.lr = 0x83003EE4;
	sub_83004DD0(ctx, base);
	// 83003EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003EE8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003EEC: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003EF0: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003EF4: 4800031D  bl 0x83004210
	ctx.lr = 0x83003EF8;
	sub_83004210(ctx, base);
	// 83003EF8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003EFC: 48000CAD  bl 0x83004ba8
	ctx.lr = 0x83003F00;
	sub_83004BA8(ctx, base);
	// 83003F00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F04: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003F08: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F0C: 48000EA5  bl 0x83004db0
	ctx.lr = 0x83003F10;
	sub_83004DB0(ctx, base);
	// 83003F10: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83003F18: 419A0010  beq cr6, 0x83003f28
	if ctx.cr[6].eq {
	pc = 0x83003F28; continue 'dispatch;
	}
	// 83003F1C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003F20: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83003F24: 4800010C  b 0x83004030
	pc = 0x83004030; continue 'dispatch;
	// 83003F28: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F2C: 4BEE460D  bl 0x82ee8538
	ctx.lr = 0x83003F30;
	sub_82EE8538(ctx, base);
	// 83003F30: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F34: 48000E9D  bl 0x83004dd0
	ctx.lr = 0x83003F38;
	sub_83004DD0(ctx, base);
	// 83003F38: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F3C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003F40: 409A003C  bne cr6, 0x83003f7c
	if !ctx.cr[6].eq {
	pc = 0x83003F7C; continue 'dispatch;
	}
	// 83003F44: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F48: 48000C61  bl 0x83004ba8
	ctx.lr = 0x83003F4C;
	sub_83004BA8(ctx, base);
	// 83003F4C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F50: 48000E81  bl 0x83004dd0
	ctx.lr = 0x83003F54;
	sub_83004DD0(ctx, base);
	// 83003F54: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F58: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003F5C: 409A0020  bne cr6, 0x83003f7c
	if !ctx.cr[6].eq {
	pc = 0x83003F7C; continue 'dispatch;
	}
	// 83003F60: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F64: 48000E6D  bl 0x83004dd0
	ctx.lr = 0x83003F68;
	sub_83004DD0(ctx, base);
	// 83003F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003F6C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003F70: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003F74: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83003F78: 480000B8  b 0x83004030
	pc = 0x83004030; continue 'dispatch;
	// 83003F7C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F80: 48000C29  bl 0x83004ba8
	ctx.lr = 0x83003F84;
	sub_83004BA8(ctx, base);
	// 83003F84: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F88: 48000E49  bl 0x83004dd0
	ctx.lr = 0x83003F8C;
	sub_83004DD0(ctx, base);
	// 83003F8C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003F90: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83003F94: 409A0048  bne cr6, 0x83003fdc
	if !ctx.cr[6].eq {
	pc = 0x83003FDC; continue 'dispatch;
	}
	// 83003F98: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003F9C: 4BEE459D  bl 0x82ee8538
	ctx.lr = 0x83003FA0;
	sub_82EE8538(ctx, base);
	// 83003FA0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003FA4: 48000E2D  bl 0x83004dd0
	ctx.lr = 0x83003FA8;
	sub_83004DD0(ctx, base);
	// 83003FA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83003FAC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003FB0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003FB4: 48000E1D  bl 0x83004dd0
	ctx.lr = 0x83003FB8;
	sub_83004DD0(ctx, base);
	// 83003FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003FBC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003FC0: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003FC4: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83003FC8: 48000679  bl 0x83004640
	ctx.lr = 0x83003FCC;
	sub_83004640(ctx, base);
	// 83003FCC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003FD0: 48000BD9  bl 0x83004ba8
	ctx.lr = 0x83003FD4;
	sub_83004BA8(ctx, base);
	// 83003FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003FD8: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83003FDC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003FE0: 48000DF1  bl 0x83004dd0
	ctx.lr = 0x83003FE4;
	sub_83004DD0(ctx, base);
	// 83003FE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003FE8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83003FEC: 48000DE5  bl 0x83004dd0
	ctx.lr = 0x83003FF0;
	sub_83004DD0(ctx, base);
	// 83003FF0: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003FF4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83003FF8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83003FFC: 48000DD5  bl 0x83004dd0
	ctx.lr = 0x83004000;
	sub_83004DD0(ctx, base);
	// 83004000: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83004004: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004008: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300400C: 48000B9D  bl 0x83004ba8
	ctx.lr = 0x83004010;
	sub_83004BA8(ctx, base);
	// 83004010: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004014: 48000DBD  bl 0x83004dd0
	ctx.lr = 0x83004018;
	sub_83004DD0(ctx, base);
	// 83004018: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300401C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004020: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004024: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83004028: 480001E9  bl 0x83004210
	ctx.lr = 0x8300402C;
	sub_83004210(ctx, base);
	// 8300402C: 48000008  b 0x83004034
	pc = 0x83004034; continue 'dispatch;
	// 83004030: 4BFFFC98  b 0x83003cc8
	pc = 0x83003CC8; continue 'dispatch;
	// 83004034: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004038: 48000D99  bl 0x83004dd0
	ctx.lr = 0x8300403C;
	sub_83004DD0(ctx, base);
	// 8300403C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83004040: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004044: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83004048: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300404C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 83004050: 48000369  bl 0x830043b8
	ctx.lr = 0x83004054;
	sub_830043B8(ctx, base);
	// 83004054: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83004058: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300405C: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83004060: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 83004064: 48000D0D  bl 0x83004d70
	ctx.lr = 0x83004068;
	sub_83004D70(ctx, base);
	// 83004068: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300406C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83004074: 40990018  ble cr6, 0x8300408c
	if !ctx.cr[6].gt {
	pc = 0x8300408C; continue 'dispatch;
	}
	// 83004078: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300407C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004080: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83004084: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83004088: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300408C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83004090: 815F00F0  lwz r10, 0xf0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 83004094: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83004098: 815F00F4  lwz r10, 0xf4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300409C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830040A0: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 830040A4: 48005B6D  bl 0x83009c10
	ctx.lr = 0x830040A8;
	sub_83009C10(ctx, base);
	// 830040A8: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830040AC: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830040B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830040B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830040B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830040BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830040C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830040C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830040C4 size=40
    let mut pc: u32 = 0x830040C4;
    'dispatch: loop {
        match pc {
            0x830040C4 => {
    //   block [0x830040C4..0x830040EC)
	// 830040C4: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830040C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830040CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830040D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830040D4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830040D8: 4B16D739  bl 0x82171810
	ctx.lr = 0x830040DC;
	sub_82171810(ctx, base);
	// 830040DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830040E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830040E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830040E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830040F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830040F0 size=132
    let mut pc: u32 = 0x830040F0;
    'dispatch: loop {
        match pc {
            0x830040F0 => {
    //   block [0x830040F0..0x83004174)
	// 830040F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830040F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830040F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830040FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004100: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004104: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004108: 48000281  bl 0x83004388
	ctx.lr = 0x8300410C;
	sub_83004388(ctx, base);
	// 8300410C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004110: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004114: 48000065  bl 0x83004178
	ctx.lr = 0x83004118;
	sub_83004178(ctx, base);
	// 83004118: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300411C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004120: 48000269  bl 0x83004388
	ctx.lr = 0x83004124;
	sub_83004388(ctx, base);
	// 83004124: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004128: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300412C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83004134: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83004138: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300413C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004140: 48000C01  bl 0x83004d40
	ctx.lr = 0x83004144;
	sub_83004D40(ctx, base);
	// 83004144: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004148: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300414C: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004150: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004154: 48000205  bl 0x83004358
	ctx.lr = 0x83004158;
	sub_83004358(ctx, base);
	// 83004158: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300415C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300416C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004178 size=152
    let mut pc: u32 = 0x83004178;
    'dispatch: loop {
        match pc {
            0x83004178 => {
    //   block [0x83004178..0x83004210)
	// 83004178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300417C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004184: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004188: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8300418C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004190: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83004194: 4800000C  b 0x830041a0
	pc = 0x830041A0; continue 'dispatch;
	// 83004198: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300419C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830041A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830041A4: 48000C0D  bl 0x83004db0
	ctx.lr = 0x830041A8;
	sub_83004DB0(ctx, base);
	// 830041A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830041B0: 409A0050  bne cr6, 0x83004200
	if !ctx.cr[6].eq {
	pc = 0x83004200; continue 'dispatch;
	}
	// 830041B4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830041B8: 4BEE4381  bl 0x82ee8538
	ctx.lr = 0x830041BC;
	sub_82EE8538(ctx, base);
	// 830041BC: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041C0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830041C4: 4BFFFFB5  bl 0x83004178
	ctx.lr = 0x830041C8;
	sub_83004178(ctx, base);
	// 830041C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830041CC: 480009DD  bl 0x83004ba8
	ctx.lr = 0x830041D0;
	sub_83004BA8(ctx, base);
	// 830041D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830041D8: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830041DC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830041E0: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830041E4: 480001D5  bl 0x830043b8
	ctx.lr = 0x830041E8;
	sub_830043B8(ctx, base);
	// 830041E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830041EC: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830041F0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830041F4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830041F8: 48000B79  bl 0x83004d70
	ctx.lr = 0x830041FC;
	sub_83004D70(ctx, base);
	// 830041FC: 4BFFFF9C  b 0x83004198
	pc = 0x83004198; continue 'dispatch;
	// 83004200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300420C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004210 size=328
    let mut pc: u32 = 0x83004210;
    'dispatch: loop {
        match pc {
            0x83004210 => {
    //   block [0x83004210..0x83004358)
	// 83004210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300421C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004220: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 83004224: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83004228: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300422C: 4800097D  bl 0x83004ba8
	ctx.lr = 0x83004230;
	sub_83004BA8(ctx, base);
	// 83004230: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004234: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83004238: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300423C: 4BEE42FD  bl 0x82ee8538
	ctx.lr = 0x83004240;
	sub_82EE8538(ctx, base);
	// 83004240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004244: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004248: 48000961  bl 0x83004ba8
	ctx.lr = 0x8300424C;
	sub_83004BA8(ctx, base);
	// 8300424C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004250: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004254: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004258: 4BEE42E1  bl 0x82ee8538
	ctx.lr = 0x8300425C;
	sub_82EE8538(ctx, base);
	// 8300425C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004260: 48000B51  bl 0x83004db0
	ctx.lr = 0x83004264;
	sub_83004DB0(ctx, base);
	// 83004264: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300426C: 409A001C  bne cr6, 0x83004288
	if !ctx.cr[6].eq {
	pc = 0x83004288; continue 'dispatch;
	}
	// 83004270: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004274: 4BEE42C5  bl 0x82ee8538
	ctx.lr = 0x83004278;
	sub_82EE8538(ctx, base);
	// 83004278: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300427C: 48000B45  bl 0x83004dc0
	ctx.lr = 0x83004280;
	sub_83004DC0(ctx, base);
	// 83004280: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004284: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004288: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300428C: 48000B35  bl 0x83004dc0
	ctx.lr = 0x83004290;
	sub_83004DC0(ctx, base);
	// 83004290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004294: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004298: 48000B29  bl 0x83004dc0
	ctx.lr = 0x8300429C;
	sub_83004DC0(ctx, base);
	// 8300429C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830042A4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830042A8: 480000E1  bl 0x83004388
	ctx.lr = 0x830042AC;
	sub_83004388(ctx, base);
	// 830042AC: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830042B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830042B8: 409A0018  bne cr6, 0x830042d0
	if !ctx.cr[6].eq {
	pc = 0x830042D0; continue 'dispatch;
	}
	// 830042BC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830042C0: 480000C9  bl 0x83004388
	ctx.lr = 0x830042C4;
	sub_83004388(ctx, base);
	// 830042C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830042C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830042CC: 48000058  b 0x83004324
	pc = 0x83004324; continue 'dispatch;
	// 830042D0: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830042D4: 48000AED  bl 0x83004dc0
	ctx.lr = 0x830042D8;
	sub_83004DC0(ctx, base);
	// 830042D8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042DC: 4BEE425D  bl 0x82ee8538
	ctx.lr = 0x830042E0;
	sub_82EE8538(ctx, base);
	// 830042E0: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830042E4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830042EC: 409A0020  bne cr6, 0x8300430c
	if !ctx.cr[6].eq {
	pc = 0x8300430C; continue 'dispatch;
	}
	// 830042F0: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830042F4: 48000ACD  bl 0x83004dc0
	ctx.lr = 0x830042F8;
	sub_83004DC0(ctx, base);
	// 830042F8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042FC: 4BEE423D  bl 0x82ee8538
	ctx.lr = 0x83004300;
	sub_82EE8538(ctx, base);
	// 83004300: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004304: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004308: 4800001C  b 0x83004324
	pc = 0x83004324; continue 'dispatch;
	// 8300430C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004310: 48000AB1  bl 0x83004dc0
	ctx.lr = 0x83004314;
	sub_83004DC0(ctx, base);
	// 83004314: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004318: 48000891  bl 0x83004ba8
	ctx.lr = 0x8300431C;
	sub_83004BA8(ctx, base);
	// 8300431C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004320: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004324: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004328: 4BEE4211  bl 0x82ee8538
	ctx.lr = 0x8300432C;
	sub_82EE8538(ctx, base);
	// 8300432C: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004330: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004334: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004338: 48000A89  bl 0x83004dc0
	ctx.lr = 0x8300433C;
	sub_83004DC0(ctx, base);
	// 8300433C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004340: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83004348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300434C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004358 size=44
    let mut pc: u32 = 0x83004358;
    'dispatch: loop {
        match pc {
            0x83004358 => {
    //   block [0x83004358..0x83004384)
	// 83004358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004364: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004368: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300436C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004370: 4BEE41C9  bl 0x82ee8538
	ctx.lr = 0x83004374;
	sub_82EE8538(ctx, base);
	// 83004374: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300437C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004388 size=44
    let mut pc: u32 = 0x83004388;
    'dispatch: loop {
        match pc {
            0x83004388 => {
    //   block [0x83004388..0x830043B4)
	// 83004388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300438C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004394: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004398: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300439C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830043A0: 48000A21  bl 0x83004dc0
	ctx.lr = 0x830043A4;
	sub_83004DC0(ctx, base);
	// 830043A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830043A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830043AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830043B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830043B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830043B8 size=44
    let mut pc: u32 = 0x830043B8;
    'dispatch: loop {
        match pc {
            0x830043B8 => {
    //   block [0x830043B8..0x830043E4)
	// 830043B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830043BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830043C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830043C4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830043C8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830043CC: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830043D0: 4BEE92A9  bl 0x82eed678
	ctx.lr = 0x830043D4;
	sub_82EED678(ctx, base);
	// 830043D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830043D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830043DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830043E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830043E8 size=76
    let mut pc: u32 = 0x830043E8;
    'dispatch: loop {
        match pc {
            0x830043E8 => {
    //   block [0x830043E8..0x83004434)
	// 830043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830043F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830043F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830043F8: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830043FC: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83004400: 83E1007C  lwz r31, 0x7c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004404: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004408: 48000939  bl 0x83004d40
	ctx.lr = 0x8300440C;
	sub_83004D40(ctx, base);
	// 8300440C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004410: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004414: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83004418: 480003B1  bl 0x830047c8
	ctx.lr = 0x8300441C;
	sub_830047C8(ctx, base);
	// 8300441C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300442C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004438 size=64
    let mut pc: u32 = 0x83004438;
    'dispatch: loop {
        match pc {
            0x83004438 => {
    //   block [0x83004438..0x83004478)
	// 83004438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300443C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004444: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004448: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8300444C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004450: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004454: 4800080D  bl 0x83004c60
	ctx.lr = 0x83004458;
	sub_83004C60(ctx, base);
	// 83004458: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8300445C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004460: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004464: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83004468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300446C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004478 size=84
    let mut pc: u32 = 0x83004478;
    'dispatch: loop {
        match pc {
            0x83004478 => {
    //   block [0x83004478..0x830044CC)
	// 83004478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004484: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004488: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300448C: 4BEE40AD  bl 0x82ee8538
	ctx.lr = 0x83004490;
	sub_82EE8538(ctx, base);
	// 83004490: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004494: 4800091D  bl 0x83004db0
	ctx.lr = 0x83004498;
	sub_83004DB0(ctx, base);
	// 83004498: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300449C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830044A0: 409A0018  bne cr6, 0x830044b8
	if !ctx.cr[6].eq {
	pc = 0x830044B8; continue 'dispatch;
	}
	// 830044A4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830044A8: 4BEE4091  bl 0x82ee8538
	ctx.lr = 0x830044AC;
	sub_82EE8538(ctx, base);
	// 830044AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830044B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830044B4: 4BFFFFD4  b 0x83004488
	pc = 0x83004488; continue 'dispatch;
	// 830044B8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830044BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830044C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830044C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830044C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830044D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830044D0 size=96
    let mut pc: u32 = 0x830044D0;
    'dispatch: loop {
        match pc {
            0x830044D0 => {
    //   block [0x830044D0..0x83004530)
	// 830044D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830044D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830044D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830044DC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830044E0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830044E4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 830044E8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830044EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830044F0: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830044F4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830044F8: 4BEE9299  bl 0x82eed790
	ctx.lr = 0x830044FC;
	sub_82EED790(ctx, base);
	// 830044FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83004500: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004504: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004508: 48000029  bl 0x83004530
	ctx.lr = 0x8300450C;
	sub_83004530(ctx, base);
	// 8300450C: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004510: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004514: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 83004518: 4BEE9279  bl 0x82eed790
	ctx.lr = 0x8300451C;
	sub_82EED790(ctx, base);
	// 8300451C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004530 size=96
    let mut pc: u32 = 0x83004530;
    'dispatch: loop {
        match pc {
            0x83004530 => {
    //   block [0x83004530..0x83004590)
	// 83004530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300453C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004540: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83004544: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83004548: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8300454C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83004550: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004554: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004558: 4BEE9239  bl 0x82eed790
	ctx.lr = 0x8300455C;
	sub_82EED790(ctx, base);
	// 8300455C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83004560: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004564: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004568: 480005F1  bl 0x83004b58
	ctx.lr = 0x8300456C;
	sub_83004B58(ctx, base);
	// 8300456C: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004570: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004574: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83004578: 4BEE9219  bl 0x82eed790
	ctx.lr = 0x8300457C;
	sub_82EED790(ctx, base);
	// 8300457C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300458C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004590 size=48
    let mut pc: u32 = 0x83004590;
    'dispatch: loop {
        match pc {
            0x83004590 => {
    //   block [0x83004590..0x830045C0)
	// 83004590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300459C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830045A0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830045A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830045A8: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830045AC: 48000015  bl 0x830045c0
	ctx.lr = 0x830045B0;
	sub_830045C0(ctx, base);
	// 830045B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830045B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830045B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830045BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830045C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830045C0 size=124
    let mut pc: u32 = 0x830045C0;
    'dispatch: loop {
        match pc {
            0x830045C0 => {
    //   block [0x830045C0..0x8300463C)
	// 830045C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830045C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830045C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830045CC: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 830045D0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 830045D4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830045D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830045DC: 41990010  bgt cr6, 0x830045ec
	if ctx.cr[6].gt {
	pc = 0x830045EC; continue 'dispatch;
	}
	// 830045E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830045E4: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830045E8: 48000038  b 0x83004620
	pc = 0x83004620; continue 'dispatch;
	// 830045EC: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830045F0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830045F4: 0CCB0000  twi 6, r11, 0
	// 830045F8: 7D6A5B96  divwu r11, r10, r11
	ctx.r[11].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 830045FC: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 83004600: 40980020  bge cr6, 0x83004620
	if !ctx.cr[6].lt {
	pc = 0x83004620; continue 'dispatch;
	}
	// 83004604: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83004608: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300460C: 4BCD3265  bl 0x82cd7870
	ctx.lr = 0x83004610;
	sub_82CD7870(ctx, base);
	// 83004610: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 83004614: 388BB534  addi r4, r11, -0x4acc
	ctx.r[4].s64 = ctx.r[11].s64 + -19148;
	// 83004618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300461C: 4BCA8BE5  bl 0x82cad200
	ctx.lr = 0x83004620;
	sub_82CAD200(ctx, base);
	// 83004620: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004624: 1C6B0014  mulli r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 * 20;
	// 83004628: 4B2E5B91  bl 0x822ea1b8
	ctx.lr = 0x8300462C;
	sub_822EA1B8(ctx, base);
	// 8300462C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83004630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004640 size=328
    let mut pc: u32 = 0x83004640;
    'dispatch: loop {
        match pc {
            0x83004640 => {
    //   block [0x83004640..0x83004788)
	// 83004640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004648: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300464C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004650: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 83004654: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83004658: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300465C: 4BEE3EDD  bl 0x82ee8538
	ctx.lr = 0x83004660;
	sub_82EE8538(ctx, base);
	// 83004660: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004664: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83004668: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300466C: 4800053D  bl 0x83004ba8
	ctx.lr = 0x83004670;
	sub_83004BA8(ctx, base);
	// 83004670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004674: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004678: 4BEE3EC1  bl 0x82ee8538
	ctx.lr = 0x8300467C;
	sub_82EE8538(ctx, base);
	// 8300467C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004680: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004684: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004688: 48000521  bl 0x83004ba8
	ctx.lr = 0x8300468C;
	sub_83004BA8(ctx, base);
	// 8300468C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004690: 48000721  bl 0x83004db0
	ctx.lr = 0x83004694;
	sub_83004DB0(ctx, base);
	// 83004694: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300469C: 409A001C  bne cr6, 0x830046b8
	if !ctx.cr[6].eq {
	pc = 0x830046B8; continue 'dispatch;
	}
	// 830046A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830046A4: 48000505  bl 0x83004ba8
	ctx.lr = 0x830046A8;
	sub_83004BA8(ctx, base);
	// 830046A8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830046AC: 48000715  bl 0x83004dc0
	ctx.lr = 0x830046B0;
	sub_83004DC0(ctx, base);
	// 830046B0: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830046B4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830046B8: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830046BC: 48000705  bl 0x83004dc0
	ctx.lr = 0x830046C0;
	sub_83004DC0(ctx, base);
	// 830046C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830046C4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830046C8: 480006F9  bl 0x83004dc0
	ctx.lr = 0x830046CC;
	sub_83004DC0(ctx, base);
	// 830046CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830046D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830046D4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830046D8: 4BFFFCB1  bl 0x83004388
	ctx.lr = 0x830046DC;
	sub_83004388(ctx, base);
	// 830046DC: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 830046E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830046E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830046E8: 409A0018  bne cr6, 0x83004700
	if !ctx.cr[6].eq {
	pc = 0x83004700; continue 'dispatch;
	}
	// 830046EC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830046F0: 4BFFFC99  bl 0x83004388
	ctx.lr = 0x830046F4;
	sub_83004388(ctx, base);
	// 830046F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830046F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830046FC: 48000058  b 0x83004754
	pc = 0x83004754; continue 'dispatch;
	// 83004700: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004704: 480006BD  bl 0x83004dc0
	ctx.lr = 0x83004708;
	sub_83004DC0(ctx, base);
	// 83004708: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300470C: 4800049D  bl 0x83004ba8
	ctx.lr = 0x83004710;
	sub_83004BA8(ctx, base);
	// 83004710: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004714: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004718: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300471C: 409A0020  bne cr6, 0x8300473c
	if !ctx.cr[6].eq {
	pc = 0x8300473C; continue 'dispatch;
	}
	// 83004720: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004724: 4800069D  bl 0x83004dc0
	ctx.lr = 0x83004728;
	sub_83004DC0(ctx, base);
	// 83004728: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300472C: 4800047D  bl 0x83004ba8
	ctx.lr = 0x83004730;
	sub_83004BA8(ctx, base);
	// 83004730: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004734: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004738: 4800001C  b 0x83004754
	pc = 0x83004754; continue 'dispatch;
	// 8300473C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004740: 48000681  bl 0x83004dc0
	ctx.lr = 0x83004744;
	sub_83004DC0(ctx, base);
	// 83004744: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004748: 4BEE3DF1  bl 0x82ee8538
	ctx.lr = 0x8300474C;
	sub_82EE8538(ctx, base);
	// 8300474C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004750: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004754: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004758: 48000451  bl 0x83004ba8
	ctx.lr = 0x8300475C;
	sub_83004BA8(ctx, base);
	// 8300475C: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004760: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004764: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83004768: 48000659  bl 0x83004dc0
	ctx.lr = 0x8300476C;
	sub_83004DC0(ctx, base);
	// 8300476C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004770: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83004778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300477C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004788 size=60
    let mut pc: u32 = 0x83004788;
    'dispatch: loop {
        match pc {
            0x83004788 => {
    //   block [0x83004788..0x830047C4)
	// 83004788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004794: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004798: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8300479C: 80A1007C  lwz r5, 0x7c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830047A0: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830047A4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830047A8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830047AC: 4800001D  bl 0x830047c8
	ctx.lr = 0x830047B0;
	sub_830047C8(ctx, base);
	// 830047B0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830047B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830047B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830047BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830047C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830047C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830047C8 size=60
    let mut pc: u32 = 0x830047C8;
    'dispatch: loop {
        match pc {
            0x830047C8 => {
    //   block [0x830047C8..0x83004804)
	// 830047C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830047CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830047D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830047D4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830047D8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830047DC: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 830047E0: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830047E4: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830047E8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830047EC: 4800001D  bl 0x83004808
	ctx.lr = 0x830047F0;
	sub_83004808(ctx, base);
	// 830047F0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830047F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830047F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830047FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004808 size=92
    let mut pc: u32 = 0x83004808;
    'dispatch: loop {
        match pc {
            0x83004808 => {
    //   block [0x83004808..0x83004864)
	// 83004808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004814: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004818: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8300481C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83004820: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004824: 480004ED  bl 0x83004d10
	ctx.lr = 0x83004828;
	sub_83004D10(ctx, base);
	// 83004828: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300482C: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83004830: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83004834: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300483C: 409A0008  bne cr6, 0x83004844
	if !ctx.cr[6].eq {
	pc = 0x83004844; continue 'dispatch;
	}
	// 83004840: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 83004844: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004848: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300484C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004850: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300485C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004868 size=60
    let mut pc: u32 = 0x83004868;
    'dispatch: loop {
        match pc {
            0x83004868 => {
    //   block [0x83004868..0x830048A4)
	// 83004868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004874: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300487C: 4800032D  bl 0x83004ba8
	ctx.lr = 0x83004880;
	sub_83004BA8(ctx, base);
	// 83004880: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83004884: 38810051  addi r4, r1, 0x51
	ctx.r[4].s64 = ctx.r[1].s64 + 81;
	// 83004888: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300488C: 4800001D  bl 0x830048a8
	ctx.lr = 0x83004890;
	sub_830048A8(ctx, base);
	// 83004890: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830048A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830048A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830048A8 size=88
    let mut pc: u32 = 0x830048A8;
    'dispatch: loop {
        match pc {
            0x830048A8 => {
    //   block [0x830048A8..0x83004900)
	// 830048A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830048AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830048B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830048B4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830048B8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830048BC: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 830048C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830048C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830048C8: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830048CC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830048D0: 4BEE8EC1  bl 0x82eed790
	ctx.lr = 0x830048D4;
	sub_82EED790(ctx, base);
	// 830048D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830048D8: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830048DC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830048E0: 4BFFFBF1  bl 0x830044d0
	ctx.lr = 0x830048E4;
	sub_830044D0(ctx, base);
	// 830048E4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830048E8: 48000019  bl 0x83004900
	ctx.lr = 0x830048EC;
	sub_83004900(ctx, base);
	// 830048EC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830048F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830048F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830048F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830048FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004900 size=148
    let mut pc: u32 = 0x83004900;
    'dispatch: loop {
        match pc {
            0x83004900 => {
    //   block [0x83004900..0x83004994)
	// 83004900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300490C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004910: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83004914: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004918: 48000089  bl 0x830049a0
	ctx.lr = 0x8300491C;
	sub_830049A0(ctx, base);
	// 8300491C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004920: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83004924: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004928: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300492C: 48000485  bl 0x83004db0
	ctx.lr = 0x83004930;
	sub_83004DB0(ctx, base);
	// 83004930: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83004934: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004938: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300493C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004940: 4BFFFA49  bl 0x83004388
	ctx.lr = 0x83004944;
	sub_83004388(ctx, base);
	// 83004944: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004948: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300494C: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004950: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004954: 480003ED  bl 0x83004d40
	ctx.lr = 0x83004958;
	sub_83004D40(ctx, base);
	// 83004958: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300495C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004960: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004964: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004968: 4BFFF9F1  bl 0x83004358
	ctx.lr = 0x8300496C;
	sub_83004358(ctx, base);
	// 8300496C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004970: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004974: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83004978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300497C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83004980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300498C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004998 size=8
    let mut pc: u32 = 0x83004998;
    'dispatch: loop {
        match pc {
            0x83004998 => {
    //   block [0x83004998..0x830049A0)
	// 83004998: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 8300499C: 8204C774  lwz r16, -0x388c(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-14476 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830049A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830049A0 size=288
    let mut pc: u32 = 0x830049A0;
    'dispatch: loop {
        match pc {
            0x830049A0 => {
    //   block [0x830049A0..0x83004AC0)
	// 830049A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830049A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830049A8: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830049AC: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830049B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830049B4: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830049B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830049BC: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 830049C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830049C4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830049C8: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830049CC: 4BFFFBC5  bl 0x83004590
	ctx.lr = 0x830049D0;
	sub_83004590(ctx, base);
	// 830049D0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830049D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830049D8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830049DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830049E0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830049E4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830049E8: 480001C1  bl 0x83004ba8
	ctx.lr = 0x830049EC;
	sub_83004BA8(ctx, base);
	// 830049EC: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 830049F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830049F4: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 830049F8: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830049FC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004A00: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83004A04: 480001B5  bl 0x83004bb8
	ctx.lr = 0x83004A08;
	sub_83004BB8(ctx, base);
	// 83004A08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83004A0C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004A10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83004A14: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83004A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83004A1C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83004A20: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004A24: 4800039D  bl 0x83004dc0
	ctx.lr = 0x83004A28;
	sub_83004DC0(ctx, base);
	// 83004A28: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83004A2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83004A30: 38BF005C  addi r5, r31, 0x5c
	ctx.r[5].s64 = ctx.r[31].s64 + 92;
	// 83004A34: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83004A38: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004A3C: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83004A40: 48000179  bl 0x83004bb8
	ctx.lr = 0x83004A44;
	sub_83004BB8(ctx, base);
	// 83004A44: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83004A48: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83004A4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83004A50: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83004A54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83004A58: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83004A5C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004A60: 4BEE3AD9  bl 0x82ee8538
	ctx.lr = 0x83004A64;
	sub_82EE8538(ctx, base);
	// 83004A64: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 83004A68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83004A6C: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 83004A70: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83004A74: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004A78: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83004A7C: 4800013D  bl 0x83004bb8
	ctx.lr = 0x83004A80;
	sub_83004BB8(ctx, base);
	// 83004A80: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83004A84: 48000004  b 0x83004a88
	pc = 0x83004A88; continue 'dispatch;
	// 83004A88: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004A8C: 48000345  bl 0x83004dd0
	ctx.lr = 0x83004A90;
	sub_83004DD0(ctx, base);
	// 83004A90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83004A94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004A98: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004A9C: 48000315  bl 0x83004db0
	ctx.lr = 0x83004AA0;
	sub_83004DB0(ctx, base);
	// 83004AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83004AA4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83004AA8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83004AAC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83004AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004AC0 size=8
    let mut pc: u32 = 0x83004AC0;
    'dispatch: loop {
        match pc {
            0x83004AC0 => {
    //   block [0x83004AC0..0x83004AC8)
	// 83004AC0: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 83004AC4: 8204C774  lwz r16, -0x388c(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-14476 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


